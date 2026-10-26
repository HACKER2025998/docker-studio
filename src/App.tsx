// @ligne Importe les dépendances explicites utilisées par cet espace.
import { useEffect, useRef, useState } from "react";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import { listen } from "@tauri-apps/api/event";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import { motion } from "framer-motion";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import {
  // @ligne Importe ou référence Activity, repère visuel ou type partagé par cette vue.
  Activity,
  // @ligne Importe ou référence ArrowUpRight, repère visuel ou type partagé par cette vue.
  ArrowUpRight,
  // @ligne Importe ou référence Box, repère visuel ou type partagé par cette vue.
  Box,
  // @ligne Importe ou référence Boxes, repère visuel ou type partagé par cette vue.
  Boxes,
  // @ligne Importe ou référence ChevronRight, repère visuel ou type partagé par cette vue.
  ChevronRight,
  // @ligne Importe ou référence CircleHelp, repère visuel ou type partagé par cette vue.
  CircleHelp,
  // @ligne Importe ou référence Code2, repère visuel ou type partagé par cette vue.
  Code2,
  // @ligne Importe ou référence Container, repère visuel ou type partagé par cette vue.
  Container,
  // @ligne Importe ou référence Database, repère visuel ou type partagé par cette vue.
  Database,
  // @ligne Importe ou référence Hammer, repère visuel ou type partagé par cette vue.
  Hammer,
  // @ligne Importe ou référence Heart, repère visuel ou type partagé par cette vue.
  Heart,
  // @ligne Importe ou référence Layers3, repère visuel ou type partagé par cette vue.
  Layers3,
  // @ligne Importe ou référence LockKeyhole, repère visuel ou type partagé par cette vue.
  LockKeyhole,
  // @ligne Importe ou référence Menu, repère visuel ou type partagé par cette vue.
  Menu,
  // @ligne Importe ou référence Network, repère visuel ou type partagé par cette vue.
  Network,
  // @ligne Importe ou référence Plus, repère visuel ou type partagé par cette vue.
  Plus,
  // @ligne Importe ou référence RefreshCw, repère visuel ou type partagé par cette vue.
  RefreshCw,
  // @ligne Importe ou référence Search, repère visuel ou type partagé par cette vue.
  Search,
  // @ligne Importe ou référence Server, repère visuel ou type partagé par cette vue.
  Server,
  // @ligne Importe ou référence ShieldCheck, repère visuel ou type partagé par cette vue.
  ShieldCheck,
  // @ligne Importe ou référence Sparkles, repère visuel ou type partagé par cette vue.
  Sparkles,
  // @ligne Importe ou référence TerminalSquare, repère visuel ou type partagé par cette vue.
  TerminalSquare,
  // @ligne Importe ou référence Users, repère visuel ou type partagé par cette vue.
  Users,
  // @ligne Importe ou référence X, repère visuel ou type partagé par cette vue.
  X,
// @ligne Termine cette expression de App et rattache sa suite au groupe englobant.
} from "lucide-react";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import { call, catalog, errorMessage, native } from "./api";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import { Forge } from "./Forge";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import {
  // @ligne Importe ou référence CommandWorkspace, repère visuel ou type partagé par cette vue.
  CommandWorkspace,
  // @ligne Importe ou référence DoctorWorkspace, repère visuel ou type partagé par cette vue.
  DoctorWorkspace,
  // @ligne Importe ou référence ImageWorkspace, repère visuel ou type partagé par cette vue.
  ImageWorkspace,
  // @ligne Importe ou référence NetworkWorkspace, repère visuel ou type partagé par cette vue.
  NetworkWorkspace,
  // @ligne Importe ou référence ShareWorkspace, repère visuel ou type partagé par cette vue.
  ShareWorkspace,
  // @ligne Importe ou référence VaultWorkspace, repère visuel ou type partagé par cette vue.
  VaultWorkspace,
// @ligne Termine cette expression de App et rattache sa suite au groupe englobant.
} from "./Workspaces";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import type { Doctor, Progress, Tab, Template } from "./types";

// @ligne Prépare une valeur locale utilisée par les étapes suivantes.
const navigation: {
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  id: Tab;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  title: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  subtitle: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  icon: typeof Box;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  group: string;
// @ligne Termine cette expression de App et rattache sa suite au groupe englobant.
}[] = [
  {
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    id: "doctor",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    title: "Docker Doctor",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    subtitle: "Un environnement en bonne santé",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    icon: Activity,
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    group: "VOTRE ATELIER",
  },
  {
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    id: "forge",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    title: "Container Forge",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    subtitle: "Le prochain service commence ici",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    icon: Container,
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    group: "VOTRE ATELIER",
  },
  {
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    id: "images",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    title: "Image Forge",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    subtitle: "Vos outils, dans votre image",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    icon: Hammer,
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    group: "VOTRE ATELIER",
  },
  {
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    id: "vault",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    title: "Conteneurs & accès",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    subtitle: "Retrouvez vos services et leurs clés",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    icon: LockKeyhole,
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    group: "VOTRE ATELIER",
  },
  {
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    id: "commands",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    title: "Command Vault",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    subtitle: "Votre mémoire du terminal",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    icon: TerminalSquare,
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    group: "VOS OUTILS",
  },
  {
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    id: "network",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    title: "NetMap Studio",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    subtitle: "Observez votre réseau local",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    icon: Network,
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    group: "VOS OUTILS",
  },
  {
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    id: "share",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    title: "ClassShare LAN",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    subtitle: "Partagez sans quitter votre réseau",
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    icon: Users,
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    group: "VOS OUTILS",
  },
];
// @ligne Prépare une valeur locale utilisée par les étapes suivantes.
const categories = [
  // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  "Tout le catalogue",
  // @ligne Construit une entrée de vue pour chaque élément de cette collection.
  ...new Set(catalog.map((t) => t.category)),
];
// @ligne Prépare une valeur locale utilisée par les étapes suivantes.
const icons: Record<string, typeof Box> = {
  // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  "Bases SQL": Database,
  // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  "Bases NoSQL": Layers3,
  // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  "Web & proxy": Code2,
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  Entreprise: Boxes,
  // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  "Sécurité & observabilité": ShieldCheck,
  // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  "IA & données": Sparkles,
  // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  Personnalisé: Box,
};

// @ligne Délimite cette responsabilité afin de garder le parcours testable.
export default function App() {
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [tab, setTab] = useState<Tab>("doctor");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [doctor, setDoctor] = useState<Doctor | null>(null);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [checking, setChecking] = useState(false);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [category, setCategory] = useState("Tout le catalogue");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [search, setSearch] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [selected, setSelected] = useState<Template | null>(null);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [initialImage, setInitialImage] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [progress, setProgress] = useState<Progress[]>([]);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [message, setMessage] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [refreshToken, setRefreshToken] = useState(0);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [menu, setMenu] = useState(false);
  // @ligne Garde une référence stable entre rendus, sans provoquer un rendu supplémentaire.
  const searchRef = useRef<HTMLInputElement>(null);
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const current = navigation.find((n) => n.id === tab)!;
  // @ligne Ne conserve que les éléments répondant au filtre demandé.
  const filtered = catalog.filter(
    // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    (t) =>
      // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      (category === "Tout le catalogue" || t.category === category) &&
      // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      `${t.name} ${t.category} ${t.image}`
        // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        .toLowerCase()
        // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        .includes(search.toLowerCase()),
  );
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const refresh = async () => {
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setChecking(true);
    // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
    try {
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      const result = await call<Doctor>("doctor");
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setDoctor(result);
      // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
      if (result.ready && !doctor?.ready)
        // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
        setMessage(
          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          "Environnement Docker prêt. Votre prochain service vous attend.",
        );
    // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
    } catch (e) {
      // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
      if (native) setMessage(errorMessage(e));
    // @ligne Termine cette expression de App et rattache sa suite au groupe englobant.
    } finally {
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setChecking(false);
    }
  };
  // @ligne Déclare un effet lié au cycle de vie ; son nettoyage évite les abonnements persistants.
  useEffect(() => {
    // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    void refresh();
  // @ligne Termine cette expression de App et rattache sa suite au groupe englobant.
  }, []);
  // @ligne Déclare un effet lié au cycle de vie ; son nettoyage évite les abonnements persistants.
  useEffect(() => {
    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
    if (!native) return;
    // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
    let stopped = false;
    // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
    let unlisten: (() => void) | undefined;
    // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    void listen<Progress>("dockforge:progress", (event) =>
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setProgress((values) => [...values.slice(-199), event.payload]),
    )
      // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      .then((fn) => {
        // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
        if (stopped) fn();
        // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        else unlisten = fn;
      })
      // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
      .catch((e) => setMessage(errorMessage(e)));
    // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
    return () => {
      // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      stopped = true;
      // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      unlisten?.();
    };
  // @ligne Termine cette expression de App et rattache sa suite au groupe englobant.
  }, []);
  // @ligne Déclare un effet lié au cycle de vie ; son nettoyage évite les abonnements persistants.
  useEffect(() => {
    // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
    const key = (e: KeyboardEvent) => {
      // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
        // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        e.preventDefault();
        // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
        setTab("forge");
        // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
        setTimeout(() => searchRef.current?.focus(), 30);
      }
    };
    // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    document.addEventListener("keydown", key);
    // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
    return () => document.removeEventListener("keydown", key);
  // @ligne Termine cette expression de App et rattache sa suite au groupe englobant.
  }, []);
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const launch = (image: string) => {
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setInitialImage(image);
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setSelected(catalog.find((t) => t.id === "custom")!);
  };
  // @ligne Déclare un effet lié au cycle de vie ; son nettoyage évite les abonnements persistants.
  useEffect(() => {
    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
    if (selected) setMessage("");
  // @ligne Termine cette expression de App et rattache sa suite au groupe englobant.
  }, [selected]);
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const close = () => {
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setSelected(null);
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setInitialImage("");
  };

  // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
  return (
    // @ligne Compose cet élément de la vue App à partir de l’état courant.
    <div className={`app-shell ${menu ? "menu-open" : ""}`}>
      {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
      <a className="skip-link" href="#main">
        Aller au contenu
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </a>
      {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
      <aside className="sidebar">
        {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
        <a
          // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
          className="brand"
          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          href="#"
          // @ligne Relie ce bouton à une action explicite de l’utilisateur.
          onClick={(e) => {
            // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            e.preventDefault();
            // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
            setTab("forge");
          }}
        // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        >
          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
          <span className="brand-symbol">
            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
            <Container size={25} strokeWidth={1.8} />
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </span>
          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
          <span>
            DockForge
            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
            <span className="brand-second">
              STUDIO <span>2.0</span>
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </span>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </span>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </a>
        {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
        <div className="workspace-label">
          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
          <span className="avatar">BE</span>
          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
          <div>
            Mon espace local<small>Personnel · sur cet ordinateur</small>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
          <ChevronRight size={14} />
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
        {/* @ligne Expose le sens de ce contrôle aux technologies d’assistance. */}
        <nav aria-label="Navigation principale">
          {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
          {["VOTRE ATELIER", "VOS OUTILS"].map((group) => (
            // @ligne Compose cet élément de la vue App à partir de l’état courant.
            <div className="nav-group" key={group}>
              {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
              <p>{group}</p>
              {/* @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
              {navigation
                // @ligne Ne conserve que les éléments répondant au filtre demandé.
                .filter((n) => n.group === group)
                // @ligne Construit une entrée de vue pour chaque élément de cette collection.
                .map((n) => (
                  // @ligne Présente une action volontaire, accessible au clavier.
                  <button
                    // @ligne Donne à React une identité stable pour conserver correctement cet élément de liste.
                    key={n.id}
                    // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                    className={`nav-item ${tab === n.id ? "active" : ""}`}
                    // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                    aria-current={tab === n.id ? "page" : undefined}
                    // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                    onClick={() => {
                      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                      setTab(n.id);
                      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                      setMenu(false);
                    }}
                  // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  >
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <n.icon size={19} strokeWidth={1.8} />
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <span>{n.title}</span>
                    {/* @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                    {n.id === "forge" && <span className="nav-badge">35</span>}
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </button>
                ))}
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </div>
          ))}
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </nav>
        {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
        <div className="sidebar-bottom">
          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
          <div className="local-card">
            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
            <ShieldCheck size={20} />
            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
            <strong>Votre machine. Vos données.</strong>
            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
            <p>
              Sans compte cloud.
              {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
              <br />
              Sans télémétrie.
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </p>
            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
            <span>
              {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
              <i />
              Local-first
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </span>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
          {/* @ligne Présente une action volontaire, accessible au clavier. */}
          <button
            // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
            className="help-link"
            // @ligne Relie ce bouton à une action explicite de l’utilisateur.
            onClick={() =>
              // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
              setMessage(
                // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                "Le README du projet détaille l’installation, les parcours, la sécurité, le chiffrement et les diagnostics. Pour commencer, ouvrez Docker Doctor.",
              )
            }
          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          >
            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
            <CircleHelp size={17} />
            Un coup de main ?<ArrowUpRight size={15} />
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </button>
          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
          <small className="version">v2.0.0-alpha.1 · Open source MIT</small>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </aside>
      {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
      <div className="app-main">
        {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
        <header className="topbar">
          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
          <div className="breadcrumbs">
            {/* @ligne Présente une action volontaire, accessible au clavier. */}
            <button
              // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
              className="icon-button mobile-menu"
              // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
              aria-label="Ouvrir la navigation"
              // @ligne Relie ce bouton à une action explicite de l’utilisateur.
              onClick={() => setMenu((v) => !v)}
            // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            >
              {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
              <Menu size={20} />
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </button>
            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
            <span>Mon espace</span>
            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
            <ChevronRight size={14} />
            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
            <strong>{current.title}</strong>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
          <div className="topbar-actions">
            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
            <span
              // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
              className={`environment-pill ${doctor?.ready ? "ready" : ""}`}
            // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            >
              {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
              <i />
              {/* @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
              {doctor?.ready
                // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                ? `Docker prêt · ${doctor.os}`
                // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                : native
                  // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  ? "Environnement à vérifier"
                  // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  : "Aperçu navigateur"}
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </span>
            {/* @ligne Présente une action volontaire, accessible au clavier. */}
            <button
              // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
              className="icon-button"
              // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
              aria-label="Rafraîchir l’environnement"
              // @ligne Relie ce bouton à une action explicite de l’utilisateur.
              onClick={() => void refresh()}
              // @ligne Bloque une action indisponible ou déjà en cours.
              disabled={checking}
            // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            >
              {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
              <RefreshCw size={17} className={checking ? "spin" : ""} />
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </button>
            {/* @ligne Présente une action volontaire, accessible au clavier. */}
            <button
              // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
              className="primary compact"
              // @ligne Relie ce bouton à une action explicite de l’utilisateur.
              onClick={() => {
                // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                setTab("forge");
                // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                setCategory("Tout le catalogue");
                // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                setSearch("");
              }}
            // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            >
              {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
              <Plus size={17} />
              Nouveau service
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </button>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </header>
        {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
        <main id="main" tabIndex={-1}>
          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
          <div className="page-title">
            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
            <div>
              {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
              <div className="eyebrow">
                {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                <span /> VOTRE ATELIER DEVOPS, EN LOCAL
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </div>
              {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
              <h1>
                {/* @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                {tab === "forge"
                  // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  ? "Qu’allez-vous créer aujourd’hui ?"
                  // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  : current.title}
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </h1>
              {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
              <p>
                {/* @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                {tab === "forge"
                  // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  ? "Une base de données, un serveur, une idée à tester. On prépare le terrain."
                  // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  : current.subtitle}
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </p>
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </div>
            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
            <div className="page-corner">
              {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
              <span className="outline-icon">
                {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                <current.icon size={26} />
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </span>
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </div>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
          {/* @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
          {!native && (
            // @ligne Compose cet élément de la vue App à partir de l’état courant.
            <div className="browser-notice">
              {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
              <ShieldCheck size={17} />
              {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
              <span>
                Vous explorez l’interface. Les actions système et les secrets
                sont accessibles uniquement dans l’application desktop.
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </span>
              {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
              <button onClick={() => setTab("forge")}>
                Explorer le catalogue
                {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                <ArrowUpRight size={15} />
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </button>
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </div>
          )}
          {/* @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
          {tab === "forge" ? (
            // @ligne Compose cet élément de la vue App à partir de l’état courant.
            <>
              {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
              <section className="forge-hero">
                {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                <div className="hero-copy">
                  {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                  <span className="hero-tag">
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <Sparkles size={14} />
                    MOINS DE CONFIGURATION. PLUS DE CRÉATION.
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </span>
                  {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                  <h2>
                    Votre prochain service,
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <br />
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <em>sans la ligne de commande.</em>
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </h2>
                  {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                  <p>
                    Choisissez une image, personnalisez l’essentiel.
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <br />
                    DockForge conserve vos accès et vous aide si ça coince.
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </p>
                  {/* @ligne Présente une action volontaire, accessible au clavier. */}
                  <button
                    // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                    className="hero-link"
                    // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                    onClick={() => launch("alpine:3.20")}
                  // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  >
                    Partir de ma propre image
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <ArrowUpRight size={16} />
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </button>
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </div>
                {/* @ligne Expose le sens de ce contrôle aux technologies d’assistance. */}
                <div className="hero-visual" aria-hidden="true">
                  {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                  <div className="hero-orbit orbit-one" />
                  {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                  <div className="hero-orbit orbit-two" />
                  {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                  <div className="floating-box box-one">
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <Database size={29} />
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <span>postgres:17</span>
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </div>
                  {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                  <div className="floating-box box-two">
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <Code2 size={26} />
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <span>nginx:stable</span>
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </div>
                  {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                  <div className="floating-box box-three">
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <Layers3 size={25} />
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <span>redis:7</span>
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </div>
                  {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                  <div className="hero-core">
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <Container size={47} strokeWidth={1.5} />
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </div>
                  {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                  <div className="hero-ready">
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <span />
                    Votre prochain projet
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </div>
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </div>
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </section>
              {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
              <section className="catalog-section">
                {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                <div className="section-heading">
                  {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                  <div>
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <h2>
                      Un catalogue pour démarrer sereinement{" "}
                      {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                      <span className="count">{catalog.length}</span>
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </h2>
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <p>
                      Des modèles embarqués, des réglages qui restent les
                      vôtres.
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </p>
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </div>
                  {/* @ligne Présente un champ de configuration associé à son libellé. */}
                  <label className="search-input">
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <Search size={17} />
                    {/* @ligne Présente un champ de configuration associé à son libellé. */}
                    <input
                      // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      ref={searchRef}
                      // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                      value={search}
                      // @ligne Répercute la saisie dans l’état du formulaire.
                      onChange={(e) => setSearch(e.target.value)}
                      // @ligne Fournit un exemple de saisie sans remplacer le libellé accessible.
                      placeholder="Rechercher un service…"
                      // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                      aria-label="Rechercher un service"
                    // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    />
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <kbd>⌘ K</kbd>
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </label>
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </div>
                {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                <div
                  // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                  className="category-tabs"
                  // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                  role="group"
                  // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                  aria-label="Filtrer par catégorie"
                // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                >
                  {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
                  {categories.map((c) => (
                    // @ligne Présente une action volontaire, accessible au clavier.
                    <button
                      // @ligne Donne à React une identité stable pour conserver correctement cet élément de liste.
                      key={c}
                      // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                      aria-pressed={category === c}
                      // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                      className={category === c ? "selected" : ""}
                      // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                      onClick={() => setCategory(c)}
                    // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    >
                      {/* @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                      {c === "Tout le catalogue" && <Boxes size={15} />}
                      {/* @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                      {c}
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </button>
                  ))}
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </div>
                {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                <div className="catalog-grid">
                  {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
                  {filtered.map((t, i) => {
                    // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
                    const Icon = icons[t.category] || Box;
                    // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
                    return (
                      // @ligne Compose cet élément de la vue App à partir de l’état courant.
                      <motion.article
                        // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        initial={{ opacity: 0, y: 8 }}
                        // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        animate={{ opacity: 1, y: 0 }}
                        // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        transition={{
                          // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
                          duration: 0.15,
                          // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
                          delay: Math.min(i * 0.015, 0.12),
                        }}
                        // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                        className={`service-card service-${t.id}`}
                        // @ligne Donne à React une identité stable pour conserver correctement cet élément de liste.
                        key={t.id}
                      // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      >
                        {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                        <div className="card-top">
                          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                          <span
                            // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                            className={`service-icon color-${t.category === "Bases SQL" ? "blue" : t.category === "IA & données" ? "purple" : t.category === "Sécurité & observabilité" ? "orange" : "teal"}`}
                          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                          >
                            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                            <Icon size={26} strokeWidth={1.7} />
                          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                          </span>
                          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                          <span className="category-mini">{t.category}</span>
                        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                        </div>
                        {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                        <h3>{t.name}</h3>
                        {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                        <p>{t.description.replace(`${t.name} — `, "")}</p>
                        {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                        <div className="service-facts">
                          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                          <span>
                            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                            <Server size={12} />
                            {/* @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                            {t.port}/{t.id === "wireguard" ? "udp" : "tcp"}
                          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                          </span>
                          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                          <span>
                            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                            <Layers3 size={12} />
                            {/* @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                            {t.volume ? "Persistance" : "À configurer"}
                          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                          </span>
                        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                        </div>
                        {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                        <div className="card-footer">
                          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                          <code>{t.image}</code>
                          {/* @ligne Présente une action volontaire, accessible au clavier. */}
                          <button
                            // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                            aria-label={`Créer un conteneur ${t.name}`}
                            // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                            onClick={() => {
                              // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                              setInitialImage("");
                              // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                              setSelected(t);
                            }}
                          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                          >
                            Créer
                            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                            <Plus size={15} />
                          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                          </button>
                        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                        </div>
                      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                      </motion.article>
                    );
                  })}
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </div>
                {/* @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                {!filtered.length && (
                  // @ligne Compose cet élément de la vue App à partir de l’état courant.
                  <div className="empty-state">
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <Search size={32} />
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <h3>Aucun service ne correspond.</h3>
                    {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                    <p>
                      Essayez un autre nom ou partez de votre image personnelle.
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </p>
                    {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
                    <button onClick={() => launch("alpine:3.20")}>
                      Configurer une image personnelle
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </button>
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </div>
                )}
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </section>
              {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
              <section className="quiet-footer">
                {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                <LockKeyhole size={17} />
                {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                <p>
                  Vos identifiants ne disparaissent pas avec votre terminal. Ils
                  restent dans votre coffre local chiffré.
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </p>
                {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
                <button onClick={() => setTab("vault")}>
                  Ouvrir le coffre
                  {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
                  <ArrowUpRight size={15} />
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </button>
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </section>
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </>
          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          ) : tab === "doctor" ? (
            // @ligne Compose cet élément de la vue App à partir de l’état courant.
            <DoctorWorkspace
              // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              doctor={doctor}
              // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              loading={checking}
              // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              refresh={() => void refresh()}
              // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              notify={setMessage}
            // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            />
          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          ) : tab === "images" ? (
            // @ligne Compose cet élément de la vue App à partir de l’état courant.
            <ImageWorkspace
              // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              ready={!!doctor?.ready}
              // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              progress={progress}
              // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              launch={launch}
              // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              notify={setMessage}
            // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            />
          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          ) : tab === "vault" ? (
            // @ligne Compose cet élément de la vue App à partir de l’état courant.
            <VaultWorkspace refreshToken={refreshToken} notify={setMessage} />
          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          ) : tab === "commands" ? (
            // @ligne Compose cet élément de la vue App à partir de l’état courant.
            <CommandWorkspace notify={setMessage} />
          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          ) : tab === "network" ? (
            // @ligne Compose cet élément de la vue App à partir de l’état courant.
            <NetworkWorkspace notify={setMessage} />
          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          ) : (
            // @ligne Compose cet élément de la vue App à partir de l’état courant.
            <ShareWorkspace doctor={doctor} notify={setMessage} />
          )}
          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
          <footer className="app-footer">
            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
            <span>Conçu pour apprendre, construire et transmettre.</span>
            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
            <span>
              Fait avec soin <Heart size={12} /> Kossi B. E. ADJOGBLE
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </span>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </footer>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </main>
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </div>
      {/* @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
      {selected && (
        // @ligne Compose cet élément de la vue App à partir de l’état courant.
        <Forge
          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          template={selected}
          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          initialImage={initialImage}
          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          doctor={doctor}
          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          progress={progress}
          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          close={close}
          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          onCreated={() => {
            // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
            setRefreshToken((v) => v + 1);
            // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
            setTab("vault");
          }}
        // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        />
      )}
      {/* @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
      {message && (
        // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
        <div className="toast" role="status">
          {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
          <p>{message}</p>
          {/* @ligne Présente une action volontaire, accessible au clavier. */}
          <button
            // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
            className="icon-button"
            // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
            aria-label="Fermer le message"
            // @ligne Relie ce bouton à une action explicite de l’utilisateur.
            onClick={() => setMessage("")}
          // @ligne Fournit cet argument à l’expression de App ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          >
            {/* @ligne Compose cet élément de la vue App à partir de l’état courant. */}
            <X size={17} />
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </button>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
      )}
    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
    </div>
  );
}
