//! L'exécutable sait fonctionner comme bureau Tauri ou outil CLI sans serveur admin HTTP.
use omnishell_core::{dispatch, Result}; // Le métier reste testable sans interface graphique.
// @ligne Importe les outils nécessaires à la mise en œuvre du bureau et des parcours utilisateur.
use serde_json::json;
// @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du bureau et des parcours utilisateur.
#[cfg(feature = "desktop")]
use serde_json::Value; // Le type IPC est utilisé seulement dans le bureau. // Entrées et sorties CLI reproductibles.
use std::io::Read; // capture lit stdin pour ne pas exposer les commandes dans argv.
// Une seule commande IPC est déclarée et route vers une liste d'actions fermée.
#[cfg(feature = "desktop")]
// @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du bureau et des parcours utilisateur.
#[tauri::command]
// @ligne Déclare « main », le point d’entrée de cette opération du bureau et des parcours utilisateur.
async fn omni(action: String, payload: Value) -> std::result::Result<Value, String> {
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
    dispatch(&action, payload).await.map_err(|e| e.to_string())
} // Les erreurs deviennent des messages UI.
// La CLI et le bureau partagent les migrations et le répertoire utilisateur.
#[tokio::main(worker_threads = 2)]
// @ligne Déclare « main », le point d’entrée de cette opération du bureau et des parcours utilisateur.
async fn main() {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
    if let Err(error) = run().await {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        eprintln!("OmniShell : {error}");
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        std::process::exit(1);
    } // Sortie non nulle utilisable par les scripts.
// @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
}
// Les sous-commandes privilégient JSON pour automatiser les opérations locales.
async fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect(); // Aucun argument implicite n'est exécuté.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
    if args.is_empty() {
        // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du bureau et des parcours utilisateur.
        #[cfg(feature = "desktop")]
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        {
            // Le binaire portable reste utilisable sans GTK.
            let _ = omnishell_core::history::import(); // Première ingestion tolérante ; les erreurs restent consultables via Réimporter.
            tauri::async_runtime::set(tokio::runtime::Handle::current()); // Le bureau réutilise les deux workers au lieu de créer un second pool.
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            tauri::Builder::default()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                .plugin(tauri_plugin_dialog::init())
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                .plugin(tauri_plugin_opener::init())
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                .invoke_handler(tauri::generate_handler![omni])
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                .run(tauri::generate_context!())
                .map_err(|e| e.to_string())?; // Tauri sert uniquement les assets locaux.
            return Ok(()); // La fermeture de la fenêtre termine le processus.
        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
        }
        // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du bureau et des parcours utilisateur.
        #[cfg(not(feature = "desktop"))]
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            println!("OmniShell Studio\n  import\n  daemon\n  search [texte]\n  capture-hook < commande.txt\n  share IP_LAN FICHIER...\n  capture < payload.json\n  analyze < commande.txt\n  interfaces\n  docker-sync\n  net-scan 192.168.1.0/24\n  sql < schema.sql\n  rpc ACTION < payload.json\nCompilez avec --features desktop pour ouvrir la fenêtre.");
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du bureau et des parcours utilisateur.
            return Ok(());
        } // Aide honnête du mode sans GUI.
    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
    }
    // @ligne Prépare « input » pour la prochaine étape du bureau et des parcours utilisateur.
    let input = || -> Result<String> {
        // @ligne Prépare « s » pour la prochaine étape du bureau et des parcours utilisateur.
        let mut s = String::new();
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        std::io::stdin().take(1_048_577).read_to_string(&mut s)?;
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
        if s.len() > 1_048_576 {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du bureau et des parcours utilisateur.
            return Err("Entrée limitée à 1 Mio".into());
        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
        }
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du bureau et des parcours utilisateur.
        Ok(s)
    }; // Borne l'entrée standard.
    // @ligne Prépare « result » pour la prochaine étape du bureau et des parcours utilisateur.
    let result = match args[0].as_str() {
        "import" => dispatch("import", json!({})).await?, // Aucune modification du shell.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        "search" => {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            dispatch(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                "commands",
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                json!({"query":args.get(1).cloned().unwrap_or_default()}),
            // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
            )
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
            .await?
        } // Recherche locale.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        "daemon" => {
            // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du bureau et des parcours utilisateur.
            #[cfg(unix)]
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            {
                use std::os::unix::process::CommandExt; // exec libère les bibliothèques GUI du processus parent.
                // @ligne Prépare « dedicated » pour la prochaine étape du bureau et des parcours utilisateur.
                let dedicated = std::env::current_exe()?.with_file_name("omnishell-daemon");
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
                if dedicated.is_file() {
                    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du bureau et des parcours utilisateur.
                    return Err(std::process::Command::new(dedicated).exec().into());
                } // L'absence de sidecar conserve le mode portable existant.
            // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
            }
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
            omnishell_core::daemon::run().await?;
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du bureau et des parcours utilisateur.
            return Ok(());
        } // Service lancé explicitement.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        "capture-hook" => {
            // @ligne Prépare « c » pour la prochaine étape du bureau et des parcours utilisateur.
            let c = omnishell_core::db::Capture {
                event_id: Some(format!("{:032x}", rand::random::<u128>())), // Un retry garde l’identité de cette capture.
                // @ligne Renseigne le champ « raw_command » de la structure utilisée ici du bureau et des parcours utilisateur.
                raw_command: input()?,
                // @ligne Renseigne le champ « shell » de la structure utilisée ici du bureau et des parcours utilisateur.
                shell: std::env::var("OMNISHELL_SHELL").unwrap_or_default(),
                // @ligne Renseigne le champ « working_directory » de la structure utilisée ici du bureau et des parcours utilisateur.
                working_directory: std::env::current_dir()?.to_string_lossy().into_owned(),
                terminal_emulator: omnishell_core::history::terminal(), // Identifie les parents du processus quand TERM_PROGRAM ne suffit pas.
                // @ligne Renseigne le champ « duration_ms » de la structure utilisée ici du bureau et des parcours utilisateur.
                duration_ms: std::env::var("OMNISHELL_DURATION")
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    .ok()
                    .and_then(|s| s.parse().ok()), // Durée mesurée par le shell, absente si non disponible.
                // @ligne Renseigne le champ « exit_code » de la structure utilisée ici du bureau et des parcours utilisateur.
                exit_code: std::env::var("OMNISHELL_EXIT")
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    .ok()
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    .and_then(|s| s.parse().ok()),
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                ..Default::default()
            }; // Le hook fournit le contexte sans construire du JSON en shell.
            // @ligne Prépare « sent » pour la prochaine étape du bureau et des parcours utilisateur.
            let sent = tokio::time::timeout(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                std::time::Duration::from_millis(500),
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                omnishell_core::daemon::send(&c),
            // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
            )
            .await; // Le hook reste autonome si le daemon est arrêté.
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
            if !matches!(sent, Ok(Ok(()))) {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                omnishell_core::db::insert(&omnishell_core::db::open()?, &c)?;
            // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
            }
            return Ok(()); // Aucun stdout à l'invite du terminal.
        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
        }
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        "share" => {
            // @ligne Prépare « host » pour la prochaine étape du bureau et des parcours utilisateur.
            let host = omnishell_core::share::start(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                args.iter().skip(2).cloned().collect(),
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                args.get(1).ok_or("Adresse LAN requise")?.clone(),
            // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
            )
            .await?; // Salle CLI indépendante du bureau.
            println!("{}", serde_json::to_string(&host.info)?); // Le maître récupère URL, PIN et certificat sur stdout.
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
            tokio::signal::ctrl_c().await?;
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            drop(host);
            return Ok(()); // Ctrl+C ferme l'écoute et tous les accès de la salle.
        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
        }
        "capture" => dispatch("capture", serde_json::from_str(&input()?)?).await?, // Les hooks doivent envoyer un objet Capture.
        "analyze" => serde_json::to_value(omnishell_core::classifier::analyze(&input()?))?, // Diagnostic sans écriture.
        "interfaces" => dispatch("interfaces", json!({})).await?, // Liste les IP réellement présentes.
        "docker-sync" => dispatch("docker_sync", json!({})).await?, // Snapshot à la demande.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        "net-scan" => {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            dispatch(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                "net_scan",
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                json!({"cidr":args.get(1).ok_or("CIDR requis")?}),
            // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
            )
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
            .await?
        } // Scan explicitement déclenché.
        "sql" => dispatch("sql_parse", json!({"ddl":input()?})).await?, // Analyse hors ligne.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        "rpc" => {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            dispatch(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                args.get(1).ok_or("Action requise")?,
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                serde_json::from_str(&input()?)?,
            // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
            )
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
            .await?
        } // Utile pour tester les actions IPC.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        _ => {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du bureau et des parcours utilisateur.
            return Err(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                "Sous-commande inconnue ; lancez omnishell sans argument pour l'aide".into(),
            // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
            );
        } // Pas de fallback dangereux.
    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
    };
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(()) // Résultat structuré ou erreur explicite.
// @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
}
