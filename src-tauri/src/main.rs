// Une ouverture desktop Windows ne crée pas de console supplémentaire en release.
// @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
#![cfg_attr(
    // @ligne [module] Vérifie cette propriété observable pour détecter une régression du contrat.
    all(target_os = "windows", not(debug_assertions)),
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    windows_subsystem = "windows"
)]
//! Adaptateur desktop : la fenêtre ne reçoit que des commandes métier, jamais un shell général.
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use dockforge_core::{
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    Reporter, catalog, doctor, engine, images,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    share::Sharing,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    store::{self, Store},
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    suite,
};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use serde_json::{Value, json};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use std::sync::Arc;
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use tauri::{Emitter, Manager};

// @ligne [module] Définit les données échangées par ce module, sans comportement implicite.
struct AppState {
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    store: Arc<Store>,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    sharing: Sharing,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    mutation: tokio::sync::Mutex<()>,
}

// @ligne [payload] Délimite cette opération pour isoler sa validation et son résultat.
fn payload<T: serde::de::DeserializeOwned>(value: Value) -> anyhow::Result<T> {
    // @ligne [payload] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(serde_json::from_value(value)?)
}
// @ligne [text] Délimite cette opération pour isoler sa validation et son résultat.
fn text<'a>(value: &'a Value, key: &str) -> anyhow::Result<&'a str> {
    // @ligne [text] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    value[key]
        // @ligne [text] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .as_str()
        // @ligne [text] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .ok_or_else(|| anyhow::anyhow!("Champ {key} absent"))
}

// @ligne [execute] Délimite cette opération pour isoler sa validation et son résultat.
async fn execute(
    // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    app: tauri::AppHandle,
    // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    state: &AppState,
    // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    action: &str,
    // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    p: Value,
// @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
) -> anyhow::Result<Value> {
    // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let emit: Reporter = Arc::new(move |event| {
        // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let _ = app.emit("dockforge:progress", event);
    });
    // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    match action {
        // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
        "catalog" => Ok(serde_json::to_value(catalog::catalog())?),
        // @ligne [execute] Attend l’opération asynchrone avant d’utiliser son résultat.
        "doctor" => Ok(serde_json::to_value(doctor::inspect().await)?),
        // @ligne [execute] Attend l’opération asynchrone avant d’utiliser son résultat.
        "remedy" => Ok(json!(doctor::remedy(text(&p, "action")?).await?)),
        // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
        "port" => Ok(serde_json::to_value(doctor::port_check(
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            text(&p, "ip")?,
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            p["port"]
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .as_u64()
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .and_then(|n| u16::try_from(n).ok())
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .ok_or_else(|| anyhow::anyhow!("Port invalide"))?,
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            p["protocol"].as_str().unwrap_or("tcp"),
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        )?)?),
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "generate_password" => {
            // @ligne [execute] Importe explicitement les types et fonctions nécessaires à ce module.
            use rand::Rng;
            // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let alphabet = b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz23456789!@#$%_-+";
            // @ligne [execute] Prépare une valeur locale qui sera complétée pendant cette opération.
            let mut rng = rand::rngs::OsRng;
            // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let password = (0..24)
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .map(|_| alphabet[rng.gen_range(0..alphabet.len())] as char)
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .collect::<String>();
            // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(json!(password))
        }
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "create" => {
            // @ligne [execute] Sérialise l’accès à cet état partagé entre les tâches.
            let _guard = state.mutation.lock().await;
            // @ligne [execute] Attend l’opération asynchrone avant d’utiliser son résultat.
            engine::create(payload(p)?, state.store.clone(), emit).await
        }
        // @ligne [execute] Attend l’opération asynchrone avant d’utiliser son résultat.
        "containers" => engine::containers().await,
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "networks" => {
            // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let docker = doctor::docker()?;
            // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(serde_json::to_value(
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                docker
                    // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    .list_networks(None::<bollard::query_parameters::ListNetworksOptions>)
                    // @ligne [execute] Attend l’opération asynchrone avant d’utiliser son résultat.
                    .await?,
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            )?)
        }
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "logs" => {
            // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let id = text(&p, "id")?;
            // @ligne [execute] Attend l’opération asynchrone avant d’utiliser son résultat.
            let raw = engine::logs(id).await?;
            // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let data = state.store.credentials(id).ok();
            // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let env = data
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .as_ref()
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .and_then(|d| d["credentials"]["environment"].as_object())
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .map(|m| {
                    // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    m.iter()
                        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        .filter_map(|(k, v)| Some((k.clone(), v.as_str()?.to_string())))
                        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        .collect()
                })
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .unwrap_or_default();
            // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(json!(engine::scrub_environment(&raw, &env)))
        }
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "link_database" => {
            // @ligne [execute] Attend l’opération asynchrone avant d’utiliser son résultat.
            engine::link_database(text(&p, "source")?, text(&p, "target")?, &state.store).await
        }
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "lifecycle" => {
            // @ligne [execute] Sérialise l’accès à cet état partagé entre les tâches.
            let _guard = state.mutation.lock().await;
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            engine::lifecycle(
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                text(&p, "id")?,
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                text(&p, "action")?,
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                &state.store,
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                p["archive"].as_bool().unwrap_or(true),
            )
            // @ligne [execute] Attend l’opération asynchrone avant d’utiliser son résultat.
            .await?;
            // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(json!(true))
        }
        // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
        "vault" => Ok(serde_json::to_value(state.store.list()?)?),
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "unlock" => {
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            state.store.unlock()?;
            // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(json!(true))
        }
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "lock" => {
            // @ligne [execute] Sérialise l’accès à cet état partagé entre les tâches.
            state.store.lock();
            // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(json!(true))
        }
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "credentials" => {
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            state.store.unlock()?;
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            state.store.credentials(text(&p, "id")?)
        }
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "export" => {
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            state.store.unlock()?;
            // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let data = state.store.credentials(text(&p, "id")?)?;
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            store::export_private(
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                std::path::Path::new(text(&p, "path")?),
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                &serde_json::to_vec_pretty(&data)?,
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            )?;
            // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(json!(true))
        }
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "backup" => {
            // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let store = state.store.clone();
            // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let path = std::path::PathBuf::from(text(&p, "path")?);
            // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let password = zeroize::Zeroizing::new(text(&p, "password")?.to_string());
            // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(json!(
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                tokio::task::spawn_blocking(move || dockforge_core::backup::export(
                    // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    &store, &path, &password
                ))
                // @ligne [execute] Attend l’opération asynchrone avant d’utiliser son résultat.
                .await??
            ))
        }
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "restore" => {
            // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let store = state.store.clone();
            // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let path = std::path::PathBuf::from(text(&p, "path")?);
            // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let password = zeroize::Zeroizing::new(text(&p, "password")?.to_string());
            // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(json!(
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                tokio::task::spawn_blocking(move || dockforge_core::backup::restore(
                    // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    &store, &path, &password
                ))
                // @ligne [execute] Attend l’opération asynchrone avant d’utiliser son résultat.
                .await??
            ))
        }
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "retire" => {
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            state
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .store
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .retire(text(&p, "id")?, p["archive"].as_bool().unwrap_or(true))?;
            // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(json!(true))
        }
        // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
        "image_options" => Ok(json!({"bases":images::BASES,"packages":images::PACKAGES})),
        // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
        "dockerfile" => Ok(json!(images::dockerfile(&payload(p)?)?)),
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "build" => {
            // @ligne [execute] Sérialise l’accès à cet état partagé entre les tâches.
            let _guard = state.mutation.lock().await;
            // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(json!(
                // @ligne [execute] Attend l’opération asynchrone avant d’utiliser son résultat.
                images::build(payload(p)?, state.store.clone(), emit).await?
            ))
        }
        // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
        "images" => Ok(json!(state.store.images()?)),
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "import" => {
            // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let shell = text(&p, "shell")?;
            // @ligne [execute] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let commands = suite::import(std::path::Path::new(text(&p, "path")?), shell)?;
            // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(json!(state.store.import_commands(commands, shell)?))
        }
        // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
        "history_sources" => Ok(json!(state.store.history_sources()?)),
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "watch_history" => {
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            state.store.watch_history(
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                text(&p, "path")?,
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                text(&p, "shell")?,
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                p["enabled"].as_bool().unwrap_or(false),
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            )?;
            // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(json!(true))
        }
        // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
        "commands" => Ok(json!(state.store.commands(
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            p["search"].as_str().unwrap_or(""),
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            p["favorites"].as_bool().unwrap_or(false)
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        )?)),
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "favorite" => {
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            state.store.favorite(
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                p["id"]
                    // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    .as_i64()
                    // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    .ok_or_else(|| anyhow::anyhow!("ID invalide"))?,
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                p["value"].as_bool().unwrap_or(false),
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            )?;
            // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(json!(true))
        }
        // @ligne [execute] Attend l’opération asynchrone avant d’utiliser son résultat.
        "netmap" => suite::netmap(text(&p, "cidr")?).await,
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "share_start" => {
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            state
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .sharing
                // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .start(
                    // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    payload(p["paths"].clone())?,
                    // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    text(&p, "ip")?,
                    // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    p["port"]
                        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        .as_u64()
                        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        .and_then(|n| u16::try_from(n).ok())
                        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        .ok_or_else(|| anyhow::anyhow!("Port invalide"))?,
                    // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    state.store.root(),
                )
                // @ligne [execute] Attend l’opération asynchrone avant d’utiliser son résultat.
                .await
        }
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "share_stop" => {
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            state.sharing.stop();
            // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(json!(true))
        }
        // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
        "share_peers" => Ok(json!(state.sharing.peers())),
        // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "share_role" => {
            // @ligne [execute] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            state.sharing.role(text(&p, "id")?, text(&p, "role")?)?;
            // @ligne [execute] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(json!(true))
        }
        // @ligne [execute] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        _ => anyhow::bail!("Commande métier inconnue"),
    }
}

// @ligne [execute] Applique cet attribut au type ou à la fonction qui suit.
#[tauri::command]
// @ligne [dispatch] Délimite cette opération pour isoler sa validation et son résultat.
async fn dispatch(
    // @ligne [dispatch] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    app: tauri::AppHandle,
    // @ligne [dispatch] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    state: tauri::State<'_, AppState>,
    // @ligne [dispatch] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    action: String,
    // @ligne [dispatch] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    data: Option<Value>,
// @ligne [dispatch] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
) -> Result<Value, String> {
    // @ligne [dispatch] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    execute(app, &state, &action, data.unwrap_or(json!({})))
        // @ligne [dispatch] Attend l’opération asynchrone avant d’utiliser son résultat.
        .await
        // @ligne [dispatch] Ajoute un contexte lisible à l’échec sans l’ignorer.
        .map_err(|e| {
            // @ligne [dispatch] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            serde_json::to_string(&engine::diagnose(&format!("{e:#}")))
                // @ligne [dispatch] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .unwrap_or_else(|_| "Erreur locale".into())
        })
}

// @ligne [dispatch] Applique cet attribut au type ou à la fonction qui suit.
#[cfg(windows)]
// @ligne [attach_parent_console_for_cli] Délimite cette opération pour isoler sa validation et son résultat.
fn attach_parent_console_for_cli() {
    // Les signatures correspondent à Kernel32 : aucune mémoire fournie par un appelant n’est déréférencée.
    // @ligne [attach_parent_console_for_cli] Applique cet attribut au type ou à la fonction qui suit.
    #[link(name = "kernel32")]
    // @ligne [attach_parent_console_for_cli] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    unsafe extern "system" {
        // @ligne [GetStdHandle] Délimite cette opération pour isoler sa validation et son résultat.
        fn GetStdHandle(kind: u32) -> *mut core::ffi::c_void;
        // @ligne [AttachConsole] Délimite cette opération pour isoler sa validation et son résultat.
        fn AttachConsole(process_id: u32) -> i32;
    }
    // Un handle hérité valide peut être un fichier ou un pipe : il faut préserver les redirections.
    // @ligne [AttachConsole] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let output = unsafe { GetStdHandle((-11_i32) as u32) };
    // @ligne [AttachConsole] Vérifie cette condition avant d’appliquer l’action protégée.
    if output.is_null() || output as isize == -1 {
        // ATTACH_PARENT_PROCESS utilise seulement la console existante du parent, sans en créer une.
        // @ligne [AttachConsole] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let _ = unsafe { AttachConsole(u32::MAX) };
    }
}

// @ligne [main] Délimite cette opération pour isoler sa validation et son résultat.
fn main() {
    // @ligne [main] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let _ = rustls::crypto::ring::default_provider().install_default();
    // @ligne [main] Vérifie cette condition avant d’appliquer l’action protégée.
    if std::env::args().any(|a| a == "--doctor") {
        // @ligne [main] Applique cet attribut au type ou à la fonction qui suit.
        #[cfg(windows)]
        // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        attach_parent_console_for_cli();
        // @ligne [main] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let runtime = tokio::runtime::Runtime::new().expect("Runtime système");
        // @ligne [main] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let report = runtime.block_on(doctor::inspect());
        // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
        // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        return;
    }
    // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    tauri::Builder::default()
        // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .plugin(tauri_plugin_dialog::init())
        // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .plugin(tauri_plugin_opener::init())
        // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .setup(|app| {
            // @ligne [main] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let root = std::env::var_os("DOCKFORGE_DATA_DIR")
                // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .map(std::path::PathBuf::from)
                // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .unwrap_or(app.path().app_data_dir()?);
            // @ligne [main] Vérifie cette condition avant d’appliquer l’action protégée.
            if !root.is_absolute() {
                // @ligne [main] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
                return Err("DOCKFORGE_DATA_DIR doit être absolu".into());
            }
            // @ligne [main] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let store = Arc::new(Store::open(&root)?);
            // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            tauri::async_runtime::spawn(suite::watch_histories(store.clone()));
            // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            app.manage(AppState {
                // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                store,
                // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                sharing: Sharing::default(),
                // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                mutation: tokio::sync::Mutex::new(()),
            });
            // @ligne [main] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(())
        })
        // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .invoke_handler(tauri::generate_handler![dispatch])
        // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .run(tauri::generate_context!())
        // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .expect("Impossible de démarrer la fenêtre DockForge");
}
