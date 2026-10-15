// Sélection native, reprise de salle et découverte locale complètent le panneau maître.
import { useEffect, useState } from "react";
// @ligne Importe les outils nécessaires à la mise en œuvre de la sélection native et des salles locales.
import { open } from "@tauri-apps/plugin-dialog";
// @ligne Importe les outils nécessaires à la mise en œuvre de la sélection native et des salles locales.
import { openUrl } from "@tauri-apps/plugin-opener";
// @ligne Importe les outils nécessaires à la mise en œuvre de la sélection native et des salles locales.
import { api } from "../api";
// Les chemins choisis restent locaux ; les participants ne voient que les noms publiés.
export function ShareTools({
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
  onPaths,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
  onRoom,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
  onError,
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
}: {
  // @ligne Renseigne le champ « onPaths » de la structure utilisée ici de la sélection native et des salles locales.
  onPaths: (paths: string[]) => void;
  // @ligne Renseigne le champ « onRoom » de la structure utilisée ici de la sélection native et des salles locales.
  onRoom: (room: any) => void;
  // @ligne Renseigne le champ « onError » de la structure utilisée ici de la sélection native et des salles locales.
  onError: (message: string) => void;
// @ligne Termine cette construction et revient au contexte parent de la sélection native et des salles locales.
}) {
  // @ligne Prépare les valeurs locales pour la prochaine étape de la sélection native et des salles locales.
  const [saved, setSaved] = useState<any[]>([]),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
    [discovered, setDiscovered] = useState<any[]>([]),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
    [url, setUrl] = useState(""),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
    [pin, setPin] = useState(""),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
    [busy, setBusy] = useState(false);
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
  useEffect(() => {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
    api<any[]>("share_saved")
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
      .then((data) => setSaved(Array.isArray(data) ? data : []))
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
      .catch(() => {});
  }, []); // Les restaurations ne démarrent aucun port sans choix de l'utilisateur.
  // @ligne Prépare « choose » pour la prochaine étape de la sélection native et des salles locales.
  const choose = async (directory: boolean) => {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
    try {
      // @ligne Prépare « paths » pour la prochaine étape de la sélection native et des salles locales.
      const paths = await open({ directory, multiple: true });
      // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la sélection native et des salles locales.
      if (paths) onPaths(typeof paths === "string" ? [paths] : paths);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
    } catch (error) {
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
      onError(String(error));
    // @ligne Termine cette construction et revient au contexte parent de la sélection native et des salles locales.
    }
  }; // Le dialogue applique les permissions natives de la fenêtre.
  // @ligne Prépare « join » pour la prochaine étape de la sélection native et des salles locales.
  const join = async () => {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
    try {
      // @ligne Prépare « target » pour la prochaine étape de la sélection native et des salles locales.
      const target = new URL(url);
      // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la sélection native et des salles locales.
      if (target.protocol !== "https:")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
        throw new Error("Le lien de salle doit utiliser HTTPS.");
      // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la sélection native et des salles locales.
      if (pin && !/^\d{6}$/.test(pin))
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
        throw new Error("Le code doit contenir six chiffres.");
      // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la sélection native et des salles locales.
      if (pin) target.hash = "pin=" + pin;
      // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la sélection native et des salles locales.
      await openUrl(target.href);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
    } catch (error) {
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
      onError(String(error));
    // @ligne Termine cette construction et revient au contexte parent de la sélection native et des salles locales.
    }
  }; // Le PIN facultatif est un fragment navigateur, jamais un paramètre envoyé au serveur ou aux journaux HTTP.
  // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la sélection native et des salles locales.
  return (
    // @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React.
    <>
      {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
      <div className="actions">
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        <button onClick={() => choose(false)}>Choisir des fichiers</button>
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        <button onClick={() => choose(true)}>Choisir des dossiers</button>
      {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
      </div>
      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales. */}
      {saved.length > 0 && (
        // @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React.
        <details className="panel">
          {/* @ligne Construit ou ferme l’élément « summaryRestaurer » de l’interface ; son contenu reste géré par React. */}
          <summary>Restaurer une salle interrompue</summary>
          {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales. */}
          {saved.map((room) => (
            // @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React.
            <p key={room.id}>
              {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales. */}
              {room.ip}:{room.port} · {room.files} source(s){" "}
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              <button
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
                disabled={busy}
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
                onClick={async () => {
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
                  setBusy(true);
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
                  try {
                    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la sélection native et des salles locales.
                    onRoom(await api("share_resume", { id: room.id }));
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
                  } catch (error) {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
                    onError(String(error));
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
                  } finally {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
                    setBusy(false);
                  // @ligne Termine cette construction et revient au contexte parent de la sélection native et des salles locales.
                  }
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
                }}
              // @ligne Termine cette construction et revient au contexte parent de la sélection native et des salles locales.
              >
                Restaurer
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              </button>
            {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
            </p>
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
          ))}
          {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
          <p className="help">
            Le même port, les jetons et les droits sont conservés jusqu'à
            expiration de la salle.
          {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
          </p>
        {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
        </details>
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
      )}
      {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
      <details className="panel">
        {/* @ligne Construit ou ferme l’élément « summaryRejoindre » de l’interface ; son contenu reste géré par React. */}
        <summary>Rejoindre une salle du réseau</summary>
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        <button
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
          disabled={busy}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
          onClick={async () => {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
            setBusy(true);
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
            try {
              // @ligne Prépare « rooms » pour la prochaine étape de la sélection native et des salles locales.
              const rooms = await api<any[]>("share_discover");
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
              setDiscovered(Array.isArray(rooms) ? rooms : []);
              // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la sélection native et des salles locales.
              if (rooms.length === 1) setUrl(rooms[0].url);
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
            } catch (error) {
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
              onError(String(error));
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
            } finally {
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
              setBusy(false);
            // @ligne Termine cette construction et revient au contexte parent de la sélection native et des salles locales.
            }
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
          }}
        // @ligne Termine cette construction et revient au contexte parent de la sélection native et des salles locales.
        >
          {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales. */}
          {busy ? "Recherche locale…" : "Découvrir les salles mDNS"}
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        </button>
        {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales. */}
        {discovered.map((room) => (
          // @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React.
          <p key={room.url}>
            {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
            <button onClick={() => setUrl(room.url)}>
              {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales. */}
              {room.name} · {room.url}
            {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
            </button>
          {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
          </p>
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
        ))}
        {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
        <label>
          Lien HTTPS de la salle
          {/* @ligne Construit ou ferme l’élément « input » de l’interface ; son contenu reste géré par React. */}
          <input
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
            value={url}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
            onChange={(e) => setUrl(e.target.value)}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
            placeholder="https://192.168.1.42:49152/"
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
          />
        {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
        </label>
        {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
        <label>
          Code à six chiffres, facultatif
          {/* @ligne Construit ou ferme l’élément « input » de l’interface ; son contenu reste géré par React. */}
          <input
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
            inputMode="numeric"
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
            pattern="[0-9]{6}"
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
            maxLength={6}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
            value={pin}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
            onChange={(e) => setPin(e.target.value)}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
            autoComplete="off"
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la sélection native et des salles locales.
          />
        {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
        </label>
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        <button disabled={!url || busy} onClick={join}>
          Rejoindre dans mon navigateur
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        </button>
        {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
        <p className="help">
          Validez le certificat de l'hôte. Si multicast est bloqué, utilisez son
          lien ou son QR code.
        {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
        </p>
      {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
      </details>
    {/* @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React. */}
    </>
  // @ligne Termine cette construction et revient au contexte parent de la sélection native et des salles locales.
  );
// @ligne Termine cette construction et revient au contexte parent de la sélection native et des salles locales.
}
