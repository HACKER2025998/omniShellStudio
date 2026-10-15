//! Le parseur SQL comprend les contraintes structurées sans exécuter le DDL.
use crate::{db, Result}; // Sauvegarde facultative des modèles dans SQLite.
use serde_json::{json, Value}; // Format neutre pour canevas et export.
use sqlparser::{
    ast::{ColumnOption, Statement, TableConstraint},
    dialect::GenericDialect,
    parser::Parser,
}; // Analyse SQL réelle plutôt qu'une découpe à la virgule.

// Le modèle relationnel est dérivé des contraintes présentes ; le MCD reste une interprétation.
pub fn parse(ddl: &str) -> Result<Value> {
    if ddl.len() > 1_048_576 {
        return Err("Script SQL limité à 1 Mio".into());
    } // Borne le coût d'analyse.
    let statements = Parser::parse_sql(&GenericDialect {}, ddl)?; // N'exécute aucune instruction SQL fournie.
    let mut tables = Vec::new();
    let mut relations = Vec::new(); // Chaque FK fournit une association et des cardinalités déductibles du DDL. // Les instructions hors CREATE TABLE sont ignorées.
    for stmt in statements {
        if let Statement::CreateTable(table) = stmt {
            let table_name = table.name.to_string(); // Les contraintes seront rattachées à leur table source.
            let mut keys: Vec<Vec<String>> = Vec::new(); // Les clés uniques limitent la cardinalité inverse.
            for c in &table.columns {
                if c.options
                    .iter()
                    .any(|o| matches!(o.option, ColumnOption::Unique { .. }))
                {
                    keys.push(vec![c.name.value.clone()]);
                }
            } // Clés déclarées sur une colonne.
            for c in &table.constraints {
                if let TableConstraint::PrimaryKey { columns, .. }
                | TableConstraint::Unique { columns, .. } = c
                {
                    keys.push(columns.iter().map(|c| c.value.clone()).collect());
                }
            } // Clés composites.
            let add_relation = |source: Vec<String>,
                                target: String,
                                referred: Vec<String>,
                                relations: &mut Vec<Value>| {
                let required=source.iter().all(|name|table.columns.iter().find(|c|c.name.value==*name).map(|c|c.options.iter().any(|o|matches!(o.option,ColumnOption::NotNull|ColumnOption::Unique{is_primary:true,..}))||table.constraints.iter().any(|k|matches!(k,TableConstraint::PrimaryKey{columns,..} if columns.iter().any(|c|c.value==*name)))).unwrap_or(false)); // Une FK nullable rend l'association facultative.
                let unique = keys
                    .iter()
                    .any(|key| key.len() == source.len() && key.iter().all(|c| source.contains(c))); // Seule une clé couvrant exactement la FK limite le nombre d'enfants.
                relations.push(json!({"from":table_name,"to":target,"columns":source,"referenced":referred,"source_cardinality":if required {"1,1"} else {"0,1"},"target_cardinality":if unique {"0,1"} else {"0,n"}}));
                // Le minimum côté parent n'est jamais inventé.
            };
            for c in &table.columns {
                for option in &c.options {
                    if let ColumnOption::ForeignKey {
                        foreign_table,
                        referred_columns,
                        ..
                    } = &option.option
                    {
                        add_relation(
                            vec![c.name.value.clone()],
                            foreign_table.to_string(),
                            referred_columns.iter().map(|c| c.value.clone()).collect(),
                            &mut relations,
                        );
                    }
                }
            } // FK déclarées directement sur les colonnes.
            for c in &table.constraints {
                if let TableConstraint::ForeignKey {
                    columns,
                    foreign_table,
                    referred_columns,
                    ..
                } = c
                {
                    add_relation(
                        columns.iter().map(|c| c.value.clone()).collect(),
                        foreign_table.to_string(),
                        referred_columns.iter().map(|c| c.value.clone()).collect(),
                        &mut relations,
                    );
                }
            } // FK simples et composites nommées.
            let columns = table.columns.iter().map(|c|json!({"name":c.name.value,"type":c.data_type.to_string(),"constraints":c.options.iter().map(|o|o.option.to_string()).collect::<Vec<_>>() })).collect::<Vec<_>>(); // Préserve types, PK, FK et nullabilité.
            tables.push(json!({"name":table.name.to_string(),"columns":columns,"constraints":table.constraints.iter().map(ToString::to_string).collect::<Vec<_>>() }));
            // Contraintes composites conservées.
        }
    }
    if tables.is_empty() {
        return Err("Aucune instruction CREATE TABLE trouvée".into());
    } // Évite un écran vide ambigu.
    let mut uml = String::from("classDiagram\n"); // Export Mermaid consultable avec un outil local.
    let mut mld = String::new(); // Le MLD textuel conserve les noms exacts.
    for table in &tables {
        let name = table["name"].as_str().unwrap_or("table"); // Le parseur fournit un nom valide.
        let safe_name: String = name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect(); // Évite l'injection de syntaxe Mermaid.
        uml.push_str(&format!("class {safe_name} {{\n")); // Ouvre la classe correspondant à la table.
        let mut cols = Vec::new(); // Colonne et contraintes restent lisibles dans le MLD.
        for c in table["columns"].as_array().unwrap() {
            let col = c["name"].as_str().unwrap_or(""); // Nom de colonne réel.
            let safe_col: String = col
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '_')
                .collect(); // Nettoie uniquement l'export graphique.
            uml.push_str(&format!("  +String {safe_col}\n")); // Le type SQL original reste affiché dans le canevas.
            cols.push(format!(
                "{} {} {}",
                col,
                c["type"].as_str().unwrap_or(""),
                c["constraints"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap_or(""))
                    .collect::<Vec<_>>()
                    .join(" ")
            )); // MLD détaillé.
        }
        uml.push_str("}\n");
        mld.push_str(&format!("{} ({})\n", name, cols.join(", "))); // Ferme les représentations d'une table.
    }
    let mut mcd = String::from("erDiagram\n"); // Le MCD assisté peut être rendu par Mermaid hors ligne.
    let ident = |name: &str| {
        name.chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect::<String>()
    }; // Protège la syntaxe des exports.
    for relation in &relations {
        let from = ident(relation["from"].as_str().unwrap_or(""));
        let to = ident(relation["to"].as_str().unwrap_or(""));
        let child = if relation["target_cardinality"] == "0,1" {
            "o|"
        } else {
            "o{"
        };
        let parent = if relation["source_cardinality"] == "1,1" {
            "||"
        } else {
            "|o"
        };
        mcd.push_str(&format!("  {to} {parent}--{child} {from} : reference\n"));
        uml.push_str(&format!("{from} --> {to} : FK\n"));
    } // Les liens sont fondés sur les références SQL effectives.
    Ok(
        json!({"tables":tables,"relations":relations,"mcd":mcd,"uml":uml,"mld":mld,"notice":"MCD assisté : les cardinalités métier doivent être validées ; SQL ne décrit pas toutes les règles métier."}),
    ) // Ne prétend pas déduire les règles absentes.
}

// Les modèles sont explicitement enregistrés par l'utilisateur.
pub fn save(name: &str, ddl: &str) -> Result<Value> {
    let model = parse(ddl)?; // Refuse de persister un modèle invalide.
    db::open()?.execute("INSERT INTO models VALUES(?1,?2,?3) ON CONFLICT(name) DO UPDATE SET ddl=excluded.ddl,updated_at=excluded.updated_at",rusqlite::params![name,ddl,db::now()])?; // Paramètres liés, même si le DDL contient du SQL.
    Ok(model) // Permet d'afficher le modèle sauvegardé sans deuxième analyse.
}

#[cfg(test)] // Vérifie notamment les virgules internes et références SQL.
mod tests {
    use super::*; // Fonctions sans connexion réseau.
    #[test]
    fn complex_ddl() {
        let m=parse("CREATE TABLE users(id INT PRIMARY KEY, amount DECIMAL(10,2), label VARCHAR(80)); CREATE TABLE posts(id INT, uid INT REFERENCES users(id));").unwrap();
        assert_eq!(m["tables"].as_array().unwrap().len(), 2);
        assert_eq!(m["tables"][0]["columns"].as_array().unwrap().len(), 3);
    }
    #[test]
    fn cardinalities_follow_constraints() {
        let model=parse("CREATE TABLE parents(id INT PRIMARY KEY); CREATE TABLE children(id INT PRIMARY KEY, parent_id INT NOT NULL UNIQUE REFERENCES parents(id)); CREATE TABLE notes(a INT, b INT, FOREIGN KEY(a,b) REFERENCES children(id,parent_id));").unwrap(); // Inclut une FK unique obligatoire et une FK composite nullable.
        assert_eq!(model["relations"][0]["source_cardinality"], "1,1"); // NOT NULL impose un parent.
        assert_eq!(model["relations"][0]["target_cardinality"], "0,1"); // UNIQUE interdit plusieurs enfants pour cette FK.
        assert_eq!(model["relations"][1]["source_cardinality"], "0,1"); // Une FK nullable ne force pas l'association.
        assert_eq!(
            model["relations"][1]["columns"].as_array().unwrap().len(),
            2
        ); // Ne perd pas les références composites.
    }
    #[test]
    fn invalid() {
        assert!(parse("not sql !!!").is_err());
        assert!(parse("SELECT 1").is_err());
    }
}

// L'introspection SQLite ne peut ni modifier la base choisie ni exécuter son contenu.
pub fn from_sqlite(path: &str) -> Result<Value> {
    let conn = rusqlite::Connection::open_with_flags(
        path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?; // Une base absente n'est jamais créée.
    conn.execute_batch("PRAGMA query_only=ON; PRAGMA trusted_schema=OFF;")?; // Ne fait pas confiance aux fonctions du schéma inspecté.
    let mut stmt=conn.prepare("SELECT sql FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND sql IS NOT NULL ORDER BY name")?; // Exclut les tables internes.
    let ddl = stmt
        .query_map([], |r| r.get::<_, String>(0))?
        .collect::<std::result::Result<Vec<_>, _>>()?
        .join(";\n"); // Agrège uniquement les définitions de tables.
    parse(&ddl) // Réutilise le même parseur sans exécuter le DDL récupéré.
}
