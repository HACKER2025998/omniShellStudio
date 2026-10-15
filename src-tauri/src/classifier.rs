//! Analyse lexicale prudente : aucune commande de l'historique n'est exécutée.
use regex::Regex; // Les expressions régulières repèrent les formes courantes de secrets.
use serde::{Deserialize, Serialize}; // Les mêmes structures traversent l'IPC et la CLI.
use std::sync::LazyLock; // Les expressions sont compilées une seule fois par processus.
// Chaque règle conserve le nom du paramètre et élimine uniquement sa valeur.
static RULES: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
    vec![
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
    (Regex::new(r#"(?i)((?:password|passwd|pwd|token|secret|api[_-]?key|access[_-]?key|authorization)[\w-]*\s*[=:]\s*)(?:"[^"]*"|'[^']*'|[^\s;&|]+)"#).unwrap(), "${1}[REDACTED]"),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
    (Regex::new(r#"(?i)((?:--password|--token|--secret|--api-key)\s+)(?:"[^"]*"|'[^']*'|[^\s;&|]+)"#).unwrap(), "${1}[REDACTED]"),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
    (Regex::new(r"(?i)(bearer\s+)[A-Za-z0-9._~+/=-]+").unwrap(), "${1}[REDACTED]"),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
    (Regex::new(r"(://[^\s/:@]+:)[^\s@]+@").unwrap(), "${1}[REDACTED]@"),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
    (Regex::new(r"\b(?:gh[pousr]_[A-Za-z0-9_]+|AKIA[A-Z0-9]{16}|sk-[A-Za-z0-9_-]{16,})\b").unwrap(), "[REDACTED]"),
// @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
]
// @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
});
// La protection précède le hachage, la classification, les logs et toute persistance.
pub fn redact(raw: &str) -> String {
    let mut result = raw.to_owned(); // Une copie évite de modifier le tampon d'origine.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la confidentialité et du classement.
    for (pattern, replacement) in RULES.iter() {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        result = pattern.replace_all(&result, *replacement).into_owned();
    } // Chaque famille de secret est traitée.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la confidentialité et du classement.
    if result.contains("mysql") || result.contains("mariadb") {
        // -p est un port pour ssh mais un mot de passe MySQL.
        result = Regex::new(r"(^|\s)-p[^\s-][^\s]*")
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
            .unwrap()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
            .replace_all(&result, "${1}-p[REDACTED]")
            .into_owned(); // Ne masque pas --port.
    // @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
    }
    result // Le masquage est volontairement irréversible.
// @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
}
#[derive(Debug, Serialize, Deserialize)] // L'analyse peut être inspectée sans connaître Rust.
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
pub struct Analysis {
    pub raw: String,      // Version persistable, secrets masqués.
    pub clean: String,    // Version débarrassée des enveloppes reconnues.
    pub binary: String,   // Binaire métier plutôt que sudo ou env.
    pub category: String, // Catégorie stable utilisée par les filtres.
    pub privileged: bool, // Seules les enveloppes d'élévation activent ce badge.
    pub dangerous: bool,  // Indication pédagogique ; aucune exécution intégrée.
// @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
}
// La taxonomie reste déterministe et fonctionne entièrement hors ligne.
pub fn analyze(raw: &str) -> Analysis {
    let safe = redact(raw); // Ne jamais analyser puis conserver la version contenant des secrets.
    // @ligne Prépare « tokens » pour la prochaine étape de la confidentialité et du classement.
    let tokens =
        shlex::split(&safe).unwrap_or_else(|| safe.split_whitespace().map(str::to_owned).collect()); // Un historique incomplet reste consultable.
    let mut i = 0; // Le curseur parcourt les enveloppes sans lancer de shell.
    let mut privileged = false; // time, env et nohup ne sont pas des élévations.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la confidentialité et du classement.
    while i < tokens.len() {
        let word = tokens[i].as_str(); // Le token courant indique le traitement approprié.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la confidentialité et du classement.
        if word.contains('=') && !word.starts_with('-') {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
            i += 1;
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
            continue;
        } // Affectations d'environnement initiales.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la confidentialité et du classement.
        if ["sudo", "doas", "pkexec", "env", "nohup", "time", "watch"].contains(&word) {
            privileged |= ["sudo", "doas", "pkexec"].contains(&word); // Distingue privilèges et simples enveloppes.
            i += 1; // Le token suivant peut être une option de l'enveloppe.
            // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la confidentialité et du classement.
            while i < tokens.len() && tokens[i].starts_with('-') {
                // @ligne Prépare « takes_value » pour la prochaine étape de la confidentialité et du classement.
                let takes_value = match word {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                    "sudo" => [
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                        "-u", "-g", "-h", "-p", "-C", "-R", "-T", "--user", "--group", "--host",
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                        "--prompt",
                    // @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
                    ]
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                    .contains(&tokens[i].as_str()),
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                    "doas" => tokens[i] == "-u",
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                    "env" => ["-u", "--unset"].contains(&tokens[i].as_str()),
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                    "watch" => ["-n", "--interval"].contains(&tokens[i].as_str()),
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                    "time" => ["-o", "-f", "--output", "--format"].contains(&tokens[i].as_str()),
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                    _ => false,
                }; // Options usuelles avec argument.
                i += 1 + usize::from(takes_value && i + 1 < tokens.len()); // Saute aussi la valeur si nécessaire.
            // @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
            }
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        } else if word == "su" && tokens.get(i + 1).map(String::as_str) == Some("-c") {
            let mut inner = analyze(tokens.get(i + 2).map(String::as_str).unwrap_or("")); // La chaîne -c est analysée récursivement.
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
            inner.raw = safe;
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
            inner.privileged = true;
            return inner; // Garde la forme d'origine masquée.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        } else {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
            break;
        } // Le premier token métier termine le retrait des enveloppes.
    // @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
    }
    // @ligne Prépare « binary » pour la prochaine étape de la confidentialité et du classement.
    let binary = tokens
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        .get(i)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        .map(|s| s.rsplit('/').next().unwrap_or(s))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        .unwrap_or("")
        .to_owned(); // Accepte /usr/bin/docker.
    // @ligne Prépare « category » pour la prochaine étape de la confidentialité et du classement.
    let category = match binary.as_str() {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        "docker" | "docker-compose" | "podman" | "kubectl" | "helm" | "minikube" => "containers",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        "nmap" | "ip" | "ifconfig" | "ss" | "netstat" | "tcpdump" | "tshark" | "ufw"
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        | "iptables" | "wg" | "dig" | "ping" | "curl" => "network",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        "pacman" | "yay" | "paru" | "apt" | "apt-get" | "dpkg" | "systemctl" | "journalctl"
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        | "htop" | "ps" | "kill" | "lsblk" | "mount" | "winget" => "system",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        "ls" | "eza" | "ll" | "find" | "fd" | "grep" | "rg" | "tar" | "zip" | "unzip" | "chmod"
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        | "chown" | "cp" | "mv" | "rm" | "rsync" | "scp" | "cd" => "files",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        "git" | "gh" | "glab" | "lazygit" => "git",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        "psql" | "pg_dump" | "mysql" | "mysqldump" | "mariadb" | "sqlite3" | "redis-cli"
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        | "prisma" | "sqlcmd" => "database",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        "ollama" | "python" | "python3" | "pip" | "uv" | "poetry" | "conda" | "jupyter"
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        | "torchrun" => "ai",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        "php" | "artisan" | "composer" | "npm" | "pnpm" | "bun" | "dotnet" | "flutter" | "dart"
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        | "cargo" | "go" | "node" => "development",
        "ssh" | "sftp" | "mosh" | "telnet" | "smbclient" => "remote", // Outils de connexion à une machine distante.
        "zfs" | "btrfs" | "restic" | "borg" | "rclone" | "cryptsetup" => "storage", // Stockage et sauvegardes structurées.
        "vmstat" | "iostat" | "sar" | "free" | "uptime" | "dstat" => "observability", // Mesures locales du système.
        _ => "other", // Les binaires inconnus restent visibles et recherchables.
    // @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
    }
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
    .to_owned();
    // @ligne Prépare « clean » pour la prochaine étape de la confidentialité et du classement.
    let clean = tokens[i..]
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        .iter()
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        .map(|t| shlex::try_quote(t).unwrap_or_default().into_owned())
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        .collect::<Vec<_>>()
        .join(" "); // Reconstruit avec échappement des arguments.
    // @ligne Prépare « dangerous » pour la prochaine étape de la confidentialité et du classement.
    let dangerous = [
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        "rm -",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        "dropdb",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        "DROP TABLE",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        "mkfs",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        "dd if=",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        "reset --hard",
    // @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
    ]
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
    .iter()
    .any(|s| safe.contains(s)); // Avertissement sans interprétation de shell.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
    Analysis {
        // @ligne Renseigne le champ « raw » de la structure utilisée ici de la confidentialité et du classement.
        raw: safe,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        clean,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        binary,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        category,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        privileged,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        dangerous,
    } // Fournit toutes les métadonnées ensemble.
// @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
}
// Le filtrage parcourt les valeurs JSON sans toucher aux guillemets structurels.
pub fn redact_json(value: &serde_json::Value) -> serde_json::Value {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la confidentialité et du classement.
    match value {
        serde_json::Value::String(s) => serde_json::Value::String(redact(s)), // Applique les règles aux chaînes seulement.
        serde_json::Value::Array(values) => values.iter().map(redact_json).collect(), // Préserve l'ordre et les types.
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        serde_json::Value::Object(values) => serde_json::Value::Object(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
            values
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                .iter()
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                .map(|(key, value)| {
                    // @ligne Prépare « sensitive » pour la prochaine étape de la confidentialité et du classement.
                    let sensitive = [
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                        "password",
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                        "passwd",
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                        "token",
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                        "secret",
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                        "api_key",
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                        "apikey",
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                        "private_key",
                    // @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
                    ]
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                    .iter()
                    .any(|term| key.to_lowercase().contains(term)); // Les noms explicites protègent aussi les valeurs non reconnaissables.
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                    (
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                        key.clone(),
                        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la confidentialité et du classement.
                        if sensitive {
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                            serde_json::json!("[REDACTED]")
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                        } else {
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                            redact_json(value)
                        // @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
                        },
                    ) // Les clés restent intactes.
                // @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
                })
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
                .collect(),
        // @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
        ),
        other => other.clone(), // Nombres, booléens et null n'ont pas de secret textuel.
    // @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
    }
// @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
}
#[cfg(test)] // Ces tests vérifient les invariants de confidentialité et de classement.
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
mod tests {
    use super::*; // Accède aux fonctions publiques et aux règles internes.
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant de la confidentialité et du classement.
    #[test]
    // @ligne Déclare « classifier », le point d’entrée de cette opération de la confidentialité et du classement.
    fn wrappers() {
        // @ligne Prépare « a » pour la prochaine étape de la confidentialité et du classement.
        let a = analyze("sudo -u root env X=1 docker compose up -d");
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        assert_eq!(a.binary, "docker");
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        assert!(a.privileged);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        assert!(!analyze("time git status").privileged);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        assert_eq!(analyze("sudo -n docker ps").binary, "docker");
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        assert_eq!(analyze("time -p git status").binary, "git");
    // @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
    }
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant de la confidentialité et du classement.
    #[test]
    // @ligne Déclare « classifier », le point d’entrée de cette opération de la confidentialité et du classement.
    fn secrets() {
        // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la confidentialité et du classement.
        for s in [
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
            "TOKEN=supersecret curl x",
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
            "mysql -psupersecret",
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
            "psql postgres://me:supersecret@db/x",
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
            "curl --token 'supersecret'",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        ] {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
            assert!(!redact(s).contains("supersecret"));
        // @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
        }
    // @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
    }
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant de la confidentialité et du classement.
    #[test]
    // @ligne Déclare « classifier », le point d’entrée de cette opération de la confidentialité et du classement.
    fn quoted_arguments() {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        assert_eq!(analyze("sudo sh -c 'echo hello world'").binary, "sh");
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la confidentialité et du classement.
        assert_eq!(analyze("su -c 'docker ps'").binary, "docker");
    // @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
    }
// @ligne Termine cette construction et revient au contexte parent de la confidentialité et du classement.
}
