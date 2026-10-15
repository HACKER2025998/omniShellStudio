//! Service utilisateur facultatif : capture locale et rafraîchissement des caches sans fenêtre.
use crate::{db, docker, history, Result}; // Le daemon utilise exactement les mêmes filtres et migrations.
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt}; // Le protocole transporte un objet JSON borné puis ferme le flux. // Le socket ne vit jamais dans /tmp avec un nom partagé entre utilisateurs.

// Le répertoire privé empêche un autre compte de soumettre ou lire les captures.
pub fn socket_path() -> PathBuf {
    db::path().parent().unwrap().join("capture.sock")
} // Respecte aussi OMNISHELL_DATA_DIR pour les tests.

// Un hook essaie le daemon, puis revient à SQLite si le service n'est pas lancé.
#[cfg(unix)]
pub async fn send(capture: &db::Capture) -> Result<()> {
    let mut socket = tokio::net::UnixStream::connect(socket_path()).await?; // Seul l'utilisateur propriétaire accède au répertoire.
    socket.write_all(&serde_json::to_vec(capture)?).await?;
    socket.shutdown().await?; // EOF délimite l'objet, sans ambiguïté sur les sauts de ligne.
    let mut ack = [0u8; 1];
    socket.read_exact(&mut ack).await?;
    if ack[0] != 1 {
        return Err("Capture refusée".into());
    }
    Ok(()) // Le fallback SQLite n'intervient que si aucune confirmation ne revient.
}

// Sous Windows, la version actuelle garde l'écriture directe SQLite asynchrone des hooks.
#[cfg(not(unix))]
pub async fn send(_capture: &db::Capture) -> Result<()> {
    Err("Le daemon socket est disponible sous Unix ; capture directe SQLite utilisée".into())
} // Aucune émulation de Named Pipe non validée.

// Le daemon est lancé explicitement par la CLI ou un service utilisateur.
#[cfg(unix)]
pub async fn run() -> Result<()> {
    use fs2::FileExt; // Un verrou vivant distingue un socket périmé d'un daemon actif.
    db::open()?; // Crée le répertoire avec permissions 0700 avant le socket.
    let lock = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(socket_path().with_extension("lock"))?;
    lock.try_lock_exclusive()
        .map_err(|_| "Un daemon OmniShell est déjà actif")?; // Le verrou est conservé jusqu'à l'arrêt.
    if socket_path().exists() {
        std::fs::remove_file(socket_path())?;
    } // Un processus tué peut laisser un socket sans propriétaire vivant.
    let listener = tokio::net::UnixListener::bind(socket_path())?; // Aucun port TCP administrateur n'est ouvert.
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(socket_path(), std::fs::Permissions::from_mode(0o600))?; // Protection en profondeur même si le répertoire est déplacé.
    let background = tokio::spawn(async {
        let mut timer = tokio::time::interval(std::time::Duration::from_secs(30));
        loop {
            timer.tick().await;
            let _ = history::import();
            let _ = docker::sync().await;
        }
    }); // Les erreurs Docker n'arrêtent jamais la capture.
    let permits = std::sync::Arc::new(tokio::sync::Semaphore::new(32)); // Borne les clients simultanés du socket.
    loop {
        tokio::select! {
            _=tokio::signal::ctrl_c()=>break, // Arrêt propre demandé par le service ou l'utilisateur.
            incoming=listener.accept()=>{
                let (mut socket,_)=incoming?;let permits=permits.clone(); // Chaque connexion possède sa limite de temps.
                tokio::spawn(async move {let Ok(_permit)=permits.acquire_owned().await else{return;};let mut bytes=Vec::new();let read=tokio::time::timeout(std::time::Duration::from_secs(2),(&mut socket).take(131_073).read_to_end(&mut bytes)).await;let accepted=matches!(read,Ok(Ok(_)))&&bytes.len()<=131_072&&serde_json::from_slice::<db::Capture>(&bytes).ok().and_then(|c|db::open().ok().and_then(|conn|db::insert(&conn,&c).ok())).is_some();let _=socket.write_all(&[u8::from(accepted)]).await;}); // Le filtre de secrets est toujours appliqué à l'intérieur du daemon.
            }
        }
    }
    background.abort();
    std::fs::remove_file(socket_path())?;
    drop(lock);
    Ok(()) // Libère le verrou et le socket pour un prochain lancement.
}

// Le mode portable reste fonctionnel sur les plateformes sans socket Unix.
#[cfg(not(unix))]
pub async fn run() -> Result<()> {
    Err("Daemon Unix uniquement dans cette version ; les hooks Windows écrivent directement dans SQLite".into())
} // Limite annoncée au lieu d'une fausse activation.
