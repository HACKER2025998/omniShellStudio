//! API métier commune au bureau et au binaire autonome.
pub mod archives; // Préparation mémoire constante des dossiers partagés.
pub mod classifier; // Analyse et confidentialité des commandes.
pub mod daemon; // Service utilisateur optionnel sans fenêtre.
pub mod database_connections; // Introspection des moteurs SQL sans conservation des identifiants.
pub mod db; // Stockage SQLite et snapshots.
pub mod dba; // DDL vers modèles relationnels et UML.
pub mod docker; // Capture locale de Docker et cache hors ligne.
pub mod history; // Lecture seule des historiques des shells.
pub mod netmap; // Découverte TCP bornée aux LAN privés.
pub mod network_protocols; // Décodeurs et observations réseau avec preuves explicites.
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
pub mod share;
pub mod shell_ast; // AST Bash, Fish et PowerShell hors ligne. // Salle LAN et contrôle serveur des permissions.
use serde_json::{json, Value}; // L'IPC ne transporte que des valeurs structurées.
use std::sync::OnceLock; // La salle est unique par processus.
use tokio::sync::Mutex; // Le verrou asynchrone n'immobilise pas un thread de Tokio.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>; // Toutes les erreurs peuvent traverser les tâches.
static HOST: OnceLock<Mutex<Option<share::Host>>> = OnceLock::new(); // Aucun serveur n'est lancé implicitement.
// Dispatch explicite : il n'existe aucune fonction d'exécution arbitraire de shell.
pub async fn dispatch(action: &str, p: Value) -> Result<Value> {
    let text = |key: &str| p[key].as_str().unwrap_or(""); // Un champ absent reste une chaîne vide à valider par le module.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la liste fermée des actions métier.
    match action {
        "command_inspect" => shell_ast::inspect(text("raw"), text("shell")), // Arbre déjà masqué et explications hors ligne.
        "sql_database" => database_connections::inspect(text("engine"), text("url")).await, // Lecture des catalogues uniquement.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
        "commands" => db::commands_filtered(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
            text("query"),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
            text("category"),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
            p["favorites"].as_bool().unwrap_or(false),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
            p["offset"].as_i64().unwrap_or(0),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
            p["privileged"].as_bool(),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
            p["since"].as_i64(),
        ), // Recherche paginée.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
        "capture" => {
            // @ligne Prépare « c » pour la prochaine étape de la liste fermée des actions métier.
            let c = serde_json::from_value::<db::Capture>(p)?;
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
            db::insert(&db::open()?, &c)?;
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la liste fermée des actions métier.
            Ok(json!({"saved":true}))
        } // Les hooks ne reçoivent pas d'accès aux autres modules.
        "import" => history::import(), // Import explicite et premier démarrage.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
        "favorite" => {
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la liste fermée des actions métier.
            db::open()?.execute(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
                "UPDATE commands SET favorite=NOT favorite WHERE id=?1",
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
                [p["id"].as_i64().ok_or("Identifiant requis")?],
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
            )?;
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la liste fermée des actions métier.
            Ok(json!(true))
        } // Bascule atomique.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
        "delete" => {
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la liste fermée des actions métier.
            db::open()?.execute(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
                "DELETE FROM commands WHERE id=?1",
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
                [p["id"].as_i64().ok_or("Identifiant requis")?],
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
            )?;
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la liste fermée des actions métier.
            Ok(json!(true))
        } // Le trigger nettoie aussi l'index.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
        "stats" => {
            // @ligne Prépare « c » pour la prochaine étape de la liste fermée des actions métier.
            let c = db::open()?;
            // @ligne Prépare « total » pour la prochaine étape de la liste fermée des actions métier.
            let total: i64 = c.query_row("SELECT count(*) FROM commands", [], |r| r.get(0))?;
            // @ligne Prépare « stmt » pour la prochaine étape de la liste fermée des actions métier.
            let mut stmt = c.prepare("SELECT category,count(*) FROM commands GROUP BY category")?;
            // @ligne Prépare « groups » pour la prochaine étape de la liste fermée des actions métier.
            let groups = stmt
                // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la liste fermée des actions métier.
                .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))?
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
                .collect::<std::result::Result<Vec<_>, _>>()?;
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la liste fermée des actions métier.
            Ok(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
                json!({"total":total,"categories":groups,"database":db::path(),"bytes":std::fs::metadata(db::path())?.len()}),
            // @ligne Termine cette construction et revient au contexte parent de la liste fermée des actions métier.
            )
        } // Statistiques réelles.
        "docker_sync" => docker::sync().await, // Échec explicite avec cache conservé.
        "docker_cache" => db::snapshots("docker"), // Fonctionne moteur éteint.
        "interfaces" => netmap::interfaces(), // Choix explicite de l'interface.
        "net_scan" => netmap::scan_with(text("cidr"), &p).await, // Le périmètre est validé dans le moteur.
        "net_oui_import" => network_protocols::import_oui(text("path")), // Lecture explicite d’un fichier IEEE choisi par l’utilisateur.
        "net_wan" => netmap::wan().await, // Appel Internet uniquement sur demande.
        "wan_cache" => db::snapshots("wan"), // Cache consultable hors ligne.
        "net_cache" => db::snapshots("netmap"), // Historique local des sous-réseaux.
        "sql_sqlite" => dba::from_sqlite(text("path")), // Introspection strictement en lecture seule.
        "sql_parse" => dba::parse(text("ddl")),         // Aucune connexion à une base externe.
        "sql_save" => dba::save(text("name"), text("ddl")), // Sauvegarde volontaire du modèle.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
        "sql_models" => {
            // @ligne Prépare « c » pour la prochaine étape de la liste fermée des actions métier.
            let c = db::open()?;
            // @ligne Prépare « stmt » pour la prochaine étape de la liste fermée des actions métier.
            let mut stmt =
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
                c.prepare("SELECT name,ddl,updated_at FROM models ORDER BY updated_at DESC")?;
            // @ligne Prépare « rows » pour la prochaine étape de la liste fermée des actions métier.
            let rows=stmt.query_map([],|r|Ok(json!({"name":r.get::<_,String>(0)?,"ddl":r.get::<_,String>(1)?,"updated_at":r.get::<_,i64>(2)?})))?.collect::<std::result::Result<Vec<_>,_>>()?;
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la liste fermée des actions métier.
            Ok(json!(rows))
        } // Recharge les travaux précédents.
        "share_saved" => share::saved(), // Salles restaurables, sans jetons dans la réponse.
        "share_discover" => share::discover().await, // Découverte locale indépendante du PIN.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
        "share_resume" => {
            // @ligne Prépare « host » pour la prochaine étape de la liste fermée des actions métier.
            let host = share::resume(text("id")).await?;
            // @ligne Prépare « info » pour la prochaine étape de la liste fermée des actions métier.
            let info = host.info.clone();
            *HOST.get_or_init(|| Mutex::new(None)).lock().await = Some(host);
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la liste fermée des actions métier.
            Ok(info)
        // @ligne Termine cette construction et revient au contexte parent de la liste fermée des actions métier.
        }
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
        "share_start" => {
            // @ligne Prépare « paths » pour la prochaine étape de la liste fermée des actions métier.
            let paths: Vec<String> = serde_json::from_value(p["paths"].clone())?;
            // @ligne Prépare « limits » pour la prochaine étape de la liste fermée des actions métier.
            let limits: share::Limits = if p["limits"].is_object() {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
                serde_json::from_value(p["limits"].clone())?
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
            } else {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
                share::Limits::default()
            }; // Les champs omis gardent la politique standard.
            // @ligne Prépare « host » pour la prochaine étape de la liste fermée des actions métier.
            let host = share::start_with_limits(paths, text("ip").into(), limits).await?;
            // @ligne Prépare « info » pour la prochaine étape de la liste fermée des actions métier.
            let info = host.info.clone();
            *HOST.get_or_init(|| Mutex::new(None)).lock().await = Some(host);
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la liste fermée des actions métier.
            Ok(info)
        } // Remplacer une salle arrête la précédente.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
        "share_stop" => {
            // @ligne Prépare « host » pour la prochaine étape de la liste fermée des actions métier.
            let mut host = HOST.get_or_init(|| Mutex::new(None)).lock().await;
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la liste fermée des actions métier.
            if let Some(host) = host.as_ref() {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
                host.close()?;
            } // Fermeture explicite, différente d’une coupure de l’hôte.
            *host = None;
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la liste fermée des actions métier.
            Ok(json!(true))
        } // Drop coupe immédiatement l'écoute.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
        "share_status" => {
            // @ligne Prépare « host » pour la prochaine étape de la liste fermée des actions métier.
            let host = HOST.get_or_init(|| Mutex::new(None)).lock().await;
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la liste fermée des actions métier.
            match host.as_ref() {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
                Some(h) => Ok(json!({"room":h.info,"peers":h.peers()?})),
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
                None => Ok(json!({"room":null,"peers":[]})),
            // @ligne Termine cette construction et revient au contexte parent de la liste fermée des actions métier.
            }
        } // Les jetons sont exclus de la sérialisation.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
        "share_permission" => {
            // @ligne Prépare « host » pour la prochaine étape de la liste fermée des actions métier.
            let host = HOST.get_or_init(|| Mutex::new(None)).lock().await;
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
            host.as_ref().ok_or("Aucune salle active")?.permission(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
                text("id"),
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
                p["upload"].as_bool().unwrap_or(false),
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
                p["banned"].as_bool().unwrap_or(false),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la liste fermée des actions métier.
            )?;
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la liste fermée des actions métier.
            Ok(json!(true))
        } // Administration par IPC uniquement.
        _ => Err(format!("Action inconnue : {action}").into()), // Refus par défaut.
    // @ligne Termine cette construction et revient au contexte parent de la liste fermée des actions métier.
    }
// @ligne Termine cette construction et revient au contexte parent de la liste fermée des actions métier.
}
