//! Serveur LAN séparé de l'IPC administrateur : les pairs n'accèdent jamais au Vault.
use crate::{db, Result}; // Les erreurs internes deviennent des messages contextualisés.
// @ligne Importe les outils nécessaires à la mise en œuvre du partage HTTPS et des permissions serveur.
use axum::{
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    body::Body,
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    extract::{
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        ws::{Message, WebSocketUpgrade},
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        DefaultBodyLimit, Json, Path, State,
    // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
    },
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    http::{header, HeaderMap, StatusCode},
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    response::{Html, IntoResponse, Response},
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    routing::{get, post, put},
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    Router,
}; // Routes strictement limitées au partage.
use rand::Rng; // Génère des identifiants imprévisibles avec le générateur système initialisé par rand.
use serde::{Deserialize, Serialize}; // Les rôles et sessions sont sérialisables pour le panneau maître.
use serde_json::{json, Value}; // Messages WebSocket d'état.
// @ligne Importe les outils nécessaires à la mise en œuvre du partage HTTPS et des permissions serveur.
use std::{
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    collections::HashMap,
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    path::PathBuf,
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    sync::{Arc, Mutex},
}; // L'état reste possédé par le processus hôte.
use tokio::{sync::broadcast, task::JoinHandle}; // Notifications de révocation et arrêt du serveur.
// @ligne Importe les outils nécessaires à la mise en œuvre du partage HTTPS et des permissions serveur.
use tower_http::services::fs::ServeFileSystemResponseBody;
use tower_http::services::ServeFile; // Prend en charge Range et le streaming sans charger les fichiers en mémoire. // Type documentant le flux de fichier fourni par tower.
// Les limites restent configurables mais ne peuvent supprimer toute réserve disque.
#[derive(Clone, Serialize, Deserialize)]
// @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du partage HTTPS et des permissions serveur.
#[serde(default)]
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
pub struct Limits {
    // @ligne Renseigne le champ « max_file_bytes » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    pub max_file_bytes: u64,
    // @ligne Renseigne le champ « max_room_bytes » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    pub max_room_bytes: u64,
    // @ligne Renseigne le champ « reserve_bytes » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    pub reserve_bytes: u64,
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
impl Default for Limits {
    // @ligne Déclare « share », le point d’entrée de cette opération du partage HTTPS et des permissions serveur.
    fn default() -> Self {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        Self {
            // @ligne Renseigne le champ « max_file_bytes » de la structure utilisée ici du partage HTTPS et des permissions serveur.
            max_file_bytes: 2 * 1024 * 1024 * 1024,
            // @ligne Renseigne le champ « max_room_bytes » de la structure utilisée ici du partage HTTPS et des permissions serveur.
            max_room_bytes: 10 * 1024 * 1024 * 1024,
            // @ligne Renseigne le champ « reserve_bytes » de la structure utilisée ici du partage HTTPS et des permissions serveur.
            reserve_bytes: 64 * 1024 * 1024,
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        }
    // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
    }
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
impl Limits {
    // @ligne Déclare « share », le point d’entrée de cette opération du partage HTTPS et des permissions serveur.
    fn validate(&self) -> Result<()> {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
        if self.max_file_bytes == 0
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            || self.max_room_bytes < self.max_file_bytes
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            || self.max_room_bytes > 10 * 1024u64.pow(4)
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            || self.reserve_bytes < 16 * 1024 * 1024
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
            return Err(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                "Quotas invalides : salle ≥ fichier, réserve ≥ 16 Mio, salle ≤ 10 Tio".into(),
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            );
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        }
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        Ok(())
    // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
    }
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
// Un jeton long est utilisé après connexion ; le PIN n'est jamais un jeton de téléchargement.
fn token() -> String {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    rand::rng()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .random::<[u8; 32]>()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .iter()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map(|b| format!("{b:02x}"))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .collect()
} // 256 bits aléatoires.
#[derive(Clone, Serialize, Deserialize)] // Les jetons ne sont jamais exposés dans la liste des pairs.
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
pub struct Peer {
    // @ligne Renseigne le champ « id » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    pub id: String,
    // @ligne Renseigne le champ « name » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    pub name: String,
    // @ligne Renseigne le champ « upload » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    pub upload: bool,
    // @ligne Renseigne le champ « banned » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    pub banned: bool,
    // @ligne Renseigne le champ « last_seen » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    pub last_seen: i64,
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du partage HTTPS et des permissions serveur.
    #[serde(skip)]
    // @ligne Renseigne le champ « token » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    pub token: String,
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du partage HTTPS et des permissions serveur.
    #[serde(default)]
    pub downloaded_bytes: u64, // Octets réellement émis dans les flux HTTP.
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du partage HTTPS et des permissions serveur.
    #[serde(default)]
    pub uploaded_bytes: u64, // Blocs validés et synchronisés sur disque.
} // Le rôle est relu côté serveur à chaque requête.
#[derive(Clone)] // Les handlers partagent la même session active.
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
struct Room {
    limits: Limits,  // La même politique est appliquée à chaque bloc.
    id: String,      // Identifiant durable de la salle, distinct du code de connexion.
    expires_at: i64, // Une session expirée doit être recréée par son hôte.
    // @ligne Renseigne le champ « pin » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    pin: String,
    files: Arc<Mutex<Vec<PathBuf>>>, // Allowlist enrichie uniquement après finalisation d’une contribution.
    names: Arc<Mutex<HashMap<String, String>>>, // Noms de présentation séparés des chemins de stockage.
    // @ligne Renseigne le champ « upload_dir » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    upload_dir: PathBuf,
    // @ligne Renseigne le champ « peers » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    peers: Arc<Mutex<HashMap<String, Peer>>>,
    // @ligne Renseigne le champ « events » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    events: broadcast::Sender<String>,
    // @ligne Renseigne le champ « upload_lock » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    upload_lock: Arc<Mutex<()>>,
    // @ligne Renseigne le champ « attempts » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    attempts: Arc<Mutex<(i64, u32)>>,
    stored: Arc<Mutex<StoredRoom>>, // Configuration privée utilisée pour restaurer la même origine HTTPS.
} // Le compteur limite les essais sur le PIN.
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
pub struct Host {
    // @ligne Renseigne le champ « room » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    room: Room,
    // @ligne Renseigne le champ « info » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    pub info: Value,
    // @ligne Renseigne le champ « task » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    task: JoinHandle<()>,
    // @ligne Renseigne le champ « server » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    server: axum_server::Handle,
    // @ligne Renseigne le champ « discovery » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    discovery: Option<mdns_sd::ServiceDaemon>,
} // Le propriétaire peut arrêter explicitement la salle.
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
impl Drop for Host {
    // @ligne Déclare « share », le point d’entrée de cette opération du partage HTTPS et des permissions serveur.
    fn drop(&mut self) {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        self.server.shutdown();
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        self.task.abort();
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
        if let Some(mdns) = &self.discovery {
            // @ligne Prépare « _ » pour la prochaine étape du partage HTTPS et des permissions serveur.
            let _ = mdns.shutdown();
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        }
    // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
    }
} // Quitter l'application coupe l'écoute LAN.
#[derive(Deserialize)] // Le client ne choisit jamais son rôle.
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
struct Join {
    // @ligne Renseigne le champ « pin » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    pin: String,
    // @ligne Renseigne le champ « name » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    name: String,
} // Nom uniquement descriptif, échappé dans le navigateur.
type HttpResult<T> = std::result::Result<T, (StatusCode, String)>; // Erreurs HTTP explicites sans stacktrace.
// La base ne conserve que les empreintes des jetons, jamais les secrets Bearer eux-mêmes.
#[derive(Clone, Serialize, Deserialize)]
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
struct StoredRoom {
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du partage HTTPS et des permissions serveur.
    #[serde(default)]
    limits: Limits, // Les anciennes sessions reprennent les limites initiales.
    // @ligne Renseigne le champ « id » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    id: String,
    // @ligne Renseigne le champ « ip » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    ip: String,
    // @ligne Renseigne le champ « port » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    port: u16,
    // @ligne Renseigne le champ « paths » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    paths: Vec<String>,
    // @ligne Renseigne le champ « pin » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    pin: String,
    // @ligne Renseigne le champ « upload_dir » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    upload_dir: PathBuf,
    // @ligne Renseigne le champ « expires_at » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    expires_at: i64,
    // @ligne Renseigne le champ « peers » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    peers: Vec<(Peer, String)>,
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du partage HTTPS et des permissions serveur.
    #[serde(default)]
    // @ligne Renseigne le champ « names » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    names: HashMap<String, String>,
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
// Les droits et les jetons hachés sont validés en une seule écriture SQLite.
fn persist(room: &Room) -> Result<()> {
    // @ligne Prépare « peers » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let peers = room
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .peers
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .lock()
        .map_err(|_| "État des pairs inaccessible")?; // Un instantané cohérent des permissions est nécessaire à la reprise.
    // @ligne Prépare « stored » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let mut stored = room
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .stored
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .lock()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map_err(|_| "Configuration de salle inaccessible")?
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .clone();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    stored.paths = room
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .files
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .lock()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map_err(|_| "Fichiers inaccessibles")?
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .iter()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map(|p| p.to_string_lossy().into_owned())
        .collect(); // Inclut les contributions terminées dans la restauration.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    stored.names = room.names.lock().map_err(|_| "Noms inaccessibles")?.clone();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    stored.peers = peers
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .values()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .cloned()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map(|peer| {
            // @ligne Prépare « digest » pour la prochaine étape du partage HTTPS et des permissions serveur.
            let digest = peer.token.clone();
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            (peer, digest)
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        })
        .collect(); // La liste publique exclut toujours les secrets.
    db::open()?.execute("INSERT INTO share_rooms(id,payload,updated_at,active) VALUES(?1,?2,?3,1) ON CONFLICT(id) DO UPDATE SET payload=excluded.payload,updated_at=excluded.updated_at,active=1",rusqlite::params![room.id,serde_json::to_string(&stored)?,db::now()])?; // Paramètres liés et stockage privé utilisateur.
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
    Ok(())
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
// Le bureau propose uniquement les salles encore actives et non expirées.
pub fn saved() -> Result<Value> {
    // @ligne Prépare « conn » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let conn = db::open()?;
    // @ligne Prépare « stmt » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let mut stmt =
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        conn.prepare("SELECT payload FROM share_rooms WHERE active=1 ORDER BY updated_at DESC")?;
    // @ligne Prépare « rows » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let rows = stmt
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du partage HTTPS et des permissions serveur.
        .query_map([], |r| r.get::<_, String>(0))?
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .collect::<std::result::Result<Vec<_>, _>>()?;
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
    Ok(json!(rows.into_iter().filter_map(|raw|serde_json::from_str::<StoredRoom>(&raw).ok()).filter(|r|r.expires_at>db::now()).map(|r|json!({"id":r.id,"ip":r.ip,"port":r.port,"expires_at":r.expires_at,"files":r.paths.len()})).collect::<Vec<_>>()))
    // Ne révèle ni PIN ni empreintes de jetons dans une liste générique.
}
// Retrouve une salle sur son port précédent pour conserver localStorage, cookies et transferts.
pub async fn resume(id: &str) -> Result<Host> {
    // @ligne Prépare « conn » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let conn = db::open()?;
    // @ligne Prépare « raw » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let raw: String = conn.query_row(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        "SELECT payload FROM share_rooms WHERE id=?1 AND active=1",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        [id],
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        |r| r.get(0),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    )?;
    // @ligne Prépare « stored » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let stored: StoredRoom = serde_json::from_str(&raw)?;
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if stored.expires_at <= db::now() {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        return Err("Cette salle a expiré ; créez une nouvelle session".into());
    } // Un ancien token ne peut prolonger indéfiniment une salle.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    start_room(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        stored.paths.clone(),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        stored.ip.clone(),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        Some(stored.clone()),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        stored.limits,
    // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
    )
    .await // Un port devenu indisponible produit une erreur explicite.
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
// La découverte mDNS ne publie aucun PIN ; l'utilisateur ouvre ensuite une connexion TLS vérifiée.
pub async fn discover() -> Result<Value> {
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du partage HTTPS et des permissions serveur.
    tokio::task::spawn_blocking(||->Result<Value>{let mdns=mdns_sd::ServiceDaemon::new()?;let receiver=mdns.browse("_omnishell-share._tcp.local.")?;let deadline=std::time::Instant::now()+std::time::Duration::from_secs(3);let mut rooms=std::collections::BTreeMap::new();while std::time::Instant::now()<deadline{if let Ok(mdns_sd::ServiceEvent::ServiceResolved(service))=receiver.recv_timeout(std::time::Duration::from_millis(150)){for ip in service.get_addresses(){if let std::net::IpAddr::V4(ip)=ip{if ip.is_private()||ip.is_link_local(){rooms.insert(service.get_fullname().to_owned(),json!({"name":service.get_fullname(),"url":format!("https://{ip}:{}/",service.get_port())}));}}}}}let _=mdns.shutdown();Ok(json!(rooms.into_values().collect::<Vec<_>>()))}).await?
    // Le lien direct reste un repli lorsque multicast est filtré.
}
// Vérifie le jeton et l'état actuel pour appliquer une révocation immédiatement.
fn auth(room: &Room, headers: &HeaderMap, write: bool) -> HttpResult<Peer> {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if db::now() > room.expires_at {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        return Err((StatusCode::UNAUTHORIZED, "La salle a expiré".into()));
    } // La durée de vie est contrôlée à chaque requête.
    // @ligne Prépare « token » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let token = headers
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .get(header::AUTHORIZATION)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .and_then(|h| h.to_str().ok())
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .and_then(|s| s.strip_prefix("Bearer "))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .or_else(|| {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            headers
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                .get(header::COOKIE)
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                .and_then(|h| h.to_str().ok())
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                .and_then(|c| {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                    c.split("; ").find_map(|p| {
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                        p.strip_prefix(format!("omnishell_peer_{}=", &room.id[..16]).as_str())
                    // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
                    })
                // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
                })
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        })
        .ok_or((StatusCode::UNAUTHORIZED, "Connexion requise".into()))?; // Aucun token dans une URL de téléchargement.
    // @ligne Prépare « peers » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let mut peers = room.peers.lock().map_err(|_| {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        (
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            StatusCode::INTERNAL_SERVER_ERROR,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "État inaccessible".into(),
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        )
    })?; // Pas de panique en cas de verrou empoisonné.
    // @ligne Prépare « p » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let p = peers
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .values_mut()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .find(|p| p.token == blake3::hash(token.as_bytes()).to_hex().as_str() && !p.banned)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .ok_or((
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            StatusCode::UNAUTHORIZED,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "Session expirée ou révoquée".into(),
        ))?; // Un pair expulsé n'a plus d'accès.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if write && !p.upload {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        return Err((
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            StatusCode::FORBIDDEN,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "Le maître doit autoriser votre upload".into(),
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        ));
    } // Contrôle serveur indépendant de l'interface.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    p.last_seen = db::now();
    Ok(p.clone()) // Conserve un indicateur de dernière activité observée.
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
// Le PIN est limité globalement à trente essais par minute, sans dépendre d'une IP falsifiable.
async fn join(State(room): State<Room>, Json(input): Json<Join>) -> HttpResult<Response> {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if db::now() > room.expires_at {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        return Err((StatusCode::UNAUTHORIZED, "La salle a expiré".into()));
    } // Une admission ne prolonge pas la date d’expiration.
    // @ligne Prépare « attempts » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let mut attempts = room.attempts.lock().map_err(|_| {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        (
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            StatusCode::INTERNAL_SERVER_ERROR,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "Verrou indisponible".into(),
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        )
    })?; // Section critique courte.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if db::now() - attempts.0 >= 60 {
        *attempts = (db::now(), 0);
    } // Fenêtre glissante approximative d'une minute.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    attempts.1 += 1;
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if attempts.1 > 30 {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        return Err((
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            StatusCode::TOO_MANY_REQUESTS,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "Trop d'essais, patientez une minute".into(),
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        ));
    } // Borne l'attaque par force brute.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if input.pin != room.pin {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        return Err((StatusCode::UNAUTHORIZED, "Code incorrect".into()));
    } // Aucun détail sur la session tant que le PIN échoue.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if room
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .peers
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .lock()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map_err(|_| {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            (
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                StatusCode::INTERNAL_SERVER_ERROR,
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                "État inaccessible".into(),
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            )
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        })?
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .len()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        >= 512
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        return Err((
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            StatusCode::TOO_MANY_REQUESTS,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "Limite de 512 participants par salle".into(),
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        ));
    } // Borne les identités mémorisées.
    // @ligne Prépare « id » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let id = token();
    let access = token(); // Identité publique distincte du secret d'authentification.
    // @ligne Prépare « peer » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let peer = Peer {
        // @ligne Renseigne le champ « id » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        id: id.clone(),
        // @ligne Renseigne le champ « name » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        name: input.name.chars().take(80).collect(),
        // @ligne Renseigne le champ « upload » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        upload: false,
        // @ligne Renseigne le champ « banned » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        banned: false,
        // @ligne Renseigne le champ « last_seen » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        last_seen: db::now(),
        // @ligne Renseigne le champ « token » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        token: blake3::hash(access.as_bytes()).to_hex().to_string(),
        // @ligne Renseigne le champ « downloaded_bytes » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        downloaded_bytes: 0,
        // @ligne Renseigne le champ « uploaded_bytes » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        uploaded_bytes: 0,
    }; // Lecture seule à l'arrivée.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    room.peers
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .lock()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map_err(|_| {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            (
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                StatusCode::INTERNAL_SERVER_ERROR,
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                "Verrou indisponible".into(),
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            )
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        })?
        .insert(id.clone(), peer); // L'état reste côté maître.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    persist(&room).map_err(|_| {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        (
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            StatusCode::INTERNAL_SERVER_ERROR,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "Session non sauvegardée".into(),
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        )
    })?; // Une admission est durable avant émission du jeton.
    let _ = room.events.send("permissions".into()); // Le panneau et les clients peuvent actualiser leur vue.
    let mut response = Json(json!({"token":access,"id":id})).into_response(); // Le cookie est propre à la salle, même si plusieurs ports partagent la même adresse.
                                                                              // Le jeton sert aussi au canal WebSocket.
    response.headers_mut().insert(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        header::SET_COOKIE,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        format!(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "omnishell_peer_{}={access}; Path=/api; HttpOnly; Secure; SameSite=Strict; Max-Age={}",
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            &room.id[..16],
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            (room.expires_at - db::now()).max(0)
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        )
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .parse()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Cookie invalide".into()))?,
    ); // Le navigateur peut télécharger en streaming natif.
    Ok(response) // Le navigateur conserve le jeton uniquement pour cette salle.
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
// La liste est accessible uniquement après authentification.
async fn list_files(State(room): State<Room>, headers: HeaderMap) -> HttpResult<Json<Value>> {
    let peer = auth(&room, &headers, false)?; // Revalide les droits à chaque consultation.
    // @ligne Prépare « allowlist » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let allowlist = room.files.lock().map_err(|_| {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        (
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            StatusCode::INTERNAL_SERVER_ERROR,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "Fichiers inaccessibles".into(),
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        )
    // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
    })?;
    // @ligne Prépare « names » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let names = room.names.lock().map_err(|_| {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        (
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            StatusCode::INTERNAL_SERVER_ERROR,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "Noms inaccessibles".into(),
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        )
    // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
    })?;
    let files=allowlist.iter().enumerate().map(|(id,p)|json!({"id":id,"name":names.get(&p.to_string_lossy().into_owned()).cloned().unwrap_or_else(||p.file_name().unwrap_or_default().to_string_lossy().into_owned()),"size":std::fs::metadata(p).map(|m|m.len()).unwrap_or(0)})).collect::<Vec<_>>(); // Ne révèle aucun chemin absolu.
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
    Ok(Json(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        json!({"files":files,"upload":peer.upload,"peer":peer.id}),
    )) // Le client adapte son interface au rôle actuel.
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
// Le téléchargement ne reçoit qu'un identifiant numérique, jamais un chemin fourni par le pair.
async fn download(
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    State(room): State<Room>,
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    Path(id): Path<usize>,
    // @ligne Renseigne le champ « headers » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    headers: HeaderMap,
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
) -> HttpResult<Response> {
    let peer = auth(&room, &headers, false)?; // Vérifie aussi les requêtes Range de reprise.
    // @ligne Prépare « path » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let path = room
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .files
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .lock()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map_err(|_| {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            (
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                StatusCode::INTERNAL_SERVER_ERROR,
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                "Fichiers inaccessibles".into(),
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            )
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        })?
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .get(id)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .cloned()
        .ok_or((StatusCode::NOT_FOUND, "Fichier inconnu".into()))?; // Copie le chemin avant toute attente réseau.
    let request_headers = headers.clone(); // Chaque segment du flux peut relire une éventuelle expulsion.
    // @ligne Prépare « request » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let request = axum::http::Request::builder()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .uri("/")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .body(Body::empty())
        .map_err(|_| (StatusCode::BAD_REQUEST, "Requête invalide".into()))?; // Le service de fichier ne reçoit pas le chemin de l'URL.
    // @ligne Prépare les valeurs locales pour la prochaine étape du partage HTTPS et des permissions serveur.
    let (mut parts, body) = request.into_parts();
    parts.headers = headers; // Transmet Range et les conditions HTTP.
    // @ligne Prépare « result » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let result: axum::http::Response<ServeFileSystemResponseBody> = ServeFile::new(&path)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .try_call(axum::http::Request::from_parts(parts, body))
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du partage HTTPS et des permissions serveur.
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "Fichier devenu inaccessible".into()))?; // Flux et 206 gérés par tower-http.
    // @ligne Prépare « display_name » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let display_name = room
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .names
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .lock()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .ok()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .and_then(|names| names.get(&path.to_string_lossy().into_owned()).cloned())
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .unwrap_or_else(|| {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            path.file_name()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                .unwrap_or_default()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                .to_string_lossy()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                .into_owned()
        }); // Un nom non fiable sera encodé, jamais concaténé à un en-tête brut.
    use futures_util::StreamExt; // Le flux reste borné et peut être interrompu après révocation.
    // @ligne Prépare « response » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let mut response = result.map(|body| {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        Body::from_stream(Body::new(body).into_data_stream().map(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            move |chunk| -> std::result::Result<_, std::io::Error> {
                // @ligne Prépare « bytes » pour la prochaine étape du partage HTTPS et des permissions serveur.
                let bytes = chunk.map_err(std::io::Error::other)?;
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                auth(&room, &request_headers, false).map_err(|_| {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                    std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Accès révoqué")
                // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
                })?;
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
                if let Ok(mut peers) = room.peers.lock() {
                    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
                    if let Some(peer) = peers.get_mut(&peer.id) {
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                        peer.downloaded_bytes =
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                            peer.downloaded_bytes.saturating_add(bytes.len() as u64);
                    // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
                    }
                } // Mesure les octets réellement servis, y compris les reprises.
                // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
                Ok(bytes)
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            },
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        ))
    }); // Adapte le corps au routeur Axum.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    response.headers_mut().insert(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        header::CONTENT_DISPOSITION,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        format!(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "attachment; filename*=UTF-8''{}",
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            percent_encoding::utf8_percent_encode(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                &display_name,
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                percent_encoding::NON_ALPHANUMERIC
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            )
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        )
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .parse()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map_err(|_| {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            (
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                StatusCode::INTERNAL_SERVER_ERROR,
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                "Nom de téléchargement invalide".into(),
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            )
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        })?,
    ); // Empêche l'exécution d'un HTML partagé dans l'origine du serveur.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    response
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .headers_mut()
        .insert(header::X_CONTENT_TYPE_OPTIONS, "nosniff".parse().unwrap()); // Les fichiers restent des téléchargements.
    Ok(response) // Le gros fichier n'est jamais matérialisé dans une Vec.
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
// Upload par blocs de 4 Mio : le serveur impose le décalage et un nom généré.
async fn upload(
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    State(room): State<Room>,
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    Path(id): Path<String>,
    // @ligne Renseigne le champ « headers » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    headers: HeaderMap,
    // @ligne Renseigne le champ « body » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    body: axum::body::Bytes,
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
) -> HttpResult<Json<Value>> {
    // @ligne Prépare « _upload_guard » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let _upload_guard = room
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .upload_lock
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Transfert occupé".into()))?; // Sérialise les blocs pour interdire les offsets concurrents.
    let peer = auth(&room, &headers, true)?; // Une révocation interdit immédiatement le prochain bloc.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if id.is_empty()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        || id.len() > 80
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        || !id
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .chars()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        return Err((
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            StatusCode::BAD_REQUEST,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "Identifiant de transfert invalide".into(),
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        ));
    } // Refuse séparateurs et traversée de chemins.
    // @ligne Prépare « offset » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let offset = headers
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .get("x-offset")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .and_then(|v| v.to_str().ok())
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .and_then(|v| v.parse::<u64>().ok())
        .ok_or((StatusCode::BAD_REQUEST, "En-tête X-Offset requis".into()))?; // Le client reprend au décalage confirmé.
    // @ligne Prépare « digest » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let digest = headers
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .get("x-blake3")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::BAD_REQUEST, "En-tête X-Blake3 requis".into()))?; // Chaque bloc est vérifiable indépendamment.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if blake3::hash(&body).to_hex().as_str() != digest {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        return Err((
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            StatusCode::BAD_REQUEST,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "Empreinte du bloc incorrecte".into(),
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        ));
    } // Rejette un bloc altéré avant écriture.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if offset.saturating_add(body.len() as u64) > room.limits.max_file_bytes {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        return Err((
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            StatusCode::PAYLOAD_TOO_LARGE,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            format!(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                "Limite de {} Mio par transfert",
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                room.limits.max_file_bytes / 1024 / 1024
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            ),
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        ));
    } // Quota par transfert.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if fs2::available_space(&room.upload_dir).unwrap_or(0)
        // @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React.
        < body.len() as u64 + room.limits.reserve_bytes
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        return Err((
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            StatusCode::INSUFFICIENT_STORAGE,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "La réserve disque configurée ne serait plus respectée".into(),
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        ));
    } // Garde une marge système avant toute écriture.
    // @ligne Prépare « received » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let received: u64 = std::fs::read_dir(&room.upload_dir)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map_err(|_| {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            (
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                StatusCode::INSUFFICIENT_STORAGE,
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                "Dossier inaccessible".into(),
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            )
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        })?
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .filter_map(|e| e.ok())
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .filter_map(|e| e.metadata().ok())
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map(|m| m.len())
        .sum(); // Quota agrégé de salle, pas seulement de fichier.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if received + body.len() as u64 > room.limits.max_room_bytes {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        return Err((
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            StatusCode::PAYLOAD_TOO_LARGE,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "Quota de salle atteint".into(),
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        ));
    } // Empêche les petits fichiers illimités.
    let path = room.upload_dir.join(format!("{}-{id}.upload", peer.id)); // Le nom du pair ne participe jamais au chemin.
    // @ligne Prépare « file » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let mut file = std::fs::OpenOptions::new()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .create(true)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .truncate(false)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .write(true)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .open(path)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map_err(|_| {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            (
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                StatusCode::INSUFFICIENT_STORAGE,
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                "Répertoire de réception inaccessible".into(),
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            )
        })?; // Aucun écrasement de fichier arbitraire.
    use std::io::{Seek, SeekFrom, Write}; // Position explicite pour la reprise.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if file
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .metadata()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map_err(|_| {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            (
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                StatusCode::INTERNAL_SERVER_ERROR,
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                "Taille inaccessible".into(),
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            )
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        })?
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .len()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        != offset
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        return Err((
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            StatusCode::CONFLICT,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "Décalage différent de la taille reçue".into(),
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        ));
    } // Refuse les trous et doublons.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    file.seek(SeekFrom::Start(offset))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .and_then(|_| file.write_all(&body))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .and_then(|_| file.sync_data())
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map_err(|_| {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            (
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                StatusCode::INSUFFICIENT_STORAGE,
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                "Écriture impossible : vérifier l'espace disque".into(),
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            )
        })?; // Signale ENOSPC sans prétendre avoir reçu le bloc.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if let Ok(mut peers) = room.peers.lock() {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
        if let Some(peer) = peers.get_mut(&peer.id) {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            peer.uploaded_bytes = peer.uploaded_bytes.saturating_add(body.len() as u64);
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        }
    } // Compte seulement les blocs effectivement durables.
    // @ligne Prépare « end » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let end = offset + body.len() as u64;
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if headers
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .get("x-total-size")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .and_then(|v| v.to_str().ok())
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .and_then(|v| v.parse::<u64>().ok())
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        == Some(end)
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    {
        // @ligne Prépare « name » pour la prochaine étape du partage HTTPS et des permissions serveur.
        let name = headers
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .get("x-file-name")
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .and_then(|v| v.to_str().ok())
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .and_then(|v| percent_encoding::percent_decode_str(v).decode_utf8().ok())
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .map(|s| s.into_owned())
            .unwrap_or_else(|| format!("contribution-{id}")); // Le nom original est une étiquette, jamais un chemin.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
        if name.len() > 512
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            || name
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                .chars()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                .any(|c| c.is_control() || c == '/' || c == '\\')
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
            return Err((StatusCode::BAD_REQUEST, "Nom de fichier invalide".into()));
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        }
        // @ligne Prépare « path » pour la prochaine étape du partage HTTPS et des permissions serveur.
        let path = room.upload_dir.join(format!("{}-{id}.upload", peer.id));
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        {
            // @ligne Prépare « files » pour la prochaine étape du partage HTTPS et des permissions serveur.
            let mut files = room.files.lock().map_err(|_| {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                (
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                    StatusCode::INTERNAL_SERVER_ERROR,
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                    "Fichiers inaccessibles".into(),
                // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
                )
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            })?;
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
            if !files.contains(&path) {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                files.push(path.clone());
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            }
        } // Publie une seule fois le fichier terminé à la classe.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        room.names
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .lock()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .map_err(|_| {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                (
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                    StatusCode::INTERNAL_SERVER_ERROR,
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                    "Noms inaccessibles".into(),
                // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
                )
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            })?
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .insert(path.to_string_lossy().into_owned(), name);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        persist(&room).map_err(|_| {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            (
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                StatusCode::INTERNAL_SERVER_ERROR,
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                "Contribution non sauvegardée".into(),
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            )
        })?; // Une reprise d'hôte retrouve aussi les fichiers publiés.
        let _ = room.events.send("files".into()); // Les navigateurs actualisent la liste de classe.
    // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
    }
    Ok(Json(json!({"offset":offset+body.len() as u64}))) // Le client mémorise uniquement un bloc durable.
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
// La reprise utilise uniquement le nombre d'octets réellement présents sur disque.
async fn upload_status(
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    State(room): State<Room>,
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    Path(id): Path<String>,
    // @ligne Renseigne le champ « headers » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    headers: HeaderMap,
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
) -> HttpResult<Json<Value>> {
    let peer = auth(&room, &headers, true)?; // La reprise ne contourne pas une révocation.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if id.is_empty()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        || id.len() > 80
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        || !id
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .chars()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        return Err((StatusCode::BAD_REQUEST, "Identifiant invalide".into()));
    } // Même validation que l'écriture.
    // @ligne Prépare « offset » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let offset = std::fs::metadata(room.upload_dir.join(format!("{}-{id}.upload", peer.id)))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map(|m| m.len())
        .unwrap_or(0); // Un transfert absent commence à zéro.
    Ok(Json(json!({"offset":offset}))) // Ne révèle pas les transferts des autres pairs.
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
// Le WebSocket ne transporte pas de fichiers ni de privilèges déclarés par le client.
async fn ws_events(State(room): State<Room>, ws: WebSocketUpgrade) -> impl IntoResponse {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    ws.on_upgrade(move |mut socket| async move {
        // @ligne Prépare « access » pour la prochaine étape du partage HTTPS et des permissions serveur.
        let access =
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
            match tokio::time::timeout(std::time::Duration::from_secs(10), socket.recv()).await {
                // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
                Ok(Some(Ok(Message::Text(token)))) => token.to_string(),
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                _ => return,
            }; // Le premier message doit authentifier le canal.
        // @ligne Prépare « headers » pour la prochaine étape du partage HTTPS et des permissions serveur.
        let mut headers = HeaderMap::new();
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
        if let Ok(value) = format!("Bearer {access}").parse() {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            headers.insert(header::AUTHORIZATION, value);
        } // Réutilise le même contrôle que HTTP.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
        if auth(&room, &headers, false).is_err() {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
            return;
        } // Refuse les canaux anonymes.
        let mut events = room.events.subscribe(); // Notifications sans données privées ni liste de pairs.
        // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du partage HTTPS et des permissions serveur.
        while let Ok(event) = events.recv().await {
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
            if auth(&room, &headers, false).is_err()
                // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du partage HTTPS et des permissions serveur.
                || socket.send(Message::Text(event.into())).await.is_err()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                break;
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            }
        } // Une déconnexion termine la tâche.
    }) // Les clients réinterrogent /api/files avec leur Bearer après notification.
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
// Démarre un serveur à port dynamique ; aucune modification de pare-feu ou de profil shell.
pub async fn start(paths: Vec<String>, host_ip: String) -> Result<Host> {
    start_room(paths, host_ip, None, Limits::default()).await // Une nouvelle salle reçoit une identité et un PIN indépendants.
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
// La restauration réutilise le port et les droits, tandis que la préparation des archives reste hors runtime async.
async fn start_room(
    // @ligne Renseigne le champ « paths » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    paths: Vec<String>,
    // @ligne Renseigne le champ « host_ip » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    host_ip: String,
    // @ligne Renseigne le champ « restored » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    restored: Option<StoredRoom>,
    // @ligne Renseigne le champ « limits » de la structure utilisée ici du partage HTTPS et des permissions serveur.
    limits: Limits,
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
) -> Result<Host> {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    limits.validate()?;
    db::open()?; // Garantit le répertoire privé même avant la première capture.
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default(); // Fixe explicitement le moteur crypto même si un client de test active aussi ring.
    let ip: std::net::IpAddr = host_ip.parse()?; // Le QR doit pointer vers une adresse explicitement choisie.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if ip.is_unspecified() || ip.is_loopback() {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        return Err("Sélectionnez l'adresse LAN de votre interface".into());
    } // Évite un QR inutilisable par les autres appareils.
    // @ligne Prépare « id » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let id = restored
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .as_ref()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map(|r| r.id.clone())
        .unwrap_or_else(token); // Ne change pas lors d'une reprise.
    let original_paths = paths.clone(); // Les dossiers sont reconstruits depuis leur source lors d'une reprise.
    // @ligne Prépare « archive_id » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let archive_id = id.clone();
    // @ligne Prépare « files » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let files =
        tokio::task::spawn_blocking(move || crate::archives::prepare(paths, &archive_id)).await??; // La compression n'immobilise pas un thread de serveur.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if files.is_empty() {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        return Err("Sélectionnez au moins un fichier ou dossier".into());
    // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
    }
    // @ligne Prépare « pin » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let pin = restored
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .as_ref()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map(|r| r.pin.clone())
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .unwrap_or_else(|| format!("{:06}", rand::rng().random_range(0..1_000_000)));
    // @ligne Prépare « upload_dir » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let upload_dir = restored
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .as_ref()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map(|r| r.upload_dir.clone())
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .unwrap_or_else(|| db::path().with_file_name("uploads").join(&id));
    std::fs::create_dir_all(&upload_dir)?; // Les contributions précédentes sont conservées lors d'une reprise.
    // @ligne Prépare « expires_at » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let expires_at = restored
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .as_ref()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map(|r| r.expires_at)
        .unwrap_or_else(|| db::now() + 24 * 60 * 60); // Expiration après 24 heures par défaut.
    let mut restored_peers = HashMap::new(); // Les pairs expulsés restent expulsés après relance.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if let Some(saved) = &restored {
        // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du partage HTTPS et des permissions serveur.
        for (peer, digest) in &saved.peers {
            // @ligne Prépare « peer » pour la prochaine étape du partage HTTPS et des permissions serveur.
            let mut peer = peer.clone();
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            peer.token = digest.clone();
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            restored_peers.insert(peer.id.clone(), peer);
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        }
    // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
    }
    // @ligne Prépare « stored » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let stored = restored.clone().unwrap_or(StoredRoom {
        // @ligne Renseigne le champ « id » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        id: id.clone(),
        // @ligne Renseigne le champ « ip » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        ip: host_ip.clone(),
        // @ligne Renseigne le champ « port » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        port: 0,
        // @ligne Renseigne le champ « paths » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        paths: original_paths,
        // @ligne Renseigne le champ « pin » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        pin: pin.clone(),
        // @ligne Renseigne le champ « upload_dir » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        upload_dir: upload_dir.clone(),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        expires_at,
        // @ligne Renseigne le champ « limits » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        limits: limits.clone(),
        // @ligne Renseigne le champ « peers » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        peers: vec![],
        // @ligne Renseigne le champ « names » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        names: HashMap::new(),
    }); // La configuration n'est jamais exposée sur le port LAN.
    let (events, _) = broadcast::channel(64); // Les notifications lentes peuvent être abandonnées sans bloquer les transferts.
    let restored_names = stored.names.clone(); // Copie les noms avant de transférer la configuration dans son verrou.
    // @ligne Prépare « room » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let room = Room {
        // @ligne Renseigne le champ « limits » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        limits: limits.clone(),
        // @ligne Renseigne le champ « id » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        id: id.clone(),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        expires_at,
        // @ligne Renseigne le champ « stored » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        stored: Arc::new(Mutex::new(stored)),
        // @ligne Renseigne le champ « pin » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        pin: pin.clone(),
        // @ligne Renseigne le champ « files » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        files: Arc::new(Mutex::new(files)),
        // @ligne Renseigne le champ « names » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        names: Arc::new(Mutex::new(restored_names)),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        upload_dir,
        // @ligne Renseigne le champ « peers » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        peers: Arc::new(Mutex::new(restored_peers)),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        events,
        // @ligne Renseigne le champ « upload_lock » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        upload_lock: Arc::new(Mutex::new(())),
        // @ligne Renseigne le champ « attempts » de la structure utilisée ici du partage HTTPS et des permissions serveur.
        attempts: Arc::new(Mutex::new((db::now(), 0))),
    }; // État propre à cette salle.
    // @ligne Prépare « app » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let app = Router::new()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .route(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "/",
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            get(|| async { Html(include_str!("../../src/share-client.html")) }),
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        )
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .route(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "/client.js",
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            get(|| async {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                (
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                    [(header::CONTENT_TYPE, "text/javascript")],
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
                    include_str!("../../assets/share-client.js"),
                // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
                )
            // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
            }),
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        )
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .route("/api/join", post(join))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .route("/api/files", get(list_files))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .route("/api/files/{id}", get(download))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .route("/api/upload/{id}", put(upload).get(upload_status))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .route("/events", get(ws_events))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .layer(DefaultBodyLimit::max(4 * 1024 * 1024))
        .with_state(room.clone()); // Aucun endpoint d'administration exposé sur LAN.
    // @ligne Prépare « listener » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let listener =
        tokio::net::TcpListener::bind((ip, restored.as_ref().map(|r| r.port).unwrap_or(0))).await?; // Écoute uniquement sur l'interface choisie.
    let address = listener.local_addr()?; // Le port retenu est celui réellement alloué par le noyau.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    room.stored
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .lock()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .map_err(|_| "Configuration inaccessible")?
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .port = address.port();
    persist(&room)?; // Le port choisi est durable avant d’annoncer la salle.
    let url = format!("https://{address}/"); // Le PIN ne circule jamais en HTTP clair.
    // @ligne Prépare « tls_dir » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let tls_dir = db::path().parent().unwrap().join("tls");
    std::fs::create_dir_all(&tls_dir)?; // Le certificat reste dans le répertoire privé utilisateur.
    // @ligne Prépare « cert_path » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let cert_path = tls_dir.join(format!("{ip}.pem"));
    let key_path = tls_dir.join(format!("{ip}.key")); // Un certificat stable par adresse réduit les changements d'empreinte.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if !cert_path.exists() || !key_path.exists() {
        // @ligne Prépare « certified » pour la prochaine étape du partage HTTPS et des permissions serveur.
        let certified =
            rcgen::generate_simple_self_signed(vec![ip.to_string(), "omnishell.local".into()])?; // Certificat local, aucune autorité cloud.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        std::fs::write(&cert_path, certified.cert.pem())?;
        std::fs::write(&key_path, certified.key_pair.serialize_pem())?; // La clé ne quitte pas le poste hôte.
    // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
    }
    // @ligne Prépare « fingerprint » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let fingerprint = blake3::hash(&std::fs::read(&cert_path)?)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .to_hex()
        .to_string(); // Empreinte du PEM à comparer via un canal connu.
    let tls = axum_server::tls_rustls::RustlsConfig::from_pem_file(&cert_path, &key_path).await?; // Le serveur annonce HTTP/1.1 et HTTP/2 via ALPN.
    // @ligne Prépare « qr » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let qr = qrcode::QrCode::new(url.as_bytes())?
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .render::<qrcode::render::svg::Color>()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        .min_dimensions(220, 220)
        .build(); // QR généré entièrement hors ligne.
    let discovery = mdns_sd::ServiceDaemon::new().ok(); // Le lien explicite reste valable si multicast interdit.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
    if let Some(mdns) = &discovery {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
        if let Ok(service) = mdns_sd::ServiceInfo::new(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "_omnishell-share._tcp.local.",
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            &format!("OmniShell {}", &id[..8]),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "omnishell.local.",
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            ip.to_string(),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            address.port(),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            [("tls", "true")].as_slice(),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        ) {
            // @ligne Prépare « _ » pour la prochaine étape du partage HTTPS et des permissions serveur.
            let _ = mdns.register(service);
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        }
    } // Annonce uniquement adresse et port, jamais le PIN.
    // @ligne Prépare « server » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let server = axum_server::Handle::new();
    let shutdown = server.clone(); // Drop peut aussi couper les connexions ouvertes.
    let std_listener = listener.into_std()?; // Toute erreur de conversion est retournée sans panique.
    // @ligne Prépare « task » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let task = tokio::spawn(async move {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du partage HTTPS et des permissions serveur.
        if let Err(e) = axum_server::from_tcp_rustls(std_listener, tls)
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .handle(shutdown)
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .serve(app.into_make_service())
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du partage HTTPS et des permissions serveur.
            .await
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            eprintln!("ClassShare : {e}");
        // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
        }
    }); // Serveur TLS isolé de l'IPC maître.
    // @ligne Prépare « info » pour la prochaine étape du partage HTTPS et des permissions serveur.
    let info = json!({"id":id,"limits":limits,"expires_at":expires_at,"url":url,"pin":pin,"qr":qr,"fingerprint":fingerprint,"upload_dir":room.upload_dir,"certificate":cert_path,"transport":"TLS auto-signé : valider le certificat de l'hôte à la première visite"});
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
    Ok(Host {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        room,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        info,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        task,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        server,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        discovery,
    }) // La confiance du certificat doit être établie par les participants.
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
impl Host {
    // Fermer volontairement une salle invalide sa restauration ; un simple arrêt du processus permet la reprise.
    pub fn close(&self) -> Result<()> {
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du partage HTTPS et des permissions serveur.
        db::open()?.execute(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            "UPDATE share_rooms SET active=0 WHERE id=?1",
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            [&self.room.id],
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        )?;
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        self.server.shutdown();
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        Ok(())
    // @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
    }
    // Le panneau maître est accessible uniquement par IPC natif, jamais sur le port LAN.
    pub fn peers(&self) -> Result<Value> {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        Ok(json!(self
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .room
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .peers
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .lock()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .map_err(|_| "État des pairs inaccessible")?
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .values()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .cloned()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .collect::<Vec<_>>()))
    } // serde exclut les jetons.
      // Chaque changement est relu dès la requête HTTP suivante.
    pub fn permission(&self, id: &str, upload: bool, banned: bool) -> Result<()> {
        // @ligne Prépare « peers » pour la prochaine étape du partage HTTPS et des permissions serveur.
        let mut peers = self
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .room
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .peers
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .lock()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
            .map_err(|_| "État des pairs inaccessible")?;
        // @ligne Prépare « p » pour la prochaine étape du partage HTTPS et des permissions serveur.
        let p = peers.get_mut(id).ok_or("Pair inconnu")?;
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        p.upload = upload;
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
        p.banned = banned;
        drop(peers); // Libère le verrou avant de sauvegarder son nouvel état.
        persist(&self.room)?; // Une révocation reste effective après redémarrage de l’hôte.
        // @ligne Prépare « _ » pour la prochaine étape du partage HTTPS et des permissions serveur.
        let _ = self.room.events.send("permissions".into());
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du partage HTTPS et des permissions serveur.
        Ok(())
    } // La révocation ne dépend pas d'un JWT déjà émis.
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
// La fenêtre peut choisir ses limites sans changer les valeurs par défaut de la CLI.
pub async fn start_with_limits(paths: Vec<String>, ip: String, limits: Limits) -> Result<Host> {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du partage HTTPS et des permissions serveur.
    limits.validate()?;
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du partage HTTPS et des permissions serveur.
    start_room(paths, ip, None, limits).await
// @ligne Termine cette construction et revient au contexte parent du partage HTTPS et des permissions serveur.
}
