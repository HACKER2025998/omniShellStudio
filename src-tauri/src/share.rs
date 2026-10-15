//! Serveur LAN séparé de l'IPC administrateur : les pairs n'accèdent jamais au Vault.
use crate::{db, Result}; // Les erreurs internes deviennent des messages contextualisés.
use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocketUpgrade},
        DefaultBodyLimit, Json, Path, State,
    },
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post, put},
    Router,
}; // Routes strictement limitées au partage.
use rand::Rng; // Génère des identifiants imprévisibles avec le générateur système initialisé par rand.
use serde::{Deserialize, Serialize}; // Les rôles et sessions sont sérialisables pour le panneau maître.
use serde_json::{json, Value}; // Messages WebSocket d'état.
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
}; // L'état reste possédé par le processus hôte.
use tokio::{sync::broadcast, task::JoinHandle}; // Notifications de révocation et arrêt du serveur.
use tower_http::services::fs::ServeFileSystemResponseBody;
use tower_http::services::ServeFile; // Prend en charge Range et le streaming sans charger les fichiers en mémoire. // Type documentant le flux de fichier fourni par tower.

// Un jeton long est utilisé après connexion ; le PIN n'est jamais un jeton de téléchargement.
fn token() -> String {
    rand::rng()
        .random::<[u8; 32]>()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
} // 256 bits aléatoires.
#[derive(Clone, Serialize)] // Les jetons ne sont jamais exposés dans la liste des pairs.
pub struct Peer {
    pub id: String,
    pub name: String,
    pub upload: bool,
    pub banned: bool,
    pub last_seen: i64,
    #[serde(skip)]
    pub token: String,
} // Le rôle est relu côté serveur à chaque requête.
#[derive(Clone)] // Les handlers partagent la même session active.
struct Room {
    pin: String,
    files: Arc<Vec<PathBuf>>,
    upload_dir: PathBuf,
    peers: Arc<Mutex<HashMap<String, Peer>>>,
    events: broadcast::Sender<String>,
    upload_lock: Arc<Mutex<()>>,
    attempts: Arc<Mutex<(i64, u32)>>,
} // Le compteur limite les essais sur le PIN.
pub struct Host {
    room: Room,
    pub info: Value,
    task: JoinHandle<()>,
    server: axum_server::Handle,
    discovery: Option<mdns_sd::ServiceDaemon>,
} // Le propriétaire peut arrêter explicitement la salle.
impl Drop for Host {
    fn drop(&mut self) {
        self.server.shutdown();
        self.task.abort();
        if let Some(mdns) = &self.discovery {
            let _ = mdns.shutdown();
        }
    }
} // Quitter l'application coupe l'écoute LAN.
#[derive(Deserialize)] // Le client ne choisit jamais son rôle.
struct Join {
    pin: String,
    name: String,
} // Nom uniquement descriptif, échappé dans le navigateur.
type HttpResult<T> = std::result::Result<T, (StatusCode, String)>; // Erreurs HTTP explicites sans stacktrace.

// Vérifie le jeton et l'état actuel pour appliquer une révocation immédiatement.
fn auth(room: &Room, headers: &HeaderMap, write: bool) -> HttpResult<Peer> {
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .or_else(|| {
            headers
                .get(header::COOKIE)
                .and_then(|h| h.to_str().ok())
                .and_then(|c| {
                    c.split("; ")
                        .find_map(|p| p.strip_prefix("omnishell_peer="))
                })
        })
        .ok_or((StatusCode::UNAUTHORIZED, "Connexion requise".into()))?; // Aucun token dans une URL de téléchargement.
    let mut peers = room.peers.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "État inaccessible".into(),
        )
    })?; // Pas de panique en cas de verrou empoisonné.
    let p = peers
        .values_mut()
        .find(|p| p.token == token && !p.banned)
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "Session expirée ou révoquée".into(),
        ))?; // Un pair expulsé n'a plus d'accès.
    if write && !p.upload {
        return Err((
            StatusCode::FORBIDDEN,
            "Le maître doit autoriser votre upload".into(),
        ));
    } // Contrôle serveur indépendant de l'interface.
    p.last_seen = db::now();
    Ok(p.clone()) // Conserve un indicateur de dernière activité observée.
}

// Le PIN est limité globalement à trente essais par minute, sans dépendre d'une IP falsifiable.
async fn join(State(room): State<Room>, Json(input): Json<Join>) -> HttpResult<Response> {
    let mut attempts = room.attempts.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Verrou indisponible".into(),
        )
    })?; // Section critique courte.
    if db::now() - attempts.0 >= 60 {
        *attempts = (db::now(), 0);
    } // Fenêtre glissante approximative d'une minute.
    attempts.1 += 1;
    if attempts.1 > 30 {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            "Trop d'essais, patientez une minute".into(),
        ));
    } // Borne l'attaque par force brute.
    if input.pin != room.pin {
        return Err((StatusCode::UNAUTHORIZED, "Code incorrect".into()));
    } // Aucun détail sur la session tant que le PIN échoue.
    let id = token();
    let access = token(); // Identité publique distincte du secret d'authentification.
    let peer = Peer {
        id: id.clone(),
        name: input.name.chars().take(80).collect(),
        upload: false,
        banned: false,
        last_seen: db::now(),
        token: access.clone(),
    }; // Lecture seule à l'arrivée.
    room.peers
        .lock()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Verrou indisponible".into(),
            )
        })?
        .insert(id.clone(), peer); // L'état reste côté maître.
    let _ = room.events.send("permissions".into()); // Le panneau et les clients peuvent actualiser leur vue.
    let mut response = Json(json!({"token":access,"id":id})).into_response(); // Le jeton sert aussi au canal WebSocket.
    response.headers_mut().insert(
        header::SET_COOKIE,
        format!("omnishell_peer={access}; Path=/api; HttpOnly; Secure; SameSite=Strict")
            .parse()
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Cookie invalide".into()))?,
    ); // Le navigateur peut télécharger en streaming natif.
    Ok(response) // Le navigateur conserve le jeton uniquement pour cette salle.
}

// La liste est accessible uniquement après authentification.
async fn list_files(State(room): State<Room>, headers: HeaderMap) -> HttpResult<Json<Value>> {
    let peer = auth(&room, &headers, false)?; // Revalide les droits à chaque consultation.
    let files=room.files.iter().enumerate().map(|(id,p)|json!({"id":id,"name":p.file_name().unwrap_or_default().to_string_lossy(),"size":std::fs::metadata(p).map(|m|m.len()).unwrap_or(0)})).collect::<Vec<_>>(); // Ne révèle aucun chemin absolu.
    Ok(Json(
        json!({"files":files,"upload":peer.upload,"peer":peer.id}),
    )) // Le client adapte son interface au rôle actuel.
}

// Le téléchargement ne reçoit qu'un identifiant numérique, jamais un chemin fourni par le pair.
async fn download(
    State(room): State<Room>,
    Path(id): Path<usize>,
    headers: HeaderMap,
) -> HttpResult<Response> {
    auth(&room, &headers, false)?; // Vérifie aussi les requêtes Range de reprise.
    let path = room
        .files
        .get(id)
        .ok_or((StatusCode::NOT_FOUND, "Fichier inconnu".into()))?; // Une sortie de l'allowlist est impossible.
    let request = axum::http::Request::builder()
        .uri("/")
        .body(Body::empty())
        .map_err(|_| (StatusCode::BAD_REQUEST, "Requête invalide".into()))?; // Le service de fichier ne reçoit pas le chemin de l'URL.
    let (mut parts, body) = request.into_parts();
    parts.headers = headers; // Transmet Range et les conditions HTTP.
    let result: axum::http::Response<ServeFileSystemResponseBody> = ServeFile::new(path)
        .try_call(axum::http::Request::from_parts(parts, body))
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "Fichier devenu inaccessible".into()))?; // Flux et 206 gérés par tower-http.
    let mut response = result.map(Body::new); // Adapte le corps au routeur Axum.
    response
        .headers_mut()
        .insert(header::CONTENT_DISPOSITION, "attachment".parse().unwrap()); // Empêche l'exécution d'un HTML partagé dans l'origine du serveur.
    response
        .headers_mut()
        .insert(header::X_CONTENT_TYPE_OPTIONS, "nosniff".parse().unwrap()); // Les fichiers restent des téléchargements.
    Ok(response) // Le gros fichier n'est jamais matérialisé dans une Vec.
}

// Upload par blocs de 4 Mio : le serveur impose le décalage et un nom généré.
async fn upload(
    State(room): State<Room>,
    Path(id): Path<String>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> HttpResult<Json<Value>> {
    let _upload_guard = room
        .upload_lock
        .lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Transfert occupé".into()))?; // Sérialise les blocs pour interdire les offsets concurrents.
    let peer = auth(&room, &headers, true)?; // Une révocation interdit immédiatement le prochain bloc.
    if id.is_empty()
        || id.len() > 80
        || !id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err((
            StatusCode::BAD_REQUEST,
            "Identifiant de transfert invalide".into(),
        ));
    } // Refuse séparateurs et traversée de chemins.
    let offset = headers
        .get("x-offset")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .ok_or((StatusCode::BAD_REQUEST, "En-tête X-Offset requis".into()))?; // Le client reprend au décalage confirmé.
    let digest = headers
        .get("x-blake3")
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::BAD_REQUEST, "En-tête X-Blake3 requis".into()))?; // Chaque bloc est vérifiable indépendamment.
    if blake3::hash(&body).to_hex().as_str() != digest {
        return Err((
            StatusCode::BAD_REQUEST,
            "Empreinte du bloc incorrecte".into(),
        ));
    } // Rejette un bloc altéré avant écriture.
    if offset + body.len() as u64 > 2 * 1024 * 1024 * 1024 {
        return Err((
            StatusCode::PAYLOAD_TOO_LARGE,
            "Limite de 2 Gio par transfert".into(),
        ));
    } // Quota par transfert.
    if fs2::available_space(&room.upload_dir).unwrap_or(0) < body.len() as u64 + 64 * 1024 * 1024 {
        return Err((
            StatusCode::INSUFFICIENT_STORAGE,
            "Moins de 64 Mio de réserve disque".into(),
        ));
    } // Garde une marge système avant toute écriture.
    let received: u64 = std::fs::read_dir(&room.upload_dir)
        .map_err(|_| {
            (
                StatusCode::INSUFFICIENT_STORAGE,
                "Dossier inaccessible".into(),
            )
        })?
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum(); // Quota agrégé de salle, pas seulement de fichier.
    if received + body.len() as u64 > 10 * 1024 * 1024 * 1024 {
        return Err((
            StatusCode::PAYLOAD_TOO_LARGE,
            "Quota de salle de 10 Gio atteint".into(),
        ));
    } // Empêche les petits fichiers illimités.
    let path = room.upload_dir.join(format!("{}-{id}.upload", peer.id)); // Le nom du pair ne participe jamais au chemin.
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(path)
        .map_err(|_| {
            (
                StatusCode::INSUFFICIENT_STORAGE,
                "Répertoire de réception inaccessible".into(),
            )
        })?; // Aucun écrasement de fichier arbitraire.
    use std::io::{Seek, SeekFrom, Write}; // Position explicite pour la reprise.
    if file
        .metadata()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Taille inaccessible".into(),
            )
        })?
        .len()
        != offset
    {
        return Err((
            StatusCode::CONFLICT,
            "Décalage différent de la taille reçue".into(),
        ));
    } // Refuse les trous et doublons.
    file.seek(SeekFrom::Start(offset))
        .and_then(|_| file.write_all(&body))
        .and_then(|_| file.sync_data())
        .map_err(|_| {
            (
                StatusCode::INSUFFICIENT_STORAGE,
                "Écriture impossible : vérifier l'espace disque".into(),
            )
        })?; // Signale ENOSPC sans prétendre avoir reçu le bloc.
    Ok(Json(json!({"offset":offset+body.len() as u64}))) // Le client mémorise uniquement un bloc durable.
}

// La reprise utilise uniquement le nombre d'octets réellement présents sur disque.
async fn upload_status(
    State(room): State<Room>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> HttpResult<Json<Value>> {
    let peer = auth(&room, &headers, true)?; // La reprise ne contourne pas une révocation.
    if id.is_empty()
        || id.len() > 80
        || !id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err((StatusCode::BAD_REQUEST, "Identifiant invalide".into()));
    } // Même validation que l'écriture.
    let offset = std::fs::metadata(room.upload_dir.join(format!("{}-{id}.upload", peer.id)))
        .map(|m| m.len())
        .unwrap_or(0); // Un transfert absent commence à zéro.
    Ok(Json(json!({"offset":offset}))) // Ne révèle pas les transferts des autres pairs.
}

// Le WebSocket ne transporte pas de fichiers ni de privilèges déclarés par le client.
async fn ws_events(State(room): State<Room>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |mut socket| async move {
        let access =
            match tokio::time::timeout(std::time::Duration::from_secs(10), socket.recv()).await {
                Ok(Some(Ok(Message::Text(token)))) => token.to_string(),
                _ => return,
            }; // Le premier message doit authentifier le canal.
        let mut headers = HeaderMap::new();
        if let Ok(value) = format!("Bearer {access}").parse() {
            headers.insert(header::AUTHORIZATION, value);
        } // Réutilise le même contrôle que HTTP.
        if auth(&room, &headers, false).is_err() {
            return;
        } // Refuse les canaux anonymes.
        let mut events = room.events.subscribe(); // Notifications sans données privées ni liste de pairs.
        while let Ok(event) = events.recv().await {
            if auth(&room, &headers, false).is_err()
                || socket.send(Message::Text(event.into())).await.is_err()
            {
                break;
            }
        } // Une déconnexion termine la tâche.
    }) // Les clients réinterrogent /api/files avec leur Bearer après notification.
}

// Démarre un serveur à port dynamique ; aucune modification de pare-feu ou de profil shell.
pub async fn start(paths: Vec<String>, host_ip: String) -> Result<Host> {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default(); // Fixe explicitement le moteur crypto même si un client de test active aussi ring.
    let ip: std::net::IpAddr = host_ip.parse()?; // Le QR doit pointer vers une adresse explicitement choisie.
    if ip.is_unspecified() || ip.is_loopback() {
        return Err("Sélectionnez l'adresse LAN de votre interface".into());
    } // Évite un QR inutilisable par les autres appareils.
    let mut files = Vec::new(); // Allowlist immuable durant la salle.
    for p in paths {
        let p = std::fs::canonicalize(p)?;
        if !p.is_file() {
            return Err("Choisissez des fichiers ; archivez les dossiers avant partage".into());
        }
        files.push(p);
    } // Ne suit pas un chemin de téléchargement fourni par un pair.
    if files.is_empty() {
        return Err("Sélectionnez au moins un fichier".into());
    } // Une salle vide n'a pas d'utilité.
    let pin = format!("{:06}", rand::rng().random_range(0..1_000_000)); // Code d'accès court, limité en essais.
    let upload_dir = db::path()
        .parent()
        .unwrap()
        .join(format!("uploads/{}", token()));
    std::fs::create_dir_all(&upload_dir)?; // Isole les réceptions de chaque salle.
    let (events, _) = broadcast::channel(64); // Les notifications lentes peuvent être abandonnées sans bloquer les transferts.
    let room = Room {
        pin: pin.clone(),
        files: Arc::new(files),
        upload_dir,
        peers: Arc::default(),
        events,
        upload_lock: Arc::new(Mutex::new(())),
        attempts: Arc::new(Mutex::new((db::now(), 0))),
    }; // État propre à cette salle.
    let app = Router::new()
        .route(
            "/",
            get(|| async { Html(include_str!("../../src/share-client.html")) }),
        )
        .route(
            "/client.js",
            get(|| async {
                (
                    [(header::CONTENT_TYPE, "text/javascript")],
                    include_str!("../../assets/share-client.js"),
                )
            }),
        )
        .route("/api/join", post(join))
        .route("/api/files", get(list_files))
        .route("/api/files/{id}", get(download))
        .route("/api/upload/{id}", put(upload).get(upload_status))
        .route("/events", get(ws_events))
        .layer(DefaultBodyLimit::max(4 * 1024 * 1024))
        .with_state(room.clone()); // Aucun endpoint d'administration exposé sur LAN.
    let listener = tokio::net::TcpListener::bind((ip, 0)).await?; // Écoute uniquement sur l'interface choisie.
    let address = listener.local_addr()?; // Le port retenu est celui réellement alloué par le noyau.
    let url = format!("https://{address}/"); // Le PIN ne circule jamais en HTTP clair.
    let tls_dir = db::path().parent().unwrap().join("tls");
    std::fs::create_dir_all(&tls_dir)?; // Le certificat reste dans le répertoire privé utilisateur.
    let cert_path = tls_dir.join(format!("{ip}.pem"));
    let key_path = tls_dir.join(format!("{ip}.key")); // Un certificat stable par adresse réduit les changements d'empreinte.
    if !cert_path.exists() || !key_path.exists() {
        let certified =
            rcgen::generate_simple_self_signed(vec![ip.to_string(), "omnishell.local".into()])?; // Certificat local, aucune autorité cloud.
        std::fs::write(&cert_path, certified.cert.pem())?;
        std::fs::write(&key_path, certified.key_pair.serialize_pem())?; // La clé ne quitte pas le poste hôte.
    }
    let fingerprint = blake3::hash(&std::fs::read(&cert_path)?)
        .to_hex()
        .to_string(); // Empreinte du PEM à comparer via un canal connu.
    let tls = axum_server::tls_rustls::RustlsConfig::from_pem_file(&cert_path, &key_path).await?; // Le serveur annonce HTTP/1.1 et HTTP/2 via ALPN.
    let qr = qrcode::QrCode::new(url.as_bytes())?
        .render::<qrcode::render::svg::Color>()
        .min_dimensions(220, 220)
        .build(); // QR généré entièrement hors ligne.
    let discovery = mdns_sd::ServiceDaemon::new().ok(); // Le lien explicite reste valable si multicast interdit.
    if let Some(mdns) = &discovery {
        if let Ok(service) = mdns_sd::ServiceInfo::new(
            "_omnishell-share._tcp.local.",
            "OmniShell Studio",
            "omnishell.local.",
            ip.to_string(),
            address.port(),
            [("tls", "true")].as_slice(),
        ) {
            let _ = mdns.register(service);
        }
    } // Annonce uniquement adresse et port, jamais le PIN.
    let server = axum_server::Handle::new();
    let shutdown = server.clone(); // Drop peut aussi couper les connexions ouvertes.
    let std_listener = listener.into_std()?; // Toute erreur de conversion est retournée sans panique.
    let task = tokio::spawn(async move {
        if let Err(e) = axum_server::from_tcp_rustls(std_listener, tls)
            .handle(shutdown)
            .serve(app.into_make_service())
            .await
        {
            eprintln!("ClassShare : {e}");
        }
    }); // Serveur TLS isolé de l'IPC maître.
    let info = json!({"url":url,"pin":pin,"qr":qr,"fingerprint":fingerprint,"upload_dir":room.upload_dir,"certificate":cert_path,"transport":"TLS auto-signé : valider le certificat de l'hôte à la première visite"});
    Ok(Host {
        room,
        info,
        task,
        server,
        discovery,
    }) // La confiance du certificat doit être établie par les participants.
}

impl Host {
    // Le panneau maître est accessible uniquement par IPC natif, jamais sur le port LAN.
    pub fn peers(&self) -> Result<Value> {
        Ok(json!(self
            .room
            .peers
            .lock()
            .map_err(|_| "État des pairs inaccessible")?
            .values()
            .cloned()
            .collect::<Vec<_>>()))
    } // serde exclut les jetons.
      // Chaque changement est relu dès la requête HTTP suivante.
    pub fn permission(&self, id: &str, upload: bool, banned: bool) -> Result<()> {
        let mut peers = self
            .room
            .peers
            .lock()
            .map_err(|_| "État des pairs inaccessible")?;
        let p = peers.get_mut(id).ok_or("Pair inconnu")?;
        p.upload = upload;
        p.banned = banned;
        let _ = self.room.events.send("permissions".into());
        Ok(())
    } // La révocation ne dépend pas d'un JWT déjà émis.
}
