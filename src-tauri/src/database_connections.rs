//! Introspection native des schémas PostgreSQL et MySQL, sans conserver les identifiants.
use crate::{dba, Result}; // Tous les modèles passent ensuite par le même parseur de DDL.
use serde_json::{json, Value}; // La réponse n'inclut jamais l'URL de connexion.
use sqlx::{Connection, Row}; // Les valeurs sont liées, les identifiants sont échappés par dialecte.
// Une échéance globale borne connexion et collecte, même si un moteur devient indisponible.
pub async fn inspect(engine: &str, url: &str) -> Result<Value> {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de l’introspection SQL en lecture de schéma.
    if url.len() > 8192 {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de l’introspection SQL en lecture de schéma.
        return Err("URL de connexion trop longue".into());
    // @ligne Termine cette construction et revient au contexte parent de l’introspection SQL en lecture de schéma.
    }
    // @ligne Prépare « result » pour la prochaine étape de l’introspection SQL en lecture de schéma.
    let result = tokio::time::timeout(std::time::Duration::from_secs(20), async {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de l’introspection SQL en lecture de schéma.
        match engine {
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de l’introspection SQL en lecture de schéma.
            "postgres" => postgres(url).await,
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de l’introspection SQL en lecture de schéma.
            "mysql" => mysql(url).await,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
            _ => Err("Moteur attendu : postgres ou mysql".into()),
        // @ligne Termine cette construction et revient au contexte parent de l’introspection SQL en lecture de schéma.
        }
    // @ligne Termine cette construction et revient au contexte parent de l’introspection SQL en lecture de schéma.
    })
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de l’introspection SQL en lecture de schéma.
    .await
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
    .map_err(|_| "Connexion ou introspection supérieure à 20 secondes")?;
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
    result.map_err(|_|"Connexion ou introspection refusée ; vérifier l'adresse, TLS et les droits de lecture du schéma".into())
    // Les erreurs du pilote peuvent contenir des informations de connexion : elles ne sont pas renvoyées brutes.
}
// PostgreSQL fournit lui-même les types et définitions de contraintes de son catalogue.
async fn postgres(url: &str) -> Result<Value> {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de l’introspection SQL en lecture de schéma.
    if !url.starts_with("postgres://") && !url.starts_with("postgresql://") {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de l’introspection SQL en lecture de schéma.
        return Err("URL PostgreSQL invalide".into());
    // @ligne Termine cette construction et revient au contexte parent de l’introspection SQL en lecture de schéma.
    }
    let mut connection = sqlx::PgConnection::connect(url).await?; // Les identifiants restent seulement dans la mémoire du pilote.
    // @ligne Prépare « transaction » pour la prochaine étape de l’introspection SQL en lecture de schéma.
    let mut transaction = connection.begin().await?;
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
    sqlx::query("SET TRANSACTION READ ONLY")
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de l’introspection SQL en lecture de schéma.
        .execute(&mut *transaction)
        .await?; // L'introspection ne peut exécuter une modification métier.
    sqlx::query("SET LOCAL search_path = ''").execute(&mut *transaction).await?; // Les références du catalogue deviennent qualifiées et stables entre schémas.
    // @ligne Prépare « tables » pour la prochaine étape de l’introspection SQL en lecture de schéma.
    let tables=sqlx::query("SELECT n.nspname AS schema_name,c.relname AS table_name,c.oid::bigint AS oid FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace WHERE c.relkind='r' AND n.nspname NOT IN ('pg_catalog','information_schema') ORDER BY n.nspname,c.relname LIMIT 501").fetch_all(&mut *transaction).await?;
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de l’introspection SQL en lecture de schéma.
    if tables.len() > 500 {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de l’introspection SQL en lecture de schéma.
        return Err("Introspection limitée à 500 tables".into());
    } // Borne la mémoire et le temps de reconstruction.
    // @ligne Prépare « quote » pour la prochaine étape de l’introspection SQL en lecture de schéma.
    let quote = |s: &str| format!("\"{}\"", s.replace('"', "\"\""));
    // @ligne Prépare « ddl » pour la prochaine étape de l’introspection SQL en lecture de schéma.
    let mut ddl = String::new();
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de l’introspection SQL en lecture de schéma.
    for table in tables {
        // @ligne Prépare « oid » pour la prochaine étape de l’introspection SQL en lecture de schéma.
        let oid: i64 = table.try_get("oid")?;
        // @ligne Prépare « schema » pour la prochaine étape de l’introspection SQL en lecture de schéma.
        let schema: String = table.try_get("schema_name")?;
        // @ligne Prépare « name » pour la prochaine étape de l’introspection SQL en lecture de schéma.
        let name: String = table.try_get("table_name")?;
        // @ligne Prépare « columns » pour la prochaine étape de l’introspection SQL en lecture de schéma.
        let columns=sqlx::query("SELECT a.attname AS name,format_type(a.atttypid,a.atttypmod) AS data_type,a.attnotnull AS required FROM pg_attribute a WHERE a.attrelid=$1::bigint::oid AND a.attnum>0 AND NOT a.attisdropped ORDER BY a.attnum").bind(oid).fetch_all(&mut *transaction).await?;
        // @ligne Prépare « fields » pour la prochaine étape de l’introspection SQL en lecture de schéma.
        let mut fields = columns
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
            .into_iter()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
            .map(|c| -> Result<String> {
                // @ligne Prépare « name » pour la prochaine étape de l’introspection SQL en lecture de schéma.
                let name: String = c.try_get("name")?;
                // @ligne Prépare « kind » pour la prochaine étape de l’introspection SQL en lecture de schéma.
                let kind: String = c.try_get("data_type")?;
                // @ligne Prépare « required » pour la prochaine étape de l’introspection SQL en lecture de schéma.
                let required: bool = c.try_get("required")?;
                // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de l’introspection SQL en lecture de schéma.
                Ok(format!(
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
                    "{} {}{}",
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
                    quote(&name),
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
                    kind,
                    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de l’introspection SQL en lecture de schéma.
                    if required { " NOT NULL" } else { "" }
                // @ligne Termine cette construction et revient au contexte parent de l’introspection SQL en lecture de schéma.
                ))
            // @ligne Termine cette construction et revient au contexte parent de l’introspection SQL en lecture de schéma.
            })
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
            .collect::<Result<Vec<_>>>()?;
        // @ligne Prépare « constraints » pour la prochaine étape de l’introspection SQL en lecture de schéma.
        let constraints=sqlx::query("SELECT pg_get_constraintdef(oid) AS definition FROM pg_constraint WHERE conrelid=$1::bigint::oid AND contype IN ('p','u','f','c') ORDER BY conname").bind(oid).fetch_all(&mut *transaction).await?;
        // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de l’introspection SQL en lecture de schéma.
        for c in constraints {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
            fields.push(c.try_get::<String, _>("definition")?);
        } // Inclut les clés composites et leurs références.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
        ddl.push_str(&format!(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
            "CREATE TABLE {}.{} ({});\n",
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
            quote(&schema),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
            quote(&name),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
            fields.join(",\n")
        // @ligne Termine cette construction et revient au contexte parent de l’introspection SQL en lecture de schéma.
        ));
    // @ligne Termine cette construction et revient au contexte parent de l’introspection SQL en lecture de schéma.
    }
    transaction.rollback().await?; // Termine explicitement la transaction de lecture.
    // @ligne Prépare « model » pour la prochaine étape de l’introspection SQL en lecture de schéma.
    let mut model = dba::parse(&ddl)?;
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
    model["ddl"] = json!(ddl);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
    model["engine"] = json!("postgres");
    Ok(model) // Seul le schéma quitte la fonction.
// @ligne Termine cette construction et revient au contexte parent de l’introspection SQL en lecture de schéma.
}
// MySQL expose son DDL canonique ; les noms proviennent uniquement d'information_schema.
async fn mysql(url: &str) -> Result<Value> {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de l’introspection SQL en lecture de schéma.
    if !url.starts_with("mysql://") {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de l’introspection SQL en lecture de schéma.
        return Err("URL MySQL invalide".into());
    // @ligne Termine cette construction et revient au contexte parent de l’introspection SQL en lecture de schéma.
    }
    // @ligne Prépare « connection » pour la prochaine étape de l’introspection SQL en lecture de schéma.
    let mut connection = sqlx::MySqlConnection::connect(url).await?;
    // @ligne Prépare « tables » pour la prochaine étape de l’introspection SQL en lecture de schéma.
    let tables=sqlx::query("SELECT TABLE_NAME AS name FROM information_schema.TABLES WHERE TABLE_SCHEMA=DATABASE() AND TABLE_TYPE='BASE TABLE' ORDER BY TABLE_NAME LIMIT 501").fetch_all(&mut connection).await?;
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de l’introspection SQL en lecture de schéma.
    if tables.len() > 500 {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de l’introspection SQL en lecture de schéma.
        return Err("Introspection limitée à 500 tables".into());
    // @ligne Termine cette construction et revient au contexte parent de l’introspection SQL en lecture de schéma.
    }
    // @ligne Prépare « ddl » pour la prochaine étape de l’introspection SQL en lecture de schéma.
    let mut ddl = String::new();
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de l’introspection SQL en lecture de schéma.
    for table in tables {
        // @ligne Prépare « name » pour la prochaine étape de l’introspection SQL en lecture de schéma.
        let name: String = table.try_get("name")?;
        // @ligne Prépare « quoted » pour la prochaine étape de l’introspection SQL en lecture de schéma.
        let quoted = format!("`{}`", name.replace('`', "``"));
        // @ligne Prépare « row » pour la prochaine étape de l’introspection SQL en lecture de schéma.
        let row = sqlx::query(&format!("SHOW CREATE TABLE {quoted}"))
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
            .fetch_one(&mut connection)
            // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de l’introspection SQL en lecture de schéma.
            .await?;
        // @ligne Prépare « definition » pour la prochaine étape de l’introspection SQL en lecture de schéma.
        let definition: String = row.try_get(1usize)?;
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
        ddl.push_str(&definition);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
        ddl.push_str(";\n");
    } // La requête n'accepte jamais un identifiant libre envoyé par l'UI.
    // @ligne Prépare « model » pour la prochaine étape de l’introspection SQL en lecture de schéma.
    let mut model = dba::parse(&ddl)?;
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
    model["ddl"] = json!(ddl);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
    model["engine"] = json!("mysql");
    Ok(model) // Aucun dump de lignes métier n'est effectué.
// @ligne Termine cette construction et revient au contexte parent de l’introspection SQL en lecture de schéma.
}
#[cfg(test)] // Les moteurs réels sont fournis par un laboratoire ou le script de recette Docker.
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
mod tests {
    use super::*; // Les identifiants de test ne sont jamais écrits dans le dépôt.
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant de l’introspection SQL en lecture de schéma.
    #[tokio::test]
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant de l’introspection SQL en lecture de schéma.
    #[ignore = "Nécessite OMNI_TEST_PG_URL pointant vers une base éphémère"]
    // @ligne Déclare « database_connections », le point d’entrée de cette opération de l’introspection SQL en lecture de schéma.
    async fn postgres_live_schema() {
        // @ligne Prépare « url » pour la prochaine étape de l’introspection SQL en lecture de schéma.
        let url = std::env::var("OMNI_TEST_PG_URL").unwrap();
        // @ligne Prépare « conn » pour la prochaine étape de l’introspection SQL en lecture de schéma.
        let mut conn = sqlx::PgConnection::connect(&url).await.unwrap();
        // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de l’introspection SQL en lecture de schéma.
        for ddl in ["CREATE TABLE omni_parents(id INTEGER PRIMARY KEY)","CREATE TABLE omni_children(id INTEGER PRIMARY KEY, parent_id INTEGER NOT NULL REFERENCES omni_parents(id))"]{sqlx::query(ddl).execute(&mut conn).await.unwrap();}
        // @ligne Prépare « model » pour la prochaine étape de l’introspection SQL en lecture de schéma.
        let model = inspect("postgres", &url).await.unwrap();
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
        assert_eq!(model["tables"].as_array().unwrap().len(), 2);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
        assert_eq!(model["relations"].as_array().unwrap().len(), 1);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
        assert_eq!(model["relations"][0]["source_cardinality"], "1,1");
        assert!(model["relations"].as_array().unwrap().iter().all(|relation|model["tables"].as_array().unwrap().iter().any(|table|table["name"]==relation["to"]))); // Chaque relation du catalogue rejoint effectivement un nœud du canevas.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
        assert!(!model.to_string().contains(&url));
    } // Le scénario utilise les vrais catalogues PostgreSQL.
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant de l’introspection SQL en lecture de schéma.
    #[tokio::test]
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant de l’introspection SQL en lecture de schéma.
    #[ignore = "Nécessite OMNI_TEST_MYSQL_URL pointant vers une base éphémère"]
    // @ligne Déclare « database_connections », le point d’entrée de cette opération de l’introspection SQL en lecture de schéma.
    async fn mysql_live_schema() {
        // @ligne Prépare « url » pour la prochaine étape de l’introspection SQL en lecture de schéma.
        let url = std::env::var("OMNI_TEST_MYSQL_URL").unwrap();
        // @ligne Prépare « conn » pour la prochaine étape de l’introspection SQL en lecture de schéma.
        let mut conn = sqlx::MySqlConnection::connect(&url).await.unwrap();
        // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de l’introspection SQL en lecture de schéma.
        for ddl in ["CREATE TABLE omni_parents(id INTEGER PRIMARY KEY)","CREATE TABLE omni_children(id INTEGER PRIMARY KEY,parent_id INTEGER NOT NULL, FOREIGN KEY(parent_id) REFERENCES omni_parents(id))"]{sqlx::query(ddl).execute(&mut conn).await.unwrap();}
        // @ligne Prépare « model » pour la prochaine étape de l’introspection SQL en lecture de schéma.
        let model = inspect("mysql", &url).await.unwrap();
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
        assert_eq!(model["tables"].as_array().unwrap().len(), 2);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
        assert_eq!(model["relations"].as_array().unwrap().len(), 1);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’introspection SQL en lecture de schéma.
        assert!(!model.to_string().contains(&url));
    } // Le scénario contrôle le DDL canonique MySQL/MariaDB.
// @ligne Termine cette construction et revient au contexte parent de l’introspection SQL en lecture de schéma.
}
