//! Génération reproductible : paquets autorisés et contexte borné sans secrets implicites.
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use crate::{Reporter, doctor, report, store::Store};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use anyhow::{Context, Result, bail};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use bollard::query_parameters::{BuildImageOptionsBuilder, BuilderVersion};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use futures_util::StreamExt;
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use serde::{Deserialize, Serialize};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use std::{path::Path, sync::Arc};

// @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
pub const BASES: &[&str] = &[
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "ubuntu:24.04",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "debian:12-slim",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "alpine:3.20",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "archlinux:base",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "python:3.12-slim",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "node:22-bookworm-slim",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "php:8.3-apache",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "rust:1-slim-bookworm",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "mcr.microsoft.com/dotnet/sdk:8.0",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "eclipse-temurin:21-jdk",
];
// @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
pub const PACKAGES: &[&str] = &[
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "curl",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "git",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "vim",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "nmap",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "iproute2",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "ffmpeg",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "supervisor",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "pdo_pgsql",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "pdo_mysql",
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    "redis",
];

// @ligne [module] Applique cet attribut au type ou à la fonction qui suit.
#[derive(Clone, Serialize, Deserialize)]
// @ligne [module] Définit les données échangées par ce module, sans comportement implicite.
pub struct Recipe {
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub operation: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub tag: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub base: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub packages: Vec<String>,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub project: Option<String>,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub command: Vec<String>,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub port: Option<u16>,
    // @ligne [module] Applique cet attribut au type ou à la fonction qui suit.
    #[serde(default)]
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub artifact: Option<String>,
}

// @ligne [dockerfile] Délimite cette opération pour isoler sa validation et son résultat.
pub fn dockerfile(r: &Recipe) -> Result<String> {
    // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
    if !BASES.contains(&r.base.as_str()) {
        // @ligne [dockerfile] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Base absente du catalogue validé");
    }
    // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
    if !regex::Regex::new(r"^[a-z0-9][a-z0-9._/-]*(?::[A-Za-z0-9_.-]+)?$")?.is_match(&r.tag)
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        || r.tag.len() > 200
    {
        // @ligne [dockerfile] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Nom d’image invalide");
    }
    // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
    if r.packages.iter().any(|s| !PACKAGES.contains(&s.as_str())) {
        // @ligne [dockerfile] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Paquet inconnu");
    }
    // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
    if let Some(artifact) = r.artifact.as_deref().filter(|s| !s.is_empty()) {
        // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
        if !regex::Regex::new(r"^[A-Za-z0-9_.-]+$")?.is_match(artifact) {
            // @ligne [dockerfile] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Nom d’artefact invalide");
        }
        // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
        if r.project.as_deref().is_none_or(|s| s.is_empty()) {
            // @ligne [dockerfile] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Un build multi-stage exige un dossier projet");
        }
        // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
        if !r.packages.is_empty() {
            // @ligne [dockerfile] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!(
                // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "Le mode multi-stage utilise ses dépendances prédéfinies ; décochez les outils supplémentaires"
            );
        }
        // @ligne [dockerfile] Prépare une valeur locale qui sera complétée pendant cette opération.
        let mut text = if r.base.starts_with("rust:") {
            // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            format!(
                // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "FROM {} AS build\nWORKDIR /src\nCOPY project/ ./\nRUN cargo build --release --locked\nFROM debian:12-slim AS runtime\nRUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*\nCOPY --from=build /src/target/release/{artifact} /usr/local/bin/app\nUSER 65532:65532\nENTRYPOINT [\"/usr/local/bin/app\"]\n",
                // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                r.base
            )
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        } else if r.base.starts_with("mcr.microsoft.com/dotnet/sdk:") {
            // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            format!(
                // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "FROM {} AS build\nWORKDIR /src\nCOPY project/ ./\nRUN dotnet publish -c Release -o /out --no-self-contained\nFROM mcr.microsoft.com/dotnet/aspnet:8.0 AS runtime\nWORKDIR /app\nCOPY --from=build /out/ ./\nUSER 1654:1654\nENTRYPOINT [\"dotnet\",\"{artifact}\"]\n",
                // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                r.base
            )
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        } else {
            // @ligne [dockerfile] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Le build multi-stage est disponible pour Rust et .NET");
        };
        // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
        if let Some(port) = r.port {
            // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
            if port == 0 {
                // @ligne [dockerfile] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
                bail!("Port invalide");
            }
            // @ligne [dockerfile] Ajoute cette donnée à la collection en cours de préparation.
            text.push_str(&format!("EXPOSE {port}\n"));
        }
        // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
        if !r.command.is_empty() {
            // @ligne [dockerfile] Ajoute cette donnée à la collection en cours de préparation.
            text.push_str(&format!("CMD {}\n", serde_json::to_string(&r.command)?));
        }
        // @ligne [dockerfile] Renvoie une réussite explicite avec les données attendues par l’appelant.
        return Ok(text);
    }
    // @ligne [dockerfile] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut packages = r
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .packages
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .iter()
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .filter(|s| !["pdo_pgsql", "pdo_mysql", "redis"].contains(&s.as_str()))
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .cloned()
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .collect::<Vec<_>>();
    // @ligne [dockerfile] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let php_ext = r
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .packages
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .iter()
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .filter(|s| ["pdo_pgsql", "pdo_mysql", "redis"].contains(&s.as_str()))
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .cloned()
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .collect::<Vec<_>>();
    // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
    if !php_ext.is_empty() && !r.base.starts_with("php:") {
        // @ligne [dockerfile] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Les extensions PHP nécessitent la base PHP");
    }
    // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
    if php_ext.iter().any(|s| s == "pdo_pgsql") {
        // @ligne [dockerfile] Ajoute cette donnée à la collection en cours de préparation.
        packages.push("libpq-dev".into());
    }
    // @ligne [dockerfile] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut text = format!(
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "# Généré par DockForge ; vérifiez le contexte avant le build.\nFROM {} AS runtime\n",
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        r.base
    );
    // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
    if !packages.is_empty() {
        // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
        if r.base.starts_with("alpine:") {
            // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            packages = packages
                // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .into_iter()
                // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .map(|s| {
                    // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
                    if s == "iproute2" {
                        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        "iproute2".into()
                    // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    } else {
                        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        s
                    }
                })
                // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .collect();
            // @ligne [dockerfile] Ajoute cette donnée à la collection en cours de préparation.
            text.push_str(&format!("RUN apk add --no-cache {}\n", packages.join(" ")));
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        } else if r.base.starts_with("archlinux:") {
            // @ligne [dockerfile] Ajoute cette donnée à la collection en cours de préparation.
            text.push_str(&format!(
                // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "RUN pacman -Syu --noconfirm {} && pacman -Scc --noconfirm\n",
                // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                packages.join(" ")
            ));
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        } else {
            // @ligne [dockerfile] Ajoute cette donnée à la collection en cours de préparation.
            text.push_str(&format!("RUN apt-get update && apt-get install -y --no-install-recommends {} && rm -rf /var/lib/apt/lists/*\n",packages.join(" ")));
        }
    }
    // @ligne [dockerfile] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let pdo = php_ext
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .iter()
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .filter(|s| s.as_str() != "redis")
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .cloned()
        // @ligne [dockerfile] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .collect::<Vec<_>>();
    // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
    if !pdo.is_empty() {
        // @ligne [dockerfile] Ajoute cette donnée à la collection en cours de préparation.
        text.push_str(&format!("RUN docker-php-ext-install {}\n", pdo.join(" ")));
    }
    // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
    if php_ext.iter().any(|s| s == "redis") {
        // @ligne [dockerfile] Ajoute cette donnée à la collection en cours de préparation.
        text.push_str("RUN pecl install redis && docker-php-ext-enable redis\n");
    }
    // @ligne [dockerfile] Ajoute cette donnée à la collection en cours de préparation.
    text.push_str("WORKDIR /app\n");
    // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
    if r.project.as_deref().is_some_and(|s| !s.is_empty()) {
        // @ligne [dockerfile] Ajoute cette donnée à la collection en cours de préparation.
        text.push_str("COPY project/ /app/\n");
    }
    // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
    if let Some(port) = r.port {
        // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
        if port == 0 {
            // @ligne [dockerfile] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Port d’image invalide");
        }
        // @ligne [dockerfile] Ajoute cette donnée à la collection en cours de préparation.
        text.push_str(&format!("EXPOSE {port}\n"));
    }
    // @ligne [dockerfile] Vérifie cette condition avant d’appliquer l’action protégée.
    if !r.command.is_empty() {
        // @ligne [dockerfile] Ajoute cette donnée à la collection en cours de préparation.
        text.push_str(&format!("CMD {}\n", serde_json::to_string(&r.command)?));
    }
    // @ligne [dockerfile] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(text)
}

/// L’archive reste sur disque ; ni .env, ni .git, ni liens symboliques ne sont embarqués.
// @ligne [context] Délimite cette opération pour isoler sa validation et son résultat.
pub fn context(r: &Recipe, recipe: &str) -> Result<tempfile::NamedTempFile> {
    // @ligne [context] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut file = tempfile::NamedTempFile::new()?;
    {
        // @ligne [context] Prépare une valeur locale qui sera complétée pendant cette opération.
        let mut tar = tar::Builder::new(file.as_file_mut());
        // @ligne [context] Prépare une valeur locale qui sera complétée pendant cette opération.
        let mut header = tar::Header::new_gnu();
        // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        header.set_size(recipe.len() as u64);
        // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        header.set_mode(0o644);
        // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        header.set_cksum();
        // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        tar.append_data(&mut header, "Dockerfile", recipe.as_bytes())?;
        // @ligne [context] Vérifie cette condition avant d’appliquer l’action protégée.
        if let Some(project) = r.project.as_deref().filter(|s| !s.is_empty()) {
            // @ligne [context] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let root = Path::new(project).canonicalize()?;
            // @ligne [context] Vérifie cette condition avant d’appliquer l’action protégée.
            if !root.is_dir() {
                // @ligne [context] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
                bail!("Le projet doit être un dossier");
            }
            // @ligne [context] Prépare une valeur locale qui sera complétée pendant cette opération.
            let mut total = 0u64;
            // @ligne [context] Prépare une valeur locale qui sera complétée pendant cette opération.
            let mut count = 0;
            // @ligne [context] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let walker = walkdir::WalkDir::new(&root)
                // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .follow_links(false)
                // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .max_depth(32)
                // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .into_iter()
                // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .filter_entry(|e| {
                    // @ligne [context] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
                    let name = e.file_name().to_string_lossy();
                    // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    ![".git", "node_modules", "target", ".ssh", ".aws", ".gnupg"]
                        // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        .contains(&name.as_ref())
                        // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        && !name.starts_with(".env")
                });
            // @ligne [context] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
            for entry in walker {
                // @ligne [context] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
                let entry = entry?;
                // @ligne [context] Vérifie cette condition avant d’appliquer l’action protégée.
                if !entry.file_type().is_file() {
                    // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    continue;
                }
                // @ligne [context] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
                let relative = entry.path().strip_prefix(&root)?;
                // @ligne [context] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
                let meta = entry.metadata()?;
                // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                total = total
                    // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    .checked_add(meta.len())
                    // @ligne [context] Ajoute un contexte lisible à l’échec sans l’ignorer.
                    .context("Contexte trop grand")?;
                // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                count += 1;
                // @ligne [context] Vérifie cette condition avant d’appliquer l’action protégée.
                if total > 512 * 1024 * 1024 || count > 20000 {
                    // @ligne [context] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
                    bail!(
                        // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        "Contexte limité à 512 Mio et 20 000 fichiers ; réduisez le dossier projet"
                    );
                }
                // @ligne [context] Prépare une valeur locale qui sera complétée pendant cette opération.
                let mut options = std::fs::OpenOptions::new();
                // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                options.read(true);
                // @ligne [context] Applique cet attribut au type ou à la fonction qui suit.
                #[cfg(unix)]
                {
                    // @ligne [context] Importe explicitement les types et fonctions nécessaires à ce module.
                    use std::os::unix::fs::OpenOptionsExt;
                    // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    options.custom_flags(libc::O_NOFOLLOW);
                }
                // @ligne [context] Prépare une valeur locale qui sera complétée pendant cette opération.
                let mut input = options.open(entry.path())?;
                // @ligne [context] Vérifie cette condition avant d’appliquer l’action protégée.
                if !input.metadata()?.is_file() {
                    // @ligne [context] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
                    bail!("Un fichier a changé pendant la préparation");
                }
                // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                tar.append_file(Path::new("project").join(relative), &mut input)?;
            }
        }
        // @ligne [context] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        tar.finish()?;
    }
    // @ligne [context] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(file)
}

// @ligne [build] Délimite cette opération pour isoler sa validation et son résultat.
pub async fn build(r: Recipe, store: Arc<Store>, out: Reporter) -> Result<String> {
    // @ligne [build] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let recipe = dockerfile(&r)?;
    // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    report(
        // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        &out,
        // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        &r.operation,
        // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "context",
        // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "Préparation du contexte filtré, limité à 512 Mio",
    );
    // @ligne [build] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let task = r.clone();
    // @ligne [build] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let content = recipe.clone();
    // @ligne [build] Attend l’opération asynchrone avant d’utiliser son résultat.
    let archive = tokio::task::spawn_blocking(move || context(&task, &content)).await??;
    // @ligne [build] Attend l’opération asynchrone avant d’utiliser son résultat.
    let file = tokio::fs::File::open(archive.path()).await?;
    // @ligne [build] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let stream = tokio_util::io::ReaderStream::new(file);
    // @ligne [build] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let docker = doctor::docker()?;
    // @ligne [build] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let options = BuildImageOptionsBuilder::default()
        // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .dockerfile("Dockerfile")
        // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .t(&r.tag)
        // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .rm(true)
        // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .forcerm(true)
        // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .version(BuilderVersion::BuilderBuildKit)
        // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .session(&format!("dockforge{:032x}", rand::random::<u128>()))
        // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .build();
    // @ligne [build] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut build = docker.build_image(options, None, Some(bollard::body_try_stream(stream)));
    // @ligne [build] Attend l’opération asynchrone avant d’utiliser son résultat.
    while let Some(event) = build.next().await {
        // @ligne [build] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let info = event?;
        // @ligne [build] Vérifie cette condition avant d’appliquer l’action protégée.
        if let Some(error) = info.error {
            // @ligne [build] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Build refusé : {}", crate::suite::redact(&error));
        }
        // @ligne [build] Vérifie cette condition avant d’appliquer l’action protégée.
        if let Some(line) = info.stream {
            // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            report(&out, &r.operation, "build", crate::suite::redact(&line));
        }
    }
    // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    docker
        // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .inspect_image(&r.tag)
        // @ligne [build] Attend l’opération asynchrone avant d’utiliser son résultat.
        .await
        // @ligne [build] Ajoute un contexte lisible à l’échec sans l’ignorer.
        .context("Le build n’a pas produit l’image attendue")?;
    // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    store.save_image(&r.tag, &serde_json::to_string(&r)?)?;
    // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    report(
        // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        &out,
        // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        &r.operation,
        // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "complete",
        // @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "Image construite et enregistrée ; vous pouvez créer son conteneur",
    );
    // @ligne [build] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(r.tag)
}

// @ligne [build] Applique cet attribut au type ou à la fonction qui suit.
#[cfg(test)]
// @ligne [build] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
mod tests {
    // @ligne [build] Importe explicitement les types et fonctions nécessaires à ce module.
    use super::*;
    // @ligne [recipe] Délimite cette opération pour isoler sa validation et son résultat.
    fn recipe() -> Recipe {
        // @ligne [recipe] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        Recipe {
            // @ligne [recipe] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            artifact: None,
            // @ligne [recipe] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            operation: "test".into(),
            // @ligne [recipe] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            tag: "my-api:v1".into(),
            // @ligne [recipe] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            base: "python:3.12-slim".into(),
            // @ligne [recipe] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            packages: vec!["curl".into()],
            // @ligne [recipe] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            project: None,
            // @ligne [recipe] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            command: vec!["python".into(), "main.py".into()],
            // @ligne [recipe] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            port: Some(8000),
        }
    }
    // @ligne [recipe] Applique cet attribut au type ou à la fonction qui suit.
    #[test]
    // @ligne [dockerfile_is_injection_resistant] Délimite cette opération pour isoler sa validation et son résultat.
    fn dockerfile_is_injection_resistant() {
        // @ligne [dockerfile_is_injection_resistant] Prépare une valeur locale qui sera complétée pendant cette opération.
        let mut r = recipe();
        // @ligne [dockerfile_is_injection_resistant] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(
            // @ligne [dockerfile_is_injection_resistant] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            dockerfile(&r)
                // @ligne [dockerfile_is_injection_resistant] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .unwrap()
                // @ligne [dockerfile_is_injection_resistant] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .contains("CMD [\"python\",\"main.py\"]")
        );
        // @ligne [dockerfile_is_injection_resistant] Ajoute cette donnée à la collection en cours de préparation.
        r.packages.push("curl; touch /bad".into());
        // @ligne [dockerfile_is_injection_resistant] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(dockerfile(&r).is_err());
    }
    // @ligne [dockerfile_is_injection_resistant] Applique cet attribut au type ou à la fonction qui suit.
    #[test]
    // @ligne [context_skips_secrets_and_symlinks] Délimite cette opération pour isoler sa validation et son résultat.
    fn context_skips_secrets_and_symlinks() {
        // @ligne [context_skips_secrets_and_symlinks] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let tmp = tempfile::tempdir().unwrap();
        // @ligne [context_skips_secrets_and_symlinks] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        std::fs::write(tmp.path().join(".env"), "secret").unwrap();
        // @ligne [context_skips_secrets_and_symlinks] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        std::fs::write(tmp.path().join("main.py"), "print(1)").unwrap();
        // @ligne [context_skips_secrets_and_symlinks] Applique cet attribut au type ou à la fonction qui suit.
        #[cfg(unix)]
        // @ligne [context_skips_secrets_and_symlinks] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        std::os::unix::fs::symlink("/etc/passwd", tmp.path().join("escape")).unwrap();
        // @ligne [context_skips_secrets_and_symlinks] Prépare une valeur locale qui sera complétée pendant cette opération.
        let mut r = recipe();
        // @ligne [context_skips_secrets_and_symlinks] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        r.project = Some(tmp.path().to_string_lossy().into_owned());
        // @ligne [context_skips_secrets_and_symlinks] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let file = context(&r, &dockerfile(&r).unwrap()).unwrap();
        // @ligne [context_skips_secrets_and_symlinks] Prépare une valeur locale qui sera complétée pendant cette opération.
        let mut archive = tar::Archive::new(file.reopen().unwrap());
        // @ligne [context_skips_secrets_and_symlinks] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let paths = archive
            // @ligne [context_skips_secrets_and_symlinks] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .entries()
            // @ligne [context_skips_secrets_and_symlinks] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap()
            // @ligne [context_skips_secrets_and_symlinks] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .map(|e| e.unwrap().path().unwrap().to_string_lossy().into_owned())
            // @ligne [context_skips_secrets_and_symlinks] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .collect::<Vec<_>>();
        // @ligne [context_skips_secrets_and_symlinks] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(paths, vec!["Dockerfile", "project/main.py"]);
    }
}

// @ligne [context_skips_secrets_and_symlinks] Applique cet attribut au type ou à la fonction qui suit.
#[cfg(test)]
// @ligne [context_skips_secrets_and_symlinks] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
mod live_tests {
    // @ligne [context_skips_secrets_and_symlinks] Importe explicitement les types et fonctions nécessaires à ce module.
    use super::*;
    // @ligne [context_skips_secrets_and_symlinks] Applique cet attribut au type ou à la fonction qui suit.
    #[tokio::test]
    // @ligne [context_skips_secrets_and_symlinks] Applique cet attribut au type ou à la fonction qui suit.
    #[ignore = "Nécessite Docker avec BuildKit et accès au registre Alpine"]
    // @ligne [real_buildkit_creates_a_usable_image] Délimite cette opération pour isoler sa validation et son résultat.
    async fn real_buildkit_creates_a_usable_image() {
        // @ligne [real_buildkit_creates_a_usable_image] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let tmp = tempfile::tempdir().unwrap();
        // @ligne [real_buildkit_creates_a_usable_image] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let store = Arc::new(Store::open(tmp.path()).unwrap());
        // @ligne [real_buildkit_creates_a_usable_image] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let tag = format!("dockforge-test-image:{:08x}", rand::random::<u32>());
        // @ligne [real_buildkit_creates_a_usable_image] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let recipe = Recipe {
            // @ligne [real_buildkit_creates_a_usable_image] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            operation: "test-build".into(),
            // @ligne [real_buildkit_creates_a_usable_image] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            tag: tag.clone(),
            // @ligne [real_buildkit_creates_a_usable_image] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            base: "alpine:3.20".into(),
            // @ligne [real_buildkit_creates_a_usable_image] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            packages: vec![],
            // @ligne [real_buildkit_creates_a_usable_image] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            project: None,
            // @ligne [real_buildkit_creates_a_usable_image] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            command: vec!["echo".into(), "dockforge-buildkit-ok".into()],
            // @ligne [real_buildkit_creates_a_usable_image] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            port: None,
            // @ligne [real_buildkit_creates_a_usable_image] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            artifact: None,
        };
        // @ligne [real_buildkit_creates_a_usable_image] Attend l’opération asynchrone avant d’utiliser son résultat.
        let result = build(recipe, store.clone(), Arc::new(|_| {})).await;
        // @ligne [real_buildkit_creates_a_usable_image] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let docker = doctor::docker().unwrap();
        // @ligne [real_buildkit_creates_a_usable_image] Attend l’opération asynchrone avant d’utiliser son résultat.
        let present = docker.inspect_image(&tag).await.is_ok();
        // @ligne [real_buildkit_creates_a_usable_image] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let _ = docker
            // @ligne [real_buildkit_creates_a_usable_image] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .remove_image(
                // @ligne [real_buildkit_creates_a_usable_image] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                &tag,
                // @ligne [real_buildkit_creates_a_usable_image] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                None::<bollard::query_parameters::RemoveImageOptions>,
                // @ligne [real_buildkit_creates_a_usable_image] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                None,
            )
            // @ligne [real_buildkit_creates_a_usable_image] Attend l’opération asynchrone avant d’utiliser son résultat.
            .await;
        // @ligne [real_buildkit_creates_a_usable_image] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(result.unwrap(), tag);
        // @ligne [real_buildkit_creates_a_usable_image] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(present);
        // @ligne [real_buildkit_creates_a_usable_image] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(store.images().unwrap().len(), 1);
    }
}
