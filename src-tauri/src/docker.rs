//! Capture Docker via sa CLI locale : la connexion native au moteur reste gérée par Docker.
use crate::{classifier::redact, db, Result}; // Les captures sont filtrées et persistées.
use serde_json::{json, Value}; // Docker inspect fournit un format structuré.
// @ligne Importe les outils nécessaires à la mise en œuvre des captures et événements Docker.
use tokio::{
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
    process::Command,
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
    time::{timeout, Duration},
}; // Toutes les commandes externes ont une échéance.
// Aucun shell : les arguments ne peuvent pas injecter une autre commande.
async fn docker(args: &[&str]) -> Result<String> {
    // @ligne Prépare « result » pour la prochaine étape des captures et événements Docker.
    let result = timeout(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
        Duration::from_secs(15),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
        Command::new("docker").args(args).output(),
    // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
    )
    .await??; // Un démon bloqué ne bloque pas l'interface indéfiniment.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des captures et événements Docker.
    if !result.status.success() {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des captures et événements Docker.
        return Err(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
            "Docker indisponible ou accès refusé ; les snapshots restent consultables".into(),
        // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
        );
    } // Ne renvoie pas de secrets de stderr.
    Ok(String::from_utf8_lossy(&result.stdout).into_owned()) // Docker produit normalement de l'UTF-8.
// @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
}
// Une reconstruction est étiquetée comme telle : Docker n'enregistre pas la CLI d'origine.
pub async fn sync() -> Result<Value> {
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours des captures et événements Docker.
    timeout(Duration::from_secs(30),sync_inner()).await.map_err(|_|"Synchronisation Docker supérieure à 30 secondes ; les captures déjà écrites restent disponibles")?
// @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
}
// Une erreur intermédiaire ne supprime jamais les captures obtenues avant elle.
async fn sync_inner() -> Result<Value> {
    let ids = docker(&["ps", "-aq", "--no-trunc"]).await?; // Inclut les conteneurs arrêtés.
    let mut count = 0; // Nombre de captures actualisées.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des captures et événements Docker.
    for id in ids.lines().take(1000) {
        let data: Value = serde_json::from_str(&docker(&["inspect", id]).await?)?; // Une lecture structurée par conteneur.
        let c = &data[0]; // docker inspect renvoie un tableau.
        let name = c["Name"].as_str().unwrap_or(id).trim_start_matches('/'); // Retire le slash Docker.
        let image = c["Config"]["Image"].as_str().unwrap_or("unknown"); // Image de création.
        let quote = |s: &str| shlex::try_quote(s).unwrap_or_default().into_owned(); // Les actions copiées gardent des arguments sûrs.
        let ports = c["HostConfig"]["PortBindings"].clone(); // Garde les bindings hôte explicites.
        let mut launch = format!("docker run --name {}", quote(name)); // Base minimale reconstruite.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des captures et événements Docker.
        if let Some(map) = ports.as_object() {
            // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des captures et événements Docker.
            for (port, bindings) in map {
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des captures et événements Docker.
                if let Some(bindings) = bindings.as_array() {
                    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des captures et événements Docker.
                    for b in bindings {
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                        launch.push_str(&format!(
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                            " -p {}",
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                            quote(&format!(
                                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                                "{}:{}:{}",
                                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                                b["HostIp"]
                                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                                    .as_str()
                                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                                    .filter(|s| !s.is_empty())
                                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                                    .unwrap_or("0.0.0.0"),
                                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                                b["HostPort"].as_str().unwrap_or(""),
                                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                                port
                            // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
                            ))
                        // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
                        ));
                    // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
                    }
                // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
                }
            // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
            }
        } // Reproduit les ports publiés.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des captures et événements Docker.
        if let Some(mounts) = c["Mounts"].as_array() {
            // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des captures et événements Docker.
            for m in mounts {
                // @ligne Prépare « source » pour la prochaine étape des captures et événements Docker.
                let source = m["Name"]
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                    .as_str()
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                    .filter(|s| !s.is_empty())
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                    .or(m["Source"].as_str())
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                    .unwrap_or("");
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                launch.push_str(&format!(
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                    " -v {}",
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                    quote(&format!(
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                        "{}:{}{}",
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                        source,
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                        m["Destination"].as_str().unwrap_or(""),
                        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des captures et événements Docker.
                        if m["RW"] == false { ":ro" } else { "" }
                    // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
                    ))
                // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
                ));
            // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
            }
        } // Préserve volumes et lecture seule.
        launch.push_str(&format!(" {}", quote(image))); // Les options non récupérées sont documentées comme limites.
        // @ligne Prépare « logs » pour la prochaine étape des captures et événements Docker.
        let logs = docker(&["logs", "--tail", "200", id])
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours des captures et événements Docker.
            .await
            .unwrap_or_else(|_| "Logs indisponibles".into()); // L'échec des logs n'annule pas la capture.
        // @ligne Prépare « env » pour la prochaine étape des captures et événements Docker.
        let env = c["Config"]["Env"]
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
            .as_array()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
            .map(|vars| {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                vars.iter()
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                    .map(|v| redact(v.as_str().unwrap_or("")))
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                    .collect::<Vec<_>>()
            // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
            })
            .unwrap_or_default(); // Aucun environnement brut sur disque.
        let captured = captured_origin(name, image)?; // Conserve séparément le texte retrouvé et la reconstruction technique.
        let labels = &c["Config"]["Labels"]; // Compose renseigne ses fichiers et son répertoire dans les labels.
        // @ligne Prépare « compose_files » pour la prochaine étape des captures et événements Docker.
        let compose_files = labels["com.docker.compose.project.config_files"]
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
            .as_str()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
            .unwrap_or("");
        // @ligne Prépare « compose » pour la prochaine étape des captures et événements Docker.
        let compose = compose_files
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
            .split(',')
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
            .filter(|p| !p.is_empty())
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
            .filter_map(|p| {
                // @ligne Prépare « path » pour la prochaine étape des captures et événements Docker.
                let path = std::path::Path::new(p);
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des captures et événements Docker.
                if std::fs::metadata(path).ok()?.len() > 1_048_576 {
                    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des captures et événements Docker.
                    return None;
                // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
                }
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                Some(json!({"path":p,"content":redact(&std::fs::read_to_string(path).ok()?)}))
            // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
            })
            .collect::<Vec<_>>(); // Ne charge jamais plus de 1 Mio par fichier Compose.
        // @ligne Prépare « backup » pour la prochaine étape des captures et événements Docker.
        let backup = if image.contains("postgres") {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
            format!(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                "docker exec {} pg_dump -U {{{{db_user}}}} -Fc {{{{database}}}} > sauvegarde.dump",
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                quote(name)
            // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
            )
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
        } else if image.contains("mysql") || image.contains("mariadb") {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
            format!("docker exec -it {} mysqldump -u {{{{db_user}}}} -p {{{{database}}}} > sauvegarde.sql",quote(name))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
        } else {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
            format!("docker run --rm --volumes-from {} -v \"$PWD\":/backup alpine tar czf /backup/volumes.tar.gz {{{{volume_path}}}}",quote(name))
        }; // Les paramètres inconnus restent des variables à renseigner.
        let snapshot = json!({"id":id,"name":name,"image":image,"state":c["State"]["Status"],"ports":ports,"mounts":c["Mounts"],"networks":c["NetworkSettings"]["Networks"],"environment":env,"logs":redact(&logs),"reconstructed":launch,"captured_origin":captured,"compose":compose,"origin":if captured.is_some(){"Commande capturée correspondant au nom et à l’image ; corrélation à vérifier"}else{"Reconstruction technique ; origine non capturée"},"actions":{"backup":backup,"shell":format!("docker exec -it {} /bin/sh",quote(name)),"logs":format!("docker logs -f --tail 100 {}",quote(name)),"restart":format!("docker restart {}",quote(name)),"inspect":format!("docker inspect {}",quote(name))}}); // Toutes les actions sont uniquement copiables.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
        db::snapshot("docker", id, &snapshot)?;
        count += 1; // Le cache reste disponible après arrêt du moteur.
    // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
    }
    Ok(json!({"updated":count,"snapshots":db::snapshots("docker")?})) // Aucun conteneur ancien n'est supprimé automatiquement.
// @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
}
// Le flux d'événements local suit les sockets et Named Pipes gérés nativement par la CLI Docker.
pub async fn watch() {
    use tokio::io::{AsyncBufReadExt, BufReader}; // Lit un événement JSON par ligne sans accumuler le flux.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des captures et événements Docker.
    loop {
        let _ = sync().await; // Capture les conteneurs déjà présents avant la connexion au flux.
        // @ligne Prépare « child » pour la prochaine étape des captures et événements Docker.
        let child = Command::new("docker")
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
            .args([
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                "events",
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                "--filter",
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                "type=container",
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                "--format",
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
                "{{json .}}",
            // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
            ])
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
            .stdout(std::process::Stdio::piped())
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
            .stderr(std::process::Stdio::null())
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
            .kill_on_drop(true)
            .spawn(); // Aucun enfant n'est abandonné à l'arrêt du daemon.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des captures et événements Docker.
        if let Ok(mut child) = child {
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des captures et événements Docker.
            if let Some(stdout) = child.stdout.take() {
                let mut lines = BufReader::new(stdout).lines(); // La mémoire ne croît pas avec la durée du service.
                // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des captures et événements Docker.
                while let Ok(Some(_event)) = lines.next_line().await {
                    tokio::time::sleep(Duration::from_millis(250)).await; // Laisse Docker appliquer le changement avant l'inspection.
                    let _ = sync().await; // Le dernier événement d'une rafale n'est jamais abandonné.
                // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
                }
            // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
            }
            let _ = child.wait().await; // Récolte le processus lorsque Docker s'arrête.
        // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
        }
        tokio::time::sleep(Duration::from_secs(5)).await; // Reconnexion bornée lorsque le moteur redevient disponible.
    // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
    }
// @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
}
// Une origine est exacte uniquement si son texte est conservé dans l'historique utilisateur.
fn captured_origin(name: &str, image: &str) -> Result<Option<String>> {
    let conn = db::open()?; // La recherche ne touche pas au moteur Docker.
    // @ligne Prépare « stmt » pour la prochaine étape des captures et événements Docker.
    let mut stmt=conn.prepare("SELECT raw FROM commands WHERE binary IN ('docker','podman') ORDER BY last_seen DESC LIMIT 10000")?;
    let rows = stmt.query_map([], |r| r.get::<_, String>(0))?; // L'historique est déjà masqué.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas des captures et événements Docker.
    for raw in rows {
        // @ligne Prépare « raw » pour la prochaine étape des captures et événements Docker.
        let raw = raw?;
        let tokens = shlex::split(&raw).unwrap_or_default(); // Respecte les noms et chemins entre guillemets.
        // @ligne Prépare « matching_name » pour la prochaine étape des captures et événements Docker.
        let matching_name = tokens.windows(2).any(|p| p[0] == "--name" && p[1] == name)
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des captures et événements Docker.
            || tokens.iter().any(|t| t == &format!("--name={name}"));
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des captures et événements Docker.
        if matching_name && tokens.iter().any(|t| t == "run") && tokens.iter().any(|t| t == image) {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des captures et événements Docker.
            return Ok(Some(raw));
        } // Corrélation explicite, sans prétendre prouver un ID absent de l'historique.
    // @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
    }
    Ok(None) // Une origine absente reste inconnue, jamais fabriquée.
// @ligne Termine cette construction et revient au contexte parent des captures et événements Docker.
}
