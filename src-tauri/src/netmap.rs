//! Découverte TCP bornée et preuves observées ; aucune topologie physique inventée.
use crate::{db, Result}; // Les graphes sont persistés pour consultation hors ligne.
use serde_json::{json, Value}; // Graphe simple exportable sans dépendance cloud.
use std::{net::Ipv4Addr, sync::Arc}; // Seuls les sous-réseaux IPv4 privés sont scannés activement.
// @ligne Importe les outils nécessaires à la mise en œuvre de la cartographie locale avec preuves.
use tokio::{
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
    net::TcpStream,
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
    sync::Semaphore,
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
    task::JoinSet,
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
    time::{timeout, Duration, Instant},
}; // Concurrence et durée de chaque sonde bornées.
// Énumère les interfaces réelles sans dépendre d'iproute2 ou de PowerShell.
pub fn interfaces() -> Result<Value> {
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la cartographie locale avec preuves.
    Ok(json!(if_addrs::get_if_addrs()?.into_iter().filter(|i|!i.is_loopback()).map(|i|json!({"name":i.name,"ip":i.ip().to_string(),"netmask":match i.addr {if_addrs::IfAddr::V4(a)=>a.netmask.to_string(),if_addrs::IfAddr::V6(a)=>a.netmask.to_string()}})).collect::<Vec<_>>()))
    // Aucune adresse fictive en démonstration.
}
// L'utilisateur fournit explicitement le /24 à sonder ; au maximum 254 hôtes.
pub async fn scan(cidr: &str) -> Result<Value> {
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la cartographie locale avec preuves.
    scan_with(cidr, &json!({})).await
} // La CLI conserve un mode sans privilèges.
// Les sondes avancées sont activées explicitement et conservent leur preuve d’observation.
pub async fn scan_with(cidr: &str, options: &Value) -> Result<Value> {
    // @ligne Prépare les valeurs locales pour la prochaine étape de la cartographie locale avec preuves.
    let (address, prefix) = cidr
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
        .split_once('/')
        .ok_or("Format attendu : 192.168.1.0/24")?; // Refuse les entrées ambiguës.
    let ip: Ipv4Addr = address.parse()?; // La bibliothèque standard valide chaque octet.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
    if prefix != "24" || !(ip.is_private() || ip.is_link_local()) {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la cartographie locale avec preuves.
        return Err("Le scan actif est limité aux réseaux IPv4 privés /24".into());
    } // Borne le périmètre et la charge réseau.
    let octets = ip.octets(); // Les trois premiers octets définissent le réseau.
    let gate = Arc::new(Semaphore::new(64)); // Ne lance jamais 2540 connexions simultanées.
    let mut tasks = JoinSet::new(); // Collecte aussi les échecs de tâches.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la cartographie locale avec preuves.
    for host in 1..=254u8 {
        let addr = Ipv4Addr::new(octets[0], octets[1], octets[2], host); // Chaque adresse est validée dès la construction.
        let gate = gate.clone(); // Toutes les tâches partagent la même limite.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
        tasks.spawn(async move {
            let _permit = gate.acquire_owned().await.ok()?; // Le permis est libéré à la fin de la tâche.
            let start = Instant::now(); // Latence totale des essais, pas un ping ICMP.
            // @ligne Prépare « ports » pour la prochaine étape de la cartographie locale avec preuves.
            let mut ports = Vec::new();
            let mut banners=serde_json::Map::new(); // Ne conserve que l’identification SSH ou Server HTTP, sans cookies. // Ne conserve que les ports réellement ouverts.
            // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la cartographie locale avec preuves.
            for port in [22,53,80,443,445,3306,5432,8006,8080,9000] {
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
                if let Ok(Ok(mut stream))=timeout(Duration::from_millis(180),TcpStream::connect((addr,port))).await {
                    ports.push(port); // Une connexion réussie constitue la preuve du port ouvert.
                    if [22,80,8080].contains(&port){use tokio::io::{AsyncReadExt,AsyncWriteExt};if port!=22{let request=format!("HEAD / HTTP/1.0\r\nHost: {addr}\r\n\r\n");let _=timeout(Duration::from_millis(100),stream.write_all(request.as_bytes())).await;}let mut bytes=[0;512];if let Ok(Ok(size))=timeout(Duration::from_millis(100),stream.read(&mut bytes)).await{let text=String::from_utf8_lossy(&bytes[..size]);let banner=if port==22{text.lines().next()}else{text.lines().find(|line|line.to_lowercase().starts_with("server:"))};if let Some(banner)=banner{banners.insert(port.to_string(),json!(crate::classifier::redact(banner)));}}} // Une bannière absente ne fait pas disparaître le port observé.
                // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
                }
            } // Timeout par tentative, aucun raw socket.
            if ports.is_empty() { None } else { Some(json!({"id":addr.to_string(),"ip":addr.to_string(),"ports":ports,"banners":banners,"probe_ms":start.elapsed().as_millis(),"evidence":"TCP connect","kind":if ports.contains(&5432)||ports.contains(&3306) {"database"} else if ports.contains(&80)||ports.contains(&443) {"web"} else {"host"}})) } // Un hôte silencieux n'est pas déclaré absent.
        // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
        });
    // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
    }
    let mut nodes = Vec::new(); // Graphe d'observations, pas de liens physiques supposés.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la cartographie locale avec preuves.
    while let Some(result) = tasks.join_next().await {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
        if let Ok(Some(node)) = result {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
            nodes.push(node);
        // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
        }
    } // Une tâche échouée ne détruit pas les autres observations.
    let passive = crate::network_protocols::passive().await; // Les tables du noyau restent accessibles sans raw sockets.
    let mut warnings = Vec::new(); // Une sonde facultative échouée ne détruit pas les observations déjà obtenues.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
    if let Some(neighbors) = passive["neighbors"].as_array() {
        // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la cartographie locale avec preuves.
        for neighbor in neighbors {
            // @ligne Prépare « address » pour la prochaine étape de la cartographie locale avec preuves.
            let address = neighbor["ip"].as_str().unwrap_or("");
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
            if let Ok(address) = address.parse::<Ipv4Addr>() {
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
                if address.octets()[..3] == octets[..3] {
                    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
                    if let Some(existing) = nodes.iter_mut().find(|n| n["ip"] == neighbor["ip"]) {
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                        existing["mac"] = neighbor["mac"].clone();
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                    } else {
                        // @ligne Prépare « node » pour la prochaine étape de la cartographie locale avec preuves.
                        let mut node = neighbor.clone();
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                        node["id"] = node["ip"].clone();
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                        node["kind"] = json!("host");
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                        node["ports"] = json!([]);
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                        nodes.push(node);
                    // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
                    }
                // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
                }
            // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
            }
        // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
        }
    } // Une entrée passive reste marquée présence inconnue.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
    if options["mdns"].as_bool().unwrap_or(true) {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
        match crate::network_protocols::mdns().await {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la cartographie locale avec preuves.
            Ok(data) => {
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
                if let Some(observed) = data.as_array() {
                    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la cartographie locale avec preuves.
                    for node in observed {
                        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
                        if let Some(address) =
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                            node["ip"].as_str().and_then(|s| s.parse::<Ipv4Addr>().ok())
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                        {
                            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
                            if address.octets()[..3] == octets[..3] {
                                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
                                if let Some(existing) =
                                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                                    nodes.iter_mut().find(|n| n["ip"] == node["ip"])
                                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                                {
                                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                                    existing["name"] = node["name"].clone();
                                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                                } else {
                                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                                    nodes.push(node.clone());
                                // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
                                }
                            // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
                            }
                        // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
                        }
                    // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
                    }
                // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
                }
            // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
            }
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la cartographie locale avec preuves.
            Err(_) => warnings.push(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                "mDNS indisponible : conserver le lien direct et les tables passives".to_owned(),
            // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
            ),
        // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
        }
    } // L'isolation multicast peut empêcher la découverte.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
    if options["raw"].as_bool().unwrap_or(false) {
        // @ligne Prépare « interface » pour la prochaine étape de la cartographie locale avec preuves.
        let interface = options["interface"].as_str().unwrap_or("").to_owned();
        // @ligne Prépare « target » pour la prochaine étape de la cartographie locale avec preuves.
        let target = cidr.to_owned();
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
        match tokio::task::spawn_blocking(move||crate::network_protocols::raw(&interface,&target)).await{Ok(Ok(observed))=>if let Some(observed)=observed.as_array(){for node in observed{if let Some(existing)=nodes.iter_mut().find(|n|n["id"]==node["id"]){existing["mac"]=node["mac"].clone();existing["evidence"]=node["evidence"].clone();existing["online"]=node["online"].clone();}else{nodes.push(node.clone());}}},_=>warnings.push("ERR_RAW_SOCKET_PERM : ARP/LLDP/CDP indisponibles ; repli TCP et tables du noyau conservés".into())}
    } // Aucun setcap n'est exécuté automatiquement.
    // @ligne Prépare « gateway » pour la prochaine étape de la cartographie locale avec preuves.
    let gateway = passive["gateway"]
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
        .as_str()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
        .and_then(|s| s.parse::<Ipv4Addr>().ok())
        .filter(|g| g.octets()[..3] == octets[..3]); // N'attribue pas une route d'une autre interface au sous-réseau demandé.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
    if let Some(gateway) = gateway {
        // @ligne Prépare « id » pour la prochaine étape de la cartographie locale avec preuves.
        let id = gateway.to_string();
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
        if let Some(node) = nodes.iter_mut().find(|n| n["id"] == id) {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
            node["kind"] = json!("gateway");
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
        } else {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
            nodes.insert(0,json!({"id":id,"ip":id,"kind":"gateway","ports":[],"evidence":"route par défaut du système"}));
        // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
        }
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
        if let Some(community) = options["snmp_community"].as_str().filter(|s| !s.is_empty()) {
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
            match crate::network_protocols::snmp(gateway, community).await {
                // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la cartographie locale avec preuves.
                Ok(data) => {
                    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la cartographie locale avec preuves.
                    if let Some(node) = nodes.iter_mut().find(|n| n["id"] == id) {
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                        node["name"] = data["name"].clone();
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                        node["description"] = data["description"].clone();
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                        node["snmp"] = json!(true);
                    // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
                    }
                // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
                }
                // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la cartographie locale avec preuves.
                Err(_) => warnings
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
                    .push("SNMP sans réponse : communauté, accès UDP ou service à vérifier".into()),
            // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
            }
        } // Le secret de communauté n'est jamais intégré au snapshot.
    // @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
    }
    crate::network_protocols::enrich_vendors(&mut nodes); // Enrichissement facultatif depuis un fichier OUI local vérifiable.
    nodes.sort_by_key(|n| n["id"].as_str().unwrap_or("").to_owned()); // Export stable pour comparer les scans.
    // @ligne Prépare « edges » pour la prochaine étape de la cartographie locale avec preuves.
    let edges = if let Some(gateway) = gateway {
        // @ligne Prépare « gateway » pour la prochaine étape de la cartographie locale avec preuves.
        let gateway = gateway.to_string();
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
        nodes.iter().filter(|n|n["id"]!=gateway).map(|n|json!({"source":gateway,"target":n["id"],"kind":"logical","evidence":"appartenance au même sous-réseau ; câblage non établi"})).collect::<Vec<_>>()
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
    } else {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
        Vec::new()
    }; // Un lien logique n'est jamais présenté comme un câble mesuré.
    // @ligne Prépare « graph » pour la prochaine étape de la cartographie locale avec preuves.
    let graph = json!({"cidr":cidr,"nodes":nodes,"edges":edges,"scanned_at":db::now(),"gateway":gateway.map(|g|g.to_string()),"warnings":warnings,"method":"TCP + tables système + mDNS ; ARP/LLDP/CDP et SNMP facultatifs","notice":"Les voisins et services sont observés. Les liens pointillés sont logiques ; un équipement silencieux peut rester inconnu. Les champs non observés ne sont pas inventés."});
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
    db::snapshot("netmap", cidr, &graph)?;
    Ok(graph) // Conserve le dernier graphe par sous-réseau.
// @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
}
// La collecte WAN est une action séparée : aucune requête Internet n'est émise par un scan local.
pub async fn wan() -> Result<Value> {
    // @ligne Prépare « client » pour la prochaine étape de la cartographie locale avec preuves.
    let client = reqwest::Client::builder()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
        .timeout(Duration::from_secs(8))
        .build()?; // Le service externe reste facultatif et borné.
    // @ligne Prépare « data » pour la prochaine étape de la cartographie locale avec preuves.
    let data: Value = client
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
        .get("https://ipapi.co/json/")
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la cartographie locale avec preuves.
        .send()
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la cartographie locale avec preuves.
        .await?
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
        .error_for_status()?
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
        .json()
        .await?; // Le fournisseur voit l'IP publique de cette requête volontaire.
    // @ligne Prépare « wan » pour la prochaine étape de la cartographie locale avec preuves.
    let wan = json!({"ip":data["ip"],"isp":data["org"],"asn":data["asn"],"evidence":"réponse HTTPS ipapi.co","observed_at":db::now()});
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
    db::snapshot("wan", "last", &wan)?;
    Ok(wan) // La dernière observation reste consultable hors ligne.
// @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
}
#[cfg(test)] // Les bornes sont vérifiées avant toute émission de trafic.
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la cartographie locale avec preuves.
mod tests {
    use super::*; // Ne nécessite aucun réseau accessible.
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant de la cartographie locale avec preuves.
    #[tokio::test]
    // @ligne Déclare « netmap », le point d’entrée de cette opération de la cartographie locale avec preuves.
    async fn rejects_unbounded_targets() {
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la cartographie locale avec preuves.
        assert!(scan("8.8.8.0/24").await.is_err());
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la cartographie locale avec preuves.
        assert!(scan("192.168.0.0/16").await.is_err());
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la cartographie locale avec preuves.
        assert!(scan("invalid").await.is_err());
    } // Un CIDR public ou trop large ne doit pas passer.
// @ligne Termine cette construction et revient au contexte parent de la cartographie locale avec preuves.
}
