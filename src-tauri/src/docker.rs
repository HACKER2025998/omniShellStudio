//! Capture Docker via sa CLI locale : la connexion native au moteur reste gérée par Docker.
use crate::{classifier::redact, db, Result}; // Les captures sont filtrées et persistées.
use serde_json::{json, Value}; // Docker inspect fournit un format structuré.
use tokio::{
    process::Command,
    time::{timeout, Duration},
}; // Toutes les commandes externes ont une échéance.

// Aucun shell : les arguments ne peuvent pas injecter une autre commande.
async fn docker(args: &[&str]) -> Result<String> {
    let result = timeout(
        Duration::from_secs(15),
        Command::new("docker").args(args).output(),
    )
    .await??; // Un démon bloqué ne bloque pas l'interface indéfiniment.
    if !result.status.success() {
        return Err(
            "Docker indisponible ou accès refusé ; les snapshots restent consultables".into(),
        );
    } // Ne renvoie pas de secrets de stderr.
    Ok(String::from_utf8_lossy(&result.stdout).into_owned()) // Docker produit normalement de l'UTF-8.
}

// Une reconstruction est étiquetée comme telle : Docker n'enregistre pas la CLI d'origine.
pub async fn sync() -> Result<Value> {
    let ids = docker(&["ps", "-aq", "--no-trunc"]).await?; // Inclut les conteneurs arrêtés.
    let mut count = 0; // Nombre de captures actualisées.
    for id in ids.lines().take(1000) {
        let data: Value = serde_json::from_str(&docker(&["inspect", id]).await?)?; // Une lecture structurée par conteneur.
        let c = &data[0]; // docker inspect renvoie un tableau.
        let name = c["Name"].as_str().unwrap_or(id).trim_start_matches('/'); // Retire le slash Docker.
        let image = c["Config"]["Image"].as_str().unwrap_or("unknown"); // Image de création.
        let quote = |s: &str| shlex::try_quote(s).unwrap_or_default().into_owned(); // Les actions copiées gardent des arguments sûrs.
        let ports = c["HostConfig"]["PortBindings"].clone(); // Garde les bindings hôte explicites.
        let mut launch = format!("docker run --name {}", quote(name)); // Base minimale reconstruite.
        if let Some(map) = ports.as_object() {
            for (port, bindings) in map {
                if let Some(bindings) = bindings.as_array() {
                    for b in bindings {
                        launch.push_str(&format!(
                            " -p {}",
                            quote(&format!(
                                "{}:{}:{}",
                                b["HostIp"]
                                    .as_str()
                                    .filter(|s| !s.is_empty())
                                    .unwrap_or("0.0.0.0"),
                                b["HostPort"].as_str().unwrap_or(""),
                                port
                            ))
                        ));
                    }
                }
            }
        } // Reproduit les ports publiés.
        if let Some(mounts) = c["Mounts"].as_array() {
            for m in mounts {
                let source = m["Name"]
                    .as_str()
                    .filter(|s| !s.is_empty())
                    .or(m["Source"].as_str())
                    .unwrap_or("");
                launch.push_str(&format!(
                    " -v {}",
                    quote(&format!(
                        "{}:{}{}",
                        source,
                        m["Destination"].as_str().unwrap_or(""),
                        if m["RW"] == false { ":ro" } else { "" }
                    ))
                ));
            }
        } // Préserve volumes et lecture seule.
        launch.push_str(&format!(" {}", quote(image))); // Les options non récupérées sont documentées comme limites.
        let logs = docker(&["logs", "--tail", "200", id])
            .await
            .unwrap_or_else(|_| "Logs indisponibles".into()); // L'échec des logs n'annule pas la capture.
        let env = c["Config"]["Env"]
            .as_array()
            .map(|vars| {
                vars.iter()
                    .map(|v| redact(v.as_str().unwrap_or("")))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(); // Aucun environnement brut sur disque.
        let snapshot = json!({"id":id,"name":name,"image":image,"state":c["State"]["Status"],"ports":ports,"mounts":c["Mounts"],"networks":c["NetworkSettings"]["Networks"],"environment":env,"logs":redact(&logs),"reconstructed":launch,"origin":"reconstruction partielle, pas la commande exacte","actions":{"shell":format!("docker exec -it {} /bin/sh",quote(name)),"logs":format!("docker logs -f --tail 100 {}",quote(name)),"restart":format!("docker restart {}",quote(name)),"inspect":format!("docker inspect {}",quote(name))}}); // Toutes les actions sont uniquement copiables.
        db::snapshot("docker", id, &snapshot)?;
        count += 1; // Le cache reste disponible après arrêt du moteur.
    }
    Ok(json!({"updated":count,"snapshots":db::snapshots("docker")?})) // Aucun conteneur ancien n'est supprimé automatiquement.
}
