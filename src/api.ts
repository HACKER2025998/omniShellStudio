// L'interface native est la seule passerelle vers les données privées de la machine.
import { invoke } from "@tauri-apps/api/core";
// Les types décrivent les données réellement persistées, sans valeurs de démonstration.
export type Command = {
  id: number;
  raw: string;
  clean: string;
  binary: string;
  category: string;
  privileged: boolean;
  shell: string;
  cwd: string;
  terminal: string;
  count: number;
  favorite: boolean;
  last_seen: number;
  exit_code: number | null;
  duration_ms: number | null;
};
// Les snapshots sont volontairement extensibles selon les champs fournis par Docker.
export type Snapshot = { data: Record<string, any>; captured_at: number };
// Le navigateur seul sert au développement visuel et explique l'absence du moteur.
export async function api<T = any>(
  action: string,
  payload: Record<string, unknown> = {},
): Promise<T> {
  if (!("__TAURI_INTERNALS__" in window))
    throw new Error(
      "Ouvrez la version desktop avec npm run tauri dev : le navigateur seul ne peut pas lire vos données système.",
    ); // Aucun serveur admin non protégé n'est créé pour le navigateur.
  return invoke<T>("omni", { action, payload }); // Tauri applique les permissions de la fenêtre.
}
// Les exports sont produits localement et libèrent leur URL après téléchargement.
export function download(
  name: string,
  content: string,
  type = "application/json",
) {
  const url = URL.createObjectURL(new Blob([content], { type })); // Le contenu ne quitte jamais le poste.
  const link = document.createElement("a");
  link.href = url;
  link.download = name;
  link.click(); // Le navigateur gère le choix du répertoire.
  setTimeout(() => URL.revokeObjectURL(url), 1000); // Évite une fuite lors d'exports répétés.
}
