// @ligne Importe les dépendances explicites utilisées par cet espace.
import { useEffect, useRef, useState } from "react";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import { motion } from "framer-motion";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import {
  // @ligne Importe ou référence X, repère visuel ou type partagé par cette vue.
  X,
  // @ligne Importe ou référence ArrowLeft, repère visuel ou type partagé par cette vue.
  ArrowLeft,
  // @ligne Importe ou référence ArrowRight, repère visuel ou type partagé par cette vue.
  ArrowRight,
  // @ligne Importe ou référence Dices, repère visuel ou type partagé par cette vue.
  Dices,
  // @ligne Importe ou référence Eye, repère visuel ou type partagé par cette vue.
  Eye,
  // @ligne Importe ou référence EyeOff, repère visuel ou type partagé par cette vue.
  EyeOff,
  // @ligne Importe ou référence FolderOpen, repère visuel ou type partagé par cette vue.
  FolderOpen,
  // @ligne Importe ou référence Rocket, repère visuel ou type partagé par cette vue.
  Rocket,
  // @ligne Importe ou référence CircleCheck, repère visuel ou type partagé par cette vue.
  CircleCheck,
  // @ligne Importe ou référence TriangleAlert, repère visuel ou type partagé par cette vue.
  TriangleAlert,
  // @ligne Importe ou référence Copy, repère visuel ou type partagé par cette vue.
  Copy,
  // @ligne Importe ou référence Download, repère visuel ou type partagé par cette vue.
  Download,
// @ligne Termine cette expression de Forge et rattache sa suite au groupe englobant.
} from "lucide-react";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import {
  // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  call,
  // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  chooseFolder,
  // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  errorMessage,
  // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  exportCredentials,
  // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  copy,
// @ligne Termine cette expression de Forge et rattache sa suite au groupe englobant.
} from "./api";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import type { Doctor, Progress, Template, Credentials } from "./types";

// @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
type Props = {
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  template: Template;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  initialImage?: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  doctor: Doctor | null;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  progress: Progress[];
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  close: () => void;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  onCreated: () => void;
};

// @ligne Délimite cette responsabilité afin de garder le parcours testable.
export function Forge({
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  template: t,
  // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  initialImage,
  // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  doctor,
  // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  progress,
  // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  close,
  // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  onCreated,
// @ligne Termine cette expression de Forge et rattache sa suite au groupe englobant.
}: Props) {
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [step, setStep] = useState(0);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [name, setName] = useState(
    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    `${t.id === "custom" ? "mon-service" : t.id}-dev`,
  );
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [image, setImage] = useState(initialImage || t.image);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [env, setEnv] = useState<Record<string, string>>(
    // @ligne Construit une entrée de vue pour chaque élément de cette collection.
    Object.fromEntries(t.fields.map((f) => [f.key, f.default])),
  );
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [extraEnv, setExtraEnv] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [additionalPorts, setAdditionalPorts] = useState("[]");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [databases, setDatabases] = useState<
    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    { id: string; name: string; image: string; archived: boolean }[]
  // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  >([]);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [database, setDatabase] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [show, setShow] = useState(false);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [ip, setIp] = useState("127.0.0.1");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [port, setPort] = useState(t.port);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [targetPort, setTargetPort] = useState(t.port);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [portState, setPortState] = useState<{
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    available: boolean;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    suggested: number;
  // @ligne Termine cette expression de Forge et rattache sa suite au groupe englobant.
  } | null>(null);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [network, setNetwork] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [staticIp, setStaticIp] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [networks, setNetworks] = useState<{ Name: string }[]>([]);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [storage, setStorage] = useState(t.volume ? "volume" : "none");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [path, setPath] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [volumeTarget, setVolumeTarget] = useState(t.volume);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [command, setCommand] = useState("[]");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [gpu, setGpu] = useState(false);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [lan, setLan] = useState(false);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [license, setLicense] = useState(false);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [capabilities, setCapabilities] = useState(false);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [platform, setPlatform] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [busy, setBusy] = useState(false);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [error, setError] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [fix, setFix] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [created, setCreated] = useState<{
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    id: string;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    name: string;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    readiness: string;
  // @ligne Termine cette expression de Forge et rattache sa suite au groupe englobant.
  } | null>(null);
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [operation, setOperation] = useState("");
  // @ligne Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.
  const [notice, setNotice] = useState("");
  // @ligne Garde une référence stable entre rendus, sans provoquer un rendu supplémentaire.
  const dialog = useRef<HTMLDivElement>(null);
  // @ligne Garde une référence stable entre rendus, sans provoquer un rendu supplémentaire.
  const previousFocus = useRef<HTMLElement | null>(null);
  // @ligne Ne conserve que les éléments répondant au filtre demandé.
  const latestProgress = progress.filter((e) => e.operation === operation);

  // @ligne Déclare un effet lié au cycle de vie ; son nettoyage évite les abonnements persistants.
  useEffect(() => {
    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    previousFocus.current = document.activeElement as HTMLElement;
    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    dialog.current?.focus();
    // @ligne Passe par une commande métier native, avec validation côté Rust.
    call<{ Name: string }[]>("networks")
      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      .then(setNetworks)
      // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
      .catch(() => undefined);
    // @ligne Passe par une commande métier native, avec validation côté Rust.
    call<typeof databases>("vault")
      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      .then((rows) =>
        // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
        setDatabases(
          // @ligne Ne conserve que les éléments répondant au filtre demandé.
          rows.filter(
            // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            (r) => !r.archived && /postgres|mysql|mariadb/.test(r.image),
          ),
        ),
      )
      // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
      .catch(() => undefined);
    // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
    return () => previousFocus.current?.focus();
  // @ligne Termine cette expression de Forge et rattache sa suite au groupe englobant.
  }, []);
  // @ligne Déclare un effet lié au cycle de vie ; son nettoyage évite les abonnements persistants.
  useEffect(() => {
    // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
    let alive = true;
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setPortState(null);
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    const timer = setTimeout(() => {
      // @ligne Passe par une commande métier native, avec validation côté Rust.
      call<{ available: boolean; suggested: number }>("port", {
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        ip,
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        port,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        protocol: t.id === "wireguard" ? "udp" : "tcp",
      })
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        .then((r) => {
          // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
          if (alive) setPortState(r);
        })
        // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
        .catch(() => undefined);
    // @ligne Termine cette expression de Forge et rattache sa suite au groupe englobant.
    }, 350);
    // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
    return () => {
      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      alive = false;
      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      clearTimeout(timer);
    };
  // @ligne Termine cette expression de Forge et rattache sa suite au groupe englobant.
  }, [ip, port, t.id]);

  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const run = async (action: () => Promise<unknown>) => {
    // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
    try {
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setNotice("");
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      await action();
    // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
    } catch (e) {
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setError(errorMessage(e));
    }
  };
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const generate = (key: string) =>
    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    run(async () => {
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      const value = await call<string>("generate_password");
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setEnv((v) => ({ ...v, [key]: value }));
    });
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const proceed = () => {
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setError("");
    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
    if (!name.trim() || !image.trim())
      // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
      return setError("Donnez un nom au conteneur et choisissez son image.");
    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
    if (step === 0 && t.fields.some((f) => f.required && !env[f.key]?.trim()))
      // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
      return setError("Complétez tous les champs obligatoires.");
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setStep((s) => Math.min(2, s + 1));
  };
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const submit = async () => {
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setError("");
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setBusy(true);
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setFix("");
    // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
    const op = crypto.randomUUID();
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setOperation(op);
    // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
    try {
      // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
      const extra: Record<string, string> = {};
      // @ligne Ne conserve que les éléments répondant au filtre demandé.
      for (const line of extraEnv.split("\n").filter((s) => s.trim())) {
        // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
        const pos = line.indexOf("=");
        // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
        if (pos < 1)
          // @ligne Signale une entrée ou une réponse invalide avant de poursuivre.
          throw new Error(
            // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            "Variables supplémentaires : utilisez NOM=valeur, une par ligne.",
          );
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        extra[line.slice(0, pos)] = line.slice(pos + 1);
      }
      // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
      const args: unknown = JSON.parse(command);
      // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
      if (!Array.isArray(args) || args.some((a) => typeof a !== "string"))
        // @ligne Signale une entrée ou une réponse invalide avant de poursuivre.
        throw new Error("La commande doit être un tableau JSON de chaînes.");
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      const result = await call<{
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        id: string;
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        name: string;
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        readiness: string;
      // @ligne Termine cette expression de Forge et rattache sa suite au groupe englobant.
      }>("create", {
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        operation: op,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        template: t.id,
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        name,
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        image,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        bind_ip: ip,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        host_port: port,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        container_port: targetPort,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        env: { ...env, ...extra },
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        storage,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        bind_path: path || null,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        volume_target: volumeTarget,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        network: network || null,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        static_ip: staticIp || null,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        command: args,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        additional_ports: JSON.parse(additionalPorts),
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        gpu,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        acknowledge_lan: lan,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        acknowledge_license: license,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        acknowledge_capabilities: capabilities,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        platform: platform || null,
      });
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setCreated(result);
      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      onCreated();
    // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
    } catch (e) {
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setError(errorMessage(e));
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setFix((e as { fix?: string }).fix || "");
    // @ligne Termine cette expression de Forge et rattache sa suite au groupe englobant.
    } finally {
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setBusy(false);
    }
  };
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const linkDatabase = () =>
    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    run(async () => {
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      const linked = await call<{
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        env: Record<string, string>;
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        network: string | null;
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        notice: string;
      // @ligne Termine cette expression de Forge et rattache sa suite au groupe englobant.
      }>("link_database", { source: database, target: t.id });
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setEnv((values) => ({ ...values, ...linked.env }));
      // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
      const extra = Object.entries(linked.env)
        // @ligne Ne conserve que les éléments répondant au filtre demandé.
        .filter(([key]) => !t.fields.some((f) => f.key === key))
        // @ligne Construit une entrée de vue pour chaque élément de cette collection.
        .map(([key, value]) => `${key}=${value}`)
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        .join("\n");
      // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
      if (extra)
        // @ligne Ne conserve que les éléments répondant au filtre demandé.
        setExtraEnv((current) => [current, extra].filter(Boolean).join("\n"));
      // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
      if (linked.network) setNetwork(linked.network);
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setNotice(linked.notice);
    });
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const repair = async () => {
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setError("");
    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
    if (fix === "port") {
      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
      const check = await call<{ suggested: number }>("port", {
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        ip,
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        port,
        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
        protocol: t.id === "wireguard" ? "udp" : "tcp",
      });
      // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
      if (check.suggested) setPort(check.suggested);
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setStep(1);
    }
    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
    if (fix === "rename") {
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setName((n) => `${n.slice(0, 55)}-${Math.floor(Math.random() * 1000)}`);
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setStep(0);
    }
    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
    if (fix === "volume") {
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setStorage("volume");
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setStep(2);
    }
    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
    if (fix === "platform") {
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setPlatform("linux/amd64");
      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
      setStep(2);
    }
    // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
    setNotice(
      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      "Configuration ajustée. Vérifiez-la avant de relancer ; aucun ancien conteneur ou volume n’a été supprimé.",
    );
  };
  // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
  const trap = (event: React.KeyboardEvent) => {
    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
    if (event.key === "Escape" && !busy) close();
    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
    if (event.key !== "Tab") return;
    // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
    const nodes = Array.from(
      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      dialog.current?.querySelectorAll<HTMLElement>(
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        'button:not(:disabled), input:not(:disabled), select, textarea, [tabindex="0"]',
      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      ) || [],
    );
    // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
    const first = nodes[0],
      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      last = nodes[nodes.length - 1];
    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
    if (event.shiftKey && document.activeElement === first) {
      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      event.preventDefault();
      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      last?.focus();
    // @ligne Termine cette expression de Forge et rattache sa suite au groupe englobant.
    } else if (!event.shiftKey && document.activeElement === last) {
      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      event.preventDefault();
      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      first?.focus();
    }
  };

  // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
  return (
    // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
    <div className="modal-backdrop">
      {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
      <motion.div
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        initial={{ opacity: 0, x: 30 }}
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        animate={{ opacity: 1, x: 0 }}
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        transition={{ duration: 0.18 }}
        // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
        className="forge-dialog"
        // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
        role="dialog"
        // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
        aria-modal="true"
        // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
        aria-labelledby="forge-title"
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        tabIndex={-1}
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        ref={dialog}
        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        onKeyDown={trap}
      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
      >
        {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
        <header className="dialog-header">
          {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
          <div>
            {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
            <span className="eyebrow">CONTAINER FORGE</span>
            {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
            <h2 id="forge-title">Votre espace {t.name}</h2>
            {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
            <p>Une configuration claire, des accès que vous retrouverez.</p>
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </div>
          {/* @ligne Présente une action volontaire, accessible au clavier. */}
          <button
            // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
            className="icon-button"
            // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
            aria-label="Fermer le formulaire"
            // @ligne Relie ce bouton à une action explicite de l’utilisateur.
            onClick={close}
            // @ligne Bloque une action indisponible ou déjà en cours.
            disabled={busy}
          // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          >
            {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
            <X size={20} />
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </button>
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </header>
        {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
        {!created && (
          // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
          <ol className="steps">
            {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
            {["Identité & accès", "Réseau", "Stockage & lancement"].map(
              // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              (label, n) => (
                // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
                <li
                  // @ligne Donne à React une identité stable pour conserver correctement cet élément de liste.
                  key={label}
                  // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                  aria-current={step === n ? "step" : undefined}
                  // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                  className={n <= step ? "active" : ""}
                // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                >
                  {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                  <span>{n + 1}</span>
                  {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                  {label}
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </li>
              ),
            )}
          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
          </ol>
        )}
        {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
        <div className="dialog-body">
          {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
          {t.notice && !created && (
            // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
            <div className="hint">
              {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
              <TriangleAlert size={18} />
              {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
              <p>{t.notice}</p>
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </div>
          )}
          {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
          {created ? (
            // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
            <section className="success-panel">
              {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
              <CircleCheck size={44} />
              {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
              <h3>{created.name} a démarré</h3>
              {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
              <p>
                {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                {created.readiness === "healthy"
                  // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  ? "Sa sonde de santé confirme que le service répond."
                  // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  : created.readiness === "health_pending"
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    ? "La sonde est encore en attente. Consultez les logs avant de vous connecter."
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    : "Cette image ne fournit pas de sonde. Le processus tourne ; vérifiez le service avant usage."}
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </p>
              {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
              <p>Les accès sont conservés dans votre coffre chiffré.</p>
              {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
              <div className="button-row">
                {/* @ligne Présente une action volontaire, accessible au clavier. */}
                <button
                  // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                  className="primary"
                  // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                  onClick={() =>
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    run(async () => {
                      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                      if (await exportCredentials(created.id, created.name))
                        // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                        setNotice(
                          // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                          "Fichier JSON exporté. Il contient des secrets en clair : conservez-le dans un emplacement protégé.",
                        );
                    })
                  }
                // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                >
                  {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                  <Download size={17} />
                  Exporter le JSON
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </button>
                {/* @ligne Présente une action volontaire, accessible au clavier. */}
                <button
                  // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                  onClick={() =>
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    run(async () => {
                      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                      const doc = await call<Credentials>("credentials", {
                        // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
                        id: created.id,
                      });
                      // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                      await copy(doc.connection_strings.uri_standard);
                      // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                      setNotice("URI copiée dans le presse-papier.");
                    })
                  }
                // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                >
                  {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                  <Copy size={17} />
                  Copier l’URI
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </button>
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </div>
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </section>
          // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          ) : (
            // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
            <>
              {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
              {step === 0 && (
                // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
                <div className="form-grid">
                  {[
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    "glpi",
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    "wordpress",
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    "odoo",
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    "nextcloud",
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    "zabbix",
                  // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                  ].includes(t.id) && (
                    // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
                    <div className="full panel">
                      {/* @ligne Présente un champ de configuration associé à son libellé. */}
                      <label>
                        Lier une base déjà gérée par DockForge
                        {/* @ligne Présente un champ de configuration associé à son libellé. */}
                        <select
                          // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                          value={database}
                          // @ligne Répercute la saisie dans l’état du formulaire.
                          onChange={(e) => setDatabase(e.target.value)}
                        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        >
                          {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                          <option value="">Choisir une base…</option>
                          {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
                          {databases.map((d) => (
                            // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
                            <option value={d.id} key={d.id}>
                              {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                              {d.name} · {d.image}
                            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                            </option>
                          ))}
                        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                        </select>
                      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                      </label>
                      {/* @ligne Relie ce bouton à une action explicite de l’utilisateur. */}
                      <button disabled={!database} onClick={linkDatabase}>
                        Appliquer les accès et le réseau de cette base
                      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                      </button>
                      {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                      <p className="muted">
                        Cette action déverrouille les accès de la base choisie
                        et remplit les champs. Elle ne crée pas d’utilisateur
                        SQL supplémentaire.
                      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                      </p>
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </div>
                  )}

                  {/* @ligne Présente un champ de configuration associé à son libellé. */}
                  <label>
                    Nom du conteneur
                    {/* @ligne Présente un champ de configuration associé à son libellé. */}
                    <input
                      // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                      value={name}
                      // @ligne Répercute la saisie dans l’état du formulaire.
                      onChange={(e) => setName(e.target.value)}
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      autoComplete="off"
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      required
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      maxLength={63}
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    />
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </label>
                  {/* @ligne Présente un champ de configuration associé à son libellé. */}
                  <label>
                    Image & version
                    {/* @ligne Présente un champ de configuration associé à son libellé. */}
                    <input
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      list="image-versions"
                      // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                      value={image}
                      // @ligne Répercute la saisie dans l’état du formulaire.
                      onChange={(e) => setImage(e.target.value)}
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      required
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    />
                    {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                    <datalist id="image-versions">
                      {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
                      {t.versions.map((v) => (
                        // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
                        <option key={v} value={v} />
                      ))}
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </datalist>
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </label>
                  {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
                  {t.fields.map((f) => (
                    // @ligne Présente un champ de configuration associé à son libellé.
                    <label key={f.key}>
                      {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                      {f.label}
                      {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                      {f.secret ? (
                        // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
                        <div className="input-with-actions">
                          {/* @ligne Présente un champ de configuration associé à son libellé. */}
                          <input
                            // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                            aria-label={f.label}
                            // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                            autoComplete="new-password"
                            // @ligne Choisit le comportement natif du contrôle, notamment le masquage des secrets.
                            type={show ? "text" : "password"}
                            // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                            value={env[f.key] || ""}
                            // @ligne Répercute la saisie dans l’état du formulaire.
                            onChange={(e) =>
                              // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                              setEnv((v) => ({ ...v, [f.key]: e.target.value }))
                            }
                            // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                            required={f.required}
                          // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                          />
                          {/* @ligne Présente une action volontaire, accessible au clavier. */}
                          <button
                            // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                            className="icon-button"
                            // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                            title="Afficher ou masquer"
                            // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                            aria-label={`Afficher ou masquer ${f.label}`}
                            // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                            onClick={() => setShow((v) => !v)}
                          // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                          >
                            {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                            {show ? <EyeOff size={17} /> : <Eye size={17} />}
                          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                          </button>
                          {/* @ligne Présente une action volontaire, accessible au clavier. */}
                          <button
                            // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                            className="icon-button"
                            // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                            title="Générer 24 caractères"
                            // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                            aria-label={`Générer ${f.label}`}
                            // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                            onClick={() => generate(f.key)}
                          // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                          >
                            {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                            <Dices size={17} />
                          {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                          </button>
                        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                        </div>
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      ) : (
                        // @ligne Présente un champ de configuration associé à son libellé.
                        <input
                          // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                          value={env[f.key] || ""}
                          // @ligne Répercute la saisie dans l’état du formulaire.
                          onChange={(e) =>
                            // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                            setEnv((v) => ({ ...v, [f.key]: e.target.value }))
                          }
                          // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                          required={f.required}
                        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        />
                      )}
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </label>
                  ))}
                  {/* @ligne Présente un champ de configuration associé à son libellé. */}
                  <label className="full">
                    Variables supplémentaires (facultatif)
                    {/* @ligne Présente un champ de configuration associé à son libellé. */}
                    <textarea
                      // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                      value={extraEnv}
                      // @ligne Répercute la saisie dans l’état du formulaire.
                      onChange={(e) => setExtraEnv(e.target.value)}
                      // @ligne Fournit un exemple de saisie sans remplacer le libellé accessible.
                      placeholder={"NOM=valeur\nUNE_AUTRE=valeur"}
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      rows={3}
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    />
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </label>
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </div>
              )}
              {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
              {step === 1 && (
                // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
                <div className="form-grid">
                  {/* @ligne Présente un champ de configuration associé à son libellé. */}
                  <label>
                    Adresse d’écoute
                    {/* @ligne Présente un champ de configuration associé à son libellé. */}
                    <select
                      // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                      value={ip}
                      // @ligne Répercute la saisie dans l’état du formulaire.
                      onChange={(e) => {
                        // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                        setIp(e.target.value);
                        // @ligne Met à jour l’état concerné pour refléter le résultat dans l’interface.
                        setLan(false);
                      }}
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    >
                      {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
                      {(doctor?.addresses || ["127.0.0.1"]).map((a) => (
                        // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
                        <option key={a}>{a}</option>
                      ))}
                      {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                      <option value="0.0.0.0">
                        0.0.0.0 — toutes les interfaces
                      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                      </option>
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </select>
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </label>
                  {/* @ligne Présente un champ de configuration associé à son libellé. */}
                  <label>
                    Port de votre ordinateur
                    {/* @ligne Présente un champ de configuration associé à son libellé. */}
                    <input
                      // @ligne Choisit le comportement natif du contrôle, notamment le masquage des secrets.
                      type="number"
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      min={1}
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      max={65535}
                      // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                      value={port}
                      // @ligne Répercute la saisie dans l’état du formulaire.
                      onChange={(e) => setPort(Number(e.target.value))}
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    />
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </label>
                  {/* @ligne Présente un champ de configuration associé à son libellé. */}
                  <label>
                    Port interne du conteneur
                    {/* @ligne Présente un champ de configuration associé à son libellé. */}
                    <input
                      // @ligne Choisit le comportement natif du contrôle, notamment le masquage des secrets.
                      type="number"
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      min={1}
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      max={65535}
                      // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                      value={targetPort}
                      // @ligne Répercute la saisie dans l’état du formulaire.
                      onChange={(e) => setTargetPort(Number(e.target.value))}
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    />
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </label>
                  {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                  {portState && (
                    // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
                    <div
                      // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                      className={`port-status ${portState.available ? "good" : "warn"}`}
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    >
                      {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                      {portState.available ? (
                        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        "Port disponible au moment du contrôle"
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      ) : (
                        // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
                        <>
                          Port occupé.{" "}
                          {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                          {portState.suggested > 0 && (
                            // @ligne Présente une action volontaire, accessible au clavier.
                            <button
                              // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                              onClick={() => setPort(portState.suggested)}
                            // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                            >
                              Utiliser {portState.suggested}
                            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                            </button>
                          )}
                        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                        </>
                      )}
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </div>
                  )}
                  {/* @ligne Présente un champ de configuration associé à son libellé. */}
                  <label>
                    Réseau Docker existant
                    {/* @ligne Présente un champ de configuration associé à son libellé. */}
                    <select
                      // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                      value={network}
                      // @ligne Répercute la saisie dans l’état du formulaire.
                      onChange={(e) => setNetwork(e.target.value)}
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    >
                      {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                      <option value="">Bridge par défaut</option>
                      {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                      {networks
                        // @ligne Ne conserve que les éléments répondant au filtre demandé.
                        .filter((n) => !["host", "none"].includes(n.Name))
                        // @ligne Construit une entrée de vue pour chaque élément de cette collection.
                        .map((n) => (
                          // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
                          <option key={n.Name}>{n.Name}</option>
                        ))}
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </select>
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </label>
                  {/* @ligne Présente un champ de configuration associé à son libellé. */}
                  <label>
                    IPv4 statique interne (facultatif)
                    {/* @ligne Présente un champ de configuration associé à son libellé. */}
                    <input
                      // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                      value={staticIp}
                      // @ligne Répercute la saisie dans l’état du formulaire.
                      onChange={(e) => setStaticIp(e.target.value)}
                      // @ligne Fournit un exemple de saisie sans remplacer le libellé accessible.
                      placeholder="172.28.0.10"
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    />
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </label>
                  {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                  <p className="muted full">
                    Une IP Docker interne n’est pas une IP LAN. Le réseau choisi
                    doit déjà avoir un sous-réseau compatible. Le contrôle du
                    port reste indicatif : Docker valide le binding final.
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </p>
                  {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                  {ip !== "127.0.0.1" && (
                    // @ligne Présente un champ de configuration associé à son libellé.
                    <label className="check-label full">
                      {/* @ligne Présente un champ de configuration associé à son libellé. */}
                      <input
                        // @ligne Choisit le comportement natif du contrôle, notamment le masquage des secrets.
                        type="checkbox"
                        // @ligne Reflète le choix courant dans la case à cocher.
                        checked={lan}
                        // @ligne Répercute la saisie dans l’état du formulaire.
                        onChange={(e) => setLan(e.target.checked)}
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      />
                      J’autorise cette exposition réseau et j’ai vérifié
                      l’authentification du service.
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </label>
                  )}
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </div>
              )}
              {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
              {step === 2 && (
                // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
                <div className="form-grid">
                  {/* @ligne Présente un champ de configuration associé à son libellé. */}
                  <label>
                    Persistance
                    {/* @ligne Présente un champ de configuration associé à son libellé. */}
                    <select
                      // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                      value={storage}
                      // @ligne Répercute la saisie dans l’état du formulaire.
                      onChange={(e) => setStorage(e.target.value)}
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    >
                      {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                      <option value="volume">Nouveau volume Docker géré</option>
                      {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                      <option value="bind">Dossier de mon ordinateur</option>
                      {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                      <option value="none">Aucun montage supplémentaire</option>
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </select>
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </label>
                  {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                  {storage !== "none" && (
                    // @ligne Présente un champ de configuration associé à son libellé.
                    <label>
                      Chemin dans le conteneur
                      {/* @ligne Présente un champ de configuration associé à son libellé. */}
                      <input
                        // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                        value={volumeTarget}
                        // @ligne Répercute la saisie dans l’état du formulaire.
                        onChange={(e) => setVolumeTarget(e.target.value)}
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      />
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </label>
                  )}
                  {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                  {storage === "bind" && (
                    // @ligne Présente un champ de configuration associé à son libellé.
                    <label className="full">
                      Dossier local
                      {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                      <div className="input-with-actions">
                        {/* @ligne Présente un champ de configuration associé à son libellé. */}
                        <input
                          // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                          value={path}
                          // @ligne Répercute la saisie dans l’état du formulaire.
                          onChange={(e) => setPath(e.target.value)}
                        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        />
                        {/* @ligne Présente une action volontaire, accessible au clavier. */}
                        <button
                          // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
                          aria-label="Choisir le dossier de stockage"
                          // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                          onClick={() =>
                            // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                            run(async () => {
                              // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
                              const selected = await chooseFolder();
                              // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
                              if (selected) setPath(selected);
                            })
                          }
                        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        >
                          {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                          <FolderOpen size={18} />
                        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                        </button>
                      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                      </div>
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </label>
                  )}
                  {/* @ligne Présente un champ de configuration associé à son libellé. */}
                  <label className="full">
                    Ports supplémentaires (tableau JSON)
                    {/* @ligne Présente un champ de configuration associé à son libellé. */}
                    <textarea
                      // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                      value={additionalPorts}
                      // @ligne Répercute la saisie dans l’état du formulaire.
                      onChange={(e) => setAdditionalPorts(e.target.value)}
                      // @ligne Fournit un exemple de saisie sans remplacer le libellé accessible.
                      placeholder={
                        // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                        '[{"host":8443,"container":443,"protocol":"tcp"}]'
                      }
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      rows={3}
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    />
                    {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                    <small>
                      Exemple : 8443 vers 443/TCP ; 16 publications
                      supplémentaires maximum.
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </small>
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </label>
                  {/* @ligne Présente un champ de configuration associé à son libellé. */}
                  <label className="full">
                    Commande (tableau JSON, [] conserve celle de l’image)
                    {/* @ligne Présente un champ de configuration associé à son libellé. */}
                    <textarea
                      // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                      value={command}
                      // @ligne Répercute la saisie dans l’état du formulaire.
                      onChange={(e) => setCommand(e.target.value)}
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      rows={2}
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    />
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </label>
                  {/* @ligne Présente un champ de configuration associé à son libellé. */}
                  <label>
                    Plateforme
                    {/* @ligne Présente un champ de configuration associé à son libellé. */}
                    <select
                      // @ligne Affiche la valeur courante de l’état, qui reste la source de vérité du champ.
                      value={platform}
                      // @ligne Répercute la saisie dans l’état du formulaire.
                      onChange={(e) => setPlatform(e.target.value)}
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    >
                      {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                      <option value="">Architecture native</option>
                      {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                      <option value="linux/amd64">
                        linux/amd64 — émulation si nécessaire
                      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                      </option>
                      {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                      <option value="linux/arm64">linux/arm64</option>
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
                      checked={gpu}
                      // @ligne Répercute la saisie dans l’état du formulaire.
                      onChange={(e) => setGpu(e.target.checked)}
                    // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                    />
                    Donner accès au GPU (pilote requis)
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </label>
                  {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                  {t.id === "mssql" && (
                    // @ligne Présente un champ de configuration associé à son libellé.
                    <label className="check-label full">
                      {/* @ligne Présente un champ de configuration associé à son libellé. */}
                      <input
                        // @ligne Choisit le comportement natif du contrôle, notamment le masquage des secrets.
                        type="checkbox"
                        // @ligne Reflète le choix courant dans la case à cocher.
                        checked={license}
                        // @ligne Répercute la saisie dans l’état du formulaire.
                        onChange={(e) => setLicense(e.target.checked)}
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      />
                      J’accepte les conditions de licence Microsoft SQL Server.
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </label>
                  )}
                  {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                  {t.capabilities.length > 0 && (
                    // @ligne Présente un champ de configuration associé à son libellé.
                    <label className="check-label full">
                      {/* @ligne Présente un champ de configuration associé à son libellé. */}
                      <input
                        // @ligne Choisit le comportement natif du contrôle, notamment le masquage des secrets.
                        type="checkbox"
                        // @ligne Reflète le choix courant dans la case à cocher.
                        checked={capabilities}
                        // @ligne Répercute la saisie dans l’état du formulaire.
                        onChange={(e) => setCapabilities(e.target.checked)}
                      // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                      />
                      J’autorise les capacités supplémentaires :{" "}
                      {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                      {t.capabilities.join(", ")}.
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </label>
                  )}
                  {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                  <div className="summary-box full">
                    {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                    <strong>{name}</strong>
                    {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                    <code>{image}</code>
                    {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                    <span>
                      {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                      {ip}:{port} → {targetPort}/
                      {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                      {t.id === "wireguard" ? "udp" : "tcp"}
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </span>
                    {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                    <span>
                      Les volumes ne sont jamais effacés automatiquement.
                    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                    </span>
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </div>
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </div>
              )}
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </>
          )}
          {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
          {busy && (
            // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
            <div
              // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
              className="operation-log"
              // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
              role="log"
              // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
              aria-label="Progression de la création"
            // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            >
              {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
              <strong>Docker prépare votre conteneur…</strong>
              {/* @ligne Construit une entrée de vue pour chaque élément de cette collection. */}
              {latestProgress.slice(-12).map((p, n) => (
                // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
                <p key={n}>
                  {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                  <span>{p.phase}</span>
                  {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                  {p.message}
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </p>
              ))}
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </div>
          )}
          {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
          {error && (
            // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
            <div role="alert" className="alert error">
              {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
              <TriangleAlert size={18} />
              {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
              <div>
                {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                <p>{error}</p>
                {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                {["port", "rename", "volume", "platform"].includes(fix) && (
                  // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                  <button onClick={() => run(repair)}>
                    Ajuster la configuration
                  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                  </button>
                )}
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </div>
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </div>
          )}
          {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
          {notice && (
            // @ligne Expose le sens de ce contrôle aux technologies d’assistance.
            <p role="status" className="hint">
              {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
              {notice}
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </p>
          )}
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </div>
        {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
        <footer className="dialog-footer">
          {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
          {created ? (
            // @ligne Relie ce bouton à une action explicite de l’utilisateur.
            <button className="primary" onClick={close}>
              Retrouver mes conteneurs
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </button>
          // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
          ) : (
            // @ligne Compose cet élément de la vue Forge à partir de l’état courant.
            <>
              {/* @ligne Présente une action volontaire, accessible au clavier. */}
              <button
                // @ligne Bloque une action indisponible ou déjà en cours.
                disabled={busy || step === 0}
                // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                onClick={() => setStep((s) => s - 1)}
              // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              >
                {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                <ArrowLeft size={16} />
                Retour
              {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
              </button>
              {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
              <span>{step + 1} / 3</span>
              {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
              {step < 2 ? (
                // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                <button className="primary" onClick={proceed}>
                  Continuer
                  {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                  <ArrowRight size={16} />
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </button>
              // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
              ) : (
                // @ligne Présente une action volontaire, accessible au clavier.
                <button
                  // @ligne Associe les styles Daylight à cet élément sans modifier ses données.
                  className="primary"
                  // @ligne Bloque une action indisponible ou déjà en cours.
                  disabled={busy || !doctor?.ready}
                  // @ligne Relie ce bouton à une action explicite de l’utilisateur.
                  onClick={submit}
                // @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
                >
                  {/* @ligne Compose cet élément de la vue Forge à partir de l’état courant. */}
                  <Rocket size={17} />
                  {/* @ligne Fournit cet argument à l’expression de Forge ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus. */}
                  {busy ? "Création en cours…" : "Créer & lancer"}
                {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
                </button>
              )}
            {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
            </>
          )}
        {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
        </footer>
      {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
      </motion.div>
    {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
    </div>
  );
}
