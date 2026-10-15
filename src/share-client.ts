// BLAKE3 est embarqué localement ; le navigateur ne contacte aucun CDN.
import { blake3 } from "@noble/hashes/blake3";
// Chaque origine de salle conserve son jeton et ses transferts indépendamment.
let token = localStorage.getItem("omnishell-peer") || "";
// Les sélecteurs sont typés pour repérer les erreurs d'interface à la compilation.
const element = <T extends HTMLElement>(selector: string) =>
  // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la session navigateur et des transferts repris.
  document.querySelector<T>(selector)!;
// Les erreurs serveur sont affichées plutôt que transformées en faux succès.
async function request(path: string, options: RequestInit = {}) {
  // @ligne Prépare « response » pour la prochaine étape de la session navigateur et des transferts repris.
  const response = await fetch(path, {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
    ...options,
    // @ligne Renseigne le champ « headers » de la structure utilisée ici de la session navigateur et des transferts repris.
    headers: { Authorization: "Bearer " + token, ...options.headers },
  // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
  });
  // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la session navigateur et des transferts repris.
  if (!response.ok) throw new Error(await response.text());
  // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la session navigateur et des transferts repris.
  return response;
// @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
}
// Une région de statut accessible explique la progression et les permissions actuelles.
function status(message: string) {
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
  element("#status").textContent = message;
// @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
}
// Les téléchargements natifs utilisent un cookie HttpOnly pour gérer les fichiers volumineux et Range.
async function refresh() {
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
  try {
    // @ligne Prépare « data » pour la prochaine étape de la session navigateur et des transferts repris.
    const data = await (await request("/api/files")).json();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
    element("#join").hidden = true;
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
    element("#refresh").hidden = false;
    // @ligne Prépare « list » pour la prochaine étape de la session navigateur et des transferts repris.
    const list = element("#files");
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
    list.replaceChildren();
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la session navigateur et des transferts repris.
    for (const file of data.files) {
      // @ligne Prépare « row » pour la prochaine étape de la session navigateur et des transferts repris.
      const row = document.createElement("li");
      // @ligne Prépare « link » pour la prochaine étape de la session navigateur et des transferts repris.
      const link = document.createElement("a");
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
      link.textContent = "Télécharger";
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
      link.href = "/api/files/" + file.id;
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
      link.download = file.name;
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
      row.append(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        document.createTextNode(file.name + " · " + file.size + " octets "),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        link,
      // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
      );
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
      list.append(row);
    // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
    }
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
    element("#upload-area").hidden = !data.upload;
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
    status(
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
      data.upload
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        ? "Vous pouvez contribuer à la salle."
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        : "Lecture seule · Votre hôte contrôle les permissions.",
    // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
    );
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
  } catch (e) {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
    status(String(e));
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
    element("#join").hidden = false;
  // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
  }
// @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
}
// L'authentification du WebSocket se fait dans son premier message et jamais dans l'URL.
let socket: WebSocket | null = null;
// Une reconnexion reprend le canal d'événements sans redemander le PIN.
function connect() {
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
  socket = new WebSocket("wss://" + location.host + "/events");
  // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la session navigateur et des transferts repris.
  socket.onopen = () => socket?.send(token);
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
  socket.onmessage = () => refresh();
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
  socket.onclose = () => {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la session navigateur et des transferts repris.
    if (token) setTimeout(connect, 4000);
  // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
  };
// @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
}
// Le formulaire reçoit un rôle lecture seule ; seul le maître peut le modifier.
element<HTMLFormElement>("#join").onsubmit = async (event) => {
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
  event.preventDefault();
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
  try {
    // @ligne Prépare « data » pour la prochaine étape de la session navigateur et des transferts repris.
    const data = await (
      // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la session navigateur et des transferts repris.
      await request("/api/join", {
        // @ligne Renseigne le champ « method » de la structure utilisée ici de la session navigateur et des transferts repris.
        method: "POST",
        // @ligne Renseigne le champ « headers » de la structure utilisée ici de la session navigateur et des transferts repris.
        headers: { "Content-Type": "application/json" },
        // @ligne Renseigne le champ « body » de la structure utilisée ici de la session navigateur et des transferts repris.
        body: JSON.stringify({
          // @ligne Renseigne le champ « name » de la structure utilisée ici de la session navigateur et des transferts repris.
          name: element<HTMLInputElement>("#name").value,
          // @ligne Renseigne le champ « pin » de la structure utilisée ici de la session navigateur et des transferts repris.
          pin: element<HTMLInputElement>("#pin").value,
        // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
        }),
      // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
      })
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
    ).json();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
    token = data.token;
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
    localStorage.setItem("omnishell-peer", token);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
    socket?.close();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
    connect();
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la session navigateur et des transferts repris.
    await refresh();
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
  } catch (e) {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
    status(String(e));
  // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
  }
// @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
};
// L'utilisateur choisit à nouveau son fichier après une interruption ; le serveur confirme la taille durable.
element<HTMLInputElement>("#upload").onchange = async (event) => {
  // @ligne Prépare « files » pour la prochaine étape de la session navigateur et des transferts repris.
  const files = (event.target as HTMLInputElement).files;
  // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la session navigateur et des transferts repris.
  if (!files) return;
  // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la session navigateur et des transferts repris.
  for (const file of files) {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
    try {
      // @ligne Prépare « signature » pour la prochaine étape de la session navigateur et des transferts repris.
      const signature = Array.from(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        blake3(
          // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la session navigateur et des transferts repris.
          new Uint8Array(await file.slice(0, 4 * 1024 * 1024).arrayBuffer()),
        // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
        ),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        (b) => b.toString(16).padStart(2, "0"),
      ).join(""); // Le début du contenu distingue deux fichiers homonymes de même taille.
      // @ligne Prépare « identity » pour la prochaine étape de la session navigateur et des transferts repris.
      const identity = [
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        file.name,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        file.size,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        file.lastModified,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        signature,
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
      ].join(":");
      // @ligne Prépare « id » pour la prochaine étape de la session navigateur et des transferts repris.
      let id = localStorage.getItem("transfer:" + identity);
      // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la session navigateur et des transferts repris.
      if (!id) {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        id = crypto.randomUUID();
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        localStorage.setItem("transfer:" + identity, id);
      // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
      }
      // @ligne Prépare « state » pour la prochaine étape de la session navigateur et des transferts repris.
      const state = await (await request("/api/upload/" + id)).json();
      // @ligne Prépare « offset » pour la prochaine étape de la session navigateur et des transferts repris.
      let offset = state.offset;
      // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la session navigateur et des transferts repris.
      if (offset > file.size)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        throw new Error("Le transfert distant ne correspond pas à ce fichier.");
      let started = false; // Un fichier vide nécessite quand même une finalisation côté serveur.
      // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la session navigateur et des transferts repris.
      while (offset < file.size || (!started && file.size === 0)) {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        started = true;
        // @ligne Prépare « bytes » pour la prochaine étape de la session navigateur et des transferts repris.
        const bytes = new Uint8Array(
          // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la session navigateur et des transferts repris.
          await file.slice(offset, offset + 4 * 1024 * 1024).arrayBuffer(),
        // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
        );
        // @ligne Prépare « digest » pour la prochaine étape de la session navigateur et des transferts repris.
        const digest = Array.from(blake3(bytes), (b) =>
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
          b.toString(16).padStart(2, "0"),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        ).join("");
        // @ligne Prépare « result » pour la prochaine étape de la session navigateur et des transferts repris.
        const result = await (
          // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours de la session navigateur et des transferts repris.
          await request("/api/upload/" + id, {
            // @ligne Renseigne le champ « method » de la structure utilisée ici de la session navigateur et des transferts repris.
            method: "PUT",
            // @ligne Renseigne le champ « headers » de la structure utilisée ici de la session navigateur et des transferts repris.
            headers: {
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
              "X-Offset": String(offset),
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
              "X-Blake3": digest,
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
              "X-Total-Size": String(file.size),
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
              "X-File-Name": encodeURIComponent(file.name),
            // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
            },
            // @ligne Renseigne le champ « body » de la structure utilisée ici de la session navigateur et des transferts repris.
            body: bytes,
          // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
          })
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        ).json();
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        offset = result.offset;
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        status(
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
          file.name +
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
            " · " +
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
            (file.size ? Math.round((offset / file.size) * 100) : 100) +
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
            " %",
        // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
        );
      // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
      }
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
      status(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        file.name +
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
          " reçu et publié dans la salle. Identifiant " +
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
          id +
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
          " dans son dossier de réception.",
      // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
      );
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
    } catch (e) {
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
      status(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
        "Transfert interrompu : " +
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
          String(e) +
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
          ". Sélectionnez à nouveau le fichier pour reprendre.",
      // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
      );
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
      break;
    // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
    }
  // @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
  }
// @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
};
// Le bouton permet un repli lorsque les WebSockets sont filtrés.
element("#refresh").onclick = refresh;
// Une session connue tente de retrouver ses permissions avant de demander un nouveau PIN.
if (token) {
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
  refresh();
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
  connect();
// @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
}
// Un code transmis par fragment ne figure ni dans la requête HTTP ni dans le Referer.
const fragment = new URLSearchParams(location.hash.slice(1));
// @ligne Prépare « sharedPin » pour la prochaine étape de la session navigateur et des transferts repris.
const sharedPin = fragment.get("pin");
// @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la session navigateur et des transferts repris.
if (sharedPin && /^\d{6}$/.test(sharedPin)) {
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
  element<HTMLInputElement>("#pin").value = sharedPin;
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la session navigateur et des transferts repris.
  history.replaceState(null, "", location.pathname);
// @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
}
// Un heartbeat léger entretient la présence et retrouve les fichiers même si les notifications ont été perdues.
setInterval(() => {
  // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la session navigateur et des transferts repris.
  if (token && !document.hidden) refresh();
// @ligne Termine cette construction et revient au contexte parent de la session navigateur et des transferts repris.
}, 15000);
