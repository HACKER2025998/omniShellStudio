//! Capture utilisateur via socket Unix ou Named Pipe Windows, sans port administrateur réseau.
use crate::{db, docker, history, Result}; // Tous les transports rejoignent le même filtre de confidentialité.
use serde::{Deserialize, Serialize}; // L'enveloppe ajoute un secret local au message de capture.
use std::path::PathBuf; // Le stockage privé sépare les comptes et les installations portables.
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt}; // Le protocole est indépendant du type de socket.
// Le socket Unix appartient au même répertoire privé que la base locale.
pub fn socket_path() -> PathBuf {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
    db::path().with_file_name("capture.sock")
// @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
}
// Windows isole le nom par utilisateur et installation ; un secret protège aussi l'accès au pipe.
#[cfg(windows)]
// @ligne Déclare « daemon », le point d’entrée de cette opération de la capture locale et du service utilisateur.
fn pipe_name() -> String {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
    format!(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        r"\\.\pipe\omnishell-{}",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        &blake3::hash(db::path().to_string_lossy().as_bytes()).to_hex()[..20]
    // @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
    )
// @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
}
// Le secret n'est ni affiché ni envoyé sur le réseau ; il reste dans le profil utilisateur.
fn secret() -> Result<String> {
    use std::io::Write; // Une création exclusive évite qu'un client écrase la clé d'un autre processus.
    let path = db::path().with_file_name("capture.key"); // La clé persiste entre deux lancements.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la capture locale et du service utilisateur.
    if let Ok(key) = std::fs::read_to_string(&path) {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la capture locale et du service utilisateur.
        return Ok(key);
    } // Le cas normal ne modifie aucun fichier.
    // @ligne Prépare « key » pour la prochaine étape de la capture locale et du service utilisateur.
    let key = rand::random::<[u8; 32]>()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        .iter()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        .map(|b| format!("{b:02x}"))
        .collect::<String>(); // 256 bits aléatoires protègent le pipe local.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la capture locale et du service utilisateur.
    match std::fs::OpenOptions::new()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        .create_new(true)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        .write(true)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        .open(&path)
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
    {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la capture locale et du service utilisateur.
        Ok(mut file) => {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
            file.write_all(key.as_bytes())?;
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
            file.sync_all()?;
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la capture locale et du service utilisateur.
            Ok(key)
        } // Valide la clé avant de la publier au daemon.
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la capture locale et du service utilisateur.
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la capture locale et du service utilisateur.
            Ok(std::fs::read_to_string(path)?)
        } // Un autre processus a gagné la création.
        Err(error) => Err(error.into()), // Une erreur d'accès n'est jamais traitée comme une clé vide.
    // @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
    }
// @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
}
#[derive(Serialize, Deserialize)] // Les champs inconnus ne deviennent pas des actions administratives.
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
struct Envelope {
    // @ligne Renseigne le champ « key » de la structure utilisée ici de la capture locale et du service utilisateur.
    key: String,
    // @ligne Renseigne le champ « capture » de la structure utilisée ici de la capture locale et du service utilisateur.
    capture: db::Capture,
// @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
}
// Échange un message borné et un accusé de réception après la transaction SQLite.
async fn exchange<S: AsyncRead + AsyncWrite + Unpin>(
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
    mut stream: S,
    // @ligne Renseigne le champ « capture » de la structure utilisée ici de la capture locale et du service utilisateur.
    capture: &db::Capture,
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
) -> Result<()> {
    // @ligne Prépare « payload » pour la prochaine étape de la capture locale et du service utilisateur.
    let payload = serde_json::to_vec(&Envelope {
        // @ligne Renseigne le champ « key » de la structure utilisée ici de la capture locale et du service utilisateur.
        key: secret()?,
        // @ligne Renseigne le champ « capture » de la structure utilisée ici de la capture locale et du service utilisateur.
        capture: capture.clone(),
    })?; // Les sauts de ligne d'une commande sont échappés par JSON.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la capture locale et du service utilisateur.
    if payload.len() > 262_144 {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la capture locale et du service utilisateur.
        return Err("Capture supérieure à 256 Kio".into());
    } // Inclut la marge d'échappement du JSON.
    stream.write_u32(payload.len() as u32).await?; // Un préfixe de longueur fonctionne également sur un Named Pipe sans EOF.
    stream.write_all(&payload).await?; // La chaîne ne figure jamais dans argv.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la capture locale et du service utilisateur.
    if stream.read_u8().await? != 1 {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la capture locale et du service utilisateur.
        return Err("Capture refusée par le daemon".into());
    } // Le fallback utilise le même identifiant d'événement.
    Ok(()) // Le daemon n'accuse réception qu'après écriture durable logique.
// @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
}
// Linux et macOS utilisent les permissions natives du répertoire et du socket.
#[cfg(unix)]
// @ligne Déclare « send », le point d’entrée de cette opération de la capture locale et du service utilisateur.
pub async fn send(capture: &db::Capture) -> Result<()> {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
    exchange(
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la capture locale et du service utilisateur.
        tokio::net::UnixStream::connect(socket_path()).await?,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        capture,
    // @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
    )
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la capture locale et du service utilisateur.
    .await
// @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
}
// Les connexions distantes au Named Pipe sont rejetées par le serveur Windows.
#[cfg(windows)]
// @ligne Déclare « send », le point d’entrée de cette opération de la capture locale et du service utilisateur.
pub async fn send(capture: &db::Capture) -> Result<()> {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
    exchange(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        tokio::net::windows::named_pipe::ClientOptions::new().open(pipe_name())?,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        capture,
    // @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
    )
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la capture locale et du service utilisateur.
    .await
// @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
}
// Cette branche conserve une erreur explicite pour les plateformes hors cibles du projet.
#[cfg(not(any(unix, windows)))]
// @ligne Déclare « send », le point d’entrée de cette opération de la capture locale et du service utilisateur.
pub async fn send(_capture: &db::Capture) -> Result<()> {
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la capture locale et du service utilisateur.
    Err("Transport de capture non pris en charge".into())
// @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
}
// Un client ne peut garder une tâche occupée plus de deux secondes.
async fn receive<S: AsyncRead + AsyncWrite + Unpin>(mut stream: S, key: String) {
    // @ligne Prépare « result » pour la prochaine étape de la capture locale et du service utilisateur.
    let result = tokio::time::timeout(std::time::Duration::from_secs(2), async {
        let length = stream.read_u32().await? as usize; // Valide la taille avant l'allocation.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la capture locale et du service utilisateur.
        if length > 262_144 {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la capture locale et du service utilisateur.
            return Err::<bool, Box<dyn std::error::Error + Send + Sync>>(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
                "Message trop long".into(),
            // @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
            );
        // @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
        }
        let mut bytes = vec![0; length]; // La limite borne la mémoire d'une connexion.
        stream.read_exact(&mut bytes).await?; // Un message tronqué n'est jamais persisté.
        let envelope: Envelope = serde_json::from_slice(&bytes)?; // Le schéma ne contient qu'une capture.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la capture locale et du service utilisateur.
        if envelope.key != key {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la capture locale et du service utilisateur.
            return Err("Clé de capture invalide".into());
        } // Protège en particulier les Named Pipes Windows.
        db::insert(&db::open()?, &envelope.capture)?; // Filtre secrets et doublons d'événement dans une transaction.
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la capture locale et du service utilisateur.
        Ok(true)
    // @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
    })
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la capture locale et du service utilisateur.
    .await;
    // @ligne Prépare « _ » pour la prochaine étape de la capture locale et du service utilisateur.
    let _ = stream
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        .write_u8(u8::from(matches!(result, Ok(Ok(true)))))
        .await; // Un accusé perdu ne crée pas de doublon lors du fallback.
// @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
}
// Le verrou de fichier empêche deux daemons d'une même installation d'entrer en concurrence.
pub async fn run() -> Result<()> {
    use fs2::FileExt; // Le verrou est relâché par le système même après un crash.
    db::open()?; // Crée le répertoire privé avant la clé ou le point d'écoute.
    // @ligne Prépare « lock » pour la prochaine étape de la capture locale et du service utilisateur.
    let lock = std::fs::OpenOptions::new()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        .create(true)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        .truncate(false)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        .write(true)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        .open(db::path().with_file_name("capture.lock"))?;
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
    lock.try_lock_exclusive()
        .map_err(|_| "Un daemon OmniShell est déjà actif")?; // Un socket périmé peut ensuite être supprimé sans gêner un autre service.
    let key = secret()?; // Le même secret est utilisé par les clients et le serveur.
    // @ligne Prépare « cache » pour la prochaine étape de la capture locale et du service utilisateur.
    let cache = tokio::spawn(async {
        // @ligne Prépare « timer » pour la prochaine étape de la capture locale et du service utilisateur.
        let mut timer = tokio::time::interval(std::time::Duration::from_secs(30));
        // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la capture locale et du service utilisateur.
        loop {
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la capture locale et du service utilisateur.
            timer.tick().await;
            // @ligne Prépare « _ » pour la prochaine étape de la capture locale et du service utilisateur.
            let _ = tokio::task::spawn_blocking(history::import).await;
        // @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
        }
    }); // L'import synchrone quitte les threads de tâches réseau.
    let docker = tokio::spawn(docker::watch()); // Les événements Docker actualisent les captures sans polling permanent.
    let result = listen(key).await; // Chaque plateforme applique ses contrôles natifs de transport.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
    cache.abort();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
    docker.abort();
    drop(lock); // Arrêter le service ne supprime jamais l'historique.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
    result
// @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
}
// Les tâches sont créées seulement après acquisition d'un permis : pas de file d'attente illimitée.
#[cfg(unix)]
// @ligne Déclare « daemon », le point d’entrée de cette opération de la capture locale et du service utilisateur.
async fn listen(key: String) -> Result<()> {
    use std::os::unix::fs::PermissionsExt; // Les autres comptes n'ont pas accès au socket.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la capture locale et du service utilisateur.
    if socket_path().exists() {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        std::fs::remove_file(socket_path())?;
    } // Nettoie une ancienne entrée sans service vivant.
    // @ligne Prépare « listener » pour la prochaine étape de la capture locale et du service utilisateur.
    let listener = tokio::net::UnixListener::bind(socket_path())?;
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
    std::fs::set_permissions(socket_path(), std::fs::Permissions::from_mode(0o600))?;
    let permits = std::sync::Arc::new(tokio::sync::Semaphore::new(32)); // Borne simultanément tâches et tampons.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la capture locale et du service utilisateur.
    loop {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        tokio::select! {
            _=tokio::signal::ctrl_c()=>break, // Interruption propre du service utilisateur.
            incoming=listener.accept()=>{let(stream,_)=incoming?;if let Ok(permit)=permits.clone().try_acquire_owned(){let key=key.clone();tokio::spawn(async move{let _permit=permit;receive(stream,key).await;});}}, // Un client excédentaire est fermé immédiatement.
        // @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
        }
    // @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
    }
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
    std::fs::remove_file(socket_path())?;
    Ok(()) // La clé reste valable pour le prochain démarrage.
// @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
}
// Le serveur refuse les clients distants et utilise un premier pipe exclusif contre l'usurpation.
#[cfg(windows)]
// @ligne Déclare « daemon », le point d’entrée de cette opération de la capture locale et du service utilisateur.
async fn listen(key: String) -> Result<()> {
    use tokio::net::windows::named_pipe::ServerOptions; // API Windows native proposée par Tokio.
    // @ligne Prépare « server » pour la prochaine étape de la capture locale et du service utilisateur.
    let mut server = ServerOptions::new()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        .first_pipe_instance(true)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        .reject_remote_clients(true)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        .create(pipe_name())?;
    let permits = std::sync::Arc::new(tokio::sync::Semaphore::new(32)); // Même plafond que le transport Unix.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la capture locale et du service utilisateur.
    loop {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la capture locale et du service utilisateur.
        tokio::select! {
            _=tokio::signal::ctrl_c()=>break, // Le service peut être arrêté sans supprimer ses données.
            connection=server.connect()=>{connection?;let next=ServerOptions::new().reject_remote_clients(true).create(pipe_name())?;let connected=std::mem::replace(&mut server,next);if let Ok(permit)=permits.clone().try_acquire_owned(){let key=key.clone();tokio::spawn(async move{let _permit=permit;receive(connected,key).await;});}}, // Prépare l'instance suivante avant de traiter le client courant.
        // @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
        }
    // @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
    }
    Ok(()) // Le système détruit les instances du pipe à la fermeture des poignées.
// @ligne Termine cette construction et revient au contexte parent de la capture locale et du service utilisateur.
}
// @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant de la capture locale et du service utilisateur.
#[cfg(not(any(unix, windows)))]
// @ligne Déclare « daemon », le point d’entrée de cette opération de la capture locale et du service utilisateur.
async fn listen(_key: String) -> Result<()> {
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la capture locale et du service utilisateur.
    Err("Daemon non pris en charge sur cette plateforme".into())
} // Refus explicite hors plateformes ciblées.
