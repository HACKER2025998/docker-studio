// @ligne Importe les dépendances explicites utilisées par cet espace.
import { useEffect, useState } from "react";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import {
  // @ligne Importe ou référence ArrowUpRight, repère visuel ou type partagé par cette vue.
  ArrowUpRight,
  // @ligne Importe ou référence Check, repère visuel ou type partagé par cette vue.
  Check,
  // @ligne Importe ou référence Copy, repère visuel ou type partagé par cette vue.
  Copy,
  // @ligne Importe ou référence Download, repère visuel ou type partagé par cette vue.
  Download,
  // @ligne Importe ou référence Eye, repère visuel ou type partagé par cette vue.
  Eye,
  // @ligne Importe ou référence EyeOff, repère visuel ou type partagé par cette vue.
  EyeOff,
  // @ligne Importe ou référence FileUp, repère visuel ou type partagé par cette vue.
  FileUp,
  // @ligne Importe ou référence FolderOpen, repère visuel ou type partagé par cette vue.
  FolderOpen,
  // @ligne Importe ou référence Hammer, repère visuel ou type partagé par cette vue.
  Hammer,
  // @ligne Importe ou référence LockKeyhole, repère visuel ou type partagé par cette vue.
  LockKeyhole,
  // @ligne Importe ou référence Play, repère visuel ou type partagé par cette vue.
  Play,
  // @ligne Importe ou référence RefreshCw, repère visuel ou type partagé par cette vue.
  RefreshCw,
  // @ligne Importe ou référence Search, repère visuel ou type partagé par cette vue.
  Search,
  // @ligne Importe ou référence Square, repère visuel ou type partagé par cette vue.
  Square,
  // @ligne Importe ou référence Star, repère visuel ou type partagé par cette vue.
  Star,
  // @ligne Importe ou référence Trash2, repère visuel ou type partagé par cette vue.
  Trash2,
  // @ligne Importe ou référence Wifi, repère visuel ou type partagé par cette vue.
  Wifi,
  // @ligne Importe ou référence X, repère visuel ou type partagé par cette vue.
  X,
// @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
} from "lucide-react";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import {
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  call,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  native,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  openDockerGuide,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  backupVault,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  restoreVault,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  chooseFiles,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  chooseFolder,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  copy,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  errorMessage,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  exportCredentials,
// @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
} from "./api";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import type {
  // @ligne Importe ou référence Container, repère visuel ou type partagé par cette vue.
  Container,
  // @ligne Importe ou référence Credentials, repère visuel ou type partagé par cette vue.
  Credentials,
  // @ligne Importe ou référence Doctor, repère visuel ou type partagé par cette vue.
  Doctor,
  // @ligne Importe ou référence Progress, repère visuel ou type partagé par cette vue.
  Progress,
  // @ligne Importe ou référence Recipe, repère visuel ou type partagé par cette vue.
  Recipe,
  // @ligne Importe ou référence VaultEntry, repère visuel ou type partagé par cette vue.
  VaultEntry,
// @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
} from "./types";

// @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
type Notify = (message: string) => void;

// @ligne Délimite cette responsabilité afin de garder le parcours testable.
export function DoctorWorkspace({
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  doctor,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  loading,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  refresh,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  notify,
// @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
}: {
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  doctor: Doctor | null;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  loading: boolean;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  refresh: () => void;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  notify: Notify;
// @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
}) {
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [busy, setBusy] = useState(false);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [confirm, setConfirm] = useState("");
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const action = async () => {
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setBusy(true);
    // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
    try {
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      const result = await call<string>("remedy", { action: confirm });
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setConfirm("");
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      notify(result || "Action terminée. Rafraîchissez le diagnostic.");
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      refresh();
    // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
    } catch (e) {
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      notify(errorMessage(e));
    // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
    } finally {
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setBusy(false);
    }
  };
  // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
  return (
    // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
    <div className="workspace-stack">
      {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
      <section className="doctor-intro">
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <div className={`status-orb ${doctor?.ready ? "ready" : ""}`}>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <Check size={28} />
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <div>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <span className="eyebrow">SYSTEM READINESS ASSISTANT</span>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <h2>
            {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
            {doctor?.ready
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              ? "Votre atelier est prêt."
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              : "Préparons votre environnement."}
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </h2>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <p>
            {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
            {doctor
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              ? `${doctor.os} · ${doctor.arch}`
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              : "Les sondes système s’exécutent uniquement dans l’application native."}
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </p>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
        {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
        <button className="primary" onClick={refresh} disabled={loading}>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <RefreshCw size={17} className={loading ? "spin" : ""} />
          {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
          {loading ? "Vérification…" : "Rafraîchir le diagnostic"}
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </button>
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </section>
      {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
      <div className="check-list">
        {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
        {doctor?.checks.map((c) => (
          // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
          <article key={c.id} className="check-row">
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <span className={`check-dot ${c.ok ? "ok" : ""}`}>
              {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
              {c.ok ? <Check size={16} /> : "!"}
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </span>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <div>
              {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
              <h3>{c.title}</h3>
              {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
              <p>{c.detail}</p>
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </div>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <span className={`badge ${c.ok ? "green" : "amber"}`}>
              {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
              {c.ok ? "Validé" : "À vérifier"}
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </span>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </article>
        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        )) || (
          // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
          <div className="empty-state">
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <LockKeyhole size={32} />
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <h3>Accès système réservé au desktop</h3>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <p>
              Vous pouvez explorer le catalogue ici. Pour créer un conteneur,
              lancez DockForge depuis Tauri ou son installateur.
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </p>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
        )}
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </div>
      {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
      {doctor && !doctor.ready && (
        // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
        <section className="panel">
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <h3>Les prochaines étapes, sans deviner</h3>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <p>{doctor.installation}</p>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <div className="button-row">
            {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
            {doctor.os === "linux" && (
              // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
              <>
                {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
                <button onClick={() => setConfirm("start")}>
                  Démarrer Docker via Polkit
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </button>
                {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
                <button onClick={() => setConfirm("group")}>
                  Configurer le groupe Docker
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </button>
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </>
            )}
            {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
            {doctor.os === "windows" && (
              // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
              <>
                {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
                <button onClick={() => setConfirm("install")}>
                  Installer Docker Desktop
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </button>
                {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
                <button onClick={() => setConfirm("launch")}>
                  Lancer Docker Desktop
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </button>
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </>
            )}
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <p className="muted">
            Le groupe Docker donne un contrôle équivalent à root sur l’hôte. Une
            fermeture complète de session est nécessaire après son ajout.
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </p>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </section>
      )}
      {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
      {confirm && (
        // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
        <section className="alert warning" role="alert">
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <div>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <h3>Autoriser cette opération système ?</h3>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <p>
              {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
              {confirm === "install" && doctor?.os === "linux"
                // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                ? "Installer les paquets Docker du gestionnaire système (Arch : docker, docker-buildx, docker-compose ; Debian/Ubuntu : Docker CE et plugins, après configuration de son dépôt officiel). Cette action télécharge des paquets et peut durer plusieurs minutes."
                // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                : confirm === "group"
                  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  ? "Ajouter votre compte au groupe docker donne accès au démon et peut permettre de prendre le contrôle de l’hôte."
                  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  : "Le système va ouvrir une demande native de privilèges pour cette action précise."}
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </p>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <div className="button-row">
              {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
              <button disabled={busy} className="primary" onClick={action}>
                Ouvrir la demande système
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </button>
              {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
              <button disabled={busy} onClick={() => setConfirm("")}>
                Annuler
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </button>
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </div>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </section>
      )}
      {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
      <section className="note-panel">
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <LockKeyhole size={19} />
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <p>
          DockForge reste sous votre compte utilisateur. Il n’installe rien et
          ne modifie aucun groupe au démarrage.
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </p>
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </section>
    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
    </div>
  );
}

// @ligne Délimite cette responsabilité afin de garder le parcours testable.
export function ImageWorkspace({
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  ready,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  progress,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  launch,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  notify,
// @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
}: {
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  ready: boolean;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  progress: Progress[];
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  launch: (image: string) => void;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  notify: Notify;
// @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
}) {
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [options, setOptions] = useState<{
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    bases: string[];
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    packages: string[];
  // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
  }>({
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    bases: [
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "ubuntu:24.04",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "debian:12-slim",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "alpine:3.20",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "archlinux:base",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "python:3.12-slim",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "node:22-bookworm-slim",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "php:8.3-apache",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "rust:1-slim-bookworm",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "mcr.microsoft.com/dotnet/sdk:8.0",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "eclipse-temurin:21-jdk",
    ],
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    packages: [
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "curl",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "git",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "vim",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "nmap",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "iproute2",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "ffmpeg",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "supervisor",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "pdo_pgsql",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "pdo_mysql",
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "redis",
    ],
  });
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [recipe, setRecipe] = useState<Recipe>({
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    operation: "",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    tag: "mon-projet:v1",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    base: "python:3.12-slim",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    packages: ["curl"],
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    project: null,
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    command: [],
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    port: 8000,
  });
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [command, setCommand] = useState('["python", "main.py"]');
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [preview, setPreview] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [busy, setBusy] = useState(false);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [built, setBuilt] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [images, setImages] = useState<{ tag: string; created_at: string }[]>(
    [],
  );
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const load = () => {
    // @ligne Passe par une commande métier native, avec validation côté Rust.
    call<typeof images>("images")
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      .then(setImages)
      // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
      .catch(() => undefined);
  };
  // @ligne Déclare un effet lié au cycle de vie ; son nettoyage évite les abonnements persistants.
  useEffect(() => {
    // @ligne Passe par une commande métier native, avec validation côté Rust.
    call<typeof options>("image_options")
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      .then(setOptions)
      // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
      .catch(() => undefined);
    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    load();
  // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
  }, []);
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const task = async (build: boolean) => {
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setBusy(true);
    // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
    try {
      // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
      const args: unknown = JSON.parse(command);
      // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
      if (!Array.isArray(args) || args.some((a) => typeof a !== "string"))
        // @ligne Signale une entrée ou une réponse invalide avant de poursuivre.
        throw new Error("Utilisez un tableau JSON pour la commande.");
      // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
      const r = {
        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        ...recipe,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        command: args as string[],
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        operation: crypto.randomUUID(),
      };
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setRecipe(r);
      // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
      if (build) {
        // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
        const image = await call<string>("build", r);
        // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
        setBuilt(image);
        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        load();
        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        notify(
          // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          "Votre image est construite. Elle peut maintenant devenir un conteneur.",
        );
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      } else setPreview(await call<string>("dockerfile", r));
    // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
    } catch (e) {
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      notify(errorMessage(e));
    // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
    } finally {
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setBusy(false);
    }
  };
  // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
  return (
    // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
    <div className="two-columns">
      {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
      <section className="panel">
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <span className="eyebrow">VISUAL IMAGE STUDIO</span>
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <h2>Une image à votre mesure.</h2>
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <p className="muted">
          Choisissez votre base et vos outils. Le Dockerfile reste consultable
          avant la construction.
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </p>
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <div className="form-grid">
          {/* @ligne Présente un champ de configuration associé à son libellé. */}
          <label>
            Image de base
            {/* @ligne Présente un champ de configuration associé à son libellé. */}
            <select
              // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
              value={recipe.base}
              // @ligne Répercute la saisie dans l’état du formulaire.
              onChange={(e) =>
                // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                setRecipe((r) => ({ ...r, base: e.target.value }))
              }
            // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            >
              {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
              {options.bases.map((base) => (
                // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
                <option key={base}>{base}</option>
              ))}
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </select>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </label>
          {/* @ligne Présente un champ de configuration associé à son libellé. */}
          <label>
            Artefact multi-stage (Rust / .NET, facultatif)
            {/* @ligne Présente un champ de configuration associé à son libellé. */}
            <input
              // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
              value={recipe.artifact || ""}
              // @ligne Répercute la saisie dans l’état du formulaire.
              onChange={(e) =>
                // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                setRecipe((r) => ({ ...r, artifact: e.target.value || null }))
              }
              // @ligne Fournit un exemple de saisie sans remplacer le libellé accessible.
              placeholder="mon-binaire ou MonApp.dll"
            // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            />
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <small>
              Renseignez l’artefact, choisissez un dossier projet et décochez
              les outils. La commande devient l’argument de l’entrypoint.
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </small>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </label>
          {/* @ligne Présente un champ de configuration associé à son libellé. */}
          <label>
            Nom & tag
            {/* @ligne Présente un champ de configuration associé à son libellé. */}
            <input
              // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
              value={recipe.tag}
              // @ligne Répercute la saisie dans l’état du formulaire.
              onChange={(e) =>
                // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                setRecipe((r) => ({ ...r, tag: e.target.value }))
              }
            // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            />
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </label>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <fieldset className="full package-grid">
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <legend>Outils & extensions</legend>
            {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
            {options.packages.map((p) => (
              // @ligne Présente un champ de configuration associé à son libellé.
              <label className="check-label" key={p}>
                {/* @ligne Présente un champ de configuration associé à son libellé. */}
                <input
                  // @ligne Choisit le comportement natif du contrôle, notamment le masquage des secrets.
                  type="checkbox"
                  // @ligne Reflète le choix courant dans la case à cocher.
                  checked={recipe.packages.includes(p)}
                  // @ligne Répercute la saisie dans l’état du formulaire.
                  onChange={(e) =>
                    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                    setRecipe((r) => ({
                      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      ...r,
                      // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
                      packages: e.target.checked
                        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        ? [...r.packages, p]
                        // @ligne Ne conserve que les éléments répondant au filtre demandé.
                        : r.packages.filter((v) => v !== p),
                    }))
                  }
                // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                />
                {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                {p}
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </label>
            ))}
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </fieldset>
          {/* @ligne Présente un champ de configuration associé à son libellé. */}
          <label className="full">
            Dossier projet (facultatif)
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <div className="input-with-actions">
              {/* @ligne Présente un champ de configuration associé à son libellé. */}
              <input
                // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                value={recipe.project || ""}
                // @ligne Répercute la saisie dans l’état du formulaire.
                onChange={(e) =>
                  // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                  setRecipe((r) => ({ ...r, project: e.target.value || null }))
                }
                // @ligne Fournit un exemple de saisie sans remplacer le libellé accessible.
                placeholder="Aucun dossier incorporé"
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              />
              {/* @ligne Présente une action volontaire, accessible au clavier. */}
              <button
                // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                aria-label="Choisir le dossier projet"
                // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                onClick={async () => {
                  // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
                  try {
                    // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                    const p = await chooseFolder();
                    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
                    if (p) setRecipe((r) => ({ ...r, project: p }));
                  // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
                  } catch (e) {
                    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    notify(errorMessage(e));
                  }
                }}
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              >
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <FolderOpen size={18} />
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </button>
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </div>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </label>
          {/* @ligne Présente un champ de configuration associé à son libellé. */}
          <label className="full">
            Commande de démarrage (tableau JSON)
            {/* @ligne Présente un champ de configuration associé à son libellé. */}
            <textarea
              // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
              value={command}
              // @ligne Répercute la saisie dans l’état du formulaire.
              onChange={(e) => setCommand(e.target.value)}
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              rows={2}
            // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            />
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </label>
          {/* @ligne Présente un champ de configuration associé à son libellé. */}
          <label>
            Port exposé
            {/* @ligne Présente un champ de configuration associé à son libellé. */}
            <input
              // @ligne Choisit le comportement natif du contrôle, notamment le masquage des secrets.
              type="number"
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              min={1}
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              max={65535}
              // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
              value={recipe.port || ""}
              // @ligne Répercute la saisie dans l’état du formulaire.
              onChange={(e) =>
                // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                setRecipe((r) => ({
                  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  ...r,
                  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
                  port: Number(e.target.value) || null,
                }))
              }
            // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            />
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </label>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <p className="muted">
          .env, .git, node_modules, target et liens symboliques sont exclus.
          Vérifiez aussi les autres fichiers sensibles de votre projet.
          L’exposition seule ne publie aucun port sur l’hôte.
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </p>
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <div className="button-row">
          {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
          <button disabled={busy} onClick={() => task(false)}>
            Voir le Dockerfile
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </button>
          {/* @ligne Présente une action volontaire, accessible au clavier. */}
          <button
            // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
            className="primary"
            // @ligne Bloque une action indisponible ou déjà en cours.
            disabled={!ready || busy}
            // @ligne Relie ce bouton à une action explicite de l’utilisateur.
            onClick={() => task(true)}
          // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          >
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <Hammer size={17} />
            {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
            {busy ? "Préparation…" : "Construire l’image"}
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </button>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
        {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
        {built && (
          // @ligne Relie ce bouton à une action explicite de l’utilisateur.
          <button className="success-button" onClick={() => launch(built)}>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <Play size={17} />
            Créer un conteneur avec {built}
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </button>
        )}
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </section>
      {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
      <aside className="workspace-stack">
        {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
        {preview && (
          // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
          <section className="panel">
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <h3>Dockerfile généré</h3>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <pre>{preview}</pre>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </section>
        )}
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <section className="panel">
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <h3>Mes images construites</h3>
          {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
          {images.length ? (
            // @ligne Construit une entrée de vue pour chaque élément de cette collection.
            images.map((i) => (
              // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
              <div className="image-row" key={i.tag}>
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <code>{i.tag}</code>
                {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
                <button onClick={() => launch(i.tag)}>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <Play size={14} />
                  Utiliser
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </button>
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </div>
            ))
          // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          ) : (
            // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
            <p className="muted">
              Vos images apparaîtront ici après un build réussi.
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </p>
          )}
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </section>
        {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
        {progress.some((e) => e.operation === recipe.operation) && (
          // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
          <div className="operation-log" role="log">
            {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
            {progress
              // @ligne Ne conserve que les éléments répondant au filtre demandé.
              .filter((e) => e.operation === recipe.operation)
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              .slice(-20)
              // @ligne Construit une entrée de vue pour chaque élément de cette collection.
              .map((p, i) => (
                // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
                <p key={i}>{p.message}</p>
              ))}
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
        )}
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </aside>
    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
    </div>
  );
}

// @ligne Délimite cette responsabilité afin de garder le parcours testable.
export function VaultWorkspace({
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  refreshToken,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  notify,
// @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
}: {
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  refreshToken: number;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  notify: Notify;
// @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
}) {
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [entries, setEntries] = useState<VaultEntry[]>([]);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [backupPass, setBackupPass] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [backupRepeat, setBackupRepeat] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [containers, setContainers] = useState<Container[]>([]);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [offline, setOffline] = useState(false);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [selected, setSelected] = useState<VaultEntry | null>(null);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [doc, setDoc] = useState<Credentials | null>(null);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [logs, setLogs] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [show, setShow] = useState(false);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [confirm, setConfirm] = useState(false);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [archive, setArchive] = useState(true);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [busy, setBusy] = useState(false);
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const load = async () => {
    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
    if (!native) return;
    // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
    try {
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      setEntries(await call<VaultEntry[]>("vault"));
    // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
    } catch (e) {
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      notify(errorMessage(e));
    }
    // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
    try {
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      setContainers(await call<Container[]>("containers"));
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setOffline(false);
    // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
    } catch {
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setOffline(true);
    }
  };
  // @ligne Déclare un effet lié au cycle de vie ; son nettoyage évite les abonnements persistants.
  useEffect(() => {
    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    void load();
  // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
  }, [refreshToken]);
  // @ligne Déclare un effet lié au cycle de vie ; son nettoyage évite les abonnements persistants.
  useEffect(() => {
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setDoc(null);
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setLogs("");
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setShow(false);
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setConfirm(false);
  // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
  }, [selected?.id]);
  // @ligne Déclare un effet lié au cycle de vie ; son nettoyage évite les abonnements persistants.
  useEffect(() => {
    // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
    const hide = () => {
      // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
      if (document.hidden) {
        // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
        setDoc(null);
        // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
        setShow(false);
        // @ligne Passe par une commande métier native, avec validation côté Rust.
        void call("lock").catch(() => undefined);
      }
    };
    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    document.addEventListener("visibilitychange", hide);
    // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
    return () => document.removeEventListener("visibilitychange", hide);
  // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
  }, []);
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const task = async (job: () => Promise<unknown>) => {
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setBusy(true);
    // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
    try {
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      await job();
    // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
    } catch (e) {
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      notify(errorMessage(e));
    // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
    } finally {
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setBusy(false);
    }
  };
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const lifecycle = (action: string) =>
    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    task(async () => {
      // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
      if (!selected) return;
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      await call("lifecycle", { id: selected.id, action, archive });
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setConfirm(false);
      // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
      if (action === "remove") {
        // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
        setSelected(null);
        // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
        setDoc(null);
      }
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      await load();
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      notify("Action terminée. Les volumes de données sont conservés.");
    });
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const state = selected
    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    ? containers.find((c) => c.Id === selected.id)?.State
    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    : "";
  // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
  return (
    // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
    <div className="two-columns vault-layout">
      {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
      <section className="panel">
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <div className="section-heading">
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <h2>Mes conteneurs & accès</h2>
          {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
          <button aria-label="Actualiser le coffre" onClick={() => void load()}>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <RefreshCw size={17} />
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </button>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <p className="muted">
          Les accès restent disponibles même lorsque Docker est arrêté.
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </p>
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <details className="backup-panel">
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <summary>Sauvegarde chiffrée & restauration</summary>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <p className="muted">
            Une sauvegarde portable protège toutes les fiches d’accès avec votre
            propre phrase. Elle ne sauvegarde pas les volumes Docker. Les fiches
            déjà présentes ne seront pas remplacées à la restauration.
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </p>
          {/* @ligne Présente un champ de configuration associé à son libellé. */}
          <label>
            Phrase de sauvegarde (12 caractères minimum)
            {/* @ligne Présente un champ de configuration associé à son libellé. */}
            <input
              // @ligne Choisit le comportement natif du contrôle, notamment le masquage des secrets.
              type="password"
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              autoComplete="new-password"
              // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
              value={backupPass}
              // @ligne Répercute la saisie dans l’état du formulaire.
              onChange={(e) => setBackupPass(e.target.value)}
            // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            />
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </label>
          {/* @ligne Présente un champ de configuration associé à son libellé. */}
          <label>
            Répéter la phrase pour créer une sauvegarde
            {/* @ligne Présente un champ de configuration associé à son libellé. */}
            <input
              // @ligne Choisit le comportement natif du contrôle, notamment le masquage des secrets.
              type="password"
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              autoComplete="new-password"
              // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
              value={backupRepeat}
              // @ligne Répercute la saisie dans l’état du formulaire.
              onChange={(e) => setBackupRepeat(e.target.value)}
            // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            />
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </label>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <div className="button-row">
            {/* @ligne Présente une action volontaire, accessible au clavier. */}
            <button
              // @ligne Bloque une action indisponible ou déjà en cours.
              disabled={
                // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                busy ||
                // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                !native ||
                // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                backupPass.length < 12 ||
                // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                backupPass !== backupRepeat
              }
              // @ligne Relie ce bouton à une action explicite de l’utilisateur.
              onClick={() =>
                // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                task(async () => {
                  // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
                  try {
                    // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                    const count = await backupVault(backupPass);
                    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
                    if (count !== null)
                      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      notify(
                        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        `${count} fiches sauvegardées. Conservez la phrase séparément du fichier.`,
                      );
                  // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
                  } finally {
                    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                    setBackupPass("");
                    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                    setBackupRepeat("");
                  }
                })
              }
            // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            >
              Sauvegarder le coffre
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </button>
            {/* @ligne Présente une action volontaire, accessible au clavier. */}
            <button
              // @ligne Bloque une action indisponible ou déjà en cours.
              disabled={busy || !native || backupPass.length < 12}
              // @ligne Relie ce bouton à une action explicite de l’utilisateur.
              onClick={() =>
                // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                task(async () => {
                  // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
                  try {
                    // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                    const count = await restoreVault(backupPass);
                    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
                    if (count !== null) {
                      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                      await load();
                      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      notify(
                        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        `${count} nouvelles fiches restaurées. Les conteneurs et volumes ne sont pas recréés.`,
                      );
                    }
                  // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
                  } finally {
                    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                    setBackupPass("");
                    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                    setBackupRepeat("");
                  }
                })
              }
            // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            >
              Restaurer un fichier .dfv
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </button>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </details>

        {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
        {offline && (
          // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
          <p className="badge amber">
            Démon inaccessible : état des conteneurs inconnu
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </p>
        )}
        {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
        {entries.length ? (
          // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
          <div className="vault-list">
            {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
            {entries.map((e) => (
              // @ligne Présente une action volontaire, accessible au clavier.
              <button
                // @ligne Donne à React une identité stable pour conserver correctement cet élément de liste.
                key={e.id}
                // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                className={`vault-card ${selected?.id === e.id ? "selected" : ""}`}
                // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                onClick={() => setSelected(e)}
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              >
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <span className="container-cube">
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <LockKeyhole size={22} />
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </span>
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <span>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <strong>{e.name}</strong>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <code>{e.image}</code>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <small>
                    {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                    {e.archived
                      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      ? "Fiche archivée"
                      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      : containers.find((c) => c.Id === e.id)?.State ||
                        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        (offline ? "État inconnu" : "Absent du démon")}
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </small>
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </span>
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <ArrowUpRight size={17} />
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </button>
            ))}
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        ) : (
          // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
          <div className="empty-state">
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <LockKeyhole size={32} />
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <h3>Votre coffre attend son premier service.</h3>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <p>
              Créez un conteneur : ses accès seront conservés ici, chiffrés sur
              votre ordinateur.
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </p>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
        )}
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </section>
      {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
      <aside className="panel inspector">
        {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
        {selected ? (
          // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
          <>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <div className="section-heading">
              {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
              <h2>{selected.name}</h2>
              {/* @ligne Présente une action volontaire, accessible au clavier. */}
              <button
                // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                aria-label="Fermer la fiche"
                // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                onClick={() => setSelected(null)}
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              >
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <X size={18} />
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </button>
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </div>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <code>{selected.image}</code>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <div className="button-row lifecycle">
              {/* @ligne Présente une action volontaire, accessible au clavier. */}
              <button
                // @ligne Bloque une action indisponible ou déjà en cours.
                disabled={
                  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  busy || selected.archived || offline || state === "running"
                }
                // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                onClick={() => lifecycle("start")}
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              >
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <Play size={15} />
                Démarrer
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </button>
              {/* @ligne Présente une action volontaire, accessible au clavier. */}
              <button
                // @ligne Bloque une action indisponible ou déjà en cours.
                disabled={busy || offline || state !== "running"}
                // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                onClick={() => lifecycle("stop")}
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              >
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <Square size={14} />
                Arrêter
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </button>
              {/* @ligne Présente une action volontaire, accessible au clavier. */}
              <button
                // @ligne Bloque une action indisponible ou déjà en cours.
                disabled={busy || offline || !state || state === "running"}
                // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                onClick={() => setConfirm(true)}
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              >
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <Trash2 size={15} />
                Supprimer
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </button>
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </div>
            {/* @ligne Présente une action volontaire, accessible au clavier. */}
            <button
              // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
              className="wide"
              // @ligne Bloque une action indisponible ou déjà en cours.
              disabled={busy}
              // @ligne Relie ce bouton à une action explicite de l’utilisateur.
              onClick={() =>
                // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                task(async () => {
                  // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
                  if (doc) {
                    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                    setDoc(null);
                    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                    setShow(false);
                    // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                    await call("lock");
                  // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
                  } else
                    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                    setDoc(
                      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                      await call<Credentials>("credentials", {
                        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
                        id: selected.id,
                      }),
                    );
                })
              }
            // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            >
              {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
              {doc ? <EyeOff size={17} /> : <Eye size={17} />}
              {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
              {doc ? "Masquer & verrouiller" : "Déverrouiller les accès"}
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </button>
            {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
            {doc && (
              // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
              <div className="credentials">
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <dl>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <dt>Adresse</dt>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <dd>
                    {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                    {doc.network.bind_ip}:{doc.network.host_port}
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </dd>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <dt>Utilisateur</dt>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <dd>
                    {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                    {doc.credentials.username || "Voir variables du service"}
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </dd>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <dt>Base</dt>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <dd>{doc.credentials.database || "—"}</dd>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <dt>Mot de passe</dt>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <dd>
                    {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                    <code>
                      {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                      {show
                        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        ? doc.credentials.password ||
                          // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                          "Voir variables ci-dessous"
                        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        : "••••••••••••"}
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </code>
                    {/* @ligne Présente une action volontaire, accessible au clavier. */}
                    <button
                      // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                      className="icon-button"
                      // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                      aria-label="Afficher ou masquer le mot de passe"
                      // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                      onClick={() => setShow((v) => !v)}
                    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    >
                      {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                      {show ? <EyeOff size={15} /> : <Eye size={15} />}
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </button>
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </dd>
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </dl>
                {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                {show && (
                  // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
                  <pre>
                    {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                    {JSON.stringify(doc.credentials.environment, null, 2)}
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </pre>
                )}
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <div className="button-row">
                  {/* @ligne Présente une action volontaire, accessible au clavier. */}
                  <button
                    // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                    onClick={() =>
                      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      task(async () => {
                        // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                        await copy(doc.connection_strings.uri_standard);
                        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        notify("URI copiée.");
                      })
                    }
                  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  >
                    {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                    <Copy size={15} />
                    URI
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </button>
                  {/* @ligne Présente une action volontaire, accessible au clavier. */}
                  <button
                    // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                    onClick={() =>
                      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      task(async () => {
                        // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                        await copy(doc.connection_strings.laravel_env);
                        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        notify("Configuration .env copiée.");
                      })
                    }
                  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  >
                    {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                    <Copy size={15} />
                    .env
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </button>
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </div>
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </div>
            )}
            {/* @ligne Présente une action volontaire, accessible au clavier. */}
            <button
              // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
              className="primary wide"
              // @ligne Bloque une action indisponible ou déjà en cours.
              disabled={busy}
              // @ligne Relie ce bouton à une action explicite de l’utilisateur.
              onClick={() =>
                // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                task(async () => {
                  // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                  if (await exportCredentials(selected.id, selected.name))
                    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    notify(
                      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      "Export terminé. Ce fichier JSON contient des secrets en clair.",
                    );
                })
              }
            // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            >
              {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
              <Download size={17} />
              Télécharger les identifiants JSON
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </button>
            {/* @ligne Présente une action volontaire, accessible au clavier. */}
            <button
              // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
              className="wide"
              // @ligne Bloque une action indisponible ou déjà en cours.
              disabled={busy || offline || !state}
              // @ligne Relie ce bouton à une action explicite de l’utilisateur.
              onClick={() =>
                // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                task(async () =>
                  // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                  setLogs(await call<string>("logs", { id: selected.id })),
                )
              }
            // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            >
              Voir les 50 dernières lignes de logs
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </button>
            {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
            {logs && <pre className="log-output">{logs}</pre>}
            {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
            {confirm && (
              // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
              <div role="alert" className="alert warning">
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <div>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <h3>Supprimer {selected.name} ?</h3>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <p>
                    Le conteneur doit être arrêté. Ses volumes seront conservés.
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </p>
                  {/* @ligne Présente un champ de configuration associé à son libellé. */}
                  <label className="check-label">
                    {/* @ligne Présente un champ de configuration associé à son libellé. */}
                    <input
                      // @ligne Choisit le comportement natif du contrôle, notamment le masquage des secrets.
                      type="checkbox"
                      // @ligne Reflète le choix courant dans la case à cocher.
                      checked={archive}
                      // @ligne Répercute la saisie dans l’état du formulaire.
                      onChange={(e) => setArchive(e.target.checked)}
                    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    />
                    Archiver la fiche d’accès (sinon l’effacer)
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </label>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <div className="button-row">
                    {/* @ligne Présente une action volontaire, accessible au clavier. */}
                    <button
                      // @ligne Bloque une action indisponible ou déjà en cours.
                      disabled={busy}
                      // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                      className="danger"
                      // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                      onClick={() => lifecycle("remove")}
                    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    >
                      Confirmer la suppression
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </button>
                    {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
                    <button onClick={() => setConfirm(false)}>Annuler</button>
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </div>
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </div>
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </div>
            )}
            {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
            {!offline && !state && (
              // @ligne Présente une action volontaire, accessible au clavier.
              <button
                // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                onClick={() =>
                  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  task(async () => {
                    // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                    await call("retire", { id: selected.id, archive: true });
                    // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                    await load();
                    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    notify("Fiche archivée.");
                  })
                }
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              >
                Archiver cette fiche orpheline
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </button>
            )}
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </>
        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        ) : (
          // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
          <div className="empty-state">
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <Eye size={32} />
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <h3>Un accès, toujours à portée.</h3>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <p>
              Sélectionnez un conteneur pour consulter ses accès, son état et
              ses logs.
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </p>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
        )}
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </aside>
    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
    </div>
  );
}

// @ligne Délimite cette responsabilité afin de garder le parcours testable.
export function CommandWorkspace({ notify }: { notify: Notify }) {
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [search, setSearch] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [watch, setWatch] = useState(true);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [sources, setSources] = useState<[string, string][]>([]);
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const loadSources = () =>
    // @ligne Passe par une commande métier native, avec validation côté Rust.
    call<[string, string][]>("history_sources")
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      .then(setSources)
      // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
      .catch(() => undefined);
  // @ligne Déclare un effet lié au cycle de vie ; son nettoyage évite les abonnements persistants.
  useEffect(() => {
    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    void loadSources();
  // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
  }, []);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [favorites, setFavorites] = useState(false);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [shell, setShell] = useState("bash");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [rows, setRows] = useState<
    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    { id: number; command: string; shell: string; favorite: boolean }[]
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  >([]);
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const load = () =>
    // @ligne Passe par une commande métier native, avec validation côté Rust.
    call<typeof rows>("commands", { search, favorites })
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      .then(setRows)
      // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
      .catch(() => undefined);
  // @ligne Déclare un effet lié au cycle de vie ; son nettoyage évite les abonnements persistants.
  useEffect(() => {
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    const timer = setTimeout(() => void load(), 180);
    // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
    return () => clearTimeout(timer);
  // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
  }, [search, favorites]);
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const ingest = async () => {
    // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
    try {
      // @ligne Associe la valeur affichée à sa fonction de mise à jour React.
      const [path] = await chooseFiles(false);
      // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
      if (!path) return;
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      const count = await call<number>("import", { path, shell });
      // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
      if (watch) {
        // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
        await call("watch_history", { path, shell, enabled: true });
        // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
        await loadSources();
      }
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      notify(
        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        `${count} nouvelles commandes importées et filtrées.${watch ? " Cet historique sera suivi pendant l’ouverture de DockForge et rattrapé au prochain lancement." : ""}`,
      );
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      await load();
    // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
    } catch (e) {
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      notify(errorMessage(e));
    }
  };
  // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
  return (
    // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
    <section className="panel">
      {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
      <div className="section-heading">
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <div>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <h2>Les commandes qui vous resservent.</h2>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <p className="muted">
            Import local, favoris et copie. Aucune commande n’est exécutée
            depuis ce coffre.
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </p>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
        {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
        <button className="primary" onClick={ingest}>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <FileUp size={17} />
          Importer un historique
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </button>
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </div>
      {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
      <div className="toolbar">
        {/* @ligne Présente un champ de configuration associé à son libellé. */}
        <label className="search-input">
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <Search size={17} />
          {/* @ligne Présente un champ de configuration associé à son libellé. */}
          <input
            // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
            aria-label="Rechercher une commande"
            // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
            value={search}
            // @ligne Répercute la saisie dans l’état du formulaire.
            onChange={(e) => setSearch(e.target.value)}
            // @ligne Fournit un exemple de saisie sans remplacer le libellé accessible.
            placeholder="Rechercher dans mes commandes…"
          // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          />
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </label>
        {/* @ligne Présente un champ de configuration associé à son libellé. */}
        <label>
          Shell
          {/* @ligne Répercute la saisie dans l’état du formulaire. */}
          <select value={shell} onChange={(e) => setShell(e.target.value)}>
            {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
            {["bash", "zsh", "fish", "powershell"].map((s) => (
              // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
              <option key={s}>{s}</option>
            ))}
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </select>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </label>
        {/* @ligne Présente un champ de configuration associé à son libellé. */}
        <label className="check-label">
          {/* @ligne Présente un champ de configuration associé à son libellé. */}
          <input
            // @ligne Choisit le comportement natif du contrôle, notamment le masquage des secrets.
            type="checkbox"
            // @ligne Reflète le choix courant dans la case à cocher.
            checked={favorites}
            // @ligne Répercute la saisie dans l’état du formulaire.
            onChange={(e) => setFavorites(e.target.checked)}
          // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          />
          Favoris
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </label>
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </div>
      {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
      {rows.length ? (
        // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
        <div className="command-list">
          {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
          {rows.map((r) => (
            // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
            <div key={r.id}>
              {/* @ligne Présente une action volontaire, accessible au clavier. */}
              <button
                // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                className="icon-button"
                // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                aria-label={
                  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  r.favorite ? "Retirer des favoris" : "Ajouter aux favoris"
                }
                // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                onClick={async () => {
                  // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
                  try {
                    // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                    await call("favorite", { id: r.id, value: !r.favorite });
                    // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                    await load();
                  // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
                  } catch (e) {
                    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    notify(errorMessage(e));
                  }
                }}
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              >
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <Star size={17} fill={r.favorite ? "currentColor" : "none"} />
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </button>
              {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
              <code>{r.command}</code>
              {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
              <span className="badge">{r.shell}</span>
              {/* @ligne Présente une action volontaire, accessible au clavier. */}
              <button
                // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                className="icon-button"
                // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                aria-label="Copier la commande"
                // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                onClick={() =>
                  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  copy(r.command)
                    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    .then(() => notify("Commande copiée."))
                    // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
                    .catch((e) => notify(errorMessage(e)))
                }
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              >
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <Copy size={17} />
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </button>
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </div>
          ))}
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      ) : (
        // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
        <div className="empty-state">
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <Search size={32} />
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <h3>
            {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
            {search
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              ? "Aucune commande correspondante."
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              : "Vos commandes utiles, au même endroit."}
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </h3>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <p>
            Choisissez un historique Bash, Zsh, Fish ou PowerShell. Le masquage
            réduit les secrets connus ; vérifiez vos commandes avant de les
            partager.
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </p>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
      )}
      {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
      <p className="muted">
        500 résultats maximum par recherche. Les doublons exacts sont ignorés.
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </p>
      {/* @ligne Présente un champ de configuration associé à son libellé. */}
      <label className="check-label">
        {/* @ligne Présente un champ de configuration associé à son libellé. */}
        <input
          // @ligne Choisit le comportement natif du contrôle, notamment le masquage des secrets.
          type="checkbox"
          // @ligne Reflète le choix courant dans la case à cocher.
          checked={watch}
          // @ligne Répercute la saisie dans l’état du formulaire.
          onChange={(e) => setWatch(e.target.checked)}
        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        />
        Suivre automatiquement les historiques que j’importe (toutes les 15
        secondes, pendant l’ouverture de DockForge)
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </label>
      {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
      {sources.map(([path, kind]) => (
        // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
        <div className="image-row" key={path}>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <code>
            {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
            {kind} · {path}
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </code>
          {/* @ligne Présente une action volontaire, accessible au clavier. */}
          <button
            // @ligne Relie ce bouton à une action explicite de l’utilisateur.
            onClick={async () => {
              // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
              try {
                // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                await call("watch_history", {
                  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  path,
                  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
                  shell: kind,
                  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
                  enabled: false,
                });
                // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                await loadSources();
              // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
              } catch (e) {
                // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                notify(errorMessage(e));
              }
            }}
          // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          >
            Arrêter le suivi
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </button>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
      ))}
    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
    </section>
  );
}

// @ligne Délimite cette responsabilité afin de garder le parcours testable.
export function NetworkWorkspace({ notify }: { notify: Notify }) {
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [cidr, setCidr] = useState("192.168.1.0/24");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [consent, setConsent] = useState(false);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [busy, setBusy] = useState(false);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [map, setMap] = useState<{
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    hosts: { ip: string; ports: number[]; evidence: string }[];
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    subnet: string;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    notice: string;
  // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
  } | null>(null);
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const scan = async () => {
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setBusy(true);
    // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
    try {
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      setMap(await call("netmap", { cidr }));
    // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
    } catch (e) {
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      notify(errorMessage(e));
    // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
    } finally {
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setBusy(false);
    }
  };
  // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
  return (
    // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
    <section className="panel">
      {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
      <div className="section-heading">
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <div>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <h2>Comprendre votre réseau local.</h2>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <p className="muted">
            Scan TCP borné, sur un /24 privé que vous choisissez. Aucune requête
            WAN automatique.
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </p>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <Wifi size={30} />
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </div>
      {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
      <div className="toolbar">
        {/* @ligne Présente un champ de configuration associé à son libellé. */}
        <label>
          Sous-réseau
          {/* @ligne Répercute la saisie dans l’état du formulaire. */}
          <input value={cidr} onChange={(e) => setCidr(e.target.value)} />
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </label>
        {/* @ligne Présente un champ de configuration associé à son libellé. */}
        <label className="check-label">
          {/* @ligne Présente un champ de configuration associé à son libellé. */}
          <input
            // @ligne Choisit le comportement natif du contrôle, notamment le masquage des secrets.
            type="checkbox"
            // @ligne Reflète le choix courant dans la case à cocher.
            checked={consent}
            // @ligne Répercute la saisie dans l’état du formulaire.
            onChange={(e) => setConsent(e.target.checked)}
          // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          />
          Je suis autorisé à sonder ce réseau.
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </label>
        {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
        <button className="primary" disabled={!consent || busy} onClick={scan}>
          {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
          {busy ? "Découverte…" : "Découvrir les hôtes"}
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </button>
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </div>
      {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
      {map ? (
        // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
        <>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <p className="hint">{map.notice}</p>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <div className="network-root">
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <Wifi size={22} />
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <strong>{map.subnet}</strong>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <span>Sous-réseau logique</span>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <div className="network-hosts">
            {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
            {map.hosts.map((h) => (
              // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
              <article key={h.ip}>
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <span className="logical-link" />
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <Wifi size={21} />
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <h3>{h.ip}</h3>
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <p>{h.ports.join(" · ")}</p>
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <small>{h.evidence}</small>
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </article>
            ))}
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
          {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
          {!map.hosts.length && (
            // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
            <p className="empty-state">
              Aucun port ciblé ne répond. Cela ne prouve pas que le réseau est
              vide.
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </p>
          )}
          {/* @ligne Présente une action volontaire, accessible au clavier. */}
          <button
            // @ligne Relie ce bouton à une action explicite de l’utilisateur.
            onClick={() => {
              // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
              const blob = new Blob([JSON.stringify(map, null, 2)], {
                // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
                type: "application/json",
              });
              // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
              const url = URL.createObjectURL(blob);
              // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
              const a = document.createElement("a");
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              a.href = url;
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              a.download = "dockforge-netmap.json";
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              a.click();
              // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
              setTimeout(() => URL.revokeObjectURL(url), 1000);
            }}
          // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          >
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <Download size={17} />
            Exporter les observations JSON
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </button>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </>
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      ) : (
        // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
        <div className="empty-state">
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <Wifi size={38} />
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <h3>Chaque observation garde sa preuve.</h3>
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <p>
            La carte n’invente ni équipement silencieux, ni liaison physique.
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </p>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
      )}
    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
    </section>
  );
}

// @ligne Délimite cette responsabilité afin de garder le parcours testable.
export function ShareWorkspace({
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  doctor,
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  notify,
// @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
}: {
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  doctor: Doctor | null;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  notify: Notify;
// @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
}) {
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [files, setFiles] = useState<string[]>([]);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [ip, setIp] = useState("127.0.0.1");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [port, setPort] = useState(9443);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [busy, setBusy] = useState(false);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [room, setRoom] = useState<{
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    url: string;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    pin: string;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    qr_svg: string;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    fingerprint_sha256: string;
  // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
  } | null>(null);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [peers, setPeers] = useState<
    // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    { id: string; name: string; role: string }[]
  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  >([]);
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const load = () =>
    // @ligne Passe par une commande métier native, avec validation côté Rust.
    call<typeof peers>("share_peers")
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      .then(setPeers)
      // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
      .catch(() => undefined);
  // @ligne Déclare un effet lié au cycle de vie ; son nettoyage évite les abonnements persistants.
  useEffect(() => {
    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
    if (!room) return;
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    const timer = setInterval(() => void load(), 4000);
    // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
    return () => clearInterval(timer);
  // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
  }, [room]);
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const start = async () => {
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setBusy(true);
    // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
    try {
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      setRoom(await call("share_start", { paths: files, ip, port }));
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      notify(
        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        "Salle HTTPS ouverte pour 24 heures. Vérifiez le pare-feu et transmettez le PIN séparément.",
      );
    // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
    } catch (e) {
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      notify(errorMessage(e));
    // @ligne Termine cette expression de Workspaces et rattache sa suite au groupe englobant.
    } finally {
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setBusy(false);
    }
  };
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const role = async (id: string, role: string) => {
    // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
    try {
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      await call("share_role", { id, role });
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      await load();
    // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
    } catch (e) {
      // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      notify(errorMessage(e));
    }
  };
  // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
  return (
    // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
    <div className="two-columns">
      {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
      <section className="panel">
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <span className="eyebrow">CLASSSHARE LAN</span>
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <h2>La bonne ressource, à toute la classe.</h2>
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <p className="muted">
          Partage HTTPS sur votre réseau. Vos fichiers restent sur votre
          machine.
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </p>
        {/* @ligne Présente une action volontaire, accessible au clavier. */}
        <button
          // @ligne Relie ce bouton à une action explicite de l’utilisateur.
          onClick={async () => {
            // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
            try {
              // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
              const selected = await chooseFiles();
              // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
              setFiles((v) => [...new Set([...v, ...selected])]);
            // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
            } catch (e) {
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              notify(errorMessage(e));
            }
          }}
        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        >
          {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
          <FileUp size={17} />
          Ajouter des fichiers
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </button>
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <ul className="file-list">
          {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
          {files.map((f) => (
            // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
            <li key={f}>
              {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
              <span>{f}</span>
              {/* @ligne Présente une action volontaire, accessible au clavier. */}
              <button
                // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                className="icon-button"
                // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                aria-label={`Retirer ${f}`}
                // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                onClick={() => setFiles((v) => v.filter((p) => p !== f))}
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              >
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <X size={16} />
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </button>
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </li>
          ))}
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </ul>
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <p className="muted">
          Pour un dossier, ajoutez son archive. Les contributions invitées sont
          limitées à 64 Mio par fichier et 1 Gio par salle.
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </p>
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <div className="form-grid">
          {/* @ligne Présente un champ de configuration associé à son libellé. */}
          <label>
            Interface réseau
            {/* @ligne Répercute la saisie dans l’état du formulaire. */}
            <select value={ip} onChange={(e) => setIp(e.target.value)}>
              {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
              {(doctor?.addresses || ["127.0.0.1"]).map((a) => (
                // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
                <option key={a}>{a}</option>
              ))}
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </select>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </label>
          {/* @ligne Présente un champ de configuration associé à son libellé. */}
          <label>
            Port HTTPS
            {/* @ligne Présente un champ de configuration associé à son libellé. */}
            <input
              // @ligne Choisit le comportement natif du contrôle, notamment le masquage des secrets.
              type="number"
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              min={1}
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              max={65535}
              // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
              value={port}
              // @ligne Répercute la saisie dans l’état du formulaire.
              onChange={(e) => setPort(Number(e.target.value))}
            // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            />
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </label>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
        {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
        {ip === "127.0.0.1" && (
          // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
          <p className="hint">
            127.0.0.1 permet un essai sur ce PC. Choisissez une IP LAN pour
            rejoindre la salle depuis un autre appareil.
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </p>
        )}
        {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
        <div className="button-row">
          {/* @ligne Présente une action volontaire, accessible au clavier. */}
          <button
            // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
            className="primary"
            // @ligne Bloque une action indisponible ou déjà en cours.
            disabled={!files.length || busy || !!room}
            // @ligne Relie ce bouton à une action explicite de l’utilisateur.
            onClick={start}
          // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          >
            Ouvrir la salle
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </button>
          {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
          {room && (
            // @ligne Présente une action volontaire, accessible au clavier.
            <button
              // @ligne Relie ce bouton à une action explicite de l’utilisateur.
              onClick={async () => {
                // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
                try {
                  // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                  await call("share_stop");
                  // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                  setRoom(null);
                  // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                  setPeers([]);
                // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
                } catch (e) {
                  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  notify(errorMessage(e));
                }
              }}
            // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            >
              Fermer la salle
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </button>
          )}
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </section>
      {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
      <aside className="panel">
        {/* @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
        {room ? (
          // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
          <>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <h3>Rejoindre la salle</h3>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <img
              // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
              className="qr"
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              alt="QR code de l’adresse HTTPS de la salle"
              // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              src={`data:image/svg+xml;charset=utf-8,${encodeURIComponent(room.qr_svg)}`}
            // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            />
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <code className="share-url">{room.url}</code>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <p className="pin">{room.pin}</p>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <p className="muted">
              Le QR contient l’adresse, pas le PIN. Le navigateur invité demande
              de valider le certificat local : comparez son empreinte avec
              l’hôte.
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </p>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <details>
              {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
              <summary>Empreinte SHA-256 du certificat</summary>
              {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
              <code className="fingerprint">{room.fingerprint_sha256}</code>
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </details>
            {/* @ligne Présente une action volontaire, accessible au clavier. */}
            <button
              // @ligne Relie ce bouton à une action explicite de l’utilisateur.
              onClick={() =>
                // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                copy(room.url)
                  // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  .then(() => notify("Lien copié."))
                  // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
                  .catch((e) => notify(errorMessage(e)))
              }
            // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            >
              {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
              <Copy size={16} />
              Copier le lien
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </button>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <h3>Participants ({peers.length})</h3>
            {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
            {peers.map((p) => (
              // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
              <div className="peer" key={p.id}>
                {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                <span>{p.name}</span>
                {/* @ligne Présente un champ de configuration associé à son libellé. */}
                <select
                  // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                  aria-label={`Droits de ${p.name}`}
                  // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                  value={p.role}
                  // @ligne Répercute la saisie dans l’état du formulaire.
                  onChange={(e) => void role(p.id, e.target.value)}
                // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                >
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <option value="read">Lecture</option>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <option value="contribute">Contribution</option>
                  {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
                  <option value="expel">Expulser</option>
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </select>
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </div>
            ))}
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </>
        // @ligne Fournit cet argument à l’expression de Workspaces ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        ) : (
          // @ligne Compose cet élément de la vue Workspaces à partir de l’état courant.
          <div className="empty-state">
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <Wifi size={34} />
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <h3>Une salle, un lien, un PIN.</h3>
            {/* @ligne Compose cet élément de la vue Workspaces à partir de l’état courant. */}
            <p>
              Les participants rejoignent depuis leur navigateur. Les droits
              sont contrôlés par l’hôte.
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </p>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
        )}
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </aside>
    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
    </div>
  );
}
