//! Partage HTTPS volontaire, droits relus à chaque requête et chemins jamais fournis par le visiteur.
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use anyhow::{Context, Result, bail};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use axum::{
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    Json, Router,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    body::{Body, to_bytes},
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    extract::{ConnectInfo, DefaultBodyLimit, Path, State},
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    http::{HeaderMap, StatusCode, header},
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    response::{Html, IntoResponse, Response},
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    routing::{get, post},
};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use axum_server::{Handle, tls_rustls::RustlsConfig};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use serde::{Deserialize, Serialize};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use serde_json::{Value, json};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use sha2::{Digest, Sha256};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use std::{
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    collections::HashMap,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    net::{IpAddr, SocketAddr},
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    path::PathBuf,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    sync::{
        // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        Arc, Mutex,
        // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        atomic::{AtomicU64, Ordering},
    },
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    time::{Duration, Instant},
};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use tokio::io::{AsyncReadExt, AsyncSeekExt};

// @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
type HttpError = (StatusCode, String);

// @ligne [module] Applique cet attribut au type ou à la fonction qui suit.
#[derive(Clone)]
// @ligne [module] Définit les données échangées par ce module, sans comportement implicite.
struct SharedFile {
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    id: String,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    name: String,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    path: PathBuf,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    size: u64,
}
// @ligne [module] Applique cet attribut au type ou à la fonction qui suit.
#[derive(Clone, Serialize)]
// @ligne [module] Définit les données échangées par ce module, sans comportement implicite.
pub struct Peer {
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub id: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub name: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub role: String,
}
// @ligne [module] Définit les données échangées par ce module, sans comportement implicite.
struct JoinBudget {
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    attempts: u32,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    since: Instant,
}
// @ligne [module] Définit les données échangées par ce module, sans comportement implicite.
struct Room {
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    pin: String,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    cookie: String,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    files: Mutex<Vec<SharedFile>>,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    peers: Mutex<HashMap<String, Peer>>,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    budget: Mutex<HashMap<IpAddr, JoinBudget>>,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    expiry: Instant,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    uploads: PathBuf,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    contributed: AtomicU64,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    upload_gate: tokio::sync::Semaphore,
}

// @ligne [module] Définit les données échangées par ce module, sans comportement implicite.
pub struct Sharing {
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    active: Mutex<Option<(Arc<Room>, Handle)>>,
}
// @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
impl Default for Sharing {
    // @ligne [default] Délimite cette opération pour isoler sa validation et son résultat.
    fn default() -> Self {
        // @ligne [default] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        Self {
            // @ligne [default] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            active: Mutex::new(None),
        }
    }
}

// @ligne [id] Délimite cette opération pour isoler sa validation et son résultat.
fn id() -> String {
    // @ligne [id] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    format!("{:032x}", rand::random::<u128>())
}
// @ligne [digest] Délimite cette opération pour isoler sa validation et son résultat.
fn digest(value: &str) -> String {
    // @ligne [digest] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    hex::encode(Sha256::digest(value.as_bytes()))
}
// @ligne [deny] Délimite cette opération pour isoler sa validation et son résultat.
fn deny(message: &str) -> HttpError {
    // @ligne [deny] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    (StatusCode::FORBIDDEN, message.into())
}

// @ligne [peer] Délimite cette opération pour isoler sa validation et son résultat.
fn peer(room: &Room, headers: &HeaderMap) -> Result<Peer, HttpError> {
    // @ligne [peer] Vérifie cette condition avant d’appliquer l’action protégée.
    if Instant::now() > room.expiry {
        // @ligne [peer] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        return Err(deny("Salle expirée"));
    }
    // @ligne [peer] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let cookie = headers
        // @ligne [peer] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .get(header::COOKIE)
        // @ligne [peer] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .and_then(|h| h.to_str().ok())
        // @ligne [peer] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .unwrap_or("");
    // @ligne [peer] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let token = cookie
        // @ligne [peer] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .split(';')
        // @ligne [peer] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .find_map(|p| p.trim().strip_prefix(&format!("{}=", room.cookie)))
        // @ligne [peer] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .unwrap_or("");
    // @ligne [peer] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    room.peers
        // @ligne [peer] Sérialise l’accès à cet état partagé entre les tâches.
        .lock()
        // @ligne [peer] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .unwrap()
        // @ligne [peer] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .get(&digest(token))
        // @ligne [peer] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .cloned()
        // @ligne [peer] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .ok_or_else(|| deny("Rejoignez la salle avec son PIN"))
}

// @ligne [peer] Applique cet attribut au type ou à la fonction qui suit.
#[derive(Deserialize)]
// @ligne [peer] Définit les données échangées par ce module, sans comportement implicite.
struct Join {
    // @ligne [peer] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    pin: String,
    // @ligne [peer] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    name: String,
}
// @ligne [join] Délimite cette opération pour isoler sa validation et son résultat.
async fn join(
    // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    State(room): State<Arc<Room>>,
    // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    Json(req): Json<Join>,
// @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
) -> Result<Response, HttpError> {
    // @ligne [join] Vérifie cette condition avant d’appliquer l’action protégée.
    if Instant::now() > room.expiry {
        // @ligne [join] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        return Err(deny("Salle expirée"));
    }
    {
        // @ligne [join] Sérialise l’accès à cet état partagé entre les tâches.
        let mut limits = room.budget.lock().unwrap();
        // @ligne [join] Vérifie cette condition avant d’appliquer l’action protégée.
        if limits.len() > 1024 {
            // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            limits.retain(|_, v| v.since.elapsed() < Duration::from_secs(60));
        }
        // @ligne [join] Vérifie cette condition avant d’appliquer l’action protégée.
        if limits.len() > 1024 {
            // @ligne [join] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            return Err((
                // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                StatusCode::TOO_MANY_REQUESTS,
                // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "Salle occupée, réessayez".into(),
            ));
        }
        // @ligne [join] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let limit = limits.entry(addr.ip()).or_insert(JoinBudget {
            // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            attempts: 0,
            // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            since: Instant::now(),
        });
        // @ligne [join] Vérifie cette condition avant d’appliquer l’action protégée.
        if limit.since.elapsed() > Duration::from_secs(60) {
            // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            limit.attempts = 0;
            // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            limit.since = Instant::now();
        }
        // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        limit.attempts += 1;
        // @ligne [join] Vérifie cette condition avant d’appliquer l’action protégée.
        if limit.attempts > 6 {
            // @ligne [join] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            return Err((
                // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                StatusCode::TOO_MANY_REQUESTS,
                // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "Six essais par minute maximum".into(),
            ));
        }
    }
    // @ligne [join] Vérifie cette condition avant d’appliquer l’action protégée.
    if req.pin != room.pin {
        // @ligne [join] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        return Err(deny("PIN incorrect"));
    }
    // @ligne [join] Vérifie cette condition avant d’appliquer l’action protégée.
    if req.name.trim().is_empty() || req.name.len() > 80 {
        // @ligne [join] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        return Err((
            // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            StatusCode::BAD_REQUEST,
            // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "Nom de 1 à 80 caractères requis".into(),
        ));
    }
    // @ligne [join] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let token = id();
    // @ligne [join] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let p = Peer {
        // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        id: id(),
        // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        name: req.name,
        // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        role: "read".into(),
    };
    // @ligne [join] Sérialise l’accès à cet état partagé entre les tâches.
    let mut peers = room.peers.lock().unwrap();
    // @ligne [join] Vérifie cette condition avant d’appliquer l’action protégée.
    if peers.len() >= 100 {
        // @ligne [join] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        return Err(deny("Salle limitée à 100 participants"));
    }
    // @ligne [join] Ajoute cette donnée à la collection en cours de préparation.
    peers.insert(digest(&token), p);
    // @ligne [join] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut response = Json(json!({"ok":true})).into_response();
    // @ligne [join] Ajoute cette donnée à la collection en cours de préparation.
    response.headers_mut().insert(
        // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        header::SET_COOKIE,
        // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        format!(
            // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "{}={token}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=86400",
            // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            room.cookie
        )
        // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .parse()
        // @ligne [join] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .unwrap(),
    );
    // @ligne [join] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(response)
}

// @ligne [list_files] Délimite cette opération pour isoler sa validation et son résultat.
async fn list_files(
    // @ligne [list_files] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    State(room): State<Arc<Room>>,
    // @ligne [list_files] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    headers: HeaderMap,
// @ligne [list_files] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
) -> Result<Json<Value>, HttpError> {
    // @ligne [list_files] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let p = peer(&room, &headers)?;
    // @ligne [list_files] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let values = room
        // @ligne [list_files] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .files
        // @ligne [list_files] Sérialise l’accès à cet état partagé entre les tâches.
        .lock()
        // @ligne [list_files] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .unwrap()
        // @ligne [list_files] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .iter()
        // @ligne [list_files] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .map(|f| json!({"id":f.id,"name":f.name,"size":f.size}))
        // @ligne [list_files] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .collect::<Vec<_>>();
    // @ligne [list_files] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(Json(json!({"files":values,"role":p.role})))
}

// @ligne [download] Délimite cette opération pour isoler sa validation et son résultat.
async fn download(
    // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    State(room): State<Arc<Room>>,
    // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    Path(id): Path<String>,
    // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    headers: HeaderMap,
// @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
) -> Result<Response, HttpError> {
    // @ligne [download] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let _ = peer(&room, &headers)?;
    // @ligne [download] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let f = room
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .files
        // @ligne [download] Sérialise l’accès à cet état partagé entre les tâches.
        .lock()
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .unwrap()
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .iter()
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .find(|f| f.id == id)
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .cloned()
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .ok_or((StatusCode::NOT_FOUND, "Fichier absent".into()))?;
    // @ligne [download] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut input = tokio::fs::File::open(&f.path)
        // @ligne [download] Attend l’opération asynchrone avant d’utiliser son résultat.
        .await
        // @ligne [download] Ajoute un contexte lisible à l’échec sans l’ignorer.
        .map_err(|_| (StatusCode::NOT_FOUND, "Fichier source absent".into()))?;
    // @ligne [download] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let size = input
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .metadata()
        // @ligne [download] Attend l’opération asynchrone avant d’utiliser son résultat.
        .await
        // @ligne [download] Ajoute un contexte lisible à l’échec sans l’ignorer.
        .map_err(|_| deny("Métadonnées indisponibles"))?
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .len();
    // @ligne [download] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut start = 0;
    // @ligne [download] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut end = size.saturating_sub(1);
    // @ligne [download] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut partial = false;
    // @ligne [download] Vérifie cette condition avant d’appliquer l’action protégée.
    if let Some(range) = headers.get(header::RANGE).and_then(|h| h.to_str().ok()) {
        // @ligne [download] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let invalid = || {
            (
                // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                StatusCode::RANGE_NOT_SATISFIABLE,
                // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "Plage non prise en charge".into(),
            )
        };
        // @ligne [download] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let (a, b) = range
            // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .strip_prefix("bytes=")
            // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .and_then(|s| s.split_once('-'))
            // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .ok_or_else(invalid)?;
        // @ligne [download] Ajoute un contexte lisible à l’échec sans l’ignorer.
        start = a.parse::<u64>().map_err(|_| invalid())?;
        // @ligne [download] Vérifie cette condition avant d’appliquer l’action protégée.
        if !b.is_empty() {
            // @ligne [download] Ajoute un contexte lisible à l’échec sans l’ignorer.
            end = b.parse::<u64>().map_err(|_| invalid())?;
        }
        // @ligne [download] Vérifie cette condition avant d’appliquer l’action protégée.
        if start >= size || end < start || end >= size {
            // @ligne [download] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            return Err(invalid());
        }
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        partial = true;
    }
    // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    input
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .seek(std::io::SeekFrom::Start(start))
        // @ligne [download] Attend l’opération asynchrone avant d’utiliser son résultat.
        .await
        // @ligne [download] Ajoute un contexte lisible à l’échec sans l’ignorer.
        .map_err(|_| deny("Lecture impossible"))?;
    // @ligne [download] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let length = if size == 0 { 0 } else { end - start + 1 };
    // @ligne [download] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let stream = tokio_util::io::ReaderStream::new(input.take(length));
    // @ligne [download] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut result = Response::new(Body::from_stream(stream));
    *result.status_mut() = if partial {
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        StatusCode::PARTIAL_CONTENT
    // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else {
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        StatusCode::OK
    };
    // @ligne [download] Ajoute cette donnée à la collection en cours de préparation.
    result.headers_mut().insert(
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        header::CONTENT_TYPE,
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "application/octet-stream".parse().unwrap(),
    );
    // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    result
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .headers_mut()
        // @ligne [download] Ajoute cette donnée à la collection en cours de préparation.
        .insert(header::ACCEPT_RANGES, "bytes".parse().unwrap());
    // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    result
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .headers_mut()
        // @ligne [download] Ajoute cette donnée à la collection en cours de préparation.
        .insert(header::CONTENT_LENGTH, length.to_string().parse().unwrap());
    // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    result
        // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .headers_mut()
        // @ligne [download] Ajoute cette donnée à la collection en cours de préparation.
        .insert(header::CONTENT_DISPOSITION, "attachment".parse().unwrap());
    // @ligne [download] Vérifie cette condition avant d’appliquer l’action protégée.
    if partial {
        // @ligne [download] Ajoute cette donnée à la collection en cours de préparation.
        result.headers_mut().insert(
            // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            header::CONTENT_RANGE,
            // @ligne [download] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            format!("bytes {start}-{end}/{size}").parse().unwrap(),
        );
    }
    // @ligne [download] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(result)
}

// @ligne [upload] Délimite cette opération pour isoler sa validation et son résultat.
async fn upload(
    // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    State(room): State<Arc<Room>>,
    // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    headers: HeaderMap,
    // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    body: Body,
// @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
) -> Result<Json<Value>, HttpError> {
    // @ligne [upload] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let p = peer(&room, &headers)?;
    // @ligne [upload] Vérifie cette condition avant d’appliquer l’action protégée.
    if p.role != "contribute" {
        // @ligne [upload] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        return Err(deny("L’hôte doit accorder le droit de contribution"));
    }
    // Le droit et la limite de concurrence sont vérifiés avant de lire le corps réseau.
    // @ligne [upload] Ajoute un contexte lisible à l’échec sans l’ignorer.
    let _permit = room.upload_gate.try_acquire().map_err(|_| {
        (
            // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            StatusCode::TOO_MANY_REQUESTS,
            // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "Quatre envois simultanés maximum".into(),
        )
    // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    })?;
    // @ligne [upload] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let bytes = tokio::time::timeout(Duration::from_secs(180), to_bytes(body, 64 * 1024 * 1024))
        // @ligne [upload] Attend l’opération asynchrone avant d’utiliser son résultat.
        .await
        // @ligne [upload] Ajoute un contexte lisible à l’échec sans l’ignorer.
        .map_err(|_| (StatusCode::REQUEST_TIMEOUT, "Envoi trop lent".into()))?
        // @ligne [upload] Ajoute un contexte lisible à l’échec sans l’ignorer.
        .map_err(|_| {
            (
                // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                StatusCode::PAYLOAD_TOO_LARGE,
                // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "Fichier limité à 64 Mio".into(),
            )
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        })?;
    // @ligne [upload] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let raw = headers
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .get("x-file-name")
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .and_then(|h| h.to_str().ok())
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .unwrap_or("contribution.bin");
    // @ligne [upload] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let parsed =
        // @ligne [upload] Ajoute un contexte lisible à l’échec sans l’ignorer.
        url::Url::parse(&format!("https://local/?name={raw}")).map_err(|_| deny("Nom invalide"))?;
    // @ligne [upload] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let name = parsed
        // @ligne [upload] Passe par SQLite et ses paramètres pour séparer les données de la requête.
        .query_pairs()
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .find(|(k, _)| k == "name")
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .map(|(_, v)| v.into_owned())
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .unwrap_or_else(|| "contribution.bin".into());
    // @ligne [upload] Vérifie cette condition avant d’appliquer l’action protégée.
    if name.len() > 240
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        || name
            // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .chars()
            // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .any(|c| c == '/' || c == '\\' || c.is_control())
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        || name == "."
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        || name == ".."
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        || name.is_empty()
    {
        // @ligne [upload] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        return Err((StatusCode::BAD_REQUEST, "Nom de fichier invalide".into()));
    }
    // @ligne [upload] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let p = peer(&room, &headers)?;
    // @ligne [upload] Vérifie cette condition avant d’appliquer l’action protégée.
    if p.role != "contribute" {
        // @ligne [upload] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        return Err(deny(
            // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "Le droit de contribution a été retiré pendant l’envoi",
        ));
    }
    // @ligne [upload] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let size = bytes.len() as u64;
    // @ligne [upload] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut current = room.contributed.load(Ordering::SeqCst);
    // @ligne [upload] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
    loop {
        // @ligne [upload] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let next = current
            // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .checked_add(size)
            // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .filter(|n| *n <= 1024 * 1024 * 1024)
            // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .ok_or((
                // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                StatusCode::PAYLOAD_TOO_LARGE,
                // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "Quota de contributions 1 Gio atteint".into(),
            // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            ))?;
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        match room.contributed.compare_exchange_weak(
            // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            current,
            // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            next,
            // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            Ordering::SeqCst,
            // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            Ordering::SeqCst,
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        ) {
            // @ligne [upload] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(_) => break,
            // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            Err(updated) => current = updated,
        }
    }
    // @ligne [upload] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let file_id = id();
    // @ligne [upload] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let path = room.uploads.join(&file_id);
    // @ligne [upload] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let destination = path.clone();
    // @ligne [upload] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let result =
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        tokio::task::spawn_blocking(move || crate::store::export_private(&destination, &bytes))
            // @ligne [upload] Attend l’opération asynchrone avant d’utiliser son résultat.
            .await;
    // @ligne [upload] Vérifie cette condition avant d’appliquer l’action protégée.
    if !matches!(result, Ok(Ok(()))) {
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        room.contributed.fetch_sub(size, Ordering::SeqCst);
        // @ligne [upload] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let _ = std::fs::remove_file(&path);
        // @ligne [upload] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        return Err((
            // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            StatusCode::INSUFFICIENT_STORAGE,
            // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "Écriture impossible".into(),
        ));
    }
    // @ligne [upload] Sérialise l’accès à cet état partagé entre les tâches.
    room.files.lock().unwrap().push(SharedFile {
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        id: file_id.clone(),
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        name,
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        path,
        // @ligne [upload] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        size,
    });
    // @ligne [upload] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(Json(json!({"id":file_id})))
}

// @ligne [landing] Délimite cette opération pour isoler sa validation et son résultat.
async fn landing() -> Html<&'static str> {
    // @ligne [landing] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    Html(include_str!("share.html"))
}
// @ligne [client_script] Délimite cette opération pour isoler sa validation et son résultat.
async fn client_script() -> impl IntoResponse {
    (
        // @ligne [client_script] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
        // @ligne [client_script] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        include_str!("share.js"),
    )
}

// @ligne [client_script] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
impl Sharing {
    // @ligne [start] Délimite cette opération pour isoler sa validation et son résultat.
    pub async fn start(
        // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        &self,
        // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        paths: Vec<String>,
        // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        ip: &str,
        // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        port: u16,
        // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        root: &std::path::Path,
    // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    ) -> Result<Value> {
        // @ligne [start] Vérifie cette condition avant d’appliquer l’action protégée.
        if !crate::doctor::addresses().contains(&ip.to_string()) {
            // @ligne [start] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Choisissez une adresse locale explicite");
        }
        // @ligne [start] Vérifie cette condition avant d’appliquer l’action protégée.
        if port == 0 || paths.is_empty() || paths.len() > 100 {
            // @ligne [start] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Choisissez 1 à 100 fichiers et un port non nul");
        }
        // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        self.stop();
        // @ligne [start] Prépare une valeur locale qui sera complétée pendant cette opération.
        let mut files = Vec::new();
        // @ligne [start] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
        for p in paths {
            // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let path = std::path::Path::new(&p).canonicalize()?;
            // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let metadata = std::fs::metadata(&path)?;
            // @ligne [start] Vérifie cette condition avant d’appliquer l’action protégée.
            if !metadata.is_file() {
                // @ligne [start] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
                bail!(
                    // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    "Le partage direct accepte des fichiers ; archivez les dossiers avant de les ajouter"
                );
            }
            // @ligne [start] Ajoute cette donnée à la collection en cours de préparation.
            files.push(SharedFile {
                // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                id: id(),
                // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                name: path
                    // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    .file_name()
                    // @ligne [start] Ajoute un contexte lisible à l’échec sans l’ignorer.
                    .context("Nom absent")?
                    // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    .to_string_lossy()
                    // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    .into_owned(),
                // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                path,
                // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                size: metadata.len(),
            });
        }
        // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let tls = rcgen::generate_simple_self_signed(vec![ip.to_string(), "localhost".into()])?;
        // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let cert = tls.cert.pem();
        // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let key = tls.key_pair.serialize_pem();
        // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let fingerprint = hex::encode(Sha256::digest(tls.cert.der().as_ref()));
        // @ligne [start] Attend l’opération asynchrone avant d’utiliser son résultat.
        let config = RustlsConfig::from_pem(cert.into_bytes(), key.into_bytes()).await?;
        // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let pin = format!("{:06}", rand::random::<u32>() % 1_000_000);
        // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let uploads = root.join("contributions");
        // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        std::fs::create_dir_all(&uploads)?;
        // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let room = Arc::new(Room {
            // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            pin: pin.clone(),
            // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            cookie: format!("df_{}", id()),
            // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            files: Mutex::new(files),
            // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            peers: Mutex::new(HashMap::new()),
            // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            budget: Mutex::new(HashMap::new()),
            // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            expiry: Instant::now() + Duration::from_secs(86400),
            // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            uploads,
            // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            contributed: AtomicU64::new(0),
            // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            upload_gate: tokio::sync::Semaphore::new(4),
        });
        // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let router=Router::new().route("/",get(landing)).route("/client.js",get(client_script)).route("/api/join",post(join)).route("/api/files",get(list_files)).route("/api/files/{id}",get(download)).route("/api/upload",post(upload)).layer(DefaultBodyLimit::max(64*1024*1024)).layer(axum::middleware::map_response(|mut response:Response|async move {
            // @ligne [start] Ajoute cette donnée à la collection en cours de préparation.
            response.headers_mut().insert(header::CONTENT_SECURITY_POLICY,"default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; object-src 'none'; base-uri 'none'; frame-ancestors 'none'".parse().unwrap());
            // @ligne [start] Ajoute cette donnée à la collection en cours de préparation.
            response.headers_mut().insert(header::X_CONTENT_TYPE_OPTIONS,"nosniff".parse().unwrap());
            // @ligne [start] Ajoute cette donnée à la collection en cours de préparation.
            response.headers_mut().insert(header::CACHE_CONTROL,"no-store".parse().unwrap());response
        // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        })).with_state(room.clone());
        // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let address: SocketAddr = format!("{ip}:{port}").parse()?;
        // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let socket = std::net::TcpListener::bind(address)?;
        // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        socket.set_nonblocking(true)?;
        // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let handle = Handle::new();
        // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let runner = handle.clone();
        // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let expiry = handle.clone();
        // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        tokio::spawn(async move {
            // @ligne [start] Attend l’opération asynchrone avant d’utiliser son résultat.
            tokio::time::sleep(Duration::from_secs(86400)).await;
            // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            expiry.shutdown();
        });
        // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        tokio::spawn(async move {
            // @ligne [start] Vérifie cette condition avant d’appliquer l’action protégée.
            if let Err(e) = axum_server::from_tcp_rustls(socket, config)
                // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .handle(runner)
                // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .serve(router.into_make_service_with_connect_info::<SocketAddr>())
                // @ligne [start] Attend l’opération asynchrone avant d’utiliser son résultat.
                .await
            {
                // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                eprintln!("Serveur LAN arrêté : {e}");
            }
        });
        *self.active.lock().unwrap() = Some((room, handle));
        // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let url = format!("https://{ip}:{port}/");
        // @ligne [start] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let svg = qrcode::QrCode::new(url.as_bytes())?
            // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .render::<qrcode::render::svg::Color>()
            // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .min_dimensions(200, 200)
            // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .build();
        // @ligne [start] Renvoie une réussite explicite avec les données attendues par l’appelant.
        Ok(
            // @ligne [start] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            json!({"url":url,"pin":pin,"fingerprint_sha256":fingerprint,"qr_svg":svg,"expires_in_seconds":86400}),
        )
    }
    // @ligne [stop] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn stop(&self) {
        // @ligne [stop] Sérialise l’accès à cet état partagé entre les tâches.
        if let Some((_, handle)) = self.active.lock().unwrap().take() {
            // @ligne [stop] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            handle.shutdown();
        }
    }
    // @ligne [peers] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn peers(&self) -> Vec<Peer> {
        // @ligne [peers] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        self.active
            // @ligne [peers] Sérialise l’accès à cet état partagé entre les tâches.
            .lock()
            // @ligne [peers] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap()
            // @ligne [peers] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .as_ref()
            // @ligne [peers] Sérialise l’accès à cet état partagé entre les tâches.
            .map(|(room, _)| room.peers.lock().unwrap().values().cloned().collect())
            // @ligne [peers] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap_or_default()
    }
    // @ligne [role] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn role(&self, id: &str, role: &str) -> Result<()> {
        // @ligne [role] Vérifie cette condition avant d’appliquer l’action protégée.
        if !["read", "contribute", "expel"].contains(&role) {
            // @ligne [role] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Rôle inconnu");
        }
        // @ligne [role] Sérialise l’accès à cet état partagé entre les tâches.
        let active = self.active.lock().unwrap();
        // @ligne [role] Ajoute un contexte lisible à l’échec sans l’ignorer.
        let (room, _) = active.as_ref().context("Aucune salle ouverte")?;
        // @ligne [role] Sérialise l’accès à cet état partagé entre les tâches.
        let mut peers = room.peers.lock().unwrap();
        // @ligne [role] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let key = peers
            // @ligne [role] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .iter()
            // @ligne [role] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .find(|(_, p)| p.id == id)
            // @ligne [role] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .map(|(k, _)| k.clone())
            // @ligne [role] Ajoute un contexte lisible à l’échec sans l’ignorer.
            .context("Participant absent")?;
        // @ligne [role] Vérifie cette condition avant d’appliquer l’action protégée.
        if role == "expel" {
            // @ligne [role] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            peers.remove(&key);
        // @ligne [role] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        } else {
            // @ligne [role] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            peers.get_mut(&key).unwrap().role = role.into();
        }
        // @ligne [role] Renvoie une réussite explicite avec les données attendues par l’appelant.
        Ok(())
    }
}

// @ligne [role] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
impl Drop for Sharing {
    // @ligne [drop] Délimite cette opération pour isoler sa validation et son résultat.
    fn drop(&mut self) {
        // @ligne [drop] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        self.stop();
    }
}

// @ligne [drop] Applique cet attribut au type ou à la fonction qui suit.
#[cfg(test)]
// @ligne [drop] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
mod tests {
    // @ligne [drop] Importe explicitement les types et fonctions nécessaires à ce module.
    use super::*;
    /// Le certificat auto-signé n’est accepté sans validation que par ce client de test isolé.
    // @ligne [drop] Applique cet attribut au type ou à la fonction qui suit.
    #[tokio::test]
    // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Délimite cette opération pour isoler sa validation et son résultat.
    async fn real_https_enforces_pin_ranges_roles_and_revocation() {
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let _ = rustls::crypto::ring::default_provider().install_default();
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let tmp = tempfile::tempdir().unwrap();
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let source = tmp.path().join("lesson.txt");
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        std::fs::write(&source, b"lesson-content").unwrap();
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let port = std::net::TcpListener::bind("127.0.0.1:0")
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap()
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .local_addr()
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap()
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .port();
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let sharing = Sharing::default();
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let room = sharing
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .start(
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                vec![source.to_string_lossy().into_owned()],
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "127.0.0.1",
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                port,
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                tmp.path(),
            )
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Attend l’opération asynchrone avant d’utiliser son résultat.
            .await
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap();
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let base = room["url"].as_str().unwrap().trim_end_matches('/');
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let client = reqwest::Client::builder()
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .danger_accept_invalid_certs(true)
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .cookie_store(true)
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .build()
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap();
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let anonymous = reqwest::Client::builder()
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .danger_accept_invalid_certs(true)
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .build()
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap();
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            anonymous
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .get(format!("{base}/api/files"))
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .send()
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Attend l’opération asynchrone avant d’utiliser son résultat.
                .await
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .unwrap()
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .status(),
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            403
        );
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            client
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .post(format!("{base}/api/join"))
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .json(&json!({"pin":"invalid","name":"Student"}))
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .send()
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Attend l’opération asynchrone avant d’utiliser son résultat.
                .await
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .unwrap()
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .status(),
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            403
        );
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let joined = client
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .post(format!("{base}/api/join"))
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .json(&json!({"pin":room["pin"],"name":"Student"}))
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .send()
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Attend l’opération asynchrone avant d’utiliser son résultat.
            .await
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap();
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(joined.status(), 200);
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let cookie = joined
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .headers()
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .get(reqwest::header::SET_COOKIE)
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap()
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .to_str()
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap();
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(cookie.contains("HttpOnly; Secure; SameSite=Strict"));
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let listing: Value = client
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .get(format!("{base}/api/files"))
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .send()
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Attend l’opération asynchrone avant d’utiliser son résultat.
            .await
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap()
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .json()
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Attend l’opération asynchrone avant d’utiliser son résultat.
            .await
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap();
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let file = listing["files"][0]["id"].as_str().unwrap();
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let part = client
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .get(format!("{base}/api/files/{file}"))
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .header("Range", "bytes=0-5")
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .send()
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Attend l’opération asynchrone avant d’utiliser son résultat.
            .await
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap();
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(part.status(), 206);
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Attend l’opération asynchrone avant d’utiliser son résultat.
        assert_eq!(part.bytes().await.unwrap().as_ref(), b"lesson");
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            client
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .post(format!("{base}/api/upload"))
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .body("contribution")
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .send()
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Attend l’opération asynchrone avant d’utiliser son résultat.
                .await
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .unwrap()
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .status(),
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            403
        );
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let peer = sharing.peers().remove(0);
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        sharing.role(&peer.id, "contribute").unwrap();
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            client
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .post(format!("{base}/api/upload"))
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .header("X-File-Name", "..%2Fescape.txt")
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .body("bad")
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .send()
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Attend l’opération asynchrone avant d’utiliser son résultat.
                .await
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .unwrap()
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .status(),
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            400
        );
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let upload = client
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .post(format!("{base}/api/upload"))
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .header("X-File-Name", "cours%20%C3%A9tudiant.txt")
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .body("student-notes")
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .send()
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Attend l’opération asynchrone avant d’utiliser son résultat.
            .await
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap();
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(upload.status(), 200);
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        sharing.role(&peer.id, "expel").unwrap();
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            client
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .get(format!("{base}/api/files"))
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .send()
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Attend l’opération asynchrone avant d’utiliser son résultat.
                .await
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .unwrap()
                // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .status(),
            // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            403
        );
        // @ligne [real_https_enforces_pin_ranges_roles_and_revocation] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        sharing.stop();
    }
}
