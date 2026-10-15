// Les identifiants d'une connexion restent dans l'état transitoire et ne sont jamais sauvegardés avec le modèle.
import { useState } from "react";
// @ligne Importe les outils nécessaires à la mise en œuvre des connexions éphémères aux catalogues SQL.
import { open } from "@tauri-apps/plugin-dialog";
// @ligne Importe les outils nécessaires à la mise en œuvre des connexions éphémères aux catalogues SQL.
import { api } from "../api";
// La connexion ne sert qu'à lire les catalogues, jamais à exécuter une requête métier.
export function DatabaseConnection({
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
  onModel,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
  onError,
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
}: {
  // @ligne Renseigne le champ « onModel » de la structure utilisée ici des connexions éphémères aux catalogues SQL.
  onModel: (model: any) => void;
  // @ligne Renseigne le champ « onError » de la structure utilisée ici des connexions éphémères aux catalogues SQL.
  onError: (message: string) => void;
// @ligne Termine cette construction et revient au contexte parent des connexions éphémères aux catalogues SQL.
}) {
  // @ligne Prépare les valeurs locales pour la prochaine étape des connexions éphémères aux catalogues SQL.
  const [engine, setEngine] = useState("sqlite"),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
    [url, setUrl] = useState(""),
    [busy, setBusy] = useState(false); // Le mot de passe est masqué à l'écran.
  // @ligne Prépare « inspect » pour la prochaine étape des connexions éphémères aux catalogues SQL.
  const inspect = async () => {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
    setBusy(true);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
    try {
      // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des connexions éphémères aux catalogues SQL.
      if (engine === "sqlite") {
        // @ligne Prépare « path » pour la prochaine étape des connexions éphémères aux catalogues SQL.
        const path = await open({
          // @ligne Renseigne le champ « multiple » de la structure utilisée ici des connexions éphémères aux catalogues SQL.
          multiple: false,
          // @ligne Renseigne le champ « filters » de la structure utilisée ici des connexions éphémères aux catalogues SQL.
          filters: [
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
            { name: "SQLite", extensions: ["sqlite", "sqlite3", "db"] },
          // @ligne Termine cette construction et revient au contexte parent des connexions éphémères aux catalogues SQL.
          ],
        // @ligne Termine cette construction et revient au contexte parent des connexions éphémères aux catalogues SQL.
        });
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des connexions éphémères aux catalogues SQL.
        if (typeof path === "string")
          // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours des connexions éphémères aux catalogues SQL.
          onModel(await api("sql_sqlite", { path }));
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
      } else {
        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours des connexions éphémères aux catalogues SQL.
        onModel(await api("sql_database", { engine, url }));
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
        setUrl("");
      // @ligne Termine cette construction et revient au contexte parent des connexions éphémères aux catalogues SQL.
      }
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
    } catch (error) {
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
      onError(String(error));
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
    } finally {
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
      setBusy(false);
    // @ligne Termine cette construction et revient au contexte parent des connexions éphémères aux catalogues SQL.
    }
  }; // Les erreurs sensibles du pilote sont filtrées côté Rust.
  // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des connexions éphémères aux catalogues SQL.
  return (
    // @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React.
    <details className="panel">
      {/* @ligne Construit ou ferme l’élément « summaryLire » de l’interface ; son contenu reste géré par React. */}
      <summary>Lire le schéma d'une base existante</summary>
      {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
      <label>
        Moteur
        {/* @ligne Construit ou ferme l’élément « select » de l’interface ; son contenu reste géré par React. */}
        <select value={engine} onChange={(e) => setEngine(e.target.value)}>
          {/* @ligne Construit ou ferme l’élément « option » de l’interface ; son contenu reste géré par React. */}
          <option value="sqlite">SQLite local</option>
          {/* @ligne Construit ou ferme l’élément « option » de l’interface ; son contenu reste géré par React. */}
          <option value="postgres">PostgreSQL</option>
          {/* @ligne Construit ou ferme l’élément « option » de l’interface ; son contenu reste géré par React. */}
          <option value="mysql">MySQL / MariaDB</option>
        {/* @ligne Construit ou ferme l’élément « select » de l’interface ; son contenu reste géré par React. */}
        </select>
      {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
      </label>
      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL. */}
      {engine !== "sqlite" && (
        // @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React.
        <label>
          URL de connexion
          {/* @ligne Construit ou ferme l’élément « input » de l’interface ; son contenu reste géré par React. */}
          <input
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
            type="password"
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
            autoComplete="off"
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
            value={url}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
            onChange={(e) => setUrl(e.target.value)}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
            placeholder={
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
              engine === "postgres"
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
                ? "postgresql://utilisateur:secret@localhost/base"
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
                : "mysql://utilisateur:secret@localhost/base"
            // @ligne Termine cette construction et revient au contexte parent des connexions éphémères aux catalogues SQL.
            }
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
          />
        {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
        </label>
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
      )}
      {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
      <button
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
        disabled={busy || (engine !== "sqlite" && !url)}
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
        onClick={inspect}
      // @ligne Termine cette construction et revient au contexte parent des connexions éphémères aux catalogues SQL.
      >
        {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL. */}
        {busy
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
          ? "Lecture du schéma…"
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
          : engine === "sqlite"
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
            ? "Choisir la base SQLite"
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des connexions éphémères aux catalogues SQL.
            : "Lire les tables et contraintes"}
      {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
      </button>
      {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
      <p className="help">
        Utilisez un compte limité à la lecture du schéma. L'URL n'est ni
        enregistrée ni exportée.
      {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
      </p>
    {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
    </details>
  // @ligne Termine cette construction et revient au contexte parent des connexions éphémères aux catalogues SQL.
  );
// @ligne Termine cette construction et revient au contexte parent des connexions éphémères aux catalogues SQL.
}
