//! Tests de bout en bout des garanties sensibles : stockage et permissions de partage.
use omnishell_core::{classifier, db, dispatch, share}; // Même API que la fenêtre de production.
use serde_json::{json, Value}; // Vérifie les réponses publiques plutôt que les détails de l'implémentation.
// Un seul scénario isolé évite que plusieurs tests ne modifient la variable de répertoire global.
#[tokio::test]
// @ligne Déclare « workflows », le point d’entrée de cette opération du scénario de recette HTTPS et de stockage.
async fn storage_and_classroom_permissions() {
    let root = std::env::temp_dir().join(format!("omnishell-test-{}", std::process::id())); // Répertoire unique à cette exécution.
    std::fs::create_dir_all(&root).unwrap(); // Aucune donnée de l'utilisateur n'est lue.
    std::env::set_var("OMNISHELL_DATA_DIR", &root); // Isole SQLite, certificats et réceptions.
    // @ligne Prépare « capture » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let capture = db::Capture {
        // @ligne Renseigne le champ « raw_command » de la structure utilisée ici du scénario de recette HTTPS et de stockage.
        raw_command: "sudo docker run -e TOKEN=highlysecret --name web nginx".into(),
        // @ligne Renseigne le champ « shell » de la structure utilisée ici du scénario de recette HTTPS et de stockage.
        shell: "zsh".into(),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        ..Default::default()
    }; // Secret de test reconnaissable dans les assertions.
    db::insert(&db::open().unwrap(), &capture).unwrap(); // Le filtrage doit intervenir dès l'écriture.
    db::insert(&db::open().unwrap(), &capture).unwrap(); // Vérifie l'upsert atomique du compteur.
    let found = db::commands("docker", "containers", false, 0).unwrap(); // Recherche via le vrai FTS5.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    assert_eq!(found["total"], 1);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    assert_eq!(found["items"][0]["count"], 2);
    assert_eq!(found["items"][0]["privileged"], true); // Ni doublon ni perte d'élévation.
    assert!(!found.to_string().contains("highlysecret")); // Aucune sortie publique ne doit restituer le secret.
    // @ligne Prépare « event » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let event = db::Capture {
        // @ligne Renseigne le champ « event_id » de la structure utilisée ici du scénario de recette HTTPS et de stockage.
        event_id: Some("stable-test-event".into()),
        // @ligne Renseigne le champ « raw_command » de la structure utilisée ici du scénario de recette HTTPS et de stockage.
        raw_command: "git status".into(),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        ..Default::default()
    }; // Même identité lors d'un ACK perdu.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    db::insert(&db::open().unwrap(), &event).unwrap();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    db::insert(&db::open().unwrap(), &event).unwrap();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    assert_eq!(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        db::commands("git status", "", false, 0).unwrap()["items"][0]["count"],
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        1
    ); // Le fallback ne double pas la fréquence.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    assert_eq!(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        db::commands_filtered("", "", false, 0, Some(true), None).unwrap()["total"],
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        1
    ); // Le filtre d'élévation est appliqué au moteur réel.
    // @ligne Prépare « writers » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let mut writers=Vec::new();
    for _ in 0..8{let event=db::Capture{event_id:Some("concurrent-event".into()),raw_command:"git log --oneline".into(),..Default::default()};writers.push(std::thread::spawn(move||db::insert(&db::open().unwrap(),&event).unwrap()));} // Le même message arrive simultanément du daemon et des fallbacks.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du scénario de recette HTTPS et de stockage.
    for writer in writers{writer.join().unwrap();}
    assert_eq!(db::commands("git log --oneline","",false,0).unwrap()["items"][0]["count"],1); // Le verrou d'écriture immédiat empêche BUSY_SNAPSHOT et les doublons.
    let id = found["items"][0]["id"].as_i64().unwrap(); // Identifiant réellement alloué par SQLite.
    dispatch("favorite", json!({"id":id})).await.unwrap(); // Teste la même action que le bouton étoile.
    assert_eq!(db::commands("", "", true, 0).unwrap()["total"], 1); // Le filtre Favoris utilise bien l'état durable.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    db::snapshot(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        "test",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        "one",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        &json!({"environment":["TOKEN=highlysecret"],"password":"othersecret","value":"ok"}),
    // @ligne Termine cette construction et revient au contexte parent du scénario de recette HTTPS et de stockage.
    )
    .unwrap(); // Le masquage ne doit pas casser le JSON.
    // @ligne Prépare « snapshots » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let snapshots = db::snapshots("test").unwrap();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    assert_eq!(snapshots[0]["data"]["value"], "ok");
    assert!(!snapshots.to_string().contains("secret")); // Détecte une corruption structurelle ou un oubli de filtre.
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
    dispatch("delete", json!({"id":id})).await.unwrap();
    assert_eq!(db::commands("docker", "", false, 0).unwrap()["total"], 0); // Le trigger FTS suit les suppressions.
    // @ligne Prépare « address » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let address = if_addrs::get_if_addrs()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .unwrap()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .into_iter()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .find_map(|i| match i.ip() {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            std::net::IpAddr::V4(ip) if !ip.is_loopback() => Some(ip.to_string()),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            _ => None,
        // @ligne Termine cette construction et revient au contexte parent du scénario de recette HTTPS et de stockage.
        })
        .expect("Une interface IPv4 est nécessaire au test LAN"); // Aucun réseau distant n'est sollicité.
    // @ligne Prépare « file » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let file = root.join("cours.txt");
    std::fs::write(&file, b"cours local offline").unwrap(); // Fichier partagé de taille connue.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    assert!(share::start_with_limits(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        vec![file.to_string_lossy().into_owned()],
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        address.clone(),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        share::Limits {
            // @ligne Renseigne le champ « max_file_bytes » de la structure utilisée ici du scénario de recette HTTPS et de stockage.
            max_file_bytes: 0,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            ..Default::default()
        // @ligne Termine cette construction et revient au contexte parent du scénario de recette HTTPS et de stockage.
        }
    // @ligne Termine cette construction et revient au contexte parent du scénario de recette HTTPS et de stockage.
    )
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
    .await
    .is_err()); // Le serveur refuse des limites incohérentes avant d'ouvrir un port.
    // @ligne Prépare « host » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let host = share::start(vec![file.to_string_lossy().into_owned()], address)
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .await
        .unwrap(); // Démarre le vrai serveur TLS à port dynamique.
    // @ligne Prépare « client » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let client = reqwest::Client::builder()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .danger_accept_invalid_certs(true)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .no_proxy()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .build()
        .unwrap(); // Le certificat auto-signé est accepté uniquement dans ce test local.
    let url = host.info["url"].as_str().unwrap().to_owned(); // Utilise le port effectivement choisi par le noyau.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    assert_eq!(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        client
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .get(format!("{url}api/files"))
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
            .send()
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
            .await
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .unwrap()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .status(),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        401
    ); // Un visiteur anonyme ne voit pas les noms de fichiers.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    assert_eq!(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        client
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .post(format!("{url}api/join"))
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .json(&json!({"pin":"invalid","name":"Élève"}))
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
            .send()
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
            .await
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .unwrap()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .status(),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        401
    ); // PIN incorrect refusé.
    // @ligne Prépare « joined » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let joined: Value = client
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .post(format!("{url}api/join"))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .json(&json!({"pin":host.info["pin"],"name":"Élève"}))
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .send()
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .await
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .unwrap()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .json()
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .await
        .unwrap(); // Le rôle initial est fixé côté serveur.
    // @ligne Prépare « token » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let token = joined["token"].as_str().unwrap();
    let peer = joined["id"].as_str().unwrap(); // Les identités publique et secrète sont distinctes.
    assert!(!host.peers().unwrap().to_string().contains(token)); // Le panneau maître n'expose pas les secrets.
    // @ligne Prépare « listing » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let listing: Value = client
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .get(format!("{url}api/files"))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .bearer_auth(token)
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .send()
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .await
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .unwrap()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .json()
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .await
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .unwrap();
    assert_eq!(listing["upload"], false); // Lecture seule par défaut.
    // @ligne Prépare « partial » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let partial = client
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .get(format!("{url}api/files/0"))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .bearer_auth(token)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .header("Range", "bytes=0-4")
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .send()
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .await
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .unwrap();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    assert_eq!(partial.status(), 206);
    assert_eq!(partial.bytes().await.unwrap().as_ref(), b"cours"); // Reprise de téléchargement HTTP réellement exercée.
    // @ligne Prépare « body » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let body = b"bloc de contribution";
    let digest = blake3::hash(body).to_hex().to_string(); // Empreinte envoyée avec le bloc.
    // @ligne Prépare « upload » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let upload = || {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        client
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .put(format!("{url}api/upload/test-transfer"))
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .bearer_auth(token)
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .header("X-Offset", "0")
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .header("X-Blake3", &digest)
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .body(body.to_vec())
    }; // Requête identique utilisée sous différents rôles.
    assert_eq!(upload().send().await.unwrap().status(), 403); // Un lecteur ne peut pas contourner le bouton désactivé.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    host.permission(peer, true, false).unwrap();
    // @ligne Prépare « invalid » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let invalid = client
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .put(format!("{url}api/upload/invalid-transfer"))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .bearer_auth(token)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .header("X-Offset", "0")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .header("X-Blake3", "incorrect")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .body(body.to_vec())
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .send()
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .await
        .unwrap(); // Le transport TLS ne remplace pas le contrôle d'intégrité applicatif.
    assert_eq!(invalid.status(), 400); // Aucun bloc au hash invalide n'est écrit.
    // @ligne Prépare « oversized » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let oversized = client
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .put(format!("{url}api/upload/large-transfer"))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .bearer_auth(token)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .header("X-Offset", (2u64 * 1024 * 1024 * 1024).to_string())
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .header("X-Blake3", &digest)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .body(body.to_vec())
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .send()
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .await
        .unwrap(); // Une déclaration d'offset ne contourne pas le plafond par fichier.
    assert_eq!(oversized.status(), 413); // Le serveur rejette avant de créer le fichier.
                                         // Le maître accorde le rôle contributeur.
    assert_eq!(upload().send().await.unwrap().status(), 200); // Le bloc valide est accepté.
    assert_eq!(upload().send().await.unwrap().status(), 409); // Un renvoi au mauvais offset ne duplique pas les données.
    // @ligne Prépare « state » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let state: Value = client
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .get(format!("{url}api/upload/test-transfer"))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .bearer_auth(token)
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .send()
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .await
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .unwrap()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .json()
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .await
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .unwrap();
    assert_eq!(state["offset"], body.len()); // La reprise consulte la taille durable sur disque.
    // @ligne Prépare « empty » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let empty = client
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .put(format!("{url}api/upload/empty-transfer"))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .bearer_auth(token)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .header("X-Offset", "0")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .header("X-Blake3", blake3::hash(b"").to_hex().as_str())
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .header("X-Total-Size", "0")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .header("X-File-Name", "vide.txt")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .body(Vec::new())
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .send()
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .await
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .unwrap();
    assert_eq!(empty.status(), 200); // Un fichier vide est un fichier valide, pas un transfert fantôme.
    // @ligne Prépare « listing » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let listing: Value = client
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .get(format!("{url}api/files"))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .bearer_auth(token)
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .send()
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .await
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .unwrap()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .json()
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
        .await
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .unwrap();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    assert!(listing["files"]
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .as_array()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .unwrap()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .iter()
        .any(|f| f["name"] == "vide.txt" && f["size"] == 0)); // La classe voit les contributions finalisées avec leur nom d'origine.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    host.permission(peer, false, false).unwrap();
    assert_eq!(upload().send().await.unwrap().status(), 403); // Une révocation agit avec le même jeton déjà émis.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    host.permission(peer, false, true).unwrap();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    assert_eq!(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        client
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .get(format!("{url}api/files"))
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .bearer_auth(token)
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
            .send()
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
            .await
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .unwrap()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .status(),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        401
    ); // Expulsion effective sur les requêtes HTTP.
    // @ligne Prépare « room_id » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let room_id = host.info["id"].as_str().unwrap().to_owned();
    drop(host); // Simule un arrêt d'hôte sans fermer volontairement la session durable.
    tokio::time::sleep(std::time::Duration::from_millis(100)).await; // Laisse la tâche TLS libérer son port.
    let resumed = share::resume(&room_id).await.unwrap(); // Doit réutiliser exactement la même origine.
    assert_eq!(resumed.info["url"], url); // Le navigateur retrouve son stockage et les offsets.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    assert!(resumed
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .peers()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .unwrap()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .as_array()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .unwrap()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        .iter()
        .any(|p| p["id"] == peer && p["banned"] == true)); // Une expulsion reste durable.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    assert_eq!(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        client
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .get(format!("{url}api/files"))
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .bearer_auth(token)
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
            .send()
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du scénario de recette HTTPS et de stockage.
            .await
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .unwrap()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
            .status(),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
        401
    ); // Un ancien jeton expulsé ne revient pas à la vie.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    resumed.close().unwrap();
    drop(resumed); // Une fermeture explicite retire la restauration.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du scénario de recette HTTPS et de stockage.
    assert_eq!(share::saved().unwrap().as_array().unwrap().len(), 0);
    // @ligne Prépare « on_disk » pour la prochaine étape du scénario de recette HTTPS et de stockage.
    let on_disk = std::fs::read(db::path()).unwrap();
    assert!(!String::from_utf8_lossy(&on_disk).contains("highlysecret")); // Le secret n'est pas seulement caché dans l'UI.
    assert!(!classifier::redact("TOKEN=highlysecret").contains("highlysecret")); // Garde un diagnostic direct si le scénario de stockage échoue.
    std::fs::remove_dir_all(root).unwrap(); // Le test ne laisse ni certificat ni historique.
// @ligne Termine cette construction et revient au contexte parent du scénario de recette HTTPS et de stockage.
}
