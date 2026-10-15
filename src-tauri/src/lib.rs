//! API métier commune au bureau et au binaire autonome.
pub mod classifier; // Analyse et confidentialité des commandes.
pub mod daemon; // Service utilisateur optionnel sans fenêtre.
pub mod db; // Stockage SQLite et snapshots.
pub mod dba; // DDL vers modèles relationnels et UML.
pub mod docker; // Capture locale de Docker et cache hors ligne.
pub mod history; // Lecture seule des historiques des shells.
pub mod netmap; // Découverte TCP bornée aux LAN privés.
pub mod share; // Salle LAN et contrôle serveur des permissions.
use serde_json::{json, Value}; // L'IPC ne transporte que des valeurs structurées.
use std::sync::OnceLock; // La salle est unique par processus.
use tokio::sync::Mutex; // Le verrou asynchrone n'immobilise pas un thread de Tokio.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>; // Toutes les erreurs peuvent traverser les tâches.
static HOST: OnceLock<Mutex<Option<share::Host>>> = OnceLock::new(); // Aucun serveur n'est lancé implicitement.

// Dispatch explicite : il n'existe aucune fonction d'exécution arbitraire de shell.
pub async fn dispatch(action: &str, p: Value) -> Result<Value> {
    let text = |key: &str| p[key].as_str().unwrap_or(""); // Un champ absent reste une chaîne vide à valider par le module.
    match action {
        "commands" => db::commands(
            text("query"),
            text("category"),
            p["favorites"].as_bool().unwrap_or(false),
            p["offset"].as_i64().unwrap_or(0),
        ), // Recherche paginée.
        "capture" => {
            let c = serde_json::from_value::<db::Capture>(p)?;
            db::insert(&db::open()?, &c)?;
            Ok(json!({"saved":true}))
        } // Les hooks ne reçoivent pas d'accès aux autres modules.
        "import" => history::import(), // Import explicite et premier démarrage.
        "favorite" => {
            db::open()?.execute(
                "UPDATE commands SET favorite=NOT favorite WHERE id=?1",
                [p["id"].as_i64().ok_or("Identifiant requis")?],
            )?;
            Ok(json!(true))
        } // Bascule atomique.
        "delete" => {
            db::open()?.execute(
                "DELETE FROM commands WHERE id=?1",
                [p["id"].as_i64().ok_or("Identifiant requis")?],
            )?;
            Ok(json!(true))
        } // Le trigger nettoie aussi l'index.
        "stats" => {
            let c = db::open()?;
            let total: i64 = c.query_row("SELECT count(*) FROM commands", [], |r| r.get(0))?;
            let mut stmt = c.prepare("SELECT category,count(*) FROM commands GROUP BY category")?;
            let groups = stmt
                .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))?
                .collect::<std::result::Result<Vec<_>, _>>()?;
            Ok(
                json!({"total":total,"categories":groups,"database":db::path(),"bytes":std::fs::metadata(db::path())?.len()}),
            )
        } // Statistiques réelles.
        "docker_sync" => docker::sync().await, // Échec explicite avec cache conservé.
        "docker_cache" => db::snapshots("docker"), // Fonctionne moteur éteint.
        "interfaces" => netmap::interfaces(), // Choix explicite de l'interface.
        "net_scan" => netmap::scan(text("cidr")).await, // Le périmètre est validé dans le moteur.
        "net_cache" => db::snapshots("netmap"), // Historique local des sous-réseaux.
        "sql_sqlite" => dba::from_sqlite(text("path")), // Introspection strictement en lecture seule.
        "sql_parse" => dba::parse(text("ddl")),         // Aucune connexion à une base externe.
        "sql_save" => dba::save(text("name"), text("ddl")), // Sauvegarde volontaire du modèle.
        "sql_models" => {
            let c = db::open()?;
            let mut stmt =
                c.prepare("SELECT name,ddl,updated_at FROM models ORDER BY updated_at DESC")?;
            let rows=stmt.query_map([],|r|Ok(json!({"name":r.get::<_,String>(0)?,"ddl":r.get::<_,String>(1)?,"updated_at":r.get::<_,i64>(2)?})))?.collect::<std::result::Result<Vec<_>,_>>()?;
            Ok(json!(rows))
        } // Recharge les travaux précédents.
        "share_start" => {
            let paths: Vec<String> = serde_json::from_value(p["paths"].clone())?;
            let host = share::start(paths, text("ip").into()).await?;
            let info = host.info.clone();
            *HOST.get_or_init(|| Mutex::new(None)).lock().await = Some(host);
            Ok(info)
        } // Remplacer une salle arrête la précédente.
        "share_stop" => {
            *HOST.get_or_init(|| Mutex::new(None)).lock().await = None;
            Ok(json!(true))
        } // Drop coupe immédiatement l'écoute.
        "share_status" => {
            let host = HOST.get_or_init(|| Mutex::new(None)).lock().await;
            match host.as_ref() {
                Some(h) => Ok(json!({"room":h.info,"peers":h.peers()?})),
                None => Ok(json!({"room":null,"peers":[]})),
            }
        } // Les jetons sont exclus de la sérialisation.
        "share_permission" => {
            let host = HOST.get_or_init(|| Mutex::new(None)).lock().await;
            host.as_ref().ok_or("Aucune salle active")?.permission(
                text("id"),
                p["upload"].as_bool().unwrap_or(false),
                p["banned"].as_bool().unwrap_or(false),
            )?;
            Ok(json!(true))
        } // Administration par IPC uniquement.
        _ => Err(format!("Action inconnue : {action}").into()), // Refus par défaut.
    }
}
