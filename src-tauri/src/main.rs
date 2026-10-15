//! L'exécutable sait fonctionner comme bureau Tauri ou outil CLI sans serveur admin HTTP.
use omnishell_core::{dispatch, Result}; // Le métier reste testable sans interface graphique.
use serde_json::json;
#[cfg(feature = "desktop")]
use serde_json::Value; // Le type IPC est utilisé seulement dans le bureau. // Entrées et sorties CLI reproductibles.
use std::io::Read; // capture lit stdin pour ne pas exposer les commandes dans argv.

// Une seule commande IPC est déclarée et route vers une liste d'actions fermée.
#[cfg(feature = "desktop")]
#[tauri::command]
async fn omni(action: String, payload: Value) -> std::result::Result<Value, String> {
    dispatch(&action, payload).await.map_err(|e| e.to_string())
} // Les erreurs deviennent des messages UI.

// La CLI et le bureau partagent les migrations et le répertoire utilisateur.
#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("OmniShell : {error}");
        std::process::exit(1);
    } // Sortie non nulle utilisable par les scripts.
}

// Les sous-commandes privilégient JSON pour automatiser les opérations locales.
async fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect(); // Aucun argument implicite n'est exécuté.
    if args.is_empty() {
        #[cfg(feature = "desktop")]
        {
            // Le binaire portable reste utilisable sans GTK.
            let _ = omnishell_core::history::import(); // Première ingestion tolérante ; les erreurs restent consultables via Réimporter.
            tauri::Builder::default()
                .invoke_handler(tauri::generate_handler![omni])
                .run(tauri::generate_context!())
                .map_err(|e| e.to_string())?; // Tauri sert uniquement les assets locaux.
            return Ok(()); // La fermeture de la fenêtre termine le processus.
        }
        #[cfg(not(feature = "desktop"))]
        {
            println!("OmniShell Studio\n  import\n  daemon\n  search [texte]\n  capture-hook < commande.txt\n  share IP_LAN FICHIER...\n  capture < payload.json\n  analyze < commande.txt\n  interfaces\n  docker-sync\n  net-scan 192.168.1.0/24\n  sql < schema.sql\n  rpc ACTION < payload.json\nCompilez avec --features desktop pour ouvrir la fenêtre.");
            return Ok(());
        } // Aide honnête du mode sans GUI.
    }
    let input = || -> Result<String> {
        let mut s = String::new();
        std::io::stdin().take(1_048_577).read_to_string(&mut s)?;
        if s.len() > 1_048_576 {
            return Err("Entrée limitée à 1 Mio".into());
        }
        Ok(s)
    }; // Borne l'entrée standard.
    let result = match args[0].as_str() {
        "import" => dispatch("import", json!({})).await?, // Aucune modification du shell.
        "search" => {
            dispatch(
                "commands",
                json!({"query":args.get(1).cloned().unwrap_or_default()}),
            )
            .await?
        } // Recherche locale.
        "daemon" => {
            omnishell_core::daemon::run().await?;
            return Ok(());
        } // Service lancé explicitement.
        "capture-hook" => {
            let c = omnishell_core::db::Capture {
                raw_command: input()?,
                shell: std::env::var("OMNISHELL_SHELL").unwrap_or_default(),
                working_directory: std::env::current_dir()?.to_string_lossy().into_owned(),
                terminal_emulator: std::env::var("TERM_PROGRAM")
                    .or_else(|_| std::env::var("TERM"))
                    .unwrap_or_default(),
                exit_code: std::env::var("OMNISHELL_EXIT")
                    .ok()
                    .and_then(|s| s.parse().ok()),
                ..Default::default()
            }; // Le hook fournit le contexte sans construire du JSON en shell.
            let sent = tokio::time::timeout(
                std::time::Duration::from_millis(500),
                omnishell_core::daemon::send(&c),
            )
            .await; // Le hook reste autonome si le daemon est arrêté.
            if !matches!(sent, Ok(Ok(()))) {
                omnishell_core::db::insert(&omnishell_core::db::open()?, &c)?;
            }
            return Ok(()); // Aucun stdout à l'invite du terminal.
        }
        "share" => {
            let host = omnishell_core::share::start(
                args.iter().skip(2).cloned().collect(),
                args.get(1).ok_or("Adresse LAN requise")?.clone(),
            )
            .await?; // Salle CLI indépendante du bureau.
            println!("{}", serde_json::to_string(&host.info)?); // Le maître récupère URL, PIN et certificat sur stdout.
            tokio::signal::ctrl_c().await?;
            drop(host);
            return Ok(()); // Ctrl+C ferme l'écoute et tous les accès de la salle.
        }
        "capture" => dispatch("capture", serde_json::from_str(&input()?)?).await?, // Les hooks doivent envoyer un objet Capture.
        "analyze" => serde_json::to_value(omnishell_core::classifier::analyze(&input()?))?, // Diagnostic sans écriture.
        "interfaces" => dispatch("interfaces", json!({})).await?, // Liste les IP réellement présentes.
        "docker-sync" => dispatch("docker_sync", json!({})).await?, // Snapshot à la demande.
        "net-scan" => {
            dispatch(
                "net_scan",
                json!({"cidr":args.get(1).ok_or("CIDR requis")?}),
            )
            .await?
        } // Scan explicitement déclenché.
        "sql" => dispatch("sql_parse", json!({"ddl":input()?})).await?, // Analyse hors ligne.
        "rpc" => {
            dispatch(
                args.get(1).ok_or("Action requise")?,
                serde_json::from_str(&input()?)?,
            )
            .await?
        } // Utile pour tester les actions IPC.
        _ => {
            return Err(
                "Sous-commande inconnue ; lancez omnishell sans argument pour l'aide".into(),
            )
        } // Pas de fallback dangereux.
    };
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(()) // Résultat structuré ou erreur explicite.
}
