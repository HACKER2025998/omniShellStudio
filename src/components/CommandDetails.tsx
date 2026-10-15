// L'inspecteur charge l'AST à la demande pour ne pas alourdir chaque carte du coffre.
import { useEffect, useState } from "react";
// @ligne Importe les outils nécessaires à la mise en œuvre de l’inspection pédagogique des commandes.
import { api, type Command } from "../api";
// Les erreurs de syntaxe restent visibles sans interdire la consultation du texte original masqué.
export function CommandDetails({ command }: { command: Command }) {
  // @ligne Prépare les valeurs locales pour la prochaine étape de l’inspection pédagogique des commandes.
  const [details, setDetails] = useState<any>(null),
    [error, setError] = useState(""); // Les données d'une ancienne sélection ne doivent pas apparaître pour la suivante.
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes.
  useEffect(() => {
    // @ligne Prépare « active » pour la prochaine étape de l’inspection pédagogique des commandes.
    let active = true;
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes.
    setDetails(null);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes.
    setError("");
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes.
    api("command_inspect", { raw: command.raw, shell: command.shell })
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes.
      .then((data) => {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de l’inspection pédagogique des commandes.
        if (active) {
          // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de l’inspection pédagogique des commandes.
          if (data?.analysis && Array.isArray(data.arguments)) setDetails(data);
          // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de l’inspection pédagogique des commandes.
          else setError("Le moteur a renvoyé une analyse incomplète.");
        // @ligne Termine cette construction et revient au contexte parent de l’inspection pédagogique des commandes.
        }
      // @ligne Termine cette construction et revient au contexte parent de l’inspection pédagogique des commandes.
      })
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes.
      .catch((error) => {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de l’inspection pédagogique des commandes.
        if (active) setError(String(error));
      // @ligne Termine cette construction et revient au contexte parent de l’inspection pédagogique des commandes.
      });
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de l’inspection pédagogique des commandes.
    return () => {
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes.
      active = false;
    // @ligne Termine cette construction et revient au contexte parent de l’inspection pédagogique des commandes.
    };
  }, [command.id, command.raw]); // Une réponse tardive ne remplace pas l'inspecteur courant.
  // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de l’inspection pédagogique des commandes.
  return (
    // @ligne Construit ou ferme l’élément « section » de l’interface ; son contenu reste géré par React.
    <section>
      {/* @ligne Construit ou ferme l’élément « h3Décomposition » de l’interface ; son contenu reste géré par React. */}
      <h3>Décomposition et options</h3>
      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes. */}
      {error && <p className="help">{error}</p>}
      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes. */}
      {details && (
        // @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React.
        <>
          {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes. */}
          {details.analysis.dangerous && (
            // @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React.
            <p className="alert">
              Commande potentiellement destructrice : vérifiez les cibles avant
              toute utilisation.
            {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
            </p>
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes.
          )}
          {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
          <p className="help">
            Grammaire {details.grammar}
            {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes. */}
            {details.has_syntax_errors
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes.
              ? " · syntaxe partiellement reconnue"
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes.
              : ""}
          {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
          </p>
          {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes. */}
          {details.arguments.map((argument: any, index: number) => (
            // @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React.
            <div key={index} className="argument-explanation">
              {/* @ligne Construit ou ferme l’élément « code{argument.value}code » de l’interface ; son contenu reste géré par React. */}
              <code>{argument.value}</code>
              {/* @ligne Construit ou ferme l’élément « p{argument.explanation}p » de l’interface ; son contenu reste géré par React. */}
              <p>{argument.explanation}</p>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            </div>
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes.
          ))}
          {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
          <details>
            {/* @ligne Construit ou ferme l’élément « summaryArbre » de l’interface ; son contenu reste géré par React. */}
            <summary>Arbre syntaxique local</summary>
            {/* @ligne Construit ou ferme l’élément « pre{JSON.stringify(details.ast, » de l’interface ; son contenu reste géré par React. */}
            <pre>{JSON.stringify(details.ast, null, 2)}</pre>
          {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
          </details>
        {/* @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React. */}
        </>
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de l’inspection pédagogique des commandes.
      )}
    {/* @ligne Construit ou ferme l’élément « section » de l’interface ; son contenu reste géré par React. */}
    </section>
  // @ligne Termine cette construction et revient au contexte parent de l’inspection pédagogique des commandes.
  );
// @ligne Termine cette construction et revient au contexte parent de l’inspection pédagogique des commandes.
}
