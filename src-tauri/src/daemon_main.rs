//! Le service résident ne référence pas Tauri : il ne charge aucune bibliothèque WebView.
#[tokio::main(worker_threads = 2)] // Deux workers suffisent aux captures et aux événements Docker locaux.
// @ligne Déclare « daemon_main », le point d’entrée de cette opération de la fonctionnalité locale.
async fn main() {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
    if let Err(error) = omnishell_core::daemon::run().await {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        eprintln!("OmniShell daemon : {error}");
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        std::process::exit(1);
    } // Une erreur reste exploitable par systemd ou le superviseur Windows.
// @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
}
