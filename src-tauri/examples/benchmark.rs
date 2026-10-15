//! Mesure reproductible du vrai chemin SQLite/FTS5 avec cent mille commandes synthétiques.
use omnishell_core::{db, Result}; // Réutilise les fonctions du produit, pas une requête simplifiée de démonstration.
use serde_json::json; // Le rapport peut être archivé avec les résultats de recette.
// @ligne Déclare « benchmark », le point d’entrée de cette opération de la mesure sur cent mille commandes synthétiques.
fn main() -> Result<()> {
    let root = std::env::temp_dir().join(format!("omni-benchmark-{}", std::process::id())); // Le corpus n'interfère jamais avec l'historique utilisateur.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
    std::fs::create_dir_all(&root)?;
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
    std::env::set_var("OMNISHELL_DATA_DIR", &root);
    // @ligne Prépare « conn » pour la prochaine étape de la mesure sur cent mille commandes synthétiques.
    let mut conn = db::open()?;
    // @ligne Prépare « start » pour la prochaine étape de la mesure sur cent mille commandes synthétiques.
    let start = std::time::Instant::now();
    let tx = conn.transaction()?; // L'import initial ne fait pas cent mille synchronisations disque séparées.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la mesure sur cent mille commandes synthétiques.
    for index in 0..100_000 {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
        db::insert(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
            &tx,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
            &db::Capture {
                // @ligne Renseigne le champ « raw_command » de la structure utilisée ici de la mesure sur cent mille commandes synthétiques.
                raw_command: format!(
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
                    "docker inspect application-{index:06} --format '{{{{json .State}}}}'"
                // @ligne Termine cette construction et revient au contexte parent de la mesure sur cent mille commandes synthétiques.
                ),
                // @ligne Renseigne le champ « shell » de la structure utilisée ici de la mesure sur cent mille commandes synthétiques.
                shell: "synthetic".into(),
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
                ..Default::default()
            // @ligne Termine cette construction et revient au contexte parent de la mesure sur cent mille commandes synthétiques.
            },
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
        )?;
    // @ligne Termine cette construction et revient au contexte parent de la mesure sur cent mille commandes synthétiques.
    }
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
    tx.commit()?;
    let import_ms = start.elapsed().as_millis(); // Inclut le filtrage, le hachage, les index et les triggers.
    let mut timings = Vec::new(); // Un ensemble de requêtes ne doit pas se limiter à un résultat en cache.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la mesure sur cent mille commandes synthétiques.
    for index in 0..200 {
        // @ligne Prépare « start » pour la prochaine étape de la mesure sur cent mille commandes synthétiques.
        let start = std::time::Instant::now();
        // @ligne Prépare « result » pour la prochaine étape de la mesure sur cent mille commandes synthétiques.
        let result = db::commands(&format!("application-{:06}", index * 499), "", false, 0)?;
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
        assert_eq!(result["total"], 1);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
        timings.push(start.elapsed().as_secs_f64() * 1000.0);
    // @ligne Termine cette construction et revient au contexte parent de la mesure sur cent mille commandes synthétiques.
    }
    timings.sort_by(f64::total_cmp); // Rapporte les percentiles de la chaîne complète d'ouverture et recherche.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
    println!(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
        "{}",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
        serde_json::to_string_pretty(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
            &json!({"corpus":100000,"requests":timings.len(),"import_ms":import_ms,"search_median_ms":timings[100],"search_p95_ms":timings[190],"search_p99_ms":timings[198],"database_bytes":std::fs::metadata(db::path())?.len(),"scope":"ouverture SQLite, migration idempotente, filtre FTS5, comptage et sérialisation JSON ; rendu UI exclu"})
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
        )?
    // @ligne Termine cette construction et revient au contexte parent de la mesure sur cent mille commandes synthétiques.
    );
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
    drop(conn);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la mesure sur cent mille commandes synthétiques.
    std::fs::remove_dir_all(root)?;
    Ok(()) // Supprime le corpus synthétique seulement après fermeture de la connexion.
// @ligne Termine cette construction et revient au contexte parent de la mesure sur cent mille commandes synthétiques.
}
