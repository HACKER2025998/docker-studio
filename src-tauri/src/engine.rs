//! Provisionnement transactionnel : le coffre est écrit avant le démarrage du service.
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use crate::{Reporter, catalog, doctor, report, store::Store};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use anyhow::{Context, Result, bail};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use bollard::{
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    models::{
        // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        ContainerCreateBody as Config, DeviceRequest, HealthConfig, HostConfig, Mount,
        // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        MountTypeEnum, NetworkingConfig, PortBinding, VolumeCreateOptions,
    },
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    query_parameters::{
        // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        CreateContainerOptionsBuilder, CreateImageOptionsBuilder, ListContainersOptionsBuilder,
        // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        LogsOptionsBuilder, RemoveContainerOptionsBuilder,
    },
};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use futures_util::StreamExt;
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use serde::{Deserialize, Serialize};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use serde_json::{Value, json};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use std::{collections::HashMap, sync::Arc, time::Duration};

// @ligne [module] Applique cet attribut au type ou à la fonction qui suit.
#[derive(Clone, Serialize, Deserialize, Debug)]
// @ligne [module] Définit les données échangées par ce module, sans comportement implicite.
pub struct CreateRequest {
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub operation: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub template: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub name: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub image: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub bind_ip: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub host_port: u16,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub container_port: u16,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub env: HashMap<String, String>,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub storage: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub bind_path: Option<String>,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub volume_target: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub network: Option<String>,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub static_ip: Option<String>,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub command: Vec<String>,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub gpu: bool,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub acknowledge_lan: bool,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub acknowledge_license: bool,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub acknowledge_capabilities: bool,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub platform: Option<String>,
    // @ligne [module] Applique cet attribut au type ou à la fonction qui suit.
    #[serde(default)]
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub additional_ports: Vec<PortSpec>,
}

// @ligne [module] Applique cet attribut au type ou à la fonction qui suit.
#[derive(Clone, Serialize, Deserialize, Debug)]
// @ligne [module] Définit les données échangées par ce module, sans comportement implicite.
pub struct PortSpec {
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub host: u16,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub container: u16,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub protocol: String,
}

// @ligne [scrub_environment] Délimite cette opération pour isoler sa validation et son résultat.
pub fn scrub_environment(text: &str, env: &HashMap<String, String>) -> String {
    // @ligne [scrub_environment] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut output = crate::suite::redact(text);
    // @ligne [scrub_environment] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let secret = regex::Regex::new("(?i)password|passwd|token|secret|api.?key").unwrap();
    // @ligne [scrub_environment] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
    for (key, value) in env {
        // @ligne [scrub_environment] Vérifie cette condition avant d’appliquer l’action protégée.
        if secret.is_match(key) && !value.is_empty() {
            // @ligne [scrub_environment] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            output = output.replace(value, "[MASQUÉ]");
        }
    }
    // @ligne [scrub_environment] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    output
}

// @ligne [validate] Délimite cette opération pour isoler sa validation et son résultat.
pub fn validate(r: &CreateRequest) -> Result<()> {
    // @ligne [validate] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let valid_name = regex::Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_.-]{0,62}$")?;
    // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
    if !valid_name.is_match(&r.name) {
        // @ligne [validate] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Nom : 1 à 63 caractères, lettres, chiffres, point, tiret ou soulignement");
    }
    // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
    if r.image.is_empty()
        // @ligne [validate] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        || r.image.len() > 255
        // @ligne [validate] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        || r.image.chars().any(char::is_whitespace)
        // @ligne [validate] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        || r.image.starts_with('-')
    {
        // @ligne [validate] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Référence d’image invalide");
    }
    // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
    if r.host_port == 0 || r.container_port == 0 {
        // @ligne [validate] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Port invalide");
    }
    // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
    if r.bind_ip != "127.0.0.1" && !r.acknowledge_lan {
        // @ligne [validate] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Confirmez explicitement l’exposition réseau");
    }
    // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
    if !doctor::addresses().contains(&r.bind_ip) && r.bind_ip != "0.0.0.0" {
        // @ligne [validate] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Adresse de liaison absente de l’hôte");
    }
    // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
    if !["volume", "bind", "none"].contains(&r.storage.as_str()) {
        // @ligne [validate] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Type de stockage inconnu");
    }
    // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
    if r.storage != "none" && (!r.volume_target.starts_with('/') || r.volume_target == "/") {
        // @ligne [validate] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Le point de montage doit être absolu et différent de /");
    }
    // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
    if r.storage == "bind" {
        // @ligne [validate] Ajoute un contexte lisible à l’échec sans l’ignorer.
        let path = std::path::Path::new(r.bind_path.as_deref().context("Sélectionnez un dossier")?);
        // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
        if !path.is_absolute() || !path.is_dir() {
            // @ligne [validate] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Le dossier de stockage doit être un dossier absolu existant");
        }
    }
    // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
    if r.additional_ports.len() > 16 {
        // @ligne [validate] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Seize ports supplémentaires maximum");
    }
    // @ligne [validate] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut ports = std::collections::HashSet::new();
    // @ligne [validate] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
    for p in &r.additional_ports {
        // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
        if p.host == 0
            // @ligne [validate] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            || p.container == 0
            // @ligne [validate] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            || !["tcp", "udp"].contains(&p.protocol.as_str())
            // @ligne [validate] Ajoute cette donnée à la collection en cours de préparation.
            || !ports.insert((p.host, p.protocol.clone()))
        {
            // @ligne [validate] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Port supplémentaire invalide ou dupliqué");
        }
    }
    // @ligne [validate] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let t = catalog::find(&r.template)?;
    // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
    if !t.capabilities.is_empty() && !r.acknowledge_capabilities {
        // @ligne [validate] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Confirmez les capacités Linux supplémentaires");
    }
    // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
    if r.template == "mssql"
        // @ligne [validate] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        && (!r.acknowledge_license || r.env.get("ACCEPT_EULA").map(String::as_str) != Some("Y"))
    {
        // @ligne [validate] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Acceptez la licence SQL Server");
    }
    // @ligne [validate] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
    for field in t.fields {
        // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
        if field.required && r.env.get(&field.key).is_none_or(|v| v.trim().is_empty()) {
            // @ligne [validate] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Le champ {} est obligatoire", field.label);
        }
    }
    // @ligne [validate] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
    for (key, value) in &r.env {
        // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
        if key.is_empty()
            // @ligne [validate] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            || !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
            // @ligne [validate] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            || value.contains('\0')
        {
            // @ligne [validate] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Variable d’environnement invalide");
        }
    }
    // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
    if let Some(ip) = r.static_ip.as_deref().filter(|s| !s.is_empty()) {
        // @ligne [validate] Ajoute un contexte lisible à l’échec sans l’ignorer.
        let parsed: std::net::Ipv4Addr = ip.parse().context("IPv4 statique invalide")?;
        // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
        if parsed.is_unspecified()
            // @ligne [validate] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            || parsed.is_multicast()
            // @ligne [validate] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            || r.network.as_deref().unwrap_or("").is_empty()
        {
            // @ligne [validate] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Une IP statique nécessite un réseau Docker personnalisé existant");
        }
    }
    // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
    if let Some(platform) = &r.platform {
        // @ligne [validate] Vérifie cette condition avant d’appliquer l’action protégée.
        if platform != "linux/amd64" && platform != "linux/arm64" {
            // @ligne [validate] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Plateforme non autorisée");
        }
    }
    // @ligne [validate] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(())
}

// @ligne [containers] Délimite cette opération pour isoler sa validation et son résultat.
pub async fn containers() -> Result<Value> {
    // @ligne [containers] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let docker = doctor::docker()?;
    // @ligne [containers] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let result = docker
        // @ligne [containers] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .list_containers(Some(
            // @ligne [containers] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            ListContainersOptionsBuilder::default().all(true).build(),
        ))
        // @ligne [containers] Attend l’opération asynchrone avant d’utiliser son résultat.
        .await?;
    // @ligne [containers] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(serde_json::to_value(result)?)
}

// @ligne [logs] Délimite cette opération pour isoler sa validation et son résultat.
pub async fn logs(id: &str) -> Result<String> {
    // @ligne [logs] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let docker = doctor::docker()?;
    // @ligne [logs] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut stream = docker.logs(
        // @ligne [logs] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        id,
        // @ligne [logs] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        Some(
            // @ligne [logs] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            LogsOptionsBuilder::default()
                // @ligne [logs] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .stdout(true)
                // @ligne [logs] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .stderr(true)
                // @ligne [logs] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .tail("50")
                // @ligne [logs] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .build(),
        ),
    );
    // @ligne [logs] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut result = String::new();
    // @ligne [logs] Attend l’opération asynchrone avant d’utiliser son résultat.
    while let Some(line) = stream.next().await {
        // @ligne [logs] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let line = line?;
        // @ligne [logs] Ajoute cette donnée à la collection en cours de préparation.
        result.push_str(&String::from_utf8_lossy(line.as_ref()));
        // @ligne [logs] Vérifie cette condition avant d’appliquer l’action protégée.
        if result.len() > 65536 {
            // @ligne [logs] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            break;
        }
    }
    // @ligne [logs] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(crate::suite::redact(&result))
}

// @ligne [diagnose] Délimite cette opération pour isoler sa validation et son résultat.
pub fn diagnose(error: &str) -> Value {
    // @ligne [diagnose] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let e = error.to_lowercase();
    // @ligne [diagnose] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let (title, fix) = if e.contains("port is already allocated")
        // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        || e.contains("address already in use")
    {
        (
            // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "Ce port est déjà utilisé. Choisissez le prochain port libre.",
            // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "port",
        )
    // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else if e.contains("name") && e.contains("already in use") {
        (
            // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "Un conteneur utilise déjà ce nom. Renommez la nouvelle instance.",
            // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "rename",
        )
    // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else if e.contains("not empty") || e.contains("incompatible") {
        (
            // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "Le volume contient des données incompatibles. Créez un nouveau volume sans effacer l’ancien.",
            // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "volume",
        )
    // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else if e.contains("no matching manifest") || e.contains("exec format error") {
        (
            // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "L’image ne correspond pas à votre processeur. L’émulation amd64 doit être installée avant de réessayer.",
            // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "platform",
        )
    // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else if e.contains("permission denied") {
        (
            // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "Les droits système ou du dossier empêchent l’opération. Consultez Docker Doctor.",
            // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "doctor",
        )
    // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else {
        (
            // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "Docker a refusé l’opération. Consultez le détail et les logs avant de réessayer.",
            // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "inspect",
        )
    };
    // @ligne [diagnose] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    json!({"title":title,"fix":fix,"detail":crate::suite::redact(error)})
}

// @ligne [escaped_env] Délimite cette opération pour isoler sa validation et son résultat.
fn escaped_env(value: &str) -> String {
    // @ligne [escaped_env] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    format!("'{}'", value.replace('\'', "'\\''"))
}

// @ligne [credential_document] Délimite cette opération pour isoler sa validation et son résultat.
pub fn credential_document(r: &CreateRequest, id: &str, ip: &str, status: &str) -> Result<Value> {
    // @ligne [credential_document] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let get = |keys: &[&str]| {
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        keys.iter()
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .find_map(|key| r.env.get(*key))
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .cloned()
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap_or_default()
    };
    // @ligne [credential_document] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let (scheme, user, password, database) = match r.template.as_str() {
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "postgres" => (
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "postgresql",
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            get(&["POSTGRES_USER"]),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            get(&["POSTGRES_PASSWORD"]),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            get(&["POSTGRES_DB"]),
        ),
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "mysql" => (
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "mysql",
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            get(&["MYSQL_USER"]),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            get(&["MYSQL_PASSWORD"]),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            get(&["MYSQL_DATABASE"]),
        ),
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "mariadb" => (
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "mysql",
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            get(&["MARIADB_USER"]),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            get(&["MARIADB_PASSWORD"]),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            get(&["MARIADB_DATABASE"]),
        ),
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "mongo" => (
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "mongodb",
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            get(&["MONGO_INITDB_ROOT_USERNAME"]),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            get(&["MONGO_INITDB_ROOT_PASSWORD"]),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            String::new(),
        ),
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "redis" => (
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "redis",
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            String::new(),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            get(&["REDIS_PASSWORD"]),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            String::new(),
        ),
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "mssql" => (
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "sqlserver",
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "sa".into(),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            get(&["MSSQL_SA_PASSWORD"]),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            String::new(),
        ),
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "oracle" => (
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "oracle",
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "SYSTEM".into(),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            get(&["ORACLE_PASSWORD"]),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "FREEPDB1".into(),
        ),
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        _ => ("http", String::new(), String::new(), String::new()),
    };
    // @ligne [credential_document] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let host = if r.bind_ip == "0.0.0.0" {
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "127.0.0.1"
    // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else {
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        &r.bind_ip
    };
    // @ligne [credential_document] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut uri = url::Url::parse(&format!("{scheme}://{host}:{}", r.host_port))?;
    // @ligne [credential_document] Vérifie cette condition avant d’appliquer l’action protégée.
    if !user.is_empty() || !password.is_empty() {
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        uri.set_username(&user)
            // @ligne [credential_document] Ajoute un contexte lisible à l’échec sans l’ignorer.
            .map_err(|_| anyhow::anyhow!("Utilisateur URI invalide"))?;
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        uri.set_password(Some(&password))
            // @ligne [credential_document] Ajoute un contexte lisible à l’échec sans l’ignorer.
            .map_err(|_| anyhow::anyhow!("Mot de passe URI invalide"))?;
    }
    // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    uri.path_segments_mut()
        // @ligne [credential_document] Ajoute un contexte lisible à l’échec sans l’ignorer.
        .map_err(|_| anyhow::anyhow!("Chemin URI invalide"))?
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .clear()
        // @ligne [credential_document] Ajoute cette donnée à la collection en cours de préparation.
        .push(&database);
    // @ligne [credential_document] Vérifie cette condition avant d’appliquer l’action protégée.
    if scheme == "mongodb" {
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        uri.set_query(Some("authSource=admin"));
    }
    // @ligne [credential_document] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let driver = if r.template == "postgres" {
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "pgsql"
    // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else {
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        scheme
    };
    // @ligne [credential_document] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let cli = if scheme == "postgresql" {
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        format!(
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "PGPASSWORD={} psql -h {} -p {} -U {} -d {}",
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            escaped_env(&password),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            escaped_env(host),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            r.host_port,
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            escaped_env(&user),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            escaped_env(&database)
        )
    // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else if scheme == "mysql" {
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        format!(
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "mysql -h {} -P {} -u {} -p {}",
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            escaped_env(host),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            r.host_port,
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            escaped_env(&user),
            // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            escaped_env(&database)
        )
    // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else {
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        String::new()
    };
    // @ligne [credential_document] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(
        // @ligne [credential_document] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        json!({"generated_by":"DockForge Studio v2.0.0-alpha.1","created_at":chrono::Utc::now().to_rfc3339(),"container":{"id":id,"name":r.name,"image":r.image,"status":if ["healthy","health_pending","running_without_healthcheck"].contains(&status){"running"}else{status},"readiness":status},"network":{"mode":if r.bind_ip=="127.0.0.1" {"local_only"} else {"lan_ip"},"bind_ip":r.bind_ip,"localhost":"127.0.0.1","internal_container_ip":ip,"host_port":r.host_port,"container_port":r.container_port,"docker_network":r.network},"credentials":{"engine":r.template,"username":user,"password":password,"database":database,"environment":r.env},"connection_strings":{"uri_standard":uri.to_string(),"jdbc_java":if ["postgresql","mysql"].contains(&scheme){format!("jdbc:{scheme}://{host}:{}/{database}",r.host_port)}else{String::new()},"dotnet_ef":if scheme=="postgresql"{format!("Host={host};Port={};Database={};Username={};Password={}",r.host_port,quote_dotnet(&database),quote_dotnet(&user),quote_dotnet(&password))}else{String::new()},"laravel_env":format!("DB_CONNECTION={driver}\nDB_HOST={host}\nDB_PORT={}\nDB_DATABASE={}\nDB_USERNAME={}\nDB_PASSWORD={}",r.host_port,serde_json::to_string(&database)?,serde_json::to_string(&user)?,serde_json::to_string(&password)?.replace('$',"\\$")),"cli_command":cli}}),
    )
}

// @ligne [quote_dotnet] Délimite cette opération pour isoler sa validation et son résultat.
fn quote_dotnet(value: &str) -> String {
    // @ligne [quote_dotnet] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    format!("\"{}\"", value.replace('"', "\"\""))
}

// @ligne [create] Délimite cette opération pour isoler sa validation et son résultat.
pub async fn create(r: CreateRequest, store: Arc<Store>, out: Reporter) -> Result<Value> {
    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    validate(&r)?;
    // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let t = catalog::find(&r.template)?;
    // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let protocol = if r.template == "wireguard" {
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "udp"
    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else {
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "tcp"
    };
    // @ligne [create] Vérifie cette condition avant d’appliquer l’action protégée.
    if !doctor::port_check(&r.bind_ip, r.host_port, protocol)?.available {
        // @ligne [create] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("address already in use : port {}", r.host_port);
    }
    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    store.unlock()?;
    // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let docker = doctor::docker()?;
    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    report(&out, &r.operation, "pull", "Vérification de l’image locale");
    // @ligne [create] Attend l’opération asynchrone avant d’utiliser son résultat.
    if docker.inspect_image(&r.image).await.is_err() {
        // @ligne [create] Prépare une valeur locale qui sera complétée pendant cette opération.
        let mut stream = docker.create_image(
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            Some(
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                CreateImageOptionsBuilder::default()
                    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    .from_image(&r.image)
                    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    .platform(r.platform.as_deref().unwrap_or(""))
                    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    .build(),
            ),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            None,
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            None,
        );
        // @ligne [create] Attend l’opération asynchrone avant d’utiliser son résultat.
        while let Some(item) = stream.next().await {
            // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let info = item?;
            // @ligne [create] Vérifie cette condition avant d’appliquer l’action protégée.
            if let Some(error) = info.error {
                // @ligne [create] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
                bail!("Téléchargement : {}", error);
            }
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            report(
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                &out,
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                &r.operation,
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "pull",
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                format!(
                    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    "{} {} {}",
                    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    info.id.unwrap_or_default(),
                    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    info.status.unwrap_or_default(),
                    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    info.progress.unwrap_or_default()
                ),
            );
        }
    }
    // @ligne [create] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut mounts = Vec::new();
    // @ligne [create] Vérifie cette condition avant d’appliquer l’action protégée.
    if r.storage == "volume" {
        // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let volume = format!("dockforge-{}-{:08x}", r.name, rand::random::<u32>());
        // @ligne [create] Prépare une valeur locale qui sera complétée pendant cette opération.
        let mut labels = HashMap::new();
        // @ligne [create] Ajoute cette donnée à la collection en cours de préparation.
        labels.insert("io.dockforge.managed".to_string(), "true".to_string());
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        docker
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .create_volume(VolumeCreateOptions {
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                name: Some(volume.clone()),
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                labels: Some(labels),
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                ..Default::default()
            })
            // @ligne [create] Attend l’opération asynchrone avant d’utiliser son résultat.
            .await?;
        // @ligne [create] Ajoute cette donnée à la collection en cours de préparation.
        mounts.push(Mount {
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            target: Some(r.volume_target.clone()),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            source: Some(volume),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            typ: Some(MountTypeEnum::VOLUME),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            ..Default::default()
        });
    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else if r.storage == "bind" {
        // @ligne [create] Ajoute cette donnée à la collection en cours de préparation.
        mounts.push(Mount {
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            target: Some(r.volume_target.clone()),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            source: Some(
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                std::fs::canonicalize(r.bind_path.as_ref().unwrap())?
                    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    .to_string_lossy()
                    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    .into_owned(),
            ),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            typ: Some(MountTypeEnum::BIND),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            ..Default::default()
        });
    }
    // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let port = format!("{}/{protocol}", r.container_port);
    // @ligne [create] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut bindings = HashMap::new();
    // @ligne [create] Ajoute cette donnée à la collection en cours de préparation.
    bindings.insert(
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        port.clone(),
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        Some(vec![PortBinding {
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            host_ip: Some(r.bind_ip.clone()),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            host_port: Some(r.host_port.to_string()),
        }]),
    );
    // @ligne [create] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
    for extra in &r.additional_ports {
        // @ligne [create] Vérifie cette condition avant d’appliquer l’action protégée.
        if !doctor::port_check(&r.bind_ip, extra.host, &extra.protocol)?.available {
            // @ligne [create] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("address already in use : port {}", extra.host);
        }
        // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let key = format!("{}/{}", extra.container, extra.protocol);
        // @ligne [create] Vérifie cette condition avant d’appliquer l’action protégée.
        if bindings.contains_key(&key) {
            // @ligne [create] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Le port interne {key} est déjà publié ; utilisez un port interne distinct");
        }
        // @ligne [create] Ajoute cette donnée à la collection en cours de préparation.
        bindings.insert(
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            key,
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            Some(vec![PortBinding {
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                host_ip: Some(r.bind_ip.clone()),
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                host_port: Some(extra.host.to_string()),
            }]),
        );
    }
    // @ligne [create] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut exposed = HashMap::new();
    // @ligne [create] Ajoute cette donnée à la collection en cours de préparation.
    exposed.insert(port, HashMap::new());
    // @ligne [create] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
    for p in &r.additional_ports {
        // @ligne [create] Ajoute cette donnée à la collection en cours de préparation.
        exposed.insert(format!("{}/{}", p.container, p.protocol), HashMap::new());
    }
    // @ligne [create] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut labels = HashMap::new();
    // @ligne [create] Ajoute cette donnée à la collection en cours de préparation.
    labels.insert("io.dockforge.managed".into(), "true".into());
    // @ligne [create] Ajoute cette donnée à la collection en cours de préparation.
    labels.insert("io.dockforge.template".into(), r.template.clone());
    // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let health = if t.health.is_empty() {
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        None
    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else {
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        Some(HealthConfig {
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            test: Some(t.health),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            interval: Some(3_000_000_000),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            timeout: Some(2_000_000_000),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            retries: Some(30),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            start_period: Some(10_000_000_000),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            ..Default::default()
        })
    };
    // @ligne [create] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut host = HostConfig {
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        port_bindings: Some(bindings),
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        mounts: Some(mounts),
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        network_mode: r.network.clone().filter(|s| !s.is_empty()),
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        cap_add: if t.capabilities.is_empty() {
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            None
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        } else {
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            Some(t.capabilities)
        },
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        ..Default::default()
    };
    // @ligne [create] Vérifie cette condition avant d’appliquer l’action protégée.
    if r.gpu {
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        host.device_requests = Some(vec![DeviceRequest {
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            count: Some(-1),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            capabilities: Some(vec![vec!["gpu".into()]]),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            ..Default::default()
        }]);
    }
    // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let networking_config = if let (Some(net), Some(ip)) = (
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        r.network.as_ref().filter(|s| !s.is_empty()),
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        r.static_ip.as_ref().filter(|s| !s.is_empty()),
    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    ) {
        // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let endpoint = bollard::models::EndpointSettings {
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            ipam_config: Some(bollard::models::EndpointIpamConfig {
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                ipv4_address: Some(ip.clone()),
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                ..Default::default()
            }),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            ..Default::default()
        };
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        Some(NetworkingConfig {
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            endpoints_config: Some(HashMap::from([(net.clone(), endpoint)])),
        })
    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else {
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        None
    };
    // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let command = if r.command.is_empty() {
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        match r.template.as_str() {
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "keycloak" => Some(vec!["start-dev".into()]),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "redis" => Some(vec![
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "sh".into(),
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "-c".into(),
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "exec redis-server --appendonly yes --requirepass \"$REDIS_PASSWORD\"".into(),
            ]),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "milvus" => Some(vec!["milvus".into(), "run".into(), "standalone".into()]),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            _ => None,
        }
    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else {
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        Some(r.command.clone())
    };
    // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let config = Config {
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        image: Some(r.image.clone()),
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        env: Some(r.env.iter().map(|(k, v)| format!("{k}={v}")).collect()),
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        cmd: command,
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        host_config: Some(host),
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        exposed_ports: Some(exposed),
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        labels: Some(labels),
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        healthcheck: health,
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        networking_config,
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        ..Default::default()
    };
    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    report(
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        &out,
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        &r.operation,
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "create",
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "Création et enregistrement chiffré des accès",
    );
    // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let created = docker
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .create_container(
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            Some(
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                CreateContainerOptionsBuilder::default()
                    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    .name(&r.name)
                    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    .platform(r.platform.as_deref().unwrap_or(""))
                    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    .build(),
            ),
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            config,
        )
        // @ligne [create] Attend l’opération asynchrone avant d’utiliser son résultat.
        .await?;
    // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let document = credential_document(&r, &created.id, "", "created")?;
    // @ligne [create] Vérifie cette condition avant d’appliquer l’action protégée.
    if let Err(e) = store.save_credentials(&created.id, &r.name, &r.image, &document) {
        // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let _ = docker
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .remove_container(
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                &created.id,
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                Some(
                    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    RemoveContainerOptionsBuilder::default()
                        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        .force(false)
                        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        .v(false)
                        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        .build(),
                ),
            )
            // @ligne [create] Attend l’opération asynchrone avant d’utiliser son résultat.
            .await;
        // @ligne [create] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        return Err(e).context("Conteneur non démarré : le coffre n’a pas pu conserver les accès. Les volumes restent préservés.");
    }
    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    docker
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .start_container(
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            &created.id,
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            None::<bollard::query_parameters::StartContainerOptions>,
        )
        // @ligne [create] Attend l’opération asynchrone avant d’utiliser son résultat.
        .await
        // @ligne [create] Ajoute un contexte lisible à l’échec sans l’ignorer.
        .context("Conteneur créé et accès conservés, mais démarrage refusé")?;
    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    report(
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        &out,
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        &r.operation,
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "health",
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "Attente de la sonde de santé ou vérification de l’état",
    );
    // @ligne [create] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut readiness = "running_without_healthcheck";
    // @ligne [create] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
    for _ in 0..60 {
        // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let inspected = docker
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .inspect_container(
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                &created.id,
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                None::<bollard::query_parameters::InspectContainerOptions>,
            )
            // @ligne [create] Attend l’opération asynchrone avant d’utiliser son résultat.
            .await?;
        // @ligne [create] Ajoute un contexte lisible à l’échec sans l’ignorer.
        let state = inspected.state.context("État du conteneur absent")?;
        // @ligne [create] Vérifie cette condition avant d’appliquer l’action protégée.
        if state.running != Some(true) {
            // @ligne [create] Attend l’opération asynchrone avant d’utiliser son résultat.
            let detail = logs(&created.id).await.unwrap_or_default();
            // @ligne [create] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!(
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "Le conteneur {} est arrêté (code {:?}). Les accès restent dans le coffre. Derniers logs :\n{}",
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                r.name,
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                state.exit_code,
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                scrub_environment(&detail, &r.env)
            );
        }
        // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let health = state
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .health
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .and_then(|h| h.status)
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .map(|s| s.to_string())
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap_or_default();
        // @ligne [create] Vérifie cette condition avant d’appliquer l’action protégée.
        if health == "healthy" {
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            readiness = "healthy";
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            break;
        }
        // @ligne [create] Vérifie cette condition avant d’appliquer l’action protégée.
        if health == "unhealthy" {
            // @ligne [create] Attend l’opération asynchrone avant d’utiliser son résultat.
            let detail = logs(&created.id).await.unwrap_or_default();
            // @ligne [create] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!(
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "Sonde de santé en échec pour {}. Les accès restent dans le coffre. Derniers logs :\n{}",
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                r.name,
                // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                scrub_environment(&detail, &r.env)
            );
        }
        // @ligne [create] Vérifie cette condition avant d’appliquer l’action protégée.
        if health.is_empty() {
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            break;
        }
        // @ligne [create] Attend l’opération asynchrone avant d’utiliser son résultat.
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let inspected = docker
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .inspect_container(
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            &created.id,
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            None::<bollard::query_parameters::InspectContainerOptions>,
        )
        // @ligne [create] Attend l’opération asynchrone avant d’utiliser son résultat.
        .await?;
    // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let health = inspected
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .state
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .as_ref()
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .and_then(|s| s.health.as_ref())
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .and_then(|h| h.status.as_ref())
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .map(|s| s.to_string())
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .unwrap_or_default();
    // @ligne [create] Vérifie cette condition avant d’appliquer l’action protégée.
    if !health.is_empty() && health != "healthy" {
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        readiness = "health_pending";
    }
    // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let internal = inspected
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .network_settings
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .and_then(|n| n.networks)
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .and_then(|n| n.into_values().find_map(|e| e.ip_address))
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .unwrap_or_default();
    // @ligne [create] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let document = credential_document(&r, &created.id, &internal, readiness)?;
    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    store.save_credentials(&created.id, &r.name, &r.image, &document)?;
    // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    report(
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        &out,
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        &r.operation,
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "complete",
        // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        match readiness {
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "healthy" => "Service validé par sa sonde de santé",
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "health_pending" => "Conteneur démarré, sonde encore en attente",
            // @ligne [create] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            _ => "Conteneur démarré ; disponibilité applicative non vérifiée",
        },
    );
    // @ligne [create] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(json!({"id":created.id,"name":r.name,"readiness":readiness}))
}

// @ligne [lifecycle] Délimite cette opération pour isoler sa validation et son résultat.
pub async fn lifecycle(id: &str, action: &str, store: &Store, archive: bool) -> Result<()> {
    // @ligne [lifecycle] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let docker = doctor::docker()?;
    // @ligne [lifecycle] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let info = docker
        // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .inspect_container(
            // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            id,
            // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            None::<bollard::query_parameters::InspectContainerOptions>,
        )
        // @ligne [lifecycle] Attend l’opération asynchrone avant d’utiliser son résultat.
        .await?;
    // @ligne [lifecycle] Vérifie cette condition avant d’appliquer l’action protégée.
    if info
        // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .config
        // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .and_then(|c| c.labels)
        // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .is_none_or(|l| l.get("io.dockforge.managed").map(String::as_str) != Some("true"))
    {
        // @ligne [lifecycle] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Action réservée aux conteneurs gérés par DockForge");
    }
    // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    match action {
        // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "start" => {
            // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            docker
                // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .start_container(id, None::<bollard::query_parameters::StartContainerOptions>)
                // @ligne [lifecycle] Attend l’opération asynchrone avant d’utiliser son résultat.
                .await?
        }
        // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "stop" => {
            // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            docker
                // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .stop_container(id, None::<bollard::query_parameters::StopContainerOptions>)
                // @ligne [lifecycle] Attend l’opération asynchrone avant d’utiliser son résultat.
                .await?
        }
        // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "remove" => {
            // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            docker
                // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .remove_container(
                    // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    id,
                    // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    Some(
                        // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        RemoveContainerOptionsBuilder::default()
                            // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                            .force(false)
                            // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                            .v(false)
                            // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                            .build(),
                    ),
                )
                // @ligne [lifecycle] Attend l’opération asynchrone avant d’utiliser son résultat.
                .await?;
            // @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            store.retire(id, archive)?;
        }
        // @ligne [lifecycle] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        _ => bail!("Action inconnue"),
    }
    // @ligne [lifecycle] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(())
}

// @ligne [lifecycle] Applique cet attribut au type ou à la fonction qui suit.
#[cfg(test)]
// @ligne [lifecycle] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
mod tests {
    // @ligne [lifecycle] Importe explicitement les types et fonctions nécessaires à ce module.
    use super::*;
    // @ligne [request] Délimite cette opération pour isoler sa validation et son résultat.
    fn request() -> CreateRequest {
        // @ligne [request] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        serde_json::from_value(json!({"operation":"test","template":"postgres","name":"demo","image":"postgres:17-alpine","bind_ip":"127.0.0.1","host_port":5432,"container_port":5432,"env":{"POSTGRES_USER":"a@b","POSTGRES_PASSWORD":"p#@'word","POSTGRES_DB":"db"},"storage":"volume","volume_target":"/var/lib/postgresql/data","command":[],"gpu":false,"acknowledge_lan":false,"acknowledge_license":false,"acknowledge_capabilities":false})).unwrap()
    }
    // @ligne [request] Applique cet attribut au type ou à la fonction qui suit.
    #[test]
    // @ligne [connection_uri_escapes_credentials] Délimite cette opération pour isoler sa validation et son résultat.
    fn connection_uri_escapes_credentials() {
        // @ligne [connection_uri_escapes_credentials] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let r = request();
        // @ligne [connection_uri_escapes_credentials] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let doc = credential_document(&r, "id", "", "healthy").unwrap();
        // @ligne [connection_uri_escapes_credentials] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let uri = doc["connection_strings"]["uri_standard"].as_str().unwrap();
        // @ligne [connection_uri_escapes_credentials] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let url = url::Url::parse(uri).unwrap();
        // @ligne [connection_uri_escapes_credentials] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(url.username().contains("%40"));
        // @ligne [connection_uri_escapes_credentials] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(uri.contains("%23"));
        // @ligne [connection_uri_escapes_credentials] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(!uri.contains("p#@"));
        // @ligne [connection_uri_escapes_credentials] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(
            // @ligne [connection_uri_escapes_credentials] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            doc["connection_strings"]["cli_command"]
                // @ligne [connection_uri_escapes_credentials] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .as_str()
                // @ligne [connection_uri_escapes_credentials] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .unwrap()
                // @ligne [connection_uri_escapes_credentials] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .contains("'\\''")
        );
    }
    // @ligne [connection_uri_escapes_credentials] Applique cet attribut au type ou à la fonction qui suit.
    #[test]
    // @ligne [validation_rejects_unsafe_input] Délimite cette opération pour isoler sa validation et son résultat.
    fn validation_rejects_unsafe_input() {
        // @ligne [validation_rejects_unsafe_input] Prépare une valeur locale qui sera complétée pendant cette opération.
        let mut r = request();
        // @ligne [validation_rejects_unsafe_input] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        validate(&r).unwrap();
        // @ligne [validation_rejects_unsafe_input] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        r.name = "$(touch /tmp/bad)".into();
        // @ligne [validation_rejects_unsafe_input] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(validate(&r).is_err());
        // @ligne [validation_rejects_unsafe_input] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        r = request();
        // @ligne [validation_rejects_unsafe_input] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        r.bind_ip = "0.0.0.0".into();
        // @ligne [validation_rejects_unsafe_input] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(validate(&r).is_err());
        // @ligne [validation_rejects_unsafe_input] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        r = request();
        // @ligne [validation_rejects_unsafe_input] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        r.env.remove("POSTGRES_PASSWORD");
        // @ligne [validation_rejects_unsafe_input] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(validate(&r).is_err());
    }
}

// @ligne [validation_rejects_unsafe_input] Applique cet attribut au type ou à la fonction qui suit.
#[cfg(test)]
// @ligne [validation_rejects_unsafe_input] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
mod live_tests {
    // @ligne [validation_rejects_unsafe_input] Importe explicitement les types et fonctions nécessaires à ce module.
    use super::*;
    /// Ce test ne touche qu’un conteneur et un volume dont il vient de créer l’identifiant.
    // @ligne [validation_rejects_unsafe_input] Applique cet attribut au type ou à la fonction qui suit.
    #[tokio::test]
    // @ligne [validation_rejects_unsafe_input] Applique cet attribut au type ou à la fonction qui suit.
    #[ignore = "Nécessite un démon Docker local et l’accès au registre postgres"]
    // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Délimite cette opération pour isoler sa validation et son résultat.
    async fn postgres_provisioning_keeps_credentials_and_passes_real_healthcheck() {
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let tmp = tempfile::tempdir().unwrap();
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let store = Arc::new(Store::open(tmp.path()).unwrap());
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        store.test_unlock();
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let port = std::net::TcpListener::bind("127.0.0.1:0")
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap()
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .local_addr()
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap()
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .port();
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let request:CreateRequest=serde_json::from_value(json!({"operation":"live-test","template":"postgres","name":format!("dockforge-test-{:08x}",rand::random::<u32>()),"image":"postgres:17-alpine","bind_ip":"127.0.0.1","host_port":port,"container_port":5432,"env":{"POSTGRES_USER":"dockforge_test","POSTGRES_PASSWORD":"Only_for_isolated_test_984!","POSTGRES_DB":"dockforge_test"},"storage":"volume","volume_target":"/var/lib/postgresql/data","command":[],"gpu":false,"acknowledge_lan":false,"acknowledge_license":false,"acknowledge_capabilities":false})).unwrap();
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let name = request.name.clone();
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let out: Reporter = Arc::new(|_| {});
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Attend l’opération asynchrone avant d’utiliser son résultat.
        let result = create(request, store.clone(), out).await;
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let docker = doctor::docker().unwrap();
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let inspected = docker
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .inspect_container(
                // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                &name,
                // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                None::<bollard::query_parameters::InspectContainerOptions>,
            )
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Attend l’opération asynchrone avant d’utiliser son résultat.
            .await;
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let volume = inspected
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .as_ref()
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .ok()
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .and_then(|i| i.mounts.as_ref())
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .and_then(|m| m.first())
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .and_then(|m| m.name.clone());
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let retained = result
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .as_ref()
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .ok()
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .and_then(|v| v["id"].as_str())
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .and_then(|id| store.credentials(id).ok());
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Vérifie cette condition avant d’appliquer l’action protégée.
        if inspected.is_ok() {
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let _ = docker
                // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .remove_container(
                    // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    &name,
                    // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    Some(
                        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        RemoveContainerOptionsBuilder::default()
                            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                            .force(true)
                            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                            .v(false)
                            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                            .build(),
                    ),
                )
                // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Attend l’opération asynchrone avant d’utiliser son résultat.
                .await;
        }
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Vérifie cette condition avant d’appliquer l’action protégée.
        if let Some(volume) = volume {
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let _ = docker
                // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .remove_volume(
                    // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    &volume,
                    // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    None::<bollard::query_parameters::RemoveVolumeOptions>,
                )
                // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Attend l’opération asynchrone avant d’utiliser son résultat.
                .await;
        }
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let result = result.unwrap();
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(result["readiness"], "healthy");
        // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            retained.unwrap()["credentials"]["username"],
            // @ligne [postgres_provisioning_keeps_credentials_and_passes_real_healthcheck] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "dockforge_test"
        );
    }
}

/// Propose les variables d’une base existante après une sélection explicite dans l’interface.
// @ligne [link_database] Délimite cette opération pour isoler sa validation et son résultat.
pub async fn link_database(source: &str, target: &str, store: &Store) -> Result<Value> {
    // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    store.unlock()?;
    // @ligne [link_database] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let doc = store.credentials(source)?;
    // @ligne [link_database] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let engine = doc["credentials"]["engine"].as_str().unwrap_or("");
    // @ligne [link_database] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let docker = doctor::docker()?;
    // @ligne [link_database] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let info = docker
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .inspect_container(
            // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            source,
            // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            None::<bollard::query_parameters::InspectContainerOptions>,
        )
        // @ligne [link_database] Attend l’opération asynchrone avant d’utiliser son résultat.
        .await?;
    // @ligne [link_database] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let name = info
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .name
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .unwrap_or_default()
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .trim_start_matches('/')
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .to_string();
    // @ligne [link_database] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let endpoints = info
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .network_settings
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .and_then(|n| n.networks)
        // @ligne [link_database] Ajoute un contexte lisible à l’échec sans l’ignorer.
        .context("Aucun réseau pour la base")?;
    // @ligne [link_database] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let custom = endpoints
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .keys()
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .filter(|n| !["bridge", "host", "none"].contains(&n.as_str()))
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .cloned()
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .collect::<Vec<_>>();
    // @ligne [link_database] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let host = if custom.is_empty() {
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        endpoints
            // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .values()
            // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .find_map(|e| e.ip_address.clone())
            // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .filter(|s| !s.is_empty())
            // @ligne [link_database] Ajoute un contexte lisible à l’échec sans l’ignorer.
            .context("La base doit être démarrée pour connaître son adresse")?
    // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else {
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        name
    };
    // @ligne [link_database] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let user = doc["credentials"]["username"].as_str().unwrap_or("");
    // @ligne [link_database] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let password = doc["credentials"]["password"].as_str().unwrap_or("");
    // @ligne [link_database] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let database = doc["credentials"]["database"].as_str().unwrap_or("");
    // @ligne [link_database] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let variables = match target {
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "glpi" if ["mysql", "mariadb"].contains(&engine) => {
            // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            json!({"GLPI_DB_HOST":host,"GLPI_DB_NAME":database,"GLPI_DB_USER":user,"GLPI_DB_PASSWORD":password})
        }
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "wordpress" if ["mysql", "mariadb"].contains(&engine) => {
            // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            json!({"WORDPRESS_DB_HOST":host,"WORDPRESS_DB_NAME":database,"WORDPRESS_DB_USER":user,"WORDPRESS_DB_PASSWORD":password})
        }
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "zabbix" if ["mysql", "mariadb"].contains(&engine) => {
            // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            json!({"DB_SERVER_HOST":host,"MYSQL_DATABASE":database,"MYSQL_USER":user,"MYSQL_PASSWORD":password})
        }
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "odoo" if engine == "postgres" => json!({"HOST":host,"USER":user,"PASSWORD":password}),
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "nextcloud" if ["mysql", "mariadb"].contains(&engine) => {
            // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            json!({"MYSQL_HOST":host,"MYSQL_DATABASE":database,"MYSQL_USER":user,"MYSQL_PASSWORD":password})
        }
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "nextcloud" if engine == "postgres" => {
            // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            json!({"POSTGRES_HOST":host,"POSTGRES_DB":database,"POSTGRES_USER":user,"POSTGRES_PASSWORD":password})
        }
        // @ligne [link_database] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        _ => bail!(
            // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "Cette association de moteur et d’application n’est pas préconfigurée ; utilisez les variables avancées"
        ),
    };
    // @ligne [link_database] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(
        // @ligne [link_database] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        json!({"env":variables,"network":custom.first(),"notice":if custom.is_empty(){"Adresse interne utilisée. Un réseau Docker personnalisé est préférable pour garder un nom stable après recréation."}else{"Les deux services utiliseront le même réseau Docker personnalisé. Cette liaison réutilise la base et ses droits existants ; elle ne crée pas de schéma ni d’utilisateur supplémentaire."}}),
    )
}
