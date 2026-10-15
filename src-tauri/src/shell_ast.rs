//! Arbres syntaxiques hors ligne des shells, analysés après masquage et jamais évalués.
use crate::{classifier, Result}; // La confidentialité précède tout retour d'inspection.
use serde_json::{json, Value}; // Le même arbre peut être affiché et exporté par le bureau.
// Les arbres sont bornés pour que les entrées incomplètes restent inspectables sans recursion excessive.
fn node_value(
    // @ligne Renseigne le champ « node » de la structure utilisée ici de l’analyse syntaxique sans exécution.
    node: tree_sitter::Node<'_>,
    // @ligne Renseigne le champ « source » de la structure utilisée ici de l’analyse syntaxique sans exécution.
    source: &str,
    // @ligne Renseigne le champ « depth » de la structure utilisée ici de l’analyse syntaxique sans exécution.
    depth: usize,
    // @ligne Renseigne le champ « budget » de la structure utilisée ici de l’analyse syntaxique sans exécution.
    budget: &mut usize,
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
) -> Value {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de l’analyse syntaxique sans exécution.
    if depth > 24 || *budget == 0 {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de l’analyse syntaxique sans exécution.
        return json!({"kind":node.kind(),"truncated":true});
    } // L'UI peut expliquer une coupe au lieu de geler.
    *budget -= 1;
    // @ligne Prépare « cursor » pour la prochaine étape de l’analyse syntaxique sans exécution.
    let mut cursor = node.walk();
    // @ligne Prépare « children » pour la prochaine étape de l’analyse syntaxique sans exécution.
    let children = node
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        .named_children(&mut cursor)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        .map(|child| node_value(child, source, depth + 1, budget))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        .collect::<Vec<_>>();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
    json!({"kind":node.kind(),"start":node.start_byte(),"end":node.end_byte(),"text":if children.is_empty(){node.utf8_text(source.as_bytes()).unwrap_or("")}else{""},"error":node.is_error()||node.is_missing(),"children":children})
    // Seules les feuilles contiennent un texte déjà filtré.
}
// Chaque grammaire couvre son langage ; Zsh utilise la grammaire Bash pour son sous-ensemble compatible.
pub fn inspect(raw: &str, shell: &str) -> Result<Value> {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de l’analyse syntaxique sans exécution.
    if raw.len() > 131_072 {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de l’analyse syntaxique sans exécution.
        return Err("Commande limitée à 128 Kio".into());
    // @ligne Termine cette construction et revient au contexte parent de l’analyse syntaxique sans exécution.
    }
    // @ligne Prépare « safe » pour la prochaine étape de l’analyse syntaxique sans exécution.
    let safe = classifier::redact(raw);
    let mut parser = tree_sitter::Parser::new(); // Aucun nœud ne reçoit la version brute sensible.
    // @ligne Prépare « language » pour la prochaine étape de l’analyse syntaxique sans exécution.
    let language = match shell {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        "fish" => tree_sitter_fish::language(),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        "pwsh" | "powershell" => tree_sitter_powershell::LANGUAGE.into(),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        _ => tree_sitter_bash::LANGUAGE.into(),
    // @ligne Termine cette construction et revient au contexte parent de l’analyse syntaxique sans exécution.
    };
    parser.set_language(&language)?; // Une grammaire incompatible produit une erreur explicite.
    // @ligne Prépare « tree » pour la prochaine étape de l’analyse syntaxique sans exécution.
    let tree = parser
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        .parse(&safe, None)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        .ok_or("Analyse syntaxique interrompue")?;
    let analysis = classifier::analyze(&safe); // La normalisation et l'AST restent consultables ensemble.
    let tokens = shlex::split(&safe).unwrap_or_default(); // Les explications d'arguments complètent la structure du langage.
    // @ligne Prépare « arguments » pour la prochaine étape de l’analyse syntaxique sans exécution.
    let arguments=tokens.iter().map(|token|json!({"value":token,"kind":if token.starts_with('-'){"option"}else{"argument"},"explanation":explain(&analysis.binary,token)})).collect::<Vec<_>>();
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de l’analyse syntaxique sans exécution.
    Ok(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        json!({"analysis":analysis,"ast":node_value(tree.root_node(),&safe,0,&mut 512),"has_syntax_errors":tree.root_node().has_error(),"grammar":match shell{"fish"=>"Fish","pwsh"|"powershell"=>"PowerShell",_=>"Bash/POSIX (sous-ensemble Zsh compatible)"},"arguments":arguments}),
    ) // Une syntaxe partiellement reconnue reste visible, sans prétendre l'exécuter.
// @ligne Termine cette construction et revient au contexte parent de l’analyse syntaxique sans exécution.
}
// Ce dictionnaire local décrit les options courantes ; il n'invente pas le sens des options inconnues.
fn explain(binary: &str, token: &str) -> &'static str {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de l’analyse syntaxique sans exécution.
    match (binary,token){
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        (_,"sudo"|"doas"|"pkexec")=>"Enveloppe de changement de privilèges.",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        ("docker"|"podman","-d"|"--detach")=>"Exécuter le conteneur en arrière-plan.",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        ("docker"|"podman","-p"|"--publish")=>"Publier un port du conteneur sur l'hôte.",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        ("docker"|"podman","-v"|"--volume")=>"Monter un volume ou un chemin dans le conteneur.",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        ("docker"|"podman","--name")=>"Attribuer un nom explicite au conteneur.",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        ("docker"|"podman","--rm")=>"Supprimer le conteneur lorsqu'il se termine.",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        ("nmap","-sS")=>"Scan SYN ; nécessite généralement des privilèges raw.",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        ("nmap","-sV")=>"Interroger les services pour estimer leurs versions.",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        ("nmap","-p-")=>"Examiner l'ensemble des ports TCP.",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        ("git","--hard")=>"Réinitialisation pouvant supprimer des modifications locales.",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        (_,"--help"|"-h")=>"Option dépendant du binaire ; consulter son aide locale avant usage.",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        _ if token.starts_with('-')=>"Option propre au programme ; sa signification exacte n'est pas dans le dictionnaire embarqué.",
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        _=>"Argument littéral ou mot-clé ; aucune expansion shell n'est exécutée par OmniShell.",
    // @ligne Termine cette construction et revient au contexte parent de l’analyse syntaxique sans exécution.
    }
// @ligne Termine cette construction et revient au contexte parent de l’analyse syntaxique sans exécution.
}
#[cfg(test)] // Les quatre chemins d'analyse sont testés sans lancer de shell.
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
mod tests {
    // @ligne Importe les outils nécessaires à la mise en œuvre de l’analyse syntaxique sans exécution.
    use super::*;
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant de l’analyse syntaxique sans exécution.
    #[test]
    // @ligne Déclare « shell_ast », le point d’entrée de cette opération de l’analyse syntaxique sans exécution.
    fn parses_supported_shells() {
        // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de l’analyse syntaxique sans exécution.
        for (shell, command) in [
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
            ("bash", "sudo docker ps | head -n 3"),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
            ("fish", "for x in a b; echo $x; end"),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
            ("pwsh", "Get-Process | Select-Object -First 3"),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        ] {
            // @ligne Prépare « data » pour la prochaine étape de l’analyse syntaxique sans exécution.
            let data = inspect(command, shell).unwrap();
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
            assert!(data["ast"].is_object());
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
            assert!(!data.to_string().contains("supersecret"));
        // @ligne Termine cette construction et revient au contexte parent de l’analyse syntaxique sans exécution.
        }
    // @ligne Termine cette construction et revient au contexte parent de l’analyse syntaxique sans exécution.
    }
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant de l’analyse syntaxique sans exécution.
    #[test]
    // @ligne Déclare « shell_ast », le point d’entrée de cette opération de l’analyse syntaxique sans exécution.
    fn masks_before_tree() {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
        assert!(!inspect("TOKEN=supersecret docker ps", "bash")
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
            .unwrap()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
            .to_string()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’analyse syntaxique sans exécution.
            .contains("supersecret"));
    // @ligne Termine cette construction et revient au contexte parent de l’analyse syntaxique sans exécution.
    }
// @ligne Termine cette construction et revient au contexte parent de l’analyse syntaxique sans exécution.
}
