// @ligne Importe les dépendances explicites utilisées par cet espace.
import { invoke, isTauri } from "@tauri-apps/api/core";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import { open, save } from "@tauri-apps/plugin-dialog";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import embeddedCatalog from "../src-tauri/src/catalog.json";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import type { Template } from "./types";

// @ligne Fournit cet argument à l’expression de api ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
export const native = isTauri();
// @ligne Fournit cet argument à l’expression de api ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
export const catalog = embeddedCatalog as Template[];

// @ligne Passe par une commande métier native, avec validation côté Rust.
export async function call<T>(action: string, data: unknown = {}): Promise<T> {
  // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
  if (!native)
    // @ligne Signale une entrée ou une réponse invalide avant de poursuivre.
    throw new Error(
      // @ligne Fournit cet argument à l’expression de api ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "Cette action nécessite l’application desktop DockForge. Le navigateur permet seulement de parcourir les formulaires.",
    );
  // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
  try {
    // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
    return await invoke<T>("dispatch", { action, data });
  // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
  } catch (error) {
    // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
    const raw = String(error);
    // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
    try {
      // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
      const parsed = JSON.parse(raw) as {
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        title: string;
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        detail: string;
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        fix: string;
      };
      // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
      const result = new Error(`${parsed.title}\n${parsed.detail}`);
      // @ligne Fournit cet argument à l’expression de api ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      Object.assign(result, { fix: parsed.fix });
      // @ligne Signale une entrée ou une réponse invalide avant de poursuivre.
      throw result;
    // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
    } catch (parsed) {
      // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
      if (parsed instanceof SyntaxError) throw new Error(raw);
      // @ligne Signale une entrée ou une réponse invalide avant de poursuivre.
      throw parsed;
    }
  }
}

// @ligne Délimite cette responsabilité afin de garder le parcours testable.
export async function chooseFolder(): Promise<string | null> {
  // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
  if (!native)
    // @ligne Signale une entrée ou une réponse invalide avant de poursuivre.
    throw new Error(
      // @ligne Fournit cet argument à l’expression de api ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "Le sélecteur de dossiers est disponible dans l’application desktop.",
    );
  // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
  const result = await open({ directory: true, multiple: false });
  // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
  return typeof result === "string" ? result : null;
}
// @ligne Délimite cette responsabilité afin de garder le parcours testable.
export async function chooseFiles(multiple = true): Promise<string[]> {
  // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
  if (!native)
    // @ligne Signale une entrée ou une réponse invalide avant de poursuivre.
    throw new Error(
      // @ligne Fournit cet argument à l’expression de api ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "Le sélecteur de fichiers est disponible dans l’application desktop.",
    );
  // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
  const result = await open({ multiple, directory: false });
  // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
  return result ? (Array.isArray(result) ? result : [result]) : [];
}
// @ligne Délimite cette responsabilité afin de garder le parcours testable.
export async function exportCredentials(
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  id: string,
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  name: string,
// @ligne Fournit cet argument à l’expression de api ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
): Promise<boolean> {
  // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
  const path = await save({
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    defaultPath: `${name}_credentials.json`,
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    filters: [{ name: "Identifiants JSON", extensions: ["json"] }],
  });
  // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
  if (!path) return false;
  // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
  await call("export", { id, path });
  // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
  return true;
}
// @ligne Délimite cette responsabilité afin de garder le parcours testable.
export async function copy(value: string): Promise<void> {
  // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
  await navigator.clipboard.writeText(value);
}
// @ligne Délimite cette responsabilité afin de garder le parcours testable.
export function errorMessage(error: unknown): string {
  // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
  return error instanceof Error ? error.message : String(error);
}

// @ligne Délimite cette responsabilité afin de garder le parcours testable.
export async function backupVault(password: string): Promise<number | null> {
  // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
  const path = await save({
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    defaultPath: "dockforge-vault.dfv",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    filters: [{ name: "Sauvegarde chiffrée DockForge", extensions: ["dfv"] }],
  });
  // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
  if (!path) return null;
  // @ligne Passe par une commande métier native, avec validation côté Rust.
  return call<number>("backup", { path, password });
}
// @ligne Délimite cette responsabilité afin de garder le parcours testable.
export async function restoreVault(password: string): Promise<number | null> {
  // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
  const path = await open({
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    multiple: false,
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    filters: [{ name: "Sauvegarde chiffrée DockForge", extensions: ["dfv"] }],
  });
  // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
  if (typeof path !== "string") return null;
  // @ligne Passe par une commande métier native, avec validation côté Rust.
  return call<number>("restore", { path, password });
}

// @ligne Délimite cette responsabilité afin de garder le parcours testable.
export async function openDockerGuide(): Promise<void> {
  // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
  const { openUrl } = await import("@tauri-apps/plugin-opener");
  // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
  await openUrl("https://docs.docker.com/engine/install/");
}
