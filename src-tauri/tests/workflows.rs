//! Tests de bout en bout des garanties sensibles : stockage et permissions de partage.
use omnishell_core::{classifier, db, dispatch, share}; // Même API que la fenêtre de production.
use serde_json::{json, Value}; // Vérifie les réponses publiques plutôt que les détails de l'implémentation.

// Un seul scénario isolé évite que plusieurs tests ne modifient la variable de répertoire global.
#[tokio::test]
async fn storage_and_classroom_permissions() {
    let root = std::env::temp_dir().join(format!("omnishell-test-{}", std::process::id())); // Répertoire unique à cette exécution.
    std::fs::create_dir_all(&root).unwrap(); // Aucune donnée de l'utilisateur n'est lue.
    std::env::set_var("OMNISHELL_DATA_DIR", &root); // Isole SQLite, certificats et réceptions.
    let capture = db::Capture {
        raw_command: "sudo docker run -e TOKEN=highlysecret --name web nginx".into(),
        shell: "zsh".into(),
        ..Default::default()
    }; // Secret de test reconnaissable dans les assertions.
    db::insert(&db::open().unwrap(), &capture).unwrap(); // Le filtrage doit intervenir dès l'écriture.
    db::insert(&db::open().unwrap(), &capture).unwrap(); // Vérifie l'upsert atomique du compteur.
    let found = db::commands("docker", "containers", false, 0).unwrap(); // Recherche via le vrai FTS5.
    assert_eq!(found["total"], 1);
    assert_eq!(found["items"][0]["count"], 2);
    assert_eq!(found["items"][0]["privileged"], true); // Ni doublon ni perte d'élévation.
    assert!(!found.to_string().contains("highlysecret")); // Aucune sortie publique ne doit restituer le secret.
    let id = found["items"][0]["id"].as_i64().unwrap(); // Identifiant réellement alloué par SQLite.
    dispatch("favorite", json!({"id":id})).await.unwrap(); // Teste la même action que le bouton étoile.
    assert_eq!(db::commands("", "", true, 0).unwrap()["total"], 1); // Le filtre Favoris utilise bien l'état durable.
    db::snapshot(
        "test",
        "one",
        &json!({"environment":["TOKEN=highlysecret"],"password":"othersecret","value":"ok"}),
    )
    .unwrap(); // Le masquage ne doit pas casser le JSON.
    let snapshots = db::snapshots("test").unwrap();
    assert_eq!(snapshots[0]["data"]["value"], "ok");
    assert!(!snapshots.to_string().contains("secret")); // Détecte une corruption structurelle ou un oubli de filtre.
    dispatch("delete", json!({"id":id})).await.unwrap();
    assert_eq!(db::commands("docker", "", false, 0).unwrap()["total"], 0); // Le trigger FTS suit les suppressions.
    let address = if_addrs::get_if_addrs()
        .unwrap()
        .into_iter()
        .find_map(|i| match i.ip() {
            std::net::IpAddr::V4(ip) if !ip.is_loopback() => Some(ip.to_string()),
            _ => None,
        })
        .expect("Une interface IPv4 est nécessaire au test LAN"); // Aucun réseau distant n'est sollicité.
    let file = root.join("cours.txt");
    std::fs::write(&file, b"cours local offline").unwrap(); // Fichier partagé de taille connue.
    let host = share::start(vec![file.to_string_lossy().into_owned()], address)
        .await
        .unwrap(); // Démarre le vrai serveur TLS à port dynamique.
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .no_proxy()
        .build()
        .unwrap(); // Le certificat auto-signé est accepté uniquement dans ce test local.
    let url = host.info["url"].as_str().unwrap(); // Utilise le port effectivement choisi par le noyau.
    assert_eq!(
        client
            .get(format!("{url}api/files"))
            .send()
            .await
            .unwrap()
            .status(),
        401
    ); // Un visiteur anonyme ne voit pas les noms de fichiers.
    assert_eq!(
        client
            .post(format!("{url}api/join"))
            .json(&json!({"pin":"invalid","name":"Élève"}))
            .send()
            .await
            .unwrap()
            .status(),
        401
    ); // PIN incorrect refusé.
    let joined: Value = client
        .post(format!("{url}api/join"))
        .json(&json!({"pin":host.info["pin"],"name":"Élève"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap(); // Le rôle initial est fixé côté serveur.
    let token = joined["token"].as_str().unwrap();
    let peer = joined["id"].as_str().unwrap(); // Les identités publique et secrète sont distinctes.
    assert!(!host.peers().unwrap().to_string().contains(token)); // Le panneau maître n'expose pas les secrets.
    let listing: Value = client
        .get(format!("{url}api/files"))
        .bearer_auth(token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(listing["upload"], false); // Lecture seule par défaut.
    let partial = client
        .get(format!("{url}api/files/0"))
        .bearer_auth(token)
        .header("Range", "bytes=0-4")
        .send()
        .await
        .unwrap();
    assert_eq!(partial.status(), 206);
    assert_eq!(partial.bytes().await.unwrap().as_ref(), b"cours"); // Reprise de téléchargement HTTP réellement exercée.
    let body = b"bloc de contribution";
    let digest = blake3::hash(body).to_hex().to_string(); // Empreinte envoyée avec le bloc.
    let upload = || {
        client
            .put(format!("{url}api/upload/test-transfer"))
            .bearer_auth(token)
            .header("X-Offset", "0")
            .header("X-Blake3", &digest)
            .body(body.to_vec())
    }; // Requête identique utilisée sous différents rôles.
    assert_eq!(upload().send().await.unwrap().status(), 403); // Un lecteur ne peut pas contourner le bouton désactivé.
    host.permission(peer, true, false).unwrap();
    let invalid = client
        .put(format!("{url}api/upload/invalid-transfer"))
        .bearer_auth(token)
        .header("X-Offset", "0")
        .header("X-Blake3", "incorrect")
        .body(body.to_vec())
        .send()
        .await
        .unwrap(); // Le transport TLS ne remplace pas le contrôle d'intégrité applicatif.
    assert_eq!(invalid.status(), 400); // Aucun bloc au hash invalide n'est écrit.
    let oversized = client
        .put(format!("{url}api/upload/large-transfer"))
        .bearer_auth(token)
        .header("X-Offset", (2u64 * 1024 * 1024 * 1024).to_string())
        .header("X-Blake3", &digest)
        .body(body.to_vec())
        .send()
        .await
        .unwrap(); // Une déclaration d'offset ne contourne pas le plafond par fichier.
    assert_eq!(oversized.status(), 413); // Le serveur rejette avant de créer le fichier.
                                         // Le maître accorde le rôle contributeur.
    assert_eq!(upload().send().await.unwrap().status(), 200); // Le bloc valide est accepté.
    assert_eq!(upload().send().await.unwrap().status(), 409); // Un renvoi au mauvais offset ne duplique pas les données.
    let state: Value = client
        .get(format!("{url}api/upload/test-transfer"))
        .bearer_auth(token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(state["offset"], body.len()); // La reprise consulte la taille durable sur disque.
    host.permission(peer, false, false).unwrap();
    assert_eq!(upload().send().await.unwrap().status(), 403); // Une révocation agit avec le même jeton déjà émis.
    host.permission(peer, false, true).unwrap();
    assert_eq!(
        client
            .get(format!("{url}api/files"))
            .bearer_auth(token)
            .send()
            .await
            .unwrap()
            .status(),
        401
    ); // Expulsion effective sur les requêtes HTTP.
    drop(host); // Coupe le serveur avant de supprimer les fichiers temporaires.
    let on_disk = std::fs::read(db::path()).unwrap();
    assert!(!String::from_utf8_lossy(&on_disk).contains("highlysecret")); // Le secret n'est pas seulement caché dans l'UI.
    assert!(!classifier::redact("TOKEN=highlysecret").contains("highlysecret")); // Garde un diagnostic direct si le scénario de stockage échoue.
    std::fs::remove_dir_all(root).unwrap(); // Le test ne laisse ni certificat ni historique.
}
