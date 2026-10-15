// L'interface native est la seule passerelle vers les données privées de la machine.
import { invoke } from "@tauri-apps/api/core";
// Les types décrivent les données réellement persistées, sans valeurs de démonstration.
export type Command = {
  // @ligne Renseigne le champ « id » de la structure utilisée ici du contrat IPC local.
  id: number;
  // @ligne Renseigne le champ « raw » de la structure utilisée ici du contrat IPC local.
  raw: string;
  // @ligne Renseigne le champ « clean » de la structure utilisée ici du contrat IPC local.
  clean: string;
  // @ligne Renseigne le champ « binary » de la structure utilisée ici du contrat IPC local.
  binary: string;
  // @ligne Renseigne le champ « category » de la structure utilisée ici du contrat IPC local.
  category: string;
  // @ligne Renseigne le champ « privileged » de la structure utilisée ici du contrat IPC local.
  privileged: boolean;
  // @ligne Renseigne le champ « shell » de la structure utilisée ici du contrat IPC local.
  shell: string;
  // @ligne Renseigne le champ « cwd » de la structure utilisée ici du contrat IPC local.
  cwd: string;
  // @ligne Renseigne le champ « terminal » de la structure utilisée ici du contrat IPC local.
  terminal: string;
  // @ligne Renseigne le champ « count » de la structure utilisée ici du contrat IPC local.
  count: number;
  // @ligne Renseigne le champ « favorite » de la structure utilisée ici du contrat IPC local.
  favorite: boolean;
  // @ligne Renseigne le champ « last_seen » de la structure utilisée ici du contrat IPC local.
  last_seen: number;
  // @ligne Renseigne le champ « exit_code » de la structure utilisée ici du contrat IPC local.
  exit_code: number | null;
  // @ligne Renseigne le champ « duration_ms » de la structure utilisée ici du contrat IPC local.
  duration_ms: number | null;
// @ligne Termine cette construction et revient au contexte parent du contrat IPC local.
};
// Les snapshots sont volontairement extensibles selon les champs fournis par Docker.
export type Snapshot = { data: Record<string, any>; captured_at: number };
// Le navigateur seul sert au développement visuel et explique l'absence du moteur.
export async function api<T = any>(
  // @ligne Renseigne le champ « action » de la structure utilisée ici du contrat IPC local.
  action: string,
  // @ligne Renseigne le champ « payload » de la structure utilisée ici du contrat IPC local.
  payload: Record<string, unknown> = {},
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du contrat IPC local.
): Promise<T> {
  // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du contrat IPC local.
  if (!("__TAURI_INTERNALS__" in window))
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du contrat IPC local.
    throw new Error(
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du contrat IPC local.
      "Ouvrez la version desktop avec npm run tauri dev : le navigateur seul ne peut pas lire vos données système.",
    ); // Aucun serveur admin non protégé n'est créé pour le navigateur.
  return invoke<T>("omni", { action, payload }); // Tauri applique les permissions de la fenêtre.
// @ligne Termine cette construction et revient au contexte parent du contrat IPC local.
}
// Les exports sont produits localement et libèrent leur URL après téléchargement.
export function download(
  // @ligne Renseigne le champ « name » de la structure utilisée ici du contrat IPC local.
  name: string,
  // @ligne Renseigne le champ « content » de la structure utilisée ici du contrat IPC local.
  content: string,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du contrat IPC local.
  type = "application/json",
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du contrat IPC local.
) {
  const url = URL.createObjectURL(new Blob([content], { type })); // Le contenu ne quitte jamais le poste.
  // @ligne Prépare « link » pour la prochaine étape du contrat IPC local.
  const link = document.createElement("a");
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du contrat IPC local.
  link.href = url;
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du contrat IPC local.
  link.download = name;
  link.click(); // Le navigateur gère le choix du répertoire.
  setTimeout(() => URL.revokeObjectURL(url), 1000); // Évite une fuite lors d'exports répétés.
// @ligne Termine cette construction et revient au contexte parent du contrat IPC local.
}
