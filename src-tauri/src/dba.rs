//! Le parseur SQL comprend les contraintes structurées sans exécuter le DDL.
use crate::{db, Result}; // Sauvegarde facultative des modèles dans SQLite.
use serde_json::{json, Value}; // Format neutre pour canevas et export.
// @ligne Importe les outils nécessaires à la mise en œuvre du modèle relationnel et de ses contraintes.
use sqlparser::{
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
    ast::{AlterTableOperation, ColumnOption, Statement, TableConstraint},
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
    dialect::GenericDialect,
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
    parser::Parser,
}; // Analyse SQL réelle plutôt qu'une découpe à la virgule.
// La clé graphique utilise les identifiants SQL, indépendamment de leur écriture avec guillemets.
fn object_key(name:&sqlparser::ast::ObjectName)->String{name.0.iter().map(|part|part.as_ident().map(|id|id.value.clone()).unwrap_or_else(||part.to_string())).collect::<Vec<_>>().join(".")}
// Le modèle relationnel est dérivé des contraintes présentes ; le MCD reste une interprétation.
pub fn parse(ddl: &str) -> Result<Value> {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du modèle relationnel et de ses contraintes.
    if ddl.len() > 1_048_576 {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du modèle relationnel et de ses contraintes.
        return Err("Script SQL limité à 1 Mio".into());
    } // Borne le coût d'analyse.
    // @ligne Prépare « statements » pour la prochaine étape du modèle relationnel et de ses contraintes.
    let statements = Parser::parse_sql(&GenericDialect {}, ddl)
        .or_else(|_| Parser::parse_sql(&sqlparser::dialect::MySqlDialect {}, ddl))?; // N'exécute aucune instruction SQL fournie.
    // @ligne Prépare « tables » pour la prochaine étape du modèle relationnel et de ses contraintes.
    let mut tables = Vec::new();
    let mut relations = Vec::new(); // Chaque FK fournit une association et des cardinalités déductibles du DDL. // Les instructions hors CREATE TABLE sont ignorées.
    // @ligne Prépare « alterations » pour la prochaine étape du modèle relationnel et de ses contraintes.
    let alterations = statements
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        .iter()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        .filter_map(|statement| {
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du modèle relationnel et de ses contraintes.
            if let Statement::AlterTable {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                name, operations, ..
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
            } = statement
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
            {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                Some((object_key(name), operations.clone()))
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
            } else {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                None
            // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
            }
        // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
        })
        .collect::<Vec<_>>(); // Les dumps PostgreSQL déclarent souvent leurs contraintes après les tables.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du modèle relationnel et de ses contraintes.
    for stmt in statements {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du modèle relationnel et de ses contraintes.
        if let Statement::CreateTable(mut table) = stmt {
            // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du modèle relationnel et de ses contraintes.
            for (name, operations) in &alterations {
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du modèle relationnel et de ses contraintes.
                if *name == object_key(&table.name) {
                    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du modèle relationnel et de ses contraintes.
                    for operation in operations {
                        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du modèle relationnel et de ses contraintes.
                        if let AlterTableOperation::AddConstraint(constraint) = operation {
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                            table.constraints.push(constraint.clone());
                        // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
                        }
                    // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
                    }
                // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
                }
            } // Intègre les FK différées dans le même modèle relationnel.
            let table_name = object_key(&table.name); // Les contraintes seront rattachées à leur table source.
            let mut keys: Vec<Vec<String>> = Vec::new(); // Les clés uniques limitent la cardinalité inverse.
            // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du modèle relationnel et de ses contraintes.
            for c in &table.columns {
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du modèle relationnel et de ses contraintes.
                if c.options
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    .iter()
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    .any(|o| matches!(o.option, ColumnOption::Unique { .. }))
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    keys.push(vec![c.name.value.clone()]);
                // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
                }
            } // Clés déclarées sur une colonne.
            // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du modèle relationnel et de ses contraintes.
            for c in &table.constraints {
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du modèle relationnel et de ses contraintes.
                if let TableConstraint::PrimaryKey { columns, .. }
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                | TableConstraint::Unique { columns, .. } = c
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    keys.push(columns.iter().map(|c| c.value.clone()).collect());
                // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
                }
            } // Clés composites.
            // @ligne Prépare « add_relation » pour la prochaine étape du modèle relationnel et de ses contraintes.
            let add_relation = |source: Vec<String>,
                                // @ligne Renseigne le champ « target » de la structure utilisée ici du modèle relationnel et de ses contraintes.
                                target: String,
                                // @ligne Renseigne le champ « referred » de la structure utilisée ici du modèle relationnel et de ses contraintes.
                                referred: Vec<String>,
                                // @ligne Renseigne le champ « relations » de la structure utilisée ici du modèle relationnel et de ses contraintes.
                                relations: &mut Vec<Value>| {
                let required=source.iter().all(|name|table.columns.iter().find(|c|c.name.value==*name).map(|c|c.options.iter().any(|o|matches!(o.option,ColumnOption::NotNull|ColumnOption::Unique{is_primary:true,..}))||table.constraints.iter().any(|k|matches!(k,TableConstraint::PrimaryKey{columns,..} if columns.iter().any(|c|c.value==*name)))).unwrap_or(false)); // Une FK nullable rend l'association facultative.
                // @ligne Prépare « unique » pour la prochaine étape du modèle relationnel et de ses contraintes.
                let unique = keys
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    .iter()
                    .any(|key| key.len() == source.len() && key.iter().all(|c| source.contains(c))); // Seule une clé couvrant exactement la FK limite le nombre d'enfants.
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                relations.push(json!({"from":table_name,"to":target,"columns":source,"referenced":referred,"source_cardinality":if required {"1,1"} else {"0,1"},"target_cardinality":if unique {"0,1"} else {"0,n"}}));
                // Le minimum côté parent n'est jamais inventé.
            };
            // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du modèle relationnel et de ses contraintes.
            for c in &table.columns {
                // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du modèle relationnel et de ses contraintes.
                for option in &c.options {
                    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du modèle relationnel et de ses contraintes.
                    if let ColumnOption::ForeignKey {
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                        foreign_table,
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                        referred_columns,
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                        ..
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    } = &option.option
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    {
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                        add_relation(
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                            vec![c.name.value.clone()],
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                            object_key(foreign_table),
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                            referred_columns.iter().map(|c| c.value.clone()).collect(),
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                            &mut relations,
                        // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
                        );
                    // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
                    }
                // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
                }
            } // FK déclarées directement sur les colonnes.
            // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du modèle relationnel et de ses contraintes.
            for c in &table.constraints {
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du modèle relationnel et de ses contraintes.
                if let TableConstraint::ForeignKey {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    columns,
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    foreign_table,
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    referred_columns,
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    ..
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                } = c
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    add_relation(
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                        columns.iter().map(|c| c.value.clone()).collect(),
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                        object_key(foreign_table),
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                        referred_columns.iter().map(|c| c.value.clone()).collect(),
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                        &mut relations,
                    // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
                    );
                // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
                }
            } // FK simples et composites nommées.
            let columns = table.columns.iter().map(|c|json!({"name":c.name.value,"type":c.data_type.to_string(),"constraints":c.options.iter().map(|o|o.option.to_string()).collect::<Vec<_>>() })).collect::<Vec<_>>(); // Préserve types, PK, FK et nullabilité.
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
            tables.push(json!({"name":object_key(&table.name),"columns":columns,"constraints":table.constraints.iter().map(ToString::to_string).collect::<Vec<_>>() }));
            // Contraintes composites conservées.
        }
    // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
    }
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du modèle relationnel et de ses contraintes.
    if tables.is_empty() {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du modèle relationnel et de ses contraintes.
        return Err("Aucune instruction CREATE TABLE trouvée".into());
    } // Évite un écran vide ambigu.
    let mut uml = String::from("classDiagram\n"); // Export Mermaid consultable avec un outil local.
    let mut mld = String::new(); // Le MLD textuel conserve les noms exacts.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du modèle relationnel et de ses contraintes.
    for table in &tables {
        let name = table["name"].as_str().unwrap_or("table"); // Le parseur fournit un nom valide.
        // @ligne Prépare « safe_name » pour la prochaine étape du modèle relationnel et de ses contraintes.
        let safe_name: String = name
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
            .chars()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
            .map(|c| {
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du modèle relationnel et de ses contraintes.
                if c.is_alphanumeric() || c == '_' {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    c
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                } else {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    '_'
                // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
                }
            // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
            })
            .collect(); // Évite l'injection de syntaxe Mermaid.
        uml.push_str(&format!("class {safe_name} {{\n")); // Ouvre la classe correspondant à la table.
        let mut cols = Vec::new(); // Colonne et contraintes restent lisibles dans le MLD.
        // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du modèle relationnel et de ses contraintes.
        for c in table["columns"].as_array().unwrap() {
            let col = c["name"].as_str().unwrap_or(""); // Nom de colonne réel.
            // @ligne Prépare « safe_col » pour la prochaine étape du modèle relationnel et de ses contraintes.
            let safe_col: String = col
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                .chars()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                .filter(|c| c.is_alphanumeric() || *c == '_')
                .collect(); // Nettoie uniquement l'export graphique.
            // @ligne Prépare « sql_type » pour la prochaine étape du modèle relationnel et de ses contraintes.
            let sql_type = c["type"]
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                .as_str()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                .unwrap_or("String")
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                .split([' ', '('])
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                .next()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                .unwrap_or("String");
            // @ligne Prépare « uml_type » pour la prochaine étape du modèle relationnel et de ses contraintes.
            let uml_type = sql_type
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                .chars()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                .filter(|c| c.is_alphanumeric() || *c == '_')
                .collect::<String>(); // Conserve le type de base SQL sans injecter de syntaxe Mermaid.
            uml.push_str(&format!("  +{uml_type} {safe_col}\n")); // Le type SQL original reste affiché dans le canevas.
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
            cols.push(format!(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                "{} {} {}",
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                col,
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                c["type"].as_str().unwrap_or(""),
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                c["constraints"]
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    .as_array()
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    .unwrap()
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    .iter()
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    .map(|v| v.as_str().unwrap_or(""))
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    .collect::<Vec<_>>()
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    .join(" ")
            )); // MLD détaillé.
        // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
        }
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        uml.push_str("}\n");
        mld.push_str(&format!("{} ({})\n", name, cols.join(", "))); // Ferme les représentations d'une table.
    // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
    }
    let mut mcd = String::from("erDiagram\n"); // Le MCD assisté peut être rendu par Mermaid hors ligne.
    // @ligne Prépare « ident » pour la prochaine étape du modèle relationnel et de ses contraintes.
    let ident = |name: &str| {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        name.chars()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
            .map(|c| {
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du modèle relationnel et de ses contraintes.
                if c.is_alphanumeric() || c == '_' {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    c
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                } else {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
                    '_'
                // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
                }
            // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
            })
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
            .collect::<String>()
    }; // Protège la syntaxe des exports.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du modèle relationnel et de ses contraintes.
    for relation in &relations {
        // @ligne Prépare « from » pour la prochaine étape du modèle relationnel et de ses contraintes.
        let from = ident(relation["from"].as_str().unwrap_or(""));
        // @ligne Prépare « to » pour la prochaine étape du modèle relationnel et de ses contraintes.
        let to = ident(relation["to"].as_str().unwrap_or(""));
        // @ligne Prépare « child » pour la prochaine étape du modèle relationnel et de ses contraintes.
        let child = if relation["target_cardinality"] == "0,1" {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
            "o|"
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        } else {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
            "o{"
        // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
        };
        // @ligne Prépare « parent » pour la prochaine étape du modèle relationnel et de ses contraintes.
        let parent = if relation["source_cardinality"] == "1,1" {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
            "||"
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        } else {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
            "|o"
        // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
        };
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        mcd.push_str(&format!("  {to} {parent}--{child} {from} : reference\n"));
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        uml.push_str(&format!("{from} --> {to} : FK\n"));
    } // Les liens sont fondés sur les références SQL effectives.
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du modèle relationnel et de ses contraintes.
    Ok(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        json!({"tables":tables,"relations":relations,"mcd":mcd,"uml":uml,"mld":mld,"notice":"MCD assisté : les cardinalités métier doivent être validées ; SQL ne décrit pas toutes les règles métier."}),
    ) // Ne prétend pas déduire les règles absentes.
// @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
}
// Les modèles sont explicitement enregistrés par l'utilisateur.
pub fn save(name: &str, ddl: &str) -> Result<Value> {
    let model = parse(ddl)?; // Refuse de persister un modèle invalide.
    db::open()?.execute("INSERT INTO models VALUES(?1,?2,?3) ON CONFLICT(name) DO UPDATE SET ddl=excluded.ddl,updated_at=excluded.updated_at",rusqlite::params![name,ddl,db::now()])?; // Paramètres liés, même si le DDL contient du SQL.
    Ok(model) // Permet d'afficher le modèle sauvegardé sans deuxième analyse.
// @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
}
// L'introspection SQLite ne peut ni modifier la base choisie ni exécuter son contenu.
pub fn from_sqlite(path: &str) -> Result<Value> {
    // @ligne Prépare « conn » pour la prochaine étape du modèle relationnel et de ses contraintes.
    let conn = rusqlite::Connection::open_with_flags(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        path,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?; // Une base absente n'est jamais créée.
    conn.execute_batch("PRAGMA query_only=ON; PRAGMA trusted_schema=OFF;")?; // Ne fait pas confiance aux fonctions du schéma inspecté.
    let mut stmt=conn.prepare("SELECT sql FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND sql IS NOT NULL ORDER BY name")?; // Exclut les tables internes.
    // @ligne Prépare « ddl » pour la prochaine étape du modèle relationnel et de ses contraintes.
    let ddl = stmt
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du modèle relationnel et de ses contraintes.
        .query_map([], |r| r.get::<_, String>(0))?
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        .collect::<std::result::Result<Vec<_>, _>>()?
        .join(";\n"); // Agrège uniquement les définitions de tables.
    parse(&ddl) // Réutilise le même parseur sans exécuter le DDL récupéré.
// @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
}
#[cfg(test)] // Vérifie notamment les virgules internes et références SQL.
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
mod tests {
    use super::*; // Fonctions sans connexion réseau.
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du modèle relationnel et de ses contraintes.
    #[test]
    // @ligne Déclare « dba », le point d’entrée de cette opération du modèle relationnel et de ses contraintes.
    fn complex_ddl() {
        // @ligne Prépare « m » pour la prochaine étape du modèle relationnel et de ses contraintes.
        let m=parse("CREATE TABLE users(id INT PRIMARY KEY, amount DECIMAL(10,2), label VARCHAR(80)); CREATE TABLE posts(id INT, uid INT REFERENCES users(id));").unwrap();
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        assert_eq!(m["tables"].as_array().unwrap().len(), 2);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        assert_eq!(m["tables"][0]["columns"].as_array().unwrap().len(), 3);
    // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
    }
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du modèle relationnel et de ses contraintes.
    #[test]
    // @ligne Déclare « dba », le point d’entrée de cette opération du modèle relationnel et de ses contraintes.
    fn cardinalities_follow_constraints() {
        let model=parse("CREATE TABLE parents(id INT PRIMARY KEY); CREATE TABLE children(id INT PRIMARY KEY, parent_id INT NOT NULL UNIQUE REFERENCES parents(id)); CREATE TABLE notes(a INT, b INT, FOREIGN KEY(a,b) REFERENCES children(id,parent_id));").unwrap(); // Inclut une FK unique obligatoire et une FK composite nullable.
        assert_eq!(model["relations"][0]["source_cardinality"], "1,1"); // NOT NULL impose un parent.
        assert_eq!(model["relations"][0]["target_cardinality"], "0,1"); // UNIQUE interdit plusieurs enfants pour cette FK.
        assert_eq!(model["relations"][1]["source_cardinality"], "0,1"); // Une FK nullable ne force pas l'association.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        assert_eq!(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
            model["relations"][1]["columns"].as_array().unwrap().len(),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
            2
        ); // Ne perd pas les références composites.
    // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
    }
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du modèle relationnel et de ses contraintes.
    #[test]
    // @ligne Déclare « dba », le point d’entrée de cette opération du modèle relationnel et de ses contraintes.
    fn deferred_constraints_from_dumps() {
        // @ligne Prépare « model » pour la prochaine étape du modèle relationnel et de ses contraintes.
        let model=parse("CREATE TABLE parents(id INT); CREATE TABLE children(id INT,parent_id INT NOT NULL); ALTER TABLE parents ADD CONSTRAINT parents_pk PRIMARY KEY(id); ALTER TABLE children ADD CONSTRAINT parent_fk FOREIGN KEY(parent_id) REFERENCES parents(id);").unwrap();
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        assert_eq!(model["relations"].as_array().unwrap().len(), 1);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        assert_eq!(model["relations"][0]["source_cardinality"], "1,1");
    } // Les contraintes déclarées après CREATE TABLE restent prises en compte.
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant du modèle relationnel et de ses contraintes.
    #[test]
    // @ligne Déclare « dba », le point d’entrée de cette opération du modèle relationnel et de ses contraintes.
    fn invalid() {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        assert!(parse("not sql !!!").is_err());
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du modèle relationnel et de ses contraintes.
        assert!(parse("SELECT 1").is_err());
    // @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
    }
// @ligne Termine cette construction et revient au contexte parent du modèle relationnel et de ses contraintes.
}
