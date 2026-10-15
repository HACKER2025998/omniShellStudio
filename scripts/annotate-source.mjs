// Ce script documente les lignes propres au projet et vérifie que l'annotation TypeScript ne modifie pas le JavaScript produit.
import fs from "node:fs";
import path from "node:path";
import ts from "typescript";
// Les fichiers générés, dépendances, images et formats JSON sans commentaires sont exclus volontairement.
const targets = [
  "src",
  "src-tauri/src",
  "src-tauri/examples",
  "src-tauri/tests",
];
// Le contexte métier donne un sens aux commentaires des lignes structurelles.
const purposes = {
  db: "du stockage SQLite et de ses requêtes paramétrées",
  share: "du partage HTTPS et des permissions serveur",
  daemon: "de la capture locale et du service utilisateur",
  docker: "des captures et événements Docker",
  netmap: "de la cartographie locale avec preuves",
  network_protocols: "des protocoles réseau bornés",
  dba: "du modèle relationnel et de ses contraintes",
  database_connections: "de l’introspection SQL en lecture de schéma",
  classifier: "de la confidentialité et du classement",
  shell_ast: "de l’analyse syntaxique sans exécution",
  archives: "de la préparation sûre des dossiers",
  main: "du bureau et des parcours utilisateur",
  api: "du contrat IPC local",
  GraphCanvas: "du canevas déplaçable et des exports",
  NetworkWorkspace: "des observations réseau et du choix des sondes",
  ShareTools: "de la sélection native et des salles locales",
  DatabaseConnection: "des connexions éphémères aux catalogues SQL",
  CommandDetails: "de l’inspection pédagogique des commandes",
  "share-client": "de la session navigateur et des transferts repris",
  workflows: "du scénario de recette HTTPS et de stockage",
  benchmark: "de la mesure sur cent mille commandes synthétiques",
  lib: "de la liste fermée des actions métier",
};
// Une phrase décrit le rôle syntaxique de la ligne en conservant le nom de l'opération concernée.
function explain(line, purpose) {
  const text = line.trim();
  const context = purposes[purpose] || "de la fonctionnalité locale";
  const identifier = text.match(
    /(?:let(?:\s+mut)?|const|pub\s+(?:async\s+)?fn|async\s+function|function|struct|enum|class)\s+([\w]+)/,
  )?.[1];
  if (/^(?:import|use)\b/.test(text))
    return `Importe les outils nécessaires à la mise en œuvre ${context}.`;
  if (/^(?:pub\s+)?(?:async\s+)?(?:fn|function)\b/.test(text))
    return `Déclare « ${identifier || purpose} », le point d’entrée de cette opération ${context}.`;
  if (/^(?:let|const)\b/.test(text))
    return `Prépare ${identifier ? `« ${identifier} »` : "les valeurs locales"} pour la prochaine étape ${context}.`;
  if (/^if\b|^else\b|^match\b|^switch\b/.test(text))
    return `Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : ${context}.`;
  if (/^for\b|^while\b|^loop\b/.test(text))
    return `Parcourt les éléments ou événements indiqués ici pour traiter chaque cas ${context}.`;
  if (/^return\b|^Ok\(|^Err\(/.test(text))
    return `Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel ${context}.`;
  if (/^\}\s*(?:[);,]|$)|^[\])>;]+[,;]?$/.test(text))
    return `Termine cette construction et revient au contexte parent ${context}.`;
  if (/^#\[/.test(text))
    return `Applique cette règle de compilation ou de sérialisation à l’élément suivant ${context}.`;
  if (/await|\.execute\(|\.query|\.send\(/.test(text))
    return `Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours ${context}.`;
  if (/^(?:pub\s+)?[\w]+\s*:(?!:)/.test(text))
    return `Renseigne le champ « ${text.match(/(?:pub\s+)?([\w]+)\s*:(?!:)/)?.[1]} » de la structure utilisée ici ${context}.`;
  if (text.startsWith("<"))
    return `Construit ou ferme l’élément « ${text.replace(/[<>/]/g, "").split(/[\s>]/)[0]} » de l’interface ; son contenu reste géré par React.`;
  return `Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre ${context}.`;
}
// Le contexte JSX distingue les commentaires enfants des commentaires permis dans les attributs ou expressions.
function contextAt(node, pos, state = "code") {
  if (node.kind === ts.SyntaxKind.JsxText && pos >= node.pos && pos < node.end)
    return "text";
  if (ts.isJsxExpression(node) && pos > node.getStart() && pos < node.end)
    state = "code";
  if (
    (ts.isJsxOpeningElement(node) || ts.isJsxSelfClosingElement(node)) &&
    pos > node.getStart() &&
    pos < node.end
  )
    state = "code";
  if (
    ts.isJsxElement(node) &&
    pos >= node.openingElement.end &&
    pos <= node.closingElement.getStart()
  )
    state = "jsx";
  if (
    ts.isJsxFragment(node) &&
    pos >= node.openingFragment.end &&
    pos <= node.closingFragment.getStart()
  )
    state = "jsx";
  for (const child of node.getChildren()) {
    if (pos >= child.getStart() && pos < child.end)
      return contextAt(child, pos, state);
  }
  return state;
}
// La sortie du compilateur est la preuve que ces commentaires restent sans effet sur le comportement.
function emitted(source, name) {
  const output = ts.transpileModule(source, {
    fileName: name,
    compilerOptions: {
      jsx: ts.JsxEmit.ReactJSX,
      target: ts.ScriptTarget.ES2022,
      module: ts.ModuleKind.ESNext,
      removeComments: true,
    },
  }).outputText;
  const tree = ts.createSourceFile(
    "output.js",
    output,
    ts.ScriptTarget.Latest,
    true,
    ts.ScriptKind.JS,
  );
  const serialise = (node) => {
    const children = [];
    node.forEachChild((child) => {
      children.push(serialise(child));
    });
    return [
      node.kind,
      typeof node.text === "string" ? node.text : null,
      children,
    ];
  };
  return JSON.stringify(serialise(tree));
}
// Les annotations existantes sont supprimées avant régénération pour rendre l'outil idempotent.
function annotate(file) {
  const isTs = /\.tsx?$/.test(file);
  const original = fs.readFileSync(file, "utf8");
  let source = original
    .replace(/\{\/\* @ligne [^\n]*?\*\/\}/g, "")
    .replace(/[ \t]*\/\/ @ligne [^\n]*/g, "");
  source = source.replace(/(^[ \t]*#\[[^\n]*\n)(?:[ \t]*\n)+/gm, "$1");
  const compact = source.replace(/^[ \t]*\n/gm, "");
  if (!isTs || emitted(source, file) === emitted(compact, file))
    source = compact;
  const ast = isTs
    ? ts.createSourceFile(
        file,
        source,
        ts.ScriptTarget.Latest,
        true,
        file.endsWith("tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS,
      )
    : null;
  const lines = source.split("\n"),
    output = [];
  let offset = 0;
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i],
      trim = line.trim(),
      indent = line.match(/^\s*/)[0];
    const skip =
      !trim ||
      /^(?:\/\/|\/\*|\*|\{\/\*)/.test(trim) ||
      /\s\/\//.test(line) ||
      /^(?:\/\/|\/\*|\{\/\*)/.test((lines[i - 1] || "").trim());
    const context = isTs ? contextAt(ast, offset + line.indexOf(trim)) : "code";
    if (!skip && context !== "text") {
      const explanation = explain(
        line,
        path.basename(file).replace(/\.(rs|tsx?)$/, ""),
      );
      output.push(
        indent +
          (context === "jsx"
            ? `{/* @ligne ${explanation} */}`
            : `// @ligne ${explanation}`),
      );
    }
    output.push(line);
    offset += line.length + 1;
  }
  let result = output.join("\n");
  if (isTs && emitted(source, file) !== emitted(result, file)) {
    throw new Error(`L’annotation modifierait le rendu compilé : ${file}`);
  }
  result = result
    .split("\n")
    .map((line) => line.trimEnd())
    .join("\n");
  if (original !== result) fs.writeFileSync(file, result);
  console.log(file);
}
// Le parcours se limite aux sources du produit et à leurs scénarios de vérification.
function walk(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const file = path.join(directory, entry.name);
    if (entry.isDirectory()) walk(file);
    else if (/\.(?:rs|ts|tsx)$/.test(file)) annotate(file);
  }
}
// Aucune commande métier ni migration n'est exécutée par cet outil documentaire.
for (const directory of targets) walk(directory);
