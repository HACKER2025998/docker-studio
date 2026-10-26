//! Outils annexes locaux : import volontaire, masquage et scan privé borné.
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use anyhow::{Context, Result, bail};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use serde_json::{Value, json};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use std::{net::Ipv4Addr, path::Path, sync::Arc, time::Duration};

// @ligne [redact] Délimite cette opération pour isoler sa validation et son résultat.
pub fn redact(input: &str) -> String {
    // @ligne [redact] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let patterns = [
        // @ligne [redact] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        r#"(?i)(password|passwd|pwd|token|secret|api[_-]?key)(\s*[=:]\s*|\s+)(?:"[^"\r\n]*"|'[^'\r\n]*'|[^\s;,]+)"#,
        // @ligne [redact] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        r"(?i)(--password|-p)(=|\s+)[^\s]+",
    ];
    // @ligne [redact] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut out = input.to_string();
    // @ligne [redact] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
    for pattern in patterns {
        // @ligne [redact] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let re = regex::Regex::new(pattern).expect("expression statique valide");
        // @ligne [redact] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        out = re.replace_all(&out, "$1$2[MASQUÉ]").into_owned();
    }
    // @ligne [redact] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let re = regex::Regex::new(r"(://[^:/\s]+:)[^@\s]+@").unwrap();
    // @ligne [redact] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    re.replace_all(&out, "${1}[MASQUÉ]@").into_owned()
}

// @ligne [import] Délimite cette opération pour isoler sa validation et son résultat.
pub fn import(path: &Path, shell: &str) -> Result<Vec<String>> {
    // @ligne [import] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let meta = std::fs::metadata(path)?;
    // @ligne [import] Vérifie cette condition avant d’appliquer l’action protégée.
    if !meta.is_file() || meta.len() > 32 * 1024 * 1024 {
        // @ligne [import] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Historique limité à 32 Mio");
    }
    // @ligne [import] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let bytes = std::fs::read(path)?;
    // @ligne [import] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let text = if bytes.starts_with(&[255, 254]) {
        // @ligne [import] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        String::from_utf16_lossy(
            // @ligne [import] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            &bytes[2..]
                // @ligne [import] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .as_chunks::<2>()
                // @ligne [import] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .0
                // @ligne [import] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .iter()
                // @ligne [import] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .map(|p| u16::from_le_bytes([p[0], p[1]]))
                // @ligne [import] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .collect::<Vec<_>>(),
        )
    // @ligne [import] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else {
        // @ligne [import] Ajoute un contexte lisible à l’échec sans l’ignorer.
        String::from_utf8(bytes).context("Encodage d’historique non reconnu")?
    };
    // @ligne [import] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let zsh = regex::Regex::new(r"^: \d+:\d+;")?;
    // @ligne [import] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(text
        // @ligne [import] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .lines()
        // @ligne [import] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .filter_map(|line| {
            // @ligne [import] Vérifie cette condition avant d’appliquer l’action protégée.
            if shell == "fish" {
                // @ligne [import] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                line.strip_prefix("- cmd: ").map(|s| s.replace("\\n", "\n"))
            // @ligne [import] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            } else if line.starts_with('#') || line.trim().is_empty() {
                // @ligne [import] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                None
            // @ligne [import] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            } else {
                // @ligne [import] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                Some(zsh.replace(line, "").into_owned())
            }
        })
        // @ligne [import] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .collect())
}

// @ligne [netmap] Délimite cette opération pour isoler sa validation et son résultat.
pub async fn netmap(cidr: &str) -> Result<Value> {
    // @ligne [netmap] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let (ip, prefix) = cidr
        // @ligne [netmap] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .split_once('/')
        // @ligne [netmap] Ajoute un contexte lisible à l’échec sans l’ignorer.
        .context("Saisissez un sous-réseau IPv4 /24")?;
    // @ligne [netmap] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let ip: Ipv4Addr = ip.parse()?;
    // @ligne [netmap] Vérifie cette condition avant d’appliquer l’action protégée.
    if prefix != "24" || !ip.is_private() {
        // @ligne [netmap] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Le scan est limité à un /24 privé explicitement demandé");
    }
    // @ligne [netmap] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let octets = ip.octets();
    // @ligne [netmap] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let semaphore = Arc::new(tokio::sync::Semaphore::new(48));
    // @ligne [netmap] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut tasks = tokio::task::JoinSet::new();
    // @ligne [netmap] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
    for n in 1..=254u8 {
        // @ligne [netmap] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let gate = semaphore.clone();
        // @ligne [netmap] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let host = Ipv4Addr::new(octets[0], octets[1], octets[2], n);
        // @ligne [netmap] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        tasks.spawn(async move{
            // @ligne [netmap] Attend l’opération asynchrone avant d’utiliser son résultat.
            let _permit=gate.acquire().await.unwrap();let mut ports=Vec::new();
            // @ligne [netmap] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
            for port in [22u16,80,443,445,3389,5432,3306,8080] {
                // @ligne [netmap] Attend l’opération asynchrone avant d’utiliser son résultat.
                if matches!(tokio::time::timeout(Duration::from_millis(180),tokio::net::TcpStream::connect((host,port))).await,Ok(Ok(_))){ports.push(port);}
            }
            // @ligne [netmap] Vérifie cette condition avant d’appliquer l’action protégée.
            if ports.is_empty(){None}else{Some(json!({"ip":host.to_string(),"ports":ports,"evidence":"Connexion TCP établie","kind":"host"}))}
        });
    }
    // @ligne [netmap] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut hosts = Vec::new();
    // @ligne [netmap] Attend l’opération asynchrone avant d’utiliser son résultat.
    while let Some(result) = tasks.join_next().await {
        // @ligne [netmap] Vérifie cette condition avant d’appliquer l’action protégée.
        if let Some(host) = result? {
            // @ligne [netmap] Ajoute cette donnée à la collection en cours de préparation.
            hosts.push(host);
        }
    }
    // @ligne [netmap] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    hosts.sort_by_key(|h| h["ip"].as_str().unwrap_or_default().to_string());
    // @ligne [netmap] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(
        // @ligne [netmap] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        json!({"subnet":cidr,"hosts":hosts,"notice":"Les liens représentent le sous-réseau logique. Un hôte sans port ouvert peut rester invisible. Aucun FAI, switch ou câble physique n’est inventé."}),
    )
}

/// Ne suit que les fichiers explicitement choisis, avec rattrapage au prochain lancement.
// @ligne [watch_histories] Délimite cette opération pour isoler sa validation et son résultat.
pub async fn watch_histories(store: Arc<crate::store::Store>) {
    // @ligne [watch_histories] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut versions = std::collections::HashMap::new();
    // @ligne [watch_histories] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
    loop {
        // @ligne [watch_histories] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
        for (path, shell) in store.history_sources().unwrap_or_default() {
            // @ligne [watch_histories] Vérifie cette condition avant d’appliquer l’action protégée.
            if let Ok(meta) = std::fs::metadata(&path) {
                // @ligne [watch_histories] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
                let version = (meta.len(), meta.modified().ok());
                // @ligne [watch_histories] Vérifie cette condition avant d’appliquer l’action protégée.
                if versions.get(&path) != Some(&version) {
                    // @ligne [watch_histories] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
                    let source = path.clone();
                    // @ligne [watch_histories] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
                    let kind = shell.clone();
                    // @ligne [watch_histories] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
                    let target = store.clone();
                    // @ligne [watch_histories] Vérifie cette condition avant d’appliquer l’action protégée.
                    if matches!(
                        // @ligne [watch_histories] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        tokio::task::spawn_blocking(move || {
                            // @ligne [watch_histories] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
                            let rows = import(Path::new(&source), &kind)?;
                            // @ligne [watch_histories] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                            target.import_commands(rows, &kind)
                        })
                        // @ligne [watch_histories] Attend l’opération asynchrone avant d’utiliser son résultat.
                        .await,
                        // @ligne [watch_histories] Renvoie une réussite explicite avec les données attendues par l’appelant.
                        Ok(Ok(_))
                    // @ligne [watch_histories] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    ) {
                        // @ligne [watch_histories] Ajoute cette donnée à la collection en cours de préparation.
                        versions.insert(path, version);
                    }
                }
            }
        }
        // @ligne [watch_histories] Attend l’opération asynchrone avant d’utiliser son résultat.
        tokio::time::sleep(Duration::from_secs(15)).await;
    }
}

// @ligne [watch_histories] Applique cet attribut au type ou à la fonction qui suit.
#[cfg(test)]
// @ligne [watch_histories] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
mod tests {
    // @ligne [watch_histories] Applique cet attribut au type ou à la fonction qui suit.
    #[test]
    // @ligne [secrets_are_masked_before_history_storage] Délimite cette opération pour isoler sa validation et son résultat.
    fn secrets_are_masked_before_history_storage() {
        // @ligne [secrets_are_masked_before_history_storage] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let out = super::redact(
            // @ligne [secrets_are_masked_before_history_storage] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "POSTGRES_PASSWORD='abc def' curl --token xyz postgres://user:secret@localhost/db",
        );
        // @ligne [secrets_are_masked_before_history_storage] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(!out.contains("abc"));
        // @ligne [secrets_are_masked_before_history_storage] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(!out.contains("xyz"));
        // @ligne [secrets_are_masked_before_history_storage] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(!out.contains(":secret@"));
    }
    // @ligne [secrets_are_masked_before_history_storage] Applique cet attribut au type ou à la fonction qui suit.
    #[tokio::test]
    // @ligne [network_scan_rejects_public_and_unbounded_targets] Délimite cette opération pour isoler sa validation et son résultat.
    async fn network_scan_rejects_public_and_unbounded_targets() {
        // @ligne [network_scan_rejects_public_and_unbounded_targets] Attend l’opération asynchrone avant d’utiliser son résultat.
        assert!(super::netmap("8.8.8.0/24").await.is_err());
        // @ligne [network_scan_rejects_public_and_unbounded_targets] Attend l’opération asynchrone avant d’utiliser son résultat.
        assert!(super::netmap("192.168.0.0/16").await.is_err());
    }
}
