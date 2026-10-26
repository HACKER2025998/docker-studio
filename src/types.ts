// @ligne Définit le contrat partagé entre le moteur et les composants.
export type Template = {
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  id: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  name: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  category: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  description: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  image: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  versions: string[];
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  port: number;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  volume: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  fields: {
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    key: string;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    label: string;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    secret: boolean;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    default: string;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    required: boolean;
  }[];
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  health: string[];
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  notice: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  capabilities: string[];
};
// @ligne Définit le contrat partagé entre le moteur et les composants.
export type Check = { id: string; title: string; ok: boolean; detail: string };
// @ligne Définit le contrat partagé entre le moteur et les composants.
export type Doctor = {
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  os: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  arch: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  distribution: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  ready: boolean;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  checks: Check[];
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  installation: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  addresses: string[];
};
// @ligne Définit le contrat partagé entre le moteur et les composants.
export type Progress = { operation: string; phase: string; message: string };
// @ligne Définit le contrat partagé entre le moteur et les composants.
export type VaultEntry = {
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  id: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  name: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  image: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  archived: boolean;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  created_at: string;
};
// @ligne Définit le contrat partagé entre le moteur et les composants.
export type Container = {
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  Id?: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  Names?: string[];
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  Image?: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  State?: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  Status?: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  Labels?: Record<string, string>;
};
// @ligne Définit le contrat partagé entre le moteur et les composants.
export type Credentials = {
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  container: { name: string; status: string };
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  credentials: {
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    username: string;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    password: string;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    database: string;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    environment: Record<string, string>;
  };
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  network: { bind_ip: string; host_port: number };
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  connection_strings: {
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    uri_standard: string;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    laravel_env: string;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    jdbc_java: string;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    dotnet_ef: string;
    // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
    cli_command: string;
  };
};
// @ligne Définit le contrat partagé entre le moteur et les composants.
export type Recipe = {
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  artifact?: string | null;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  operation: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  tag: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  base: string;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  packages: string[];
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  project: string | null;
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  command: string[];
  // @ligne Renseigne ce champ du contrat avec son type ou sa valeur explicite.
  port: number | null;
};
// @ligne Définit le contrat partagé entre le moteur et les composants.
export type Tab =
  // @ligne Fournit cet argument à l’expression de types ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
  "doctor" | "forge" | "images" | "vault" | "commands" | "network" | "share";
