//! Stockage local : transactions, recherche indexée et export ne sortent jamais du poste.
use crate::{classifier, Result}; // Le filtre de secrets est partagé par tous les points d'entrée.
use rusqlite::{params, Connection}; // Les paramètres empêchent l'injection SQL.
use serde::{Deserialize, Serialize}; // Formats communs à la CLI et à la fenêtre.
use serde_json::{json, Value}; // Les snapshots gardent les champs spécifiques à chaque module.
use std::path::PathBuf; // Chemins natifs Linux, macOS et Windows.
// Les métadonnées manquantes ne sont pas inventées lors d'une rétro-ingestion.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
pub struct Capture {
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du stockage SQLite et de ses requêtes paramétrées.
    #[serde(default)]
    pub event_id: Option<String>, // Identifiant stable partagé par le daemon et le fallback.
    pub raw_command: String, // Commande reçue avant filtrage de secrets.
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du stockage SQLite et de ses requêtes paramétrées.
    #[serde(default)]
    pub shell: String, // Shell d'origine, vide si inconnu.
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du stockage SQLite et de ses requêtes paramétrées.
    #[serde(default)]
    pub working_directory: String, // Le hook connaît ce chemin, l'historique classique non.
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du stockage SQLite et de ses requêtes paramétrées.
    #[serde(default)]
    pub terminal_emulator: String, // TERM_PROGRAM ou TERM fourni par le hook.
    pub exit_code: Option<i32>, // Un code absent reste inconnu.
    pub duration_ms: Option<i64>, // Durée optionnelle réellement mesurée.
    pub timestamp: Option<i64>, // Date historique si disponible.
// @ligne Termine cette construction et revient au contexte parent du stockage SQLite et de ses requêtes paramétrées.
}
// Le chemin peut être isolé par les tests ou par une installation portable.
pub fn path() -> PathBuf {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
    std::env::var_os("OMNISHELL_DATA_DIR")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        .map(PathBuf::from)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        .unwrap_or_else(|| {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            dirs::data_local_dir()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
                .unwrap_or_else(std::env::temp_dir)
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
                .join("omnishell-studio")
        // @ligne Termine cette construction et revient au contexte parent du stockage SQLite et de ses requêtes paramétrées.
        })
        .join("omnishell.sqlite3") // Aucun fichier dans le dépôt par défaut.
// @ligne Termine cette construction et revient au contexte parent du stockage SQLite et de ses requêtes paramétrées.
}
// Chaque opération ouvre une connexion courte ; SQLite arbitre les écritures simultanées.
pub fn open() -> Result<Connection> {
    let path = path(); // Calcule le répertoire utilisateur une seule fois.
    std::fs::create_dir_all(path.parent().ok_or("Répertoire de données invalide")?)?; // Crée le stockage au premier démarrage.
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du stockage SQLite et de ses requêtes paramétrées.
    #[cfg(unix)]
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
    {
        // @ligne Importe les outils nécessaires à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        use std::os::unix::fs::PermissionsExt;
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        std::fs::set_permissions(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            path.parent().unwrap(),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            std::fs::Permissions::from_mode(0o700),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        )?;
    } // L'historique n'est pas lisible par les autres comptes.
    let conn = Connection::open(path)?; // Le fichier sera créé par SQLite s'il est absent.
    conn.busy_timeout(std::time::Duration::from_secs(5))?; // Le délai est actif avant même la première négociation WAL.
    conn.execute_batch(include_str!("../migrations/001_init.sql"))?; // Migration idempotente incluant les pragmas par connexion.
    Ok(conn) // Le propriétaire libère la connexion automatiquement à la fin de l'opération.
// @ligne Termine cette construction et revient au contexte parent du stockage SQLite et de ses requêtes paramétrées.
}
// L'identité est calculée après masquage pour ne pas conserver un hash de secret.
pub fn insert(conn: &Connection, c: &Capture) -> Result<bool> {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du stockage SQLite et de ses requêtes paramétrées.
    if let Some(event_id) = &c.event_id {
        let tx = rusqlite::Transaction::new_unchecked(conn,rusqlite::TransactionBehavior::Immediate)?; // La déduplication et l'écriture sont atomiques.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du stockage SQLite et de ses requêtes paramétrées.
        if tx.query_row(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            "SELECT EXISTS(SELECT 1 FROM capture_receipts WHERE event_id=?1)",
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            [event_id],
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            |r| r.get::<_, bool>(0),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        )? {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du stockage SQLite et de ses requêtes paramétrées.
            return Ok(false);
        } // Un ACK perdu ne recompte pas la commande.
        // @ligne Prépare « saved » pour la prochaine étape du stockage SQLite et de ses requêtes paramétrées.
        let saved = insert(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            &tx,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            &Capture {
                // @ligne Renseigne le champ « event_id » de la structure utilisée ici du stockage SQLite et de ses requêtes paramétrées.
                event_id: None,
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
                ..c.clone()
            // @ligne Termine cette construction et revient au contexte parent du stockage SQLite et de ses requêtes paramétrées.
            },
        )?; // Le chemin normal applique tous les filtres métier.
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du stockage SQLite et de ses requêtes paramétrées.
        tx.execute(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            "INSERT INTO capture_receipts VALUES(?1,?2)",
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            params![event_id, now()],
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        )?;
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        tx.commit()?;
        return Ok(saved); // L'ACK peut maintenant être envoyé.
    // @ligne Termine cette construction et revient au contexte parent du stockage SQLite et de ses requêtes paramétrées.
    }
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du stockage SQLite et de ses requêtes paramétrées.
    if c.raw_command.trim().is_empty() || c.raw_command.starts_with(' ') {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du stockage SQLite et de ses requêtes paramétrées.
        return Ok(false);
    } // Respecte la convention HISTCONTROL=ignorespace.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du stockage SQLite et de ses requêtes paramétrées.
    if c.raw_command.len() > 131_072 {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du stockage SQLite et de ses requêtes paramétrées.
        return Err("Commande trop longue (128 Kio maximum)".into());
    } // Évite les captures accidentelles massives.
    let a = classifier::analyze(c.raw_command.trim()); // Normalisation sans exécution.
    let hash = blake3::hash(a.raw.as_bytes()).to_hex().to_string(); // Déduplique sans perdre le badge sudo.
    let stamp = c.timestamp.unwrap_or_else(now); // Une date manquante est explicitement la date d'import.
    conn.execute("INSERT INTO commands(hash,raw,clean,binary,category,privileged,shell,cwd,terminal,exit_code,duration_ms,first_seen,last_seen) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?12) ON CONFLICT(hash) DO UPDATE SET count=count+1,last_seen=max(last_seen,excluded.last_seen),first_seen=min(first_seen,excluded.first_seen),shell=excluded.shell,cwd=excluded.cwd,terminal=excluded.terminal,exit_code=excluded.exit_code,duration_ms=excluded.duration_ms", params![hash,a.raw,a.clean,a.binary,a.category,a.privileged,c.shell,classifier::redact(&c.working_directory),c.terminal_emulator,c.exit_code,c.duration_ms,stamp])?; // Un seul upsert garde le compteur atomique.
    Ok(true) // Compte une occurrence, même si elle rejoint une entrée existante.
// @ligne Termine cette construction et revient au contexte parent du stockage SQLite et de ses requêtes paramétrées.
}
// Les recherches courtes utilisent LIKE ; FTS5 prend le relais à partir de trois caractères.
pub fn commands(query: &str, category: &str, favorites: bool, offset: i64) -> Result<Value> {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
    commands_filtered(query, category, favorites, offset, None, None)
} // Compatibilité CLI et appels existants.
// Les filtres supplémentaires restent paramétrés et peuvent être combinés aux favoris.
pub fn commands_filtered(
    // @ligne Renseigne le champ « query » de la structure utilisée ici du stockage SQLite et de ses requêtes paramétrées.
    query: &str,
    // @ligne Renseigne le champ « category » de la structure utilisée ici du stockage SQLite et de ses requêtes paramétrées.
    category: &str,
    // @ligne Renseigne le champ « favorites » de la structure utilisée ici du stockage SQLite et de ses requêtes paramétrées.
    favorites: bool,
    // @ligne Renseigne le champ « offset » de la structure utilisée ici du stockage SQLite et de ses requêtes paramétrées.
    offset: i64,
    // @ligne Renseigne le champ « privileged » de la structure utilisée ici du stockage SQLite et de ses requêtes paramétrées.
    privileged: Option<bool>,
    // @ligne Renseigne le champ « since » de la structure utilisée ici du stockage SQLite et de ses requêtes paramétrées.
    since: Option<i64>,
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
) -> Result<Value> {
    let conn = open()?; // Connexion locale uniquement.
    let indexed = query.chars().count() >= 3; // Le tokenizer trigram ne recherche pas un ou deux caractères.
    // @ligne Prépare « condition » pour la prochaine étape du stockage SQLite et de ses requêtes paramétrées.
    let condition = if indexed {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        "id IN (SELECT rowid FROM command_fts WHERE command_fts MATCH ?1)"
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
    } else {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        "raw LIKE ?1 ESCAPE '\\'"
    }; // Aucune interpolation de contenu utilisateur.
    // @ligne Prépare « search » pour la prochaine étape du stockage SQLite et de ses requêtes paramétrées.
    let search = if indexed {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        format!("\"{}\"", query.replace('"', "\"\""))
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
    } else {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        format!(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            "%{}%",
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            query
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
                .replace('\\', "\\\\")
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
                .replace('%', "\\%")
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
                .replace('_', "\\_")
        // @ligne Termine cette construction et revient au contexte parent du stockage SQLite et de ses requêtes paramétrées.
        )
    }; // Traite la recherche comme du texte littéral.
    let filter = format!("{condition} AND (?2='' OR category=?2) AND (?3=0 OR favorite=1) AND (?5 IS NULL OR privileged=?5) AND (?6 IS NULL OR last_seen>=?6)"); // Les filtres utilisent toujours des paramètres liés.
    // @ligne Prépare « total » pour la prochaine étape du stockage SQLite et de ses requêtes paramétrées.
    let total: i64 = conn.query_row(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        &format!("SELECT count(*) FROM commands WHERE {filter}"),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        params![
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            search,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            category,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            favorites,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            offset.max(0),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            privileged,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
            since
        // @ligne Termine cette construction et revient au contexte parent du stockage SQLite et de ses requêtes paramétrées.
        ],
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        |r| r.get(0),
    )?; // Total pour la pagination.
    let mut stmt = conn.prepare(&format!("SELECT id,raw,clean,binary,category,privileged,shell,cwd,terminal,count,favorite,last_seen,exit_code,duration_ms FROM commands WHERE {filter} ORDER BY last_seen DESC,id DESC LIMIT 100 OFFSET ?4"))?; // Plafond de résultats par page.
    let rows = stmt.query_map(params![search,category,favorites,offset.max(0),privileged,since], |r| Ok(json!({"id":r.get::<_,i64>(0)?,"raw":r.get::<_,String>(1)?,"clean":r.get::<_,String>(2)?,"binary":r.get::<_,String>(3)?,"category":r.get::<_,String>(4)?,"privileged":r.get::<_,bool>(5)?,"shell":r.get::<_,String>(6)?,"cwd":r.get::<_,String>(7)?,"terminal":r.get::<_,String>(8)?,"count":r.get::<_,i64>(9)?,"favorite":r.get::<_,bool>(10)?,"last_seen":r.get::<_,i64>(11)?,"exit_code":r.get::<_,Option<i32>>(12)?,"duration_ms":r.get::<_,Option<i64>>(13)?})))?.collect::<std::result::Result<Vec<_>,_>>()?; // Propage une éventuelle erreur de lecture.
    Ok(json!({"items":rows,"total":total,"offset":offset})) // Réponse stable pour React et la CLI.
// @ligne Termine cette construction et revient au contexte parent du stockage SQLite et de ses requêtes paramétrées.
}
// Les snapshots ne remplacent jamais les commandes de l'utilisateur.
pub fn snapshot(kind: &str, key: &str, payload: &Value) -> Result<()> {
    open()?.execute("INSERT INTO snapshots VALUES(?1,?2,?3,?4) ON CONFLICT(kind,key) DO UPDATE SET payload=excluded.payload,captured_at=excluded.captured_at", params![kind,key,classifier::redact_json(payload).to_string(),now()])?; // Filtre aussi les secrets des inspecteurs Docker.
    Ok(()) // L'upsert conserve une capture par objet.
// @ligne Termine cette construction et revient au contexte parent du stockage SQLite et de ses requêtes paramétrées.
}
// Restitue les captures anciennes même si le service d'origine est inaccessible.
pub fn snapshots(kind: &str) -> Result<Value> {
    let conn = open()?; // Connexion indépendante pour éviter les verrous longs.
    // @ligne Prépare « stmt » pour la prochaine étape du stockage SQLite et de ses requêtes paramétrées.
    let mut stmt = conn.prepare(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        "SELECT payload,captured_at FROM snapshots WHERE kind=?1 ORDER BY captured_at DESC",
    )?; // Ordre du plus récent au plus ancien.
    // @ligne Prépare « rows » pour la prochaine étape du stockage SQLite et de ses requêtes paramétrées.
    let rows = stmt
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du stockage SQLite et de ses requêtes paramétrées.
        .query_map([kind], |r| {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du stockage SQLite et de ses requêtes paramétrées.
            Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?))
        // @ligne Termine cette construction et revient au contexte parent du stockage SQLite et de ses requêtes paramétrées.
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?; // Copie les résultats avant fermeture.
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du stockage SQLite et de ses requêtes paramétrées.
    Ok(Value::Array(rows.into_iter().map(|(p,t)| json!({"data":serde_json::from_str::<Value>(&p).unwrap_or(Value::Null),"captured_at":t})).collect()))
    // Un snapshot invalide reste identifié sans paniquer.
}
// Les timestamps Unix évitent toute dépendance à un fuseau horaire particulier.
pub fn now() -> i64 {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
    std::time::SystemTime::now()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        .duration_since(std::time::UNIX_EPOCH)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        .unwrap_or_default()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du stockage SQLite et de ses requêtes paramétrées.
        .as_secs() as i64
} // L'interface assure le formatage local.
