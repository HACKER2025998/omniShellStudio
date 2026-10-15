//! Découverte TCP bornée et preuves observées ; aucune topologie physique inventée.
use crate::{db, Result}; // Les graphes sont persistés pour consultation hors ligne.
use serde_json::{json, Value}; // Graphe simple exportable sans dépendance cloud.
use std::{net::Ipv4Addr, sync::Arc}; // Seuls les sous-réseaux IPv4 privés sont scannés activement.
use tokio::{
    net::TcpStream,
    sync::Semaphore,
    task::JoinSet,
    time::{timeout, Duration, Instant},
}; // Concurrence et durée de chaque sonde bornées.

// Énumère les interfaces réelles sans dépendre d'iproute2 ou de PowerShell.
pub fn interfaces() -> Result<Value> {
    Ok(json!(if_addrs::get_if_addrs()?.into_iter().filter(|i|!i.is_loopback()).map(|i|json!({"name":i.name,"ip":i.ip().to_string(),"netmask":match i.addr {if_addrs::IfAddr::V4(a)=>a.netmask.to_string(),if_addrs::IfAddr::V6(a)=>a.netmask.to_string()}})).collect::<Vec<_>>()))
    // Aucune adresse fictive en démonstration.
}

// L'utilisateur fournit explicitement le /24 à sonder ; au maximum 254 hôtes.
pub async fn scan(cidr: &str) -> Result<Value> {
    let (address, prefix) = cidr
        .split_once('/')
        .ok_or("Format attendu : 192.168.1.0/24")?; // Refuse les entrées ambiguës.
    let ip: Ipv4Addr = address.parse()?; // La bibliothèque standard valide chaque octet.
    if prefix != "24" || !(ip.is_private() || ip.is_link_local()) {
        return Err("Le scan actif est limité aux réseaux IPv4 privés /24".into());
    } // Borne le périmètre et la charge réseau.
    let octets = ip.octets(); // Les trois premiers octets définissent le réseau.
    let gate = Arc::new(Semaphore::new(64)); // Ne lance jamais 2540 connexions simultanées.
    let mut tasks = JoinSet::new(); // Collecte aussi les échecs de tâches.
    for host in 1..=254u8 {
        let addr = Ipv4Addr::new(octets[0], octets[1], octets[2], host); // Chaque adresse est validée dès la construction.
        let gate = gate.clone(); // Toutes les tâches partagent la même limite.
        tasks.spawn(async move {
            let _permit = gate.acquire_owned().await.ok()?; // Le permis est libéré à la fin de la tâche.
            let start = Instant::now(); // Latence totale des essais, pas un ping ICMP.
            let mut ports = Vec::new(); // Ne conserve que les ports réellement ouverts.
            for port in [22,53,80,443,445,3306,5432,8006,8080,9000] { if let Ok(Ok(_)) = timeout(Duration::from_millis(180),TcpStream::connect((addr,port))).await { ports.push(port); } } // Timeout par tentative, aucun raw socket.
            if ports.is_empty() { None } else { Some(json!({"id":addr.to_string(),"ip":addr.to_string(),"ports":ports,"probe_ms":start.elapsed().as_millis(),"evidence":"TCP connect","kind":if ports.contains(&5432)||ports.contains(&3306) {"database"} else if ports.contains(&80)||ports.contains(&443) {"web"} else {"host"}})) } // Un hôte silencieux n'est pas déclaré absent.
        });
    }
    let mut nodes = Vec::new(); // Graphe d'observations, pas de liens physiques supposés.
    while let Some(result) = tasks.join_next().await {
        if let Ok(Some(node)) = result {
            nodes.push(node);
        }
    } // Une tâche échouée ne détruit pas les autres observations.
    nodes.sort_by_key(|n| n["ip"].as_str().unwrap_or("").to_owned()); // Export stable pour comparer les scans.
    let graph = json!({"cidr":cidr,"nodes":nodes,"edges":[],"scanned_at":db::now(),"method":"TCP non privilégié","notice":"Les hôtes sans port ouvert, switches, FAI et liens physiques ne sont pas déductibles de ces sondes."}); // Rend les limites visibles dans l'interface.
    db::snapshot("netmap", cidr, &graph)?;
    Ok(graph) // Conserve le dernier graphe par sous-réseau.
}

#[cfg(test)] // Les bornes sont vérifiées avant toute émission de trafic.
mod tests {
    use super::*; // Ne nécessite aucun réseau accessible.
    #[tokio::test]
    async fn rejects_unbounded_targets() {
        assert!(scan("8.8.8.0/24").await.is_err());
        assert!(scan("192.168.0.0/16").await.is_err());
        assert!(scan("invalid").await.is_err());
    } // Un CIDR public ou trop large ne doit pas passer.
}
