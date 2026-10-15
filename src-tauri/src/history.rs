//! Importation tolérante des quatre familles d'historiques sans modifier les profils.
use crate::{
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
    db::{self, Capture},
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
    Result,
}; // Toute insertion passe par le même filtre de confidentialité.
use serde_json::{json, Value}; // Le rapport présente les erreurs fichier par fichier.
use std::path::PathBuf; // Gère les chemins natifs sans concaténation manuelle.
// Décode aussi PSReadLine UTF-16LE et conserve les octets invalides sous forme de remplacement.
pub fn decode(bytes: &[u8]) -> String {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
    if bytes.starts_with(&[255, 254]) {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        String::from_utf16_lossy(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            &bytes[2..]
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                .as_chunks::<2>()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                .0
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                .iter()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                .map(|b| u16::from_le_bytes([b[0], b[1]]))
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                .collect::<Vec<_>>(),
        // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
        )
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
    } else {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        String::from_utf8_lossy(bytes)
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            .trim_start_matches('\u{feff}')
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            .to_owned()
    } // Un fichier corrompu ne bloque pas les autres.
// @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
}
// Les continuations sont regroupées avant la classification pour préserver la commande complète.
pub fn parse(text: &str, shell: &str) -> Vec<Capture> {
    let mut result = Vec::new(); // Accumule les occurrences, pas seulement les commandes uniques.
    let mut timestamp = None; // Bash et Fish fournissent parfois une date séparée.
    let mut pending = String::new(); // Tampon de commande multiligne.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la fonctionnalité locale.
    for line in text.lines() {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
        if line.contains('\u{fffd}') {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            continue;
        } // Ignore une ligne irrécupérable plutôt que de fabriquer une commande.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
        if shell == "fish" {
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
            if let Some(cmd) = line.strip_prefix("- cmd: ") {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                result.push(Capture {
                    // @ligne Renseigne le champ « raw_command » de la structure utilisée ici de la fonctionnalité locale.
                    raw_command: cmd.replace("\\n", "\n").replace("\\\\", "\\"),
                    // @ligne Renseigne le champ « shell » de la structure utilisée ici de la fonctionnalité locale.
                    shell: shell.into(),
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    ..Default::default()
                // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
                });
            } // Fish échappe les nouvelles lignes.
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
            if let Some(t) = line.trim().strip_prefix("when: ") {
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
                if let Some(c) = result.last_mut() {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    c.timestamp = t.parse().ok();
                // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
                }
            } // La date appartient à la commande précédente.
            continue; // Les métadonnées paths ne sont pas des commandes.
        // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
        }
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
        if shell == "bash" && line.starts_with('#') && line[1..].chars().all(|c| c.is_ascii_digit())
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            timestamp = line[1..].parse().ok();
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            continue;
        } // Timestamp HISTTIMEFORMAT.
        let mut cmd = line; // Par défaut la ligne entière est une commande.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
        if shell == "zsh" && line.starts_with(": ") {
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
            if let Some((meta, rest)) = line[2..].split_once(';') {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                timestamp = meta.split(':').next().and_then(|s| s.parse().ok());
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                cmd = rest;
            // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
            }
        } // Format EXTENDED_HISTORY.
        let continuation = if shell == "pwsh" { '`' } else { '\\' }; // PowerShell emploie le backtick.
        let continues = cmd.chars().rev().take_while(|c| *c == continuation).count() % 2 == 1; // Un échappement doublé reste littéral.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        pending.push_str(if continues {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            &cmd[..cmd.len() - 1]
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        } else {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            cmd
        }); // Retire uniquement le caractère de continuation.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
        if continues {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            pending.push('\n');
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            continue;
        } // Préserve la séparation logique des lignes.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
        if !pending.trim().is_empty() {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            result.push(Capture {
                // @ligne Renseigne le champ « raw_command » de la structure utilisée ici de la fonctionnalité locale.
                raw_command: std::mem::take(&mut pending),
                // @ligne Renseigne le champ « shell » de la structure utilisée ici de la fonctionnalité locale.
                shell: shell.into(),
                // @ligne Renseigne le champ « timestamp » de la structure utilisée ici de la fonctionnalité locale.
                timestamp: timestamp.take(),
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                ..Default::default()
            // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
            });
        } // Libère le tampon pour la suivante.
    // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
    }
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
    if !pending.is_empty() {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        result.push(Capture {
            // @ligne Renseigne le champ « raw_command » de la structure utilisée ici de la fonctionnalité locale.
            raw_command: pending,
            // @ligne Renseigne le champ « shell » de la structure utilisée ici de la fonctionnalité locale.
            shell: shell.into(),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            timestamp,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            ..Default::default()
        // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
        });
    } // Une dernière ligne incomplète reste récupérable.
    result // Les lignes vides ne sont jamais ajoutées.
// @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
}
// La liste ne nécessite aucun accès root ni exploration d'autres comptes.
pub fn sources() -> Vec<(PathBuf, &'static str)> {
    let home = dirs::home_dir().unwrap_or_default(); // Répertoire du seul utilisateur courant.
    // @ligne Prépare « list » pour la prochaine étape de la fonctionnalité locale.
    let mut list = vec![
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        (home.join(".bash_history"), "bash"),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        (home.join(".zsh_history"), "zsh"),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        (home.join(".zhistory"), "zsh"),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        (home.join(".local/share/fish/fish_history"), "fish"),
    ]; // Sources Unix usuelles.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
    if let Some(p) = std::env::var_os("HISTFILE") {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        list.push((
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            p.into(),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            std::env::var("SHELL")
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                .unwrap_or_default()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                .rsplit('/')
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                .next()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                .filter(|s| *s == "bash")
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                .map(|_| "bash")
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                .unwrap_or("zsh"),
        // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
        ));
    } // Respecte la personnalisation du shell.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
    if let Some(p) = std::env::var_os("APPDATA") {
        // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la fonctionnalité locale.
        for sub in [
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            "Microsoft/Windows/PowerShell/PSReadLine/ConsoleHost_history.txt",
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            "Microsoft/PowerShell/PSReadLine/ConsoleHost_history.txt",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        ] {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            list.push((PathBuf::from(&p).join(sub), "pwsh"));
        // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
        }
    } // Windows PowerShell et PowerShell moderne.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
    list.sort_by(|a, b| a.0.cmp(&b.0));
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
    list.dedup_by(|a, b| a.0 == b.0);
    list // Évite un double import via HISTFILE.
// @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
}
// Le digest et les insertions sont validés ensemble pour résister à un arrêt brutal.
pub fn import() -> Result<Value> {
    let mut conn = db::open()?; // L'import utilise une seule connexion.
    let mut reports = Vec::new(); // Retour détaillé même si certains chemins sont absents.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la fonctionnalité locale.
    for (path, shell) in sources() {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
        if !path.is_file() {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            continue;
        } // Un shell non installé n'est pas une erreur.
        // @ligne Prépare « outcome » pour la prochaine étape de la fonctionnalité locale.
        let outcome = (|| -> Result<Value> {
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
            if std::fs::metadata(&path)?.len() > 64 * 1024 * 1024 {
                // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la fonctionnalité locale.
                return Err("Historique supérieur à 64 Mio : import manuel requis".into());
            } // Borne la mémoire du processus.
            let bytes = std::fs::read(&path)?; // Lecture seule du fichier source.
            let digest = blake3::hash(&bytes).to_hex().to_string(); // Détecte un fichier strictement identique.
            let key = path.to_string_lossy().to_string(); // Identifiant de source portable.
            // @ligne Prépare « old » pour la prochaine étape de la fonctionnalité locale.
            let old: Option<String> = conn
                // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la fonctionnalité locale.
                .query_row("SELECT digest FROM imports WHERE path=?1", [&key], |r| {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    r.get(0)
                // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
                })
                .ok(); // Première lecture : aucune entrée.
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
            if old.as_deref() == Some(&digest) {
                // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la fonctionnalité locale.
                return Ok(json!({"path":key,"imported":0,"unchanged":true}));
            } // Réimporter ne gonfle pas les compteurs.
            let tx = conn.transaction()?; // Toutes les occurrences sont atomiques avec le curseur.
            let commands = parse(&decode(&bytes), shell); // Les quatre formats convergent vers Capture.
            let mut count = 0; // Compte uniquement les commandes acceptées.
            // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la fonctionnalité locale.
            for c in commands {
                // Un fichier modifié est fusionné sans recompter les commandes déjà présentes.
                let hash = blake3::hash(crate::classifier::redact(c.raw_command.trim()).as_bytes())
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    .to_hex()
                    .to_string(); // Même identité que db::insert.
                // @ligne Prépare « exists » pour la prochaine étape de la fonctionnalité locale.
                let exists: bool = tx.query_row(
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    "SELECT EXISTS(SELECT 1 FROM commands WHERE hash=?1)",
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    [hash],
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    |r| r.get(0),
                )?; // Préserve l'idempotence même après rotation du fichier.
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
                if !exists && db::insert(&tx, &c)? {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    count += 1;
                } // Les occurrences historiques initiales sont estimées à un minimum.
            // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
            }
            tx.execute("INSERT INTO imports VALUES(?1,?2) ON CONFLICT(path) DO UPDATE SET digest=excluded.digest",rusqlite::params![key,digest])?; // Curseur mis à jour uniquement après import réussi.
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            tx.commit()?;
            Ok(json!({"path":key,"imported":count,"unchanged":false})) // Rend le résultat durable.
        // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
        })();
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        reports.push(outcome.unwrap_or_else(|e| json!({"path":path,"error":e.to_string()})));
        // Une erreur n'interrompt pas les autres shells.
    }
    Ok(json!({"sources":reports})) // Le frontend peut expliquer chaque repli.
// @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
}
// Les émulateurs connus sont reconnus dans l'ascendance du hook sans privilège supplémentaire.
pub fn terminal() -> String {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
    if let Ok(value) = std::env::var("TERM_PROGRAM") {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la fonctionnalité locale.
        return value;
    } // Les terminaux qui s'identifient eux-mêmes restent prioritaires.
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant de la fonctionnalité locale.
    #[cfg(target_os = "linux")]
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
    {
        // @ligne Prépare « pid » pour la prochaine étape de la fonctionnalité locale.
        let mut pid = std::process::id();
        // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la fonctionnalité locale.
        for _ in 0..12 {
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
            if let Ok(name) = std::fs::read_to_string(format!("/proc/{pid}/comm")) {
                // @ligne Prépare « name » pour la prochaine étape de la fonctionnalité locale.
                let name = name.trim();
                // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
                if [
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    "kitty",
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    "alacritty",
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    "gnome-terminal",
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    "gnome-terminal-server",
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    "konsole",
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    "wezterm-gui",
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    "foot",
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    "xterm",
                // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
                ]
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                .contains(&name)
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                {
                    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la fonctionnalité locale.
                    return name.to_owned();
                // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
                }
            // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
            }
            // @ligne Prépare « Ok » pour la prochaine étape de la fonctionnalité locale.
            let Ok(status) = std::fs::read_to_string(format!("/proc/{pid}/status")) else {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                break;
            // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
            };
            // @ligne Prépare « Some » pour la prochaine étape de la fonctionnalité locale.
            let Some(parent) = status.lines().find_map(|line| {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                line.strip_prefix("PPid:")
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                    .and_then(|s| s.trim().parse::<u32>().ok())
            // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
            }) else {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                break;
            // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
            };
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la fonctionnalité locale.
            if parent == 0 || parent == pid {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
                break;
            // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
            }
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            pid = parent;
        // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
        }
    } // /proc peut être restreint : TERM reste le repli.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
    std::env::var("TERM").unwrap_or_default()
// @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
}
#[cfg(test)] // Cas représentatifs des formats réellement rencontrés.
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
mod tests {
    use super::*; // Fonctions d'analyse pures.
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant de la fonctionnalité locale.
    #[test]
    // @ligne Déclare « history », le point d’entrée de cette opération de la fonctionnalité locale.
    fn formats() {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        assert_eq!(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            parse(": 1728900000:0;sudo docker ps", "zsh")[0].timestamp,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            Some(1728900000)
        // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
        );
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        assert_eq!(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            parse("#1728900000\ngit status", "bash")[0].raw_command,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            "git status"
        // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
        );
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        assert_eq!(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            parse("- cmd: docker ps\n  when: 42", "fish")[0].timestamp,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
            Some(42)
        // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
        );
    // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
    }
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant de la fonctionnalité locale.
    #[test]
    // @ligne Déclare « history », le point d’entrée de cette opération de la fonctionnalité locale.
    fn multiline() {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        assert_eq!(parse("echo hello \\\nworld", "bash").len(), 1);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        assert_eq!(parse("echo hi `\nthere", "pwsh").len(), 1);
    // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
    }
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant de la fonctionnalité locale.
    #[test]
    // @ligne Déclare « history », le point d’entrée de cette opération de la fonctionnalité locale.
    fn utf16() {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la fonctionnalité locale.
        assert_eq!(decode(&[255, 254, 65, 0, 66, 0]), "AB");
    // @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
    }
// @ligne Termine cette construction et revient au contexte parent de la fonctionnalité locale.
}
