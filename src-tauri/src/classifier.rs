//! Analyse lexicale prudente : aucune commande de l'historique n'est exécutée.
use regex::Regex; // Les expressions régulières repèrent les formes courantes de secrets.
use serde::{Deserialize, Serialize}; // Les mêmes structures traversent l'IPC et la CLI.
use std::sync::LazyLock; // Les expressions sont compilées une seule fois par processus.

// Chaque règle conserve le nom du paramètre et élimine uniquement sa valeur.
static RULES: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    vec![
    (Regex::new(r#"(?i)((?:password|passwd|pwd|token|secret|api[_-]?key|access[_-]?key|authorization)[\w-]*\s*[=:]\s*)(?:"[^"]*"|'[^']*'|[^\s;&|]+)"#).unwrap(), "${1}[REDACTED]"),
    (Regex::new(r#"(?i)((?:--password|--token|--secret|--api-key)\s+)(?:"[^"]*"|'[^']*'|[^\s;&|]+)"#).unwrap(), "${1}[REDACTED]"),
    (Regex::new(r"(?i)(bearer\s+)[A-Za-z0-9._~+/=-]+").unwrap(), "${1}[REDACTED]"),
    (Regex::new(r"(://[^\s/:@]+:)[^\s@]+@").unwrap(), "${1}[REDACTED]@"),
    (Regex::new(r"\b(?:gh[pousr]_[A-Za-z0-9_]+|AKIA[A-Z0-9]{16}|sk-[A-Za-z0-9_-]{16,})\b").unwrap(), "[REDACTED]"),
]
});

// La protection précède le hachage, la classification, les logs et toute persistance.
pub fn redact(raw: &str) -> String {
    let mut result = raw.to_owned(); // Une copie évite de modifier le tampon d'origine.
    for (pattern, replacement) in RULES.iter() {
        result = pattern.replace_all(&result, *replacement).into_owned();
    } // Chaque famille de secret est traitée.
    if result.contains("mysql") || result.contains("mariadb") {
        // -p est un port pour ssh mais un mot de passe MySQL.
        result = Regex::new(r"(^|\s)-p[^\s-][^\s]*")
            .unwrap()
            .replace_all(&result, "${1}-p[REDACTED]")
            .into_owned(); // Ne masque pas --port.
    }
    result // Le masquage est volontairement irréversible.
}

#[derive(Debug, Serialize, Deserialize)] // L'analyse peut être inspectée sans connaître Rust.
pub struct Analysis {
    pub raw: String,      // Version persistable, secrets masqués.
    pub clean: String,    // Version débarrassée des enveloppes reconnues.
    pub binary: String,   // Binaire métier plutôt que sudo ou env.
    pub category: String, // Catégorie stable utilisée par les filtres.
    pub privileged: bool, // Seules les enveloppes d'élévation activent ce badge.
    pub dangerous: bool,  // Indication pédagogique ; aucune exécution intégrée.
}

// La taxonomie reste déterministe et fonctionne entièrement hors ligne.
pub fn analyze(raw: &str) -> Analysis {
    let safe = redact(raw); // Ne jamais analyser puis conserver la version contenant des secrets.
    let tokens =
        shlex::split(&safe).unwrap_or_else(|| safe.split_whitespace().map(str::to_owned).collect()); // Un historique incomplet reste consultable.
    let mut i = 0; // Le curseur parcourt les enveloppes sans lancer de shell.
    let mut privileged = false; // time, env et nohup ne sont pas des élévations.
    while i < tokens.len() {
        let word = tokens[i].as_str(); // Le token courant indique le traitement approprié.
        if word.contains('=') && !word.starts_with('-') {
            i += 1;
            continue;
        } // Affectations d'environnement initiales.
        if ["sudo", "doas", "pkexec", "env", "nohup", "time", "watch"].contains(&word) {
            privileged |= ["sudo", "doas", "pkexec"].contains(&word); // Distingue privilèges et simples enveloppes.
            i += 1; // Le token suivant peut être une option de l'enveloppe.
            while i < tokens.len() && tokens[i].starts_with('-') {
                let takes_value = match word {
                    "sudo" => [
                        "-u", "-g", "-h", "-p", "-C", "-R", "-T", "--user", "--group", "--host",
                        "--prompt",
                    ]
                    .contains(&tokens[i].as_str()),
                    "doas" => tokens[i] == "-u",
                    "env" => ["-u", "--unset"].contains(&tokens[i].as_str()),
                    "watch" => ["-n", "--interval"].contains(&tokens[i].as_str()),
                    "time" => ["-o", "-f", "--output", "--format"].contains(&tokens[i].as_str()),
                    _ => false,
                }; // Options usuelles avec argument.
                i += 1 + usize::from(takes_value && i + 1 < tokens.len()); // Saute aussi la valeur si nécessaire.
            }
        } else if word == "su" && tokens.get(i + 1).map(String::as_str) == Some("-c") {
            let mut inner = analyze(tokens.get(i + 2).map(String::as_str).unwrap_or("")); // La chaîne -c est analysée récursivement.
            inner.raw = safe;
            inner.privileged = true;
            return inner; // Garde la forme d'origine masquée.
        } else {
            break;
        } // Le premier token métier termine le retrait des enveloppes.
    }
    let binary = tokens
        .get(i)
        .map(|s| s.rsplit('/').next().unwrap_or(s))
        .unwrap_or("")
        .to_owned(); // Accepte /usr/bin/docker.
    let category = match binary.as_str() {
        "docker" | "docker-compose" | "podman" | "kubectl" | "helm" | "minikube" => "containers",
        "nmap" | "ip" | "ifconfig" | "ss" | "netstat" | "tcpdump" | "tshark" | "ufw"
        | "iptables" | "wg" | "dig" | "ping" | "ssh" | "curl" => "network",
        "pacman" | "yay" | "paru" | "apt" | "apt-get" | "dpkg" | "systemctl" | "journalctl"
        | "htop" | "ps" | "kill" | "lsblk" | "mount" | "winget" => "system",
        "ls" | "eza" | "ll" | "find" | "fd" | "grep" | "rg" | "tar" | "zip" | "unzip" | "chmod"
        | "chown" | "cp" | "mv" | "rm" | "rsync" | "scp" | "cd" => "files",
        "git" | "gh" | "glab" | "lazygit" => "git",
        "psql" | "pg_dump" | "mysql" | "mysqldump" | "mariadb" | "sqlite3" | "redis-cli"
        | "prisma" | "sqlcmd" => "database",
        "ollama" | "python" | "python3" | "pip" | "uv" | "poetry" | "conda" | "jupyter"
        | "torchrun" => "ai",
        "php" | "artisan" | "composer" | "npm" | "pnpm" | "bun" | "dotnet" | "flutter" | "dart"
        | "cargo" | "go" | "node" => "development",
        _ => "other", // Les binaires inconnus restent visibles et recherchables.
    }
    .to_owned();
    let clean = tokens[i..]
        .iter()
        .map(|t| shlex::try_quote(t).unwrap_or_default().into_owned())
        .collect::<Vec<_>>()
        .join(" "); // Reconstruit avec échappement des arguments.
    let dangerous = [
        "rm -",
        "dropdb",
        "DROP TABLE",
        "mkfs",
        "dd if=",
        "reset --hard",
    ]
    .iter()
    .any(|s| safe.contains(s)); // Avertissement sans interprétation de shell.
    Analysis {
        raw: safe,
        clean,
        binary,
        category,
        privileged,
        dangerous,
    } // Fournit toutes les métadonnées ensemble.
}

// Le filtrage parcourt les valeurs JSON sans toucher aux guillemets structurels.
pub fn redact_json(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::String(s) => serde_json::Value::String(redact(s)), // Applique les règles aux chaînes seulement.
        serde_json::Value::Array(values) => values.iter().map(redact_json).collect(), // Préserve l'ordre et les types.
        serde_json::Value::Object(values) => serde_json::Value::Object(
            values
                .iter()
                .map(|(key, value)| {
                    let sensitive = [
                        "password",
                        "passwd",
                        "token",
                        "secret",
                        "api_key",
                        "apikey",
                        "private_key",
                    ]
                    .iter()
                    .any(|term| key.to_lowercase().contains(term)); // Les noms explicites protègent aussi les valeurs non reconnaissables.
                    (
                        key.clone(),
                        if sensitive {
                            serde_json::json!("[REDACTED]")
                        } else {
                            redact_json(value)
                        },
                    ) // Les clés restent intactes.
                })
                .collect(),
        ),
        other => other.clone(), // Nombres, booléens et null n'ont pas de secret textuel.
    }
}

#[cfg(test)] // Ces tests vérifient les invariants de confidentialité et de classement.
mod tests {
    use super::*; // Accède aux fonctions publiques et aux règles internes.
    #[test]
    fn wrappers() {
        let a = analyze("sudo -u root env X=1 docker compose up -d");
        assert_eq!(a.binary, "docker");
        assert!(a.privileged);
        assert!(!analyze("time git status").privileged);
        assert_eq!(analyze("sudo -n docker ps").binary, "docker");
        assert_eq!(analyze("time -p git status").binary, "git");
    }
    #[test]
    fn secrets() {
        for s in [
            "TOKEN=supersecret curl x",
            "mysql -psupersecret",
            "psql postgres://me:supersecret@db/x",
            "curl --token 'supersecret'",
        ] {
            assert!(!redact(s).contains("supersecret"));
        }
    }
    #[test]
    fn quoted_arguments() {
        assert_eq!(analyze("sudo sh -c 'echo hello world'").binary, "sh");
        assert_eq!(analyze("su -c 'docker ps'").binary, "docker");
    }
}
