// BLAKE3 est embarqué localement ; le navigateur ne contacte aucun CDN.
import { blake3 } from "@noble/hashes/blake3";
// Chaque origine de salle conserve son jeton et ses transferts indépendamment.
let token = localStorage.getItem("omnishell-peer") || "";
// Les sélecteurs sont typés pour repérer les erreurs d'interface à la compilation.
const element = <T extends HTMLElement>(selector: string) =>
  document.querySelector<T>(selector)!;
// Les erreurs serveur sont affichées plutôt que transformées en faux succès.
async function request(path: string, options: RequestInit = {}) {
  const response = await fetch(path, {
    ...options,
    headers: { Authorization: "Bearer " + token, ...options.headers },
  });
  if (!response.ok) throw new Error(await response.text());
  return response;
}
// Une région de statut accessible explique la progression et les permissions actuelles.
function status(message: string) {
  element("#status").textContent = message;
}
// Les téléchargements natifs utilisent un cookie HttpOnly pour gérer les fichiers volumineux et Range.
async function refresh() {
  try {
    const data = await (await request("/api/files")).json();
    element("#join").hidden = true;
    element("#refresh").hidden = false;
    const list = element("#files");
    list.replaceChildren();
    for (const file of data.files) {
      const row = document.createElement("li");
      const link = document.createElement("a");
      link.textContent = "Télécharger";
      link.href = "/api/files/" + file.id;
      link.download = file.name;
      row.append(
        document.createTextNode(file.name + " · " + file.size + " octets "),
        link,
      );
      list.append(row);
    }
    element("#upload-area").hidden = !data.upload;
    status(
      data.upload
        ? "Vous pouvez contribuer à la salle."
        : "Lecture seule · Votre hôte contrôle les permissions.",
    );
  } catch (e) {
    status(String(e));
    element("#join").hidden = false;
  }
}
// L'authentification du WebSocket se fait dans son premier message et jamais dans l'URL.
let socket: WebSocket | null = null;
// Une reconnexion reprend le canal d'événements sans redemander le PIN.
function connect() {
  socket = new WebSocket("wss://" + location.host + "/events");
  socket.onopen = () => socket?.send(token);
  socket.onmessage = () => refresh();
  socket.onclose = () => {
    if (token) setTimeout(connect, 4000);
  };
}
// Le formulaire reçoit un rôle lecture seule ; seul le maître peut le modifier.
element<HTMLFormElement>("#join").onsubmit = async (event) => {
  event.preventDefault();
  try {
    const data = await (
      await request("/api/join", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          name: element<HTMLInputElement>("#name").value,
          pin: element<HTMLInputElement>("#pin").value,
        }),
      })
    ).json();
    token = data.token;
    localStorage.setItem("omnishell-peer", token);
    socket?.close();
    connect();
    await refresh();
  } catch (e) {
    status(String(e));
  }
};
// L'utilisateur choisit à nouveau son fichier après une interruption ; le serveur confirme la taille durable.
element<HTMLInputElement>("#upload").onchange = async (event) => {
  const files = (event.target as HTMLInputElement).files;
  if (!files) return;
  for (const file of files) {
    try {
      const identity = [file.name, file.size, file.lastModified].join(":");
      let id = localStorage.getItem("transfer:" + identity);
      if (!id) {
        id = crypto.randomUUID();
        localStorage.setItem("transfer:" + identity, id);
      }
      const state = await (await request("/api/upload/" + id)).json();
      let offset = state.offset;
      while (offset < file.size) {
        const bytes = new Uint8Array(
          await file.slice(offset, offset + 4 * 1024 * 1024).arrayBuffer(),
        );
        const digest = Array.from(blake3(bytes), (b) =>
          b.toString(16).padStart(2, "0"),
        ).join("");
        const result = await (
          await request("/api/upload/" + id, {
            method: "PUT",
            headers: { "X-Offset": String(offset), "X-Blake3": digest },
            body: bytes,
          })
        ).json();
        offset = result.offset;
        status(
          file.name + " · " + Math.round((offset / file.size) * 100) + " %",
        );
      }
      status(
        file.name +
          " reçu. Le maître retrouvera le transfert " +
          id +
          " dans son dossier de réception.",
      );
    } catch (e) {
      status(
        "Transfert interrompu : " +
          String(e) +
          ". Sélectionnez à nouveau le fichier pour reprendre.",
      );
      break;
    }
  }
};
// Le bouton permet un repli lorsque les WebSockets sont filtrés.
element("#refresh").onclick = refresh;
// Une session connue tente de retrouver ses permissions avant de demander un nouveau PIN.
if (token) {
  refresh();
  connect();
}
