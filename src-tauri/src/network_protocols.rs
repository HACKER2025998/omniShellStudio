//! Découverte multi-couches : chaque observation conserve sa preuve et ses limites.
use crate::Result; // Les permissions manquantes produisent un repli, jamais une élévation implicite.
use serde_json::{json, Value}; // Les protocoles rejoignent un graphe exportable unique.
// @ligne Importe les outils nécessaires à la mise en œuvre des protocoles réseau bornés.
use std::{
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    net::Ipv4Addr,
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    time::{Duration, Instant},
}; // Toutes les écoutes et sondes possèdent une échéance.
// Les routes et voisins sont lus via les outils natifs ; aucun texte utilisateur n'est interpolé dans un shell.
async fn system(program: &str, args: &[&str]) -> Option<String> {
    // @ligne Prépare « output » pour la prochaine étape des protocoles réseau bornés.
    let output = tokio::time::timeout(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        Duration::from_secs(3),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        tokio::process::Command::new(program)
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            .args(args)
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            .kill_on_drop(true)
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            .output(),
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    )
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours des protocoles réseau bornés.
    .await
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    .ok()?
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    .ok()?;
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    output
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .status
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .success()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
// @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
}
// Les tables du noyau fournissent une preuve passive, même sans CAP_NET_RAW.
pub async fn passive() -> Value {
    // @ligne Prépare « gateway » pour la prochaine étape des protocoles réseau bornés.
    let mut gateway = None;
    let mut neighbors = Vec::new(); // Une information absente reste inconnue.
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant des protocoles réseau bornés.
    #[cfg(target_os = "linux")]
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        if let Some(raw) = system("ip", &["-j", "route", "show", "default"]).await {
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
            if let Ok(rows) = serde_json::from_str::<Value>(&raw) {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                gateway = rows
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                    .as_array()
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                    .and_then(|r| r.first())
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                    .and_then(|r| r["gateway"].as_str())
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                    .map(str::to_owned);
            // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
            }
        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
        }
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        if gateway.is_none() {
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
            if let Ok(routes) = std::fs::read_to_string("/proc/net/route") {
                // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des protocoles réseau bornés.
                for line in routes.lines().skip(1) {
                    // @ligne Prépare « fields » pour la prochaine étape des protocoles réseau bornés.
                    let fields = line.split_whitespace().collect::<Vec<_>>();
                    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
                    if fields.get(1) == Some(&"00000000") {
                        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
                        if let Some(Ok(raw)) = fields.get(2).map(|v| u32::from_str_radix(v, 16)) {
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                            gateway = Some(Ipv4Addr::from(raw.to_le_bytes()).to_string());
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                            break;
                        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
                        }
                    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
                    }
                // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
                }
            // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
            }
        } // Le noyau expose la passerelle en petit-boutiste.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        if let Ok(arp) = std::fs::read_to_string("/proc/net/arp") {
            // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des protocoles réseau bornés.
            for line in arp.lines().skip(1) {
                // @ligne Prépare « f » pour la prochaine étape des protocoles réseau bornés.
                let f = line.split_whitespace().collect::<Vec<_>>();
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
                if f.len() >= 6 && f[3] != "00:00:00:00:00:00" {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                    neighbors.push(json!({"ip":f[0],"mac":f[3],"interface":f[5],"evidence":"table ARP noyau","online":null}));
                // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
                }
            // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
            }
        } // Une entrée de cache n'est pas une preuve de présence actuelle.
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant des protocoles réseau bornés.
    #[cfg(target_os = "macos")]
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        if let Some(raw) = system("route", &["-n", "get", "default"]).await {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            gateway = raw.lines().find_map(|l| {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                l.trim()
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                    .strip_prefix("gateway:")
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                    .map(|v| v.trim().to_owned())
            // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
            });
        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
        }
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        if let Some(raw) = system("arp", &["-an"]).await {
            // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des protocoles réseau bornés.
            for line in raw.lines() {
                // @ligne Prépare « fields » pour la prochaine étape des protocoles réseau bornés.
                let fields = line.split_whitespace().collect::<Vec<_>>();
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
                if fields.len() > 3 {
                    // @ligne Prépare « ip » pour la prochaine étape des protocoles réseau bornés.
                    let ip = fields[1].trim_matches(['(', ')']);
                    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
                    if ip.parse::<Ipv4Addr>().is_ok() {
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                        neighbors.push(json!({"ip":ip,"mac":fields[3],"evidence":"table ARP macOS","online":null}));
                    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
                    }
                // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
                }
            // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
            }
        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
        }
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant des protocoles réseau bornés.
    #[cfg(windows)]
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        if let Some(raw)=system("powershell",&["-NoProfile","-NonInteractive","-Command","Get-NetRoute -DestinationPrefix '0.0.0.0/0' | Sort-Object RouteMetric | Select-Object -First 1 -ExpandProperty NextHop"]).await{gateway=Some(raw.trim().to_owned());}
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        if let Some(raw)=system("powershell",&["-NoProfile","-NonInteractive","-Command","ConvertTo-Json -InputObject @(Get-NetNeighbor -AddressFamily IPv4 | Select-Object IPAddress,LinkLayerAddress,InterfaceAlias)"]).await{if let Ok(rows)=serde_json::from_str::<Vec<Value>>(&raw){neighbors=rows.into_iter().map(|r|json!({"ip":r["IPAddress"],"mac":r["LinkLayerAddress"],"interface":r["InterfaceAlias"],"evidence":"table voisins Windows","online":null})).collect();}}
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
    json!({"gateway":gateway,"neighbors":neighbors}) // Aucun fabricant ou système d'exploitation n'est deviné sans preuve.
// @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
}
// Les champs TLV sont validés avant lecture ; une trame tronquée n'est jamais indexée hors limites.
pub fn lldp(payload: &[u8]) -> Option<Value> {
    // @ligne Prépare les valeurs locales pour la prochaine étape des protocoles réseau bornés.
    let (mut offset, mut name, mut port, mut description, mut vlan) =
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        (0, String::new(), String::new(), String::new(), None);
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des protocoles réseau bornés.
    while offset + 2 <= payload.len() {
        // @ligne Prépare « header » pour la prochaine étape des protocoles réseau bornés.
        let header = u16::from_be_bytes([payload[offset], payload[offset + 1]]);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        offset += 2;
        // @ligne Prépare « kind » pour la prochaine étape des protocoles réseau bornés.
        let kind = header >> 9;
        // @ligne Prépare « length » pour la prochaine étape des protocoles réseau bornés.
        let length = (header & 511) as usize;
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        if offset + length > payload.len() {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des protocoles réseau bornés.
            return None;
        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
        }
        // @ligne Prépare « data » pour la prochaine étape des protocoles réseau bornés.
        let data = &payload[offset..offset + length];
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        offset += length;
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        match kind {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            0 => break,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            2 if !data.is_empty() => port = String::from_utf8_lossy(&data[1..]).into_owned(),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            5 => name = String::from_utf8_lossy(data).into_owned(),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            6 => description = String::from_utf8_lossy(data).into_owned(),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            127 if data.len() >= 6 && data[..4] == [0, 128, 194, 1] => {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                vlan = Some(u16::from_be_bytes([data[4], data[5]]))
            // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
            }
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            _ => {}
        } // Les TLV inconnus restent ignorables.
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
    if name.is_empty() && port.is_empty() {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        None
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    } else {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        Some(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            json!({"name":name,"port":port,"description":description,"vlan":vlan,"evidence":"LLDP reçu","kind":"switch"}),
        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
        )
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
// @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
}
// CDP est reconnu par l'en-tête LLC/SNAP Cisco avant d'analyser ses TLV.
pub fn cdp(payload: &[u8]) -> Option<Value> {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
    if payload.len() < 12 || payload[..8] != [170, 170, 3, 0, 0, 12, 32, 0] {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des protocoles réseau bornés.
        return None;
    } // Pas de décodage CDP sur un paquet arbitraire.
    // @ligne Prépare les valeurs locales pour la prochaine étape des protocoles réseau bornés.
    let (mut offset, mut name, mut port, mut platform) =
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        (12, String::new(), String::new(), String::new());
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des protocoles réseau bornés.
    while offset + 4 <= payload.len() {
        // @ligne Prépare « kind » pour la prochaine étape des protocoles réseau bornés.
        let kind = u16::from_be_bytes([payload[offset], payload[offset + 1]]);
        // @ligne Prépare « len » pour la prochaine étape des protocoles réseau bornés.
        let len = u16::from_be_bytes([payload[offset + 2], payload[offset + 3]]) as usize;
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        if len < 4 || offset + len > payload.len() {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des protocoles réseau bornés.
            return None;
        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
        }
        // @ligne Prépare « text » pour la prochaine étape des protocoles réseau bornés.
        let text = String::from_utf8_lossy(&payload[offset + 4..offset + len]).into_owned();
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        match kind {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            1 => name = text,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            3 => port = text,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            6 => platform = text,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            _ => {}
        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
        }
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        offset += len;
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    (!name.is_empty()).then(||json!({"name":name,"port":port,"description":platform,"evidence":"CDP reçu","kind":"switch"}))
    // Une annonce Cisco constitue une preuve, pas une simple supposition de marque.
}
// L'ARP actif reste facultatif et limité au même /24 que l'interface sélectionnée.
#[cfg(unix)]
// @ligne Déclare « raw », le point d’entrée de cette opération des protocoles réseau bornés.
pub fn raw(interface_name: &str, cidr: &str) -> Result<Value> {
    // @ligne Importe les outils nécessaires à la mise en œuvre des protocoles réseau bornés.
    use pnet::{
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        datalink::{self, Channel, Config},
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        packet::{
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket},
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket},
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            MutablePacket, Packet,
        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
        },
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        util::MacAddr,
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    };
    // @ligne Prépare « interface » pour la prochaine étape des protocoles réseau bornés.
    let interface = datalink::interfaces()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .into_iter()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .find(|i| i.name == interface_name)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .ok_or("Interface inconnue")?;
    // @ligne Prépare « local » pour la prochaine étape des protocoles réseau bornés.
    let local = interface
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .ips
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .iter()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .find_map(|n| match n.ip() {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            std::net::IpAddr::V4(ip) => Some(ip),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            _ => None,
        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
        })
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .ok_or("Interface sans IPv4")?;
    // @ligne Prépare « network » pour la prochaine étape des protocoles réseau bornés.
    let network: Ipv4Addr = cidr.split('/').next().ok_or("CIDR invalide")?.parse()?;
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
    if local.octets()[..3] != network.octets()[..3] {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des protocoles réseau bornés.
        return Err("L'interface et le /24 doivent correspondre pour l'ARP actif".into());
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
    // @ligne Prépare « mac » pour la prochaine étape des protocoles réseau bornés.
    let mac = interface.mac.ok_or("Adresse MAC indisponible")?;
    // @ligne Prépare « config » pour la prochaine étape des protocoles réseau bornés.
    let config = Config {
        // @ligne Renseigne le champ « read_timeout » de la structure utilisée ici des protocoles réseau bornés.
        read_timeout: Some(Duration::from_millis(150)),
        // @ligne Renseigne le champ « write_timeout » de la structure utilisée ici des protocoles réseau bornés.
        write_timeout: Some(Duration::from_millis(150)),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        ..Default::default()
    }; // Ne bloque pas indéfiniment l'écoute passive.
    // @ligne Prépare les valeurs locales pour la prochaine étape des protocoles réseau bornés.
    let (mut tx, mut rx) = match datalink::channel(&interface, config)? {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        Channel::Ethernet(tx, rx) => (tx, rx),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        _ => return Err("Canal Ethernet indisponible".into()),
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    };
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des protocoles réseau bornés.
    for host in 1..=254u8 {
        // @ligne Prépare « octets » pour la prochaine étape des protocoles réseau bornés.
        let octets = network.octets();
        // @ligne Prépare « target » pour la prochaine étape des protocoles réseau bornés.
        let target = Ipv4Addr::new(octets[0], octets[1], octets[2], host);
        // @ligne Prépare « frame » pour la prochaine étape des protocoles réseau bornés.
        let mut frame = [0u8; 42];
        // @ligne Prépare « ethernet » pour la prochaine étape des protocoles réseau bornés.
        let mut ethernet =
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            MutableEthernetPacket::new(&mut frame).ok_or("Tampon Ethernet invalide")?;
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        ethernet.set_destination(MacAddr::broadcast());
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        ethernet.set_source(mac);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        ethernet.set_ethertype(EtherTypes::Arp);
        // @ligne Prépare « arp » pour la prochaine étape des protocoles réseau bornés.
        let mut arp = MutableArpPacket::new(ethernet.payload_mut()).ok_or("Tampon ARP invalide")?;
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        arp.set_hardware_type(ArpHardwareTypes::Ethernet);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        arp.set_protocol_type(EtherTypes::Ipv4);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        arp.set_hw_addr_len(6);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        arp.set_proto_addr_len(4);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        arp.set_operation(ArpOperations::Request);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        arp.set_sender_hw_addr(mac);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        arp.set_sender_proto_addr(local);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        arp.set_target_hw_addr(MacAddr::zero());
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        arp.set_target_proto_addr(target);
        // @ligne Prépare « _ » pour la prochaine étape des protocoles réseau bornés.
        let _ = tx.send_to(&frame, None);
    } // Un seul sweep /24, sans scan SYN privilégié.
    // @ligne Prépare « deadline » pour la prochaine étape des protocoles réseau bornés.
    let deadline = Instant::now() + Duration::from_secs(3);
    // @ligne Prépare « observations » pour la prochaine étape des protocoles réseau bornés.
    let mut observations = std::collections::BTreeMap::new();
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des protocoles réseau bornés.
    while Instant::now() < deadline {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        if let Ok(bytes) = rx.next() {
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
            if let Some(frame) = EthernetPacket::new(bytes) {
                // @ligne Prépare « source » pour la prochaine étape des protocoles réseau bornés.
                let source = frame.get_source().to_string();
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
                if frame.get_ethertype() == EtherTypes::Arp {
                    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
                    if let Some(arp) = ArpPacket::new(frame.payload()) {
                        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
                        if arp.get_operation() == ArpOperations::Reply {
                            // @ligne Prépare « ip » pour la prochaine étape des protocoles réseau bornés.
                            let ip = arp.get_sender_proto_addr().to_string();
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                            observations.insert(ip.clone(),json!({"id":ip,"ip":ip,"mac":arp.get_sender_hw_addr().to_string(),"kind":"host","ports":[],"evidence":"réponse ARP","online":true}));
                        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
                        }
                    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
                    }
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                } else if frame.get_ethertype() == EtherTypes::Lldp {
                    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
                    if let Some(mut node) = lldp(frame.payload()) {
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                        node["id"] = json!(source);
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                        node["mac"] = node["id"].clone();
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                        observations.insert(node["id"].as_str().unwrap_or("").into(), node);
                    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
                    }
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                } else if let Some(mut node) = cdp(frame.payload()) {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                    node["id"] = json!(source);
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                    node["mac"] = node["id"].clone();
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                    observations.insert(node["id"].as_str().unwrap_or("").into(), node);
                // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
                }
            // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
            }
        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
        }
    } // Les droits bruts manquants sont traités par l'appelant avec un repli TCP.
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des protocoles réseau bornés.
    Ok(json!(observations.into_values().collect::<Vec<_>>()))
// @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
}
// Les systèmes sans accès datalink brut conservent les sondes TCP et les tables natives.
#[cfg(not(unix))]
// @ligne Déclare « raw », le point d’entrée de cette opération des protocoles réseau bornés.
pub fn raw(_interface_name: &str, _cidr: &str) -> Result<Value> {
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des protocoles réseau bornés.
    Err("Capture brute non disponible sans pilote dédié sur cette plateforme ; repli non privilégié actif".into())
// @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
}
// La résolution mDNS ajoute les noms et services annoncés par les appareils eux-mêmes.
pub async fn mdns() -> Result<Value> {
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours des protocoles réseau bornés.
    tokio::task::spawn_blocking(||->Result<Value>{let daemon=mdns_sd::ServiceDaemon::new()?;let receiver=daemon.browse("_workstation._tcp.local.")?;let deadline=Instant::now()+Duration::from_secs(2);let mut nodes=Vec::new();while Instant::now()<deadline{if let Ok(mdns_sd::ServiceEvent::ServiceResolved(s))=receiver.recv_timeout(Duration::from_millis(100)){for ip in s.get_addresses(){if ip.is_ipv4(){nodes.push(json!({"id":ip.to_string(),"ip":ip.to_string(),"name":s.get_hostname(),"kind":"host","ports":[s.get_port()],"evidence":"mDNS annoncé"}));}}}}let _=daemon.shutdown();Ok(json!(nodes))}).await?
// @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
}
// L'encodage BER est volontairement borné aux messages SNMPv2c de lecture.
fn tlv(kind: u8, data: &[u8]) -> Vec<u8> {
    // @ligne Prépare « out » pour la prochaine étape des protocoles réseau bornés.
    let mut out = vec![kind];
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
    if data.len() < 128 {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        out.push(data.len() as u8);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    } else {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        out.extend([130, (data.len() >> 8) as u8, data.len() as u8]);
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    out.extend(data);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    out
// @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
}
// Les OID utilisés sont sysDescr, sysName et la table de forwarding des bridges.
fn oid(value: &[u8]) -> Vec<u8> {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    tlv(6, value)
// @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
}
// Un lecteur BER refuse les longueurs indéfinies, excessives et les paquets tronqués.
fn fields(data: &[u8]) -> Option<Vec<(u8, &[u8])>> {
    // @ligne Prépare les valeurs locales pour la prochaine étape des protocoles réseau bornés.
    let (mut offset, mut result) = (0, Vec::new());
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des protocoles réseau bornés.
    while offset < data.len() {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        if offset + 2 > data.len() {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des protocoles réseau bornés.
            return None;
        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
        }
        // @ligne Prépare « kind » pour la prochaine étape des protocoles réseau bornés.
        let kind = data[offset];
        // @ligne Prépare « len » pour la prochaine étape des protocoles réseau bornés.
        let mut len = data[offset + 1] as usize;
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        offset += 2;
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        if len & 128 != 0 {
            // @ligne Prépare « count » pour la prochaine étape des protocoles réseau bornés.
            let count = len & 127;
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
            if count == 0 || count > 4 || offset + count > data.len() {
                // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des protocoles réseau bornés.
                return None;
            // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
            }
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            len = 0;
            // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des protocoles réseau bornés.
            for b in &data[offset..offset + count] {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                len = (len << 8) | *b as usize;
            // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
            }
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
            offset += count;
        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
        }
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        if offset.checked_add(len)? > data.len() {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des protocoles réseau bornés.
            return None;
        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
        }
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        result.push((kind, &data[offset..offset + len]));
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        offset += len;
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    Some(result)
// @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
}
// La communauté est fournie explicitement et n'est jamais ajoutée au graphe ou aux logs.
pub async fn snmp(target: Ipv4Addr, community: &str) -> Result<Value> {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
    if community.is_empty() || community.len() > 128 {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des protocoles réseau bornés.
        return Err("Communauté SNMP vide ou trop longue".into());
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
    let request_id = (rand::random::<u32>() & 0x7fffffff).to_be_bytes(); // Identifiant vérifié dans la réponse associée au socket connecté.
    // @ligne Prépare « varbinds » pour la prochaine étape des protocoles réseau bornés.
    let mut varbinds = Vec::new();
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des protocoles réseau bornés.
    for suffix in [1, 5] {
        // @ligne Prépare « name » pour la prochaine étape des protocoles réseau bornés.
        let mut name = vec![43, 6, 1, 2, 1, 1, suffix, 0];
        // @ligne Prépare « pair » pour la prochaine étape des protocoles réseau bornés.
        let mut pair = oid(&name);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        pair.extend(tlv(5, &[]));
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        varbinds.extend(tlv(48, &pair));
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        name.clear();
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
    // @ligne Prépare « pdu » pour la prochaine étape des protocoles réseau bornés.
    let mut pdu = tlv(2, &request_id);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    pdu.extend(tlv(2, &[0]));
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    pdu.extend(tlv(2, &[0]));
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    pdu.extend(tlv(48, &varbinds));
    // @ligne Prépare « message » pour la prochaine étape des protocoles réseau bornés.
    let mut message = tlv(2, &[1]);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    message.extend(tlv(4, community.as_bytes()));
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    message.extend(tlv(160, &pdu));
    // @ligne Prépare « socket » pour la prochaine étape des protocoles réseau bornés.
    let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours des protocoles réseau bornés.
    socket.connect((target, 161)).await?;
    socket.send(&tlv(48, &message)).await?; // Le noyau n'accepte que le correspondant choisi.
    // @ligne Prépare « bytes » pour la prochaine étape des protocoles réseau bornés.
    let mut bytes = [0u8; 65535];
    // @ligne Prépare « length » pour la prochaine étape des protocoles réseau bornés.
    let length =
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours des protocoles réseau bornés.
        tokio::time::timeout(Duration::from_millis(1200), socket.recv(&mut bytes)).await??;
    // @ligne Prépare « outer » pour la prochaine étape des protocoles réseau bornés.
    let outer = fields(&bytes[..length]).ok_or("Réponse SNMP invalide")?;
    // @ligne Prépare « body » pour la prochaine étape des protocoles réseau bornés.
    let body = fields(outer.first().ok_or("SNMP vide")?.1).ok_or("Message SNMP invalide")?;
    // @ligne Prépare « response » pour la prochaine étape des protocoles réseau bornés.
    let response = fields(body.get(2).ok_or("PDU absent")?.1).ok_or("PDU invalide")?;
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
    if response.first().map(|f| f.1) != Some(request_id.as_slice()) {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des protocoles réseau bornés.
        return Err("Identifiant SNMP inattendu".into());
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
    // @ligne Prépare « values » pour la prochaine étape des protocoles réseau bornés.
    let mut values = Vec::new();
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
    if let Some((_, list)) = response.get(3) {
        // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des protocoles réseau bornés.
        for (_, pair) in fields(list).ok_or("Varbinds invalides")? {
            // @ligne Prépare « pair » pour la prochaine étape des protocoles réseau bornés.
            let pair = fields(pair).ok_or("Varbind invalide")?;
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
            if let Some((4, text)) = pair.get(1) {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                values.push(String::from_utf8_lossy(text).into_owned());
            // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
            }
        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
        }
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des protocoles réseau bornés.
    Ok(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        json!({"id":target.to_string(),"ip":target.to_string(),"description":values.first(),"name":values.get(1),"evidence":"SNMPv2c sysDescr/sysName","ports":[161],"kind":"network-device"}),
    ) // Ne déclare pas un switch si sa nature n'est pas établie.
// @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
}
// Un fichier OUI IEEE fourni par le mainteneur peut être importé une fois puis utilisé entièrement hors ligne.
pub fn import_oui(path: &str) -> Result<Value> {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
    if std::fs::metadata(path)?.len() > 16 * 1024 * 1024 {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des protocoles réseau bornés.
        return Err("Fichier OUI limité à 16 Mio".into());
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
    // @ligne Prépare « reader » pour la prochaine étape des protocoles réseau bornés.
    let mut reader = csv::Reader::from_path(path)?;
    // @ligne Prépare « headers » pour la prochaine étape des protocoles réseau bornés.
    let headers = reader.headers()?.clone();
    // @ligne Prépare « prefix » pour la prochaine étape des protocoles réseau bornés.
    let prefix = headers
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .iter()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .position(|h| h == "Assignment")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .ok_or("Colonne IEEE Assignment absente")?;
    // @ligne Prépare « organization » pour la prochaine étape des protocoles réseau bornés.
    let organization = headers
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .iter()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .position(|h| h == "Organization Name")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .ok_or("Colonne IEEE Organization Name absente")?;
    // @ligne Prépare « map » pour la prochaine étape des protocoles réseau bornés.
    let mut map = std::collections::BTreeMap::new();
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des protocoles réseau bornés.
    for row in reader.records() {
        // @ligne Prépare « row » pour la prochaine étape des protocoles réseau bornés.
        let row = row?;
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        if let (Some(prefix), Some(name)) = (row.get(prefix), row.get(organization)) {
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
            if prefix.len() == 6 && prefix.chars().all(|c| c.is_ascii_hexdigit()) {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                map.insert(prefix.to_uppercase(), name.to_owned());
            // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
            }
        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
        }
    } // Les préfixes longs MA-M/MA-S ne sont pas assimilés à un OUI de 24 bits.
    // @ligne Prépare « count » pour la prochaine étape des protocoles réseau bornés.
    let count = map.len();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
    crate::db::snapshot("oui", "vendors", &json!(map))?;
    Ok(json!({"vendors":count})) // Les licences et l'origine du fichier restent celles de son fournisseur.
// @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
}
// Une résolution absente demeure inconnue ; le logiciel ne devine jamais une marque.
pub fn enrich_vendors(nodes: &mut [Value]) {
    // @ligne Prépare « Ok » pour la prochaine étape des protocoles réseau bornés.
    let Ok(cache) = crate::db::snapshots("oui") else {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des protocoles réseau bornés.
        return;
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    };
    // @ligne Prépare « Some » pour la prochaine étape des protocoles réseau bornés.
    let Some(map) = cache
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .as_array()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .and_then(|c| c.first())
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        .and_then(|c| c["data"].as_object())
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
    else {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des protocoles réseau bornés.
        return;
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    };
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des protocoles réseau bornés.
    for node in nodes {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
        if let Some(mac) = node["mac"].as_str() {
            // @ligne Prépare « prefix » pour la prochaine étape des protocoles réseau bornés.
            let prefix = mac
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                .chars()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                .filter(|c| c.is_ascii_hexdigit())
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                .take(6)
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                .collect::<String>()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                .to_uppercase();
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des protocoles réseau bornés.
            if let Some(vendor) = map.get(&prefix) {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
                node["vendor"] = vendor.clone();
            // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
            }
        // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
        }
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
// @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
}
#[cfg(test)] // Des trames contrôlées vérifient les parseurs sans privilèges réseau.
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
mod tests {
    // @ligne Importe les outils nécessaires à la mise en œuvre des protocoles réseau bornés.
    use super::*;
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant des protocoles réseau bornés.
    #[test]
    // @ligne Déclare « network_protocols », le point d’entrée de cette opération des protocoles réseau bornés.
    fn lldp_bounds() {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        assert!(lldp(&[10, 99, 1]).is_none());
        // @ligne Prépare « name » pour la prochaine étape des protocoles réseau bornés.
        let name = b"switch-lab";
        // @ligne Prépare « data » pour la prochaine étape des protocoles réseau bornés.
        let mut data = vec![10, name.len() as u8];
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        data.extend(name);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        data.extend([0, 0]);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        assert_eq!(lldp(&data).unwrap()["name"], "switch-lab");
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant des protocoles réseau bornés.
    #[test]
    // @ligne Déclare « network_protocols », le point d’entrée de cette opération des protocoles réseau bornés.
    fn ber_rejects_truncated() {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        assert!(fields(&[48, 130, 255, 255, 1]).is_none());
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des protocoles réseau bornés.
        assert!(fields(&[48, 128]).is_none());
    // @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
    }
// @ligne Termine cette construction et revient au contexte parent des protocoles réseau bornés.
}
