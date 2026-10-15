//! Stockage local : transactions, recherche indexée et export ne sortent jamais du poste.
use crate::{classifier, Result}; // Le filtre de secrets est partagé par tous les points d'entrée.
use rusqlite::{params, Connection}; // Les paramètres empêchent l'injection SQL.
use serde::{Deserialize, Serialize}; // Formats communs à la CLI et à la fenêtre.
use serde_json::{json, Value}; // Les snapshots gardent les champs spécifiques à chaque module.
use std::path::PathBuf; // Chemins natifs Linux, macOS et Windows.

// Les métadonnées manquantes ne sont pas inventées lors d'une rétro-ingestion.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Capture {
    pub raw_command: String, // Commande reçue avant filtrage de secrets.
    #[serde(default)]
    pub shell: String, // Shell d'origine, vide si inconnu.
    #[serde(default)]
    pub working_directory: String, // Le hook connaît ce chemin, l'historique classique non.
    #[serde(default)]
    pub terminal_emulator: String, // TERM_PROGRAM ou TERM fourni par le hook.
    pub exit_code: Option<i32>, // Un code absent reste inconnu.
    pub duration_ms: Option<i64>, // Durée optionnelle réellement mesurée.
    pub timestamp: Option<i64>, // Date historique si disponible.
}

// Le chemin peut être isolé par les tests ou par une installation portable.
pub fn path() -> PathBuf {
    std::env::var_os("OMNISHELL_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            dirs::data_local_dir()
                .unwrap_or_else(std::env::temp_dir)
                .join("omnishell-studio")
        })
        .join("omnishell.sqlite3") // Aucun fichier dans le dépôt par défaut.
}

// Chaque opération ouvre une connexion courte ; SQLite arbitre les écritures simultanées.
pub fn open() -> Result<Connection> {
    let path = path(); // Calcule le répertoire utilisateur une seule fois.
    std::fs::create_dir_all(path.parent().ok_or("Répertoire de données invalide")?)?; // Crée le stockage au premier démarrage.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(
            path.parent().unwrap(),
            std::fs::Permissions::from_mode(0o700),
        )?;
    } // L'historique n'est pas lisible par les autres comptes.
    let conn = Connection::open(path)?; // Le fichier sera créé par SQLite s'il est absent.
    conn.execute_batch(include_str!("../migrations/001_init.sql"))?; // Migration idempotente incluant les pragmas par connexion.
    Ok(conn) // Le propriétaire libère la connexion automatiquement à la fin de l'opération.
}

// L'identité est calculée après masquage pour ne pas conserver un hash de secret.
pub fn insert(conn: &Connection, c: &Capture) -> Result<bool> {
    if c.raw_command.trim().is_empty() || c.raw_command.starts_with(' ') {
        return Ok(false);
    } // Respecte la convention HISTCONTROL=ignorespace.
    if c.raw_command.len() > 131_072 {
        return Err("Commande trop longue (128 Kio maximum)".into());
    } // Évite les captures accidentelles massives.
    let a = classifier::analyze(c.raw_command.trim()); // Normalisation sans exécution.
    let hash = blake3::hash(a.raw.as_bytes()).to_hex().to_string(); // Déduplique sans perdre le badge sudo.
    let stamp = c.timestamp.unwrap_or_else(now); // Une date manquante est explicitement la date d'import.
    conn.execute("INSERT INTO commands(hash,raw,clean,binary,category,privileged,shell,cwd,terminal,exit_code,duration_ms,first_seen,last_seen) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?12) ON CONFLICT(hash) DO UPDATE SET count=count+1,last_seen=max(last_seen,excluded.last_seen),first_seen=min(first_seen,excluded.first_seen),shell=excluded.shell,cwd=excluded.cwd,terminal=excluded.terminal,exit_code=excluded.exit_code,duration_ms=excluded.duration_ms", params![hash,a.raw,a.clean,a.binary,a.category,a.privileged,c.shell,classifier::redact(&c.working_directory),c.terminal_emulator,c.exit_code,c.duration_ms,stamp])?; // Un seul upsert garde le compteur atomique.
    Ok(true) // Compte une occurrence, même si elle rejoint une entrée existante.
}

// Les recherches courtes utilisent LIKE ; FTS5 prend le relais à partir de trois caractères.
pub fn commands(query: &str, category: &str, favorites: bool, offset: i64) -> Result<Value> {
    let conn = open()?; // Connexion locale uniquement.
    let indexed = query.chars().count() >= 3; // Le tokenizer trigram ne recherche pas un ou deux caractères.
    let condition = if indexed {
        "id IN (SELECT rowid FROM command_fts WHERE command_fts MATCH ?1)"
    } else {
        "raw LIKE ?1 ESCAPE '\\'"
    }; // Aucune interpolation de contenu utilisateur.
    let search = if indexed {
        format!("\"{}\"", query.replace('"', "\"\""))
    } else {
        format!(
            "%{}%",
            query
                .replace('\\', "\\\\")
                .replace('%', "\\%")
                .replace('_', "\\_")
        )
    }; // Traite la recherche comme du texte littéral.
    let filter = format!("{condition} AND (?2='' OR category=?2) AND (?3=0 OR favorite=1)"); // Les filtres utilisent toujours des paramètres liés.
    let total: i64 = conn.query_row(
        &format!("SELECT count(*) FROM commands WHERE {filter}"),
        params![search, category, favorites],
        |r| r.get(0),
    )?; // Total pour la pagination.
    let mut stmt = conn.prepare(&format!("SELECT id,raw,clean,binary,category,privileged,shell,cwd,terminal,count,favorite,last_seen,exit_code,duration_ms FROM commands WHERE {filter} ORDER BY last_seen DESC,id DESC LIMIT 100 OFFSET ?4"))?; // Plafond de résultats par page.
    let rows = stmt.query_map(params![search,category,favorites,offset.max(0)], |r| Ok(json!({"id":r.get::<_,i64>(0)?,"raw":r.get::<_,String>(1)?,"clean":r.get::<_,String>(2)?,"binary":r.get::<_,String>(3)?,"category":r.get::<_,String>(4)?,"privileged":r.get::<_,bool>(5)?,"shell":r.get::<_,String>(6)?,"cwd":r.get::<_,String>(7)?,"terminal":r.get::<_,String>(8)?,"count":r.get::<_,i64>(9)?,"favorite":r.get::<_,bool>(10)?,"last_seen":r.get::<_,i64>(11)?,"exit_code":r.get::<_,Option<i32>>(12)?,"duration_ms":r.get::<_,Option<i64>>(13)?})))?.collect::<std::result::Result<Vec<_>,_>>()?; // Propage une éventuelle erreur de lecture.
    Ok(json!({"items":rows,"total":total,"offset":offset})) // Réponse stable pour React et la CLI.
}

// Les snapshots ne remplacent jamais les commandes de l'utilisateur.
pub fn snapshot(kind: &str, key: &str, payload: &Value) -> Result<()> {
    open()?.execute("INSERT INTO snapshots VALUES(?1,?2,?3,?4) ON CONFLICT(kind,key) DO UPDATE SET payload=excluded.payload,captured_at=excluded.captured_at", params![kind,key,classifier::redact_json(payload).to_string(),now()])?; // Filtre aussi les secrets des inspecteurs Docker.
    Ok(()) // L'upsert conserve une capture par objet.
}

// Restitue les captures anciennes même si le service d'origine est inaccessible.
pub fn snapshots(kind: &str) -> Result<Value> {
    let conn = open()?; // Connexion indépendante pour éviter les verrous longs.
    let mut stmt = conn.prepare(
        "SELECT payload,captured_at FROM snapshots WHERE kind=?1 ORDER BY captured_at DESC",
    )?; // Ordre du plus récent au plus ancien.
    let rows = stmt
        .query_map([kind], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?; // Copie les résultats avant fermeture.
    Ok(Value::Array(rows.into_iter().map(|(p,t)| json!({"data":serde_json::from_str::<Value>(&p).unwrap_or(Value::Null),"captured_at":t})).collect()))
    // Un snapshot invalide reste identifié sans paniquer.
}

// Les timestamps Unix évitent toute dépendance à un fuseau horaire particulier.
pub fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
} // L'interface assure le formatage local.
