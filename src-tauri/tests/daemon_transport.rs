//! Le même scénario exerce le socket Unix et le Named Pipe Windows avec un vrai processus enfant.
use omnishell_core::{daemon, db}; // Les messages suivent le contrat réel des hooks.
use std::process::{Child, Command, Stdio}; // Aucun profil shell ni service système n'est modifié.
// Une garde tue toujours le processus temporaire même si une assertion de recette échoue.
struct Service(Child);
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
impl Drop for Service {
    // @ligne Déclare « daemon_transport », le point d’entrée de cette opération de la fonctionnalité locale.
    fn drop(&mut self) {
        // @ligne Prépare « _ » pour la prochaine étape de la fonctionnalité locale.
        let _ = self.0.kill();
        // @ligne Prépare « _ » pour la prochaine étape de la fonctionnalité locale.
        let _ = self.0.wait();
    // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
    }
// @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
}
// Le profil synthétique et un PATH vide empêchent l'import d'historiques ou de conteneurs réels.
fn launch(root: &std::path::Path) -> Service {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
    Service(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        Command::new(env!("CARGO_BIN_EXE_omnishell-daemon"))
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            .env("OMNISHELL_DATA_DIR", root)
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            .env("HOME", root)
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            .env("USERPROFILE", root)
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            .env("APPDATA", root)
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            .env("PATH", root)
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            .env_remove("HISTFILE")
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            .stdout(Stdio::null())
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            .stderr(Stdio::null())
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            .spawn()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            .unwrap(),
    // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
    )
// @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
}
#[tokio::test] // Un redémarrage doit garder la clé et les reçus mais libérer le transport précédent.
// @ligne Déclare « daemon_transport », le point d’entrée de cette opération de la fonctionnalité locale.
async fn authenticated_capture_survives_daemon_restart() {
    // @ligne Prépare « root » pour la prochaine étape de la fonctionnalité locale.
    let root = std::env::temp_dir().join(format!("omni-transport-{}", std::process::id()));
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
    std::fs::create_dir_all(&root).unwrap();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
    std::env::set_var("OMNISHELL_DATA_DIR", &root);
    // @ligne Prépare « capture » pour la prochaine étape de la fonctionnalité locale.
    let capture = db::Capture {
        // @ligne Renseigne le champ « event_id » de la structure utilisée ici de la fonctionnalité locale.
        event_id: Some("same-event-across-retries".into()),
        // @ligne Renseigne le champ « raw_command » de la structure utilisée ici de la fonctionnalité locale.
        raw_command: "TOKEN=transport-secret git status".into(),
        // @ligne Renseigne le champ « shell » de la structure utilisée ici de la fonctionnalité locale.
        shell: "test".into(),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        ..Default::default()
    // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
    };
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la fonctionnalité locale.
    for _ in 0..2 {
        // @ligne Prépare « service » pour la prochaine étape de la fonctionnalité locale.
        let service = launch(&root);
        // @ligne Prépare « accepted » pour la prochaine étape de la fonctionnalité locale.
        let mut accepted = false;
        // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la fonctionnalité locale.
        for _ in 0..50 {
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
            if daemon::send(&capture).await.is_ok() {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                accepted = true;
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                break;
            // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
            }
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la fonctionnalité locale.
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
        }
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        assert!(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            accepted,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            "Le daemon doit accepter le transport local authentifié"
        // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
        );
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        drop(service);
    // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
    }
    // @ligne Prépare « data » pour la prochaine étape de la fonctionnalité locale.
    let data = db::commands("git status", "", false, 0).unwrap();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
    assert_eq!(data["total"], 1);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
    assert_eq!(data["items"][0]["count"], 1);
    assert!(!data.to_string().contains("transport-secret")); // Le retry n'expose ni ne recompte un secret.
    std::fs::remove_dir_all(root).unwrap(); // Le scénario ne laisse pas de socket, clé ou historique sur le compte courant.
// @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
}
