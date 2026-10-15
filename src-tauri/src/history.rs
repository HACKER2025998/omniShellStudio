//! Importation tolérante des quatre familles d'historiques sans modifier les profils.
use crate::{
    db::{self, Capture},
    Result,
}; // Toute insertion passe par le même filtre de confidentialité.
use serde_json::{json, Value}; // Le rapport présente les erreurs fichier par fichier.
use std::path::PathBuf; // Gère les chemins natifs sans concaténation manuelle.

// Décode aussi PSReadLine UTF-16LE et conserve les octets invalides sous forme de remplacement.
pub fn decode(bytes: &[u8]) -> String {
    if bytes.starts_with(&[255, 254]) {
        String::from_utf16_lossy(
            &bytes[2..]
                .as_chunks::<2>()
                .0
                .iter()
                .map(|b| u16::from_le_bytes([b[0], b[1]]))
                .collect::<Vec<_>>(),
        )
    } else {
        String::from_utf8_lossy(bytes)
            .trim_start_matches('\u{feff}')
            .to_owned()
    } // Un fichier corrompu ne bloque pas les autres.
}

// Les continuations sont regroupées avant la classification pour préserver la commande complète.
pub fn parse(text: &str, shell: &str) -> Vec<Capture> {
    let mut result = Vec::new(); // Accumule les occurrences, pas seulement les commandes uniques.
    let mut timestamp = None; // Bash et Fish fournissent parfois une date séparée.
    let mut pending = String::new(); // Tampon de commande multiligne.
    for line in text.lines() {
        if line.contains('\u{fffd}') {
            continue;
        } // Ignore une ligne irrécupérable plutôt que de fabriquer une commande.
        if shell == "fish" {
            if let Some(cmd) = line.strip_prefix("- cmd: ") {
                result.push(Capture {
                    raw_command: cmd.replace("\\n", "\n").replace("\\\\", "\\"),
                    shell: shell.into(),
                    ..Default::default()
                });
            } // Fish échappe les nouvelles lignes.
            if let Some(t) = line.trim().strip_prefix("when: ") {
                if let Some(c) = result.last_mut() {
                    c.timestamp = t.parse().ok();
                }
            } // La date appartient à la commande précédente.
            continue; // Les métadonnées paths ne sont pas des commandes.
        }
        if shell == "bash" && line.starts_with('#') && line[1..].chars().all(|c| c.is_ascii_digit())
        {
            timestamp = line[1..].parse().ok();
            continue;
        } // Timestamp HISTTIMEFORMAT.
        let mut cmd = line; // Par défaut la ligne entière est une commande.
        if shell == "zsh" && line.starts_with(": ") {
            if let Some((meta, rest)) = line[2..].split_once(';') {
                timestamp = meta.split(':').next().and_then(|s| s.parse().ok());
                cmd = rest;
            }
        } // Format EXTENDED_HISTORY.
        let continuation = if shell == "pwsh" { '`' } else { '\\' }; // PowerShell emploie le backtick.
        let continues = cmd.chars().rev().take_while(|c| *c == continuation).count() % 2 == 1; // Un échappement doublé reste littéral.
        pending.push_str(if continues {
            &cmd[..cmd.len() - 1]
        } else {
            cmd
        }); // Retire uniquement le caractère de continuation.
        if continues {
            pending.push('\n');
            continue;
        } // Préserve la séparation logique des lignes.
        if !pending.trim().is_empty() {
            result.push(Capture {
                raw_command: std::mem::take(&mut pending),
                shell: shell.into(),
                timestamp: timestamp.take(),
                ..Default::default()
            });
        } // Libère le tampon pour la suivante.
    }
    if !pending.is_empty() {
        result.push(Capture {
            raw_command: pending,
            shell: shell.into(),
            timestamp,
            ..Default::default()
        });
    } // Une dernière ligne incomplète reste récupérable.
    result // Les lignes vides ne sont jamais ajoutées.
}

// La liste ne nécessite aucun accès root ni exploration d'autres comptes.
pub fn sources() -> Vec<(PathBuf, &'static str)> {
    let home = dirs::home_dir().unwrap_or_default(); // Répertoire du seul utilisateur courant.
    let mut list = vec![
        (home.join(".bash_history"), "bash"),
        (home.join(".zsh_history"), "zsh"),
        (home.join(".zhistory"), "zsh"),
        (home.join(".local/share/fish/fish_history"), "fish"),
    ]; // Sources Unix usuelles.
    if let Some(p) = std::env::var_os("HISTFILE") {
        list.push((
            p.into(),
            std::env::var("SHELL")
                .unwrap_or_default()
                .rsplit('/')
                .next()
                .filter(|s| *s == "bash")
                .map(|_| "bash")
                .unwrap_or("zsh"),
        ));
    } // Respecte la personnalisation du shell.
    if let Some(p) = std::env::var_os("APPDATA") {
        for sub in [
            "Microsoft/Windows/PowerShell/PSReadLine/ConsoleHost_history.txt",
            "Microsoft/PowerShell/PSReadLine/ConsoleHost_history.txt",
        ] {
            list.push((PathBuf::from(&p).join(sub), "pwsh"));
        }
    } // Windows PowerShell et PowerShell moderne.
    list.sort_by(|a, b| a.0.cmp(&b.0));
    list.dedup_by(|a, b| a.0 == b.0);
    list // Évite un double import via HISTFILE.
}

// Le digest et les insertions sont validés ensemble pour résister à un arrêt brutal.
pub fn import() -> Result<Value> {
    let mut conn = db::open()?; // L'import utilise une seule connexion.
    let mut reports = Vec::new(); // Retour détaillé même si certains chemins sont absents.
    for (path, shell) in sources() {
        if !path.is_file() {
            continue;
        } // Un shell non installé n'est pas une erreur.
        let outcome = (|| -> Result<Value> {
            if std::fs::metadata(&path)?.len() > 64 * 1024 * 1024 {
                return Err("Historique supérieur à 64 Mio : import manuel requis".into());
            } // Borne la mémoire du processus.
            let bytes = std::fs::read(&path)?; // Lecture seule du fichier source.
            let digest = blake3::hash(&bytes).to_hex().to_string(); // Détecte un fichier strictement identique.
            let key = path.to_string_lossy().to_string(); // Identifiant de source portable.
            let old: Option<String> = conn
                .query_row("SELECT digest FROM imports WHERE path=?1", [&key], |r| {
                    r.get(0)
                })
                .ok(); // Première lecture : aucune entrée.
            if old.as_deref() == Some(&digest) {
                return Ok(json!({"path":key,"imported":0,"unchanged":true}));
            } // Réimporter ne gonfle pas les compteurs.
            let tx = conn.transaction()?; // Toutes les occurrences sont atomiques avec le curseur.
            let commands = parse(&decode(&bytes), shell); // Les quatre formats convergent vers Capture.
            let mut count = 0; // Compte uniquement les commandes acceptées.
            for c in commands {
                // Un fichier modifié est fusionné sans recompter les commandes déjà présentes.
                let hash = blake3::hash(crate::classifier::redact(c.raw_command.trim()).as_bytes())
                    .to_hex()
                    .to_string(); // Même identité que db::insert.
                let exists: bool = tx.query_row(
                    "SELECT EXISTS(SELECT 1 FROM commands WHERE hash=?1)",
                    [hash],
                    |r| r.get(0),
                )?; // Préserve l'idempotence même après rotation du fichier.
                if !exists && db::insert(&tx, &c)? {
                    count += 1;
                } // Les occurrences historiques initiales sont estimées à un minimum.
            }
            tx.execute("INSERT INTO imports VALUES(?1,?2) ON CONFLICT(path) DO UPDATE SET digest=excluded.digest",rusqlite::params![key,digest])?; // Curseur mis à jour uniquement après import réussi.
            tx.commit()?;
            Ok(json!({"path":key,"imported":count,"unchanged":false})) // Rend le résultat durable.
        })();
        reports.push(outcome.unwrap_or_else(|e| json!({"path":path,"error":e.to_string()})));
        // Une erreur n'interrompt pas les autres shells.
    }
    Ok(json!({"sources":reports})) // Le frontend peut expliquer chaque repli.
}

#[cfg(test)] // Cas représentatifs des formats réellement rencontrés.
mod tests {
    use super::*; // Fonctions d'analyse pures.
    #[test]
    fn formats() {
        assert_eq!(
            parse(": 1728900000:0;sudo docker ps", "zsh")[0].timestamp,
            Some(1728900000)
        );
        assert_eq!(
            parse("#1728900000\ngit status", "bash")[0].raw_command,
            "git status"
        );
        assert_eq!(
            parse("- cmd: docker ps\n  when: 42", "fish")[0].timestamp,
            Some(42)
        );
    }
    #[test]
    fn multiline() {
        assert_eq!(parse("echo hello \\\nworld", "bash").len(), 1);
        assert_eq!(parse("echo hi `\nthere", "pwsh").len(), 1);
    }
    #[test]
    fn utf16() {
        assert_eq!(decode(&[255, 254, 65, 0, 66, 0]), "AB");
    }
}
