//! Sauvegarde portable : Argon2id dérive une clé distincte, AES-GCM protège tout le contenu.
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use crate::store::{Store, export_private};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use aes_gcm::{
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    Aes256Gcm, KeyInit, Nonce,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    aead::{Aead, Payload},
};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use anyhow::{Context, Result, bail};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use rand::RngCore;
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use serde_json::{Value, json};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use std::path::Path;
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use zeroize::Zeroizing;

// @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
const MAGIC: &[u8] = b"DFVAULT1";

// @ligne [derive] Délimite cette opération pour isoler sa validation et son résultat.
fn derive(password: &str, salt: &[u8]) -> Result<Zeroizing<Vec<u8>>> {
    // @ligne [derive] Vérifie cette condition avant d’appliquer l’action protégée.
    if password.chars().count() < 12 || password.len() > 1024 {
        // @ligne [derive] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Utilisez une phrase de sauvegarde de 12 caractères minimum, 1024 octets maximum");
    }
    // @ligne [derive] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut key = Zeroizing::new(vec![0u8; 32]);
    // @ligne [derive] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    argon2::Argon2::default()
        // @ligne [derive] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .hash_password_into(password.as_bytes(), salt, &mut key)
        // @ligne [derive] Ajoute un contexte lisible à l’échec sans l’ignorer.
        .map_err(|_| anyhow::anyhow!("Dérivation Argon2id impossible"))?;
    // @ligne [derive] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(key)
}

// @ligne [export] Délimite cette opération pour isoler sa validation et son résultat.
pub fn export(store: &Store, path: &Path, password: &str) -> Result<usize> {
    // @ligne [export] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    store.unlock()?;
    // @ligne [export] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let entries = store.list()?;
    // @ligne [export] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut rows = Vec::new();
    // @ligne [export] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
    for entry in entries {
        // @ligne [export] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let document = store.credentials(&entry.id)?;
        // @ligne [export] Ajoute cette donnée à la collection en cours de préparation.
        rows.push(json!({"entry":entry,"document":document}));
    }
    // @ligne [export] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let clear = Zeroizing::new(serde_json::to_vec(&json!({"schema":1,"rows":rows}))?);
    // @ligne [export] Vérifie cette condition avant d’appliquer l’action protégée.
    if clear.len() > 32 * 1024 * 1024 {
        // @ligne [export] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("La sauvegarde dépasse 32 Mio");
    }
    // @ligne [export] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut salt = [0u8; 16];
    // @ligne [export] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut nonce = [0u8; 12];
    // @ligne [export] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    rand::rngs::OsRng.fill_bytes(&mut salt);
    // @ligne [export] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    rand::rngs::OsRng.fill_bytes(&mut nonce);
    // @ligne [export] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let key = derive(password, &salt)?;
    // @ligne [export] Ajoute un contexte lisible à l’échec sans l’ignorer.
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| anyhow::anyhow!("Clé invalide"))?;
    // @ligne [export] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let encrypted = cipher
        // @ligne [export] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .encrypt(
            // @ligne [export] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            Nonce::from_slice(&nonce),
            // @ligne [export] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            Payload {
                // @ligne [export] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                msg: &clear,
                // @ligne [export] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                aad: MAGIC,
            },
        )
        // @ligne [export] Ajoute un contexte lisible à l’échec sans l’ignorer.
        .map_err(|_| anyhow::anyhow!("Chiffrement de sauvegarde impossible"))?;
    // @ligne [export] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut bytes = MAGIC.to_vec();
    // @ligne [export] Ajoute cette donnée à la collection en cours de préparation.
    bytes.extend(salt);
    // @ligne [export] Ajoute cette donnée à la collection en cours de préparation.
    bytes.extend(nonce);
    // @ligne [export] Ajoute cette donnée à la collection en cours de préparation.
    bytes.extend(encrypted);
    // @ligne [export] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    export_private(path, &bytes)?;
    // @ligne [export] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(rows.len())
}

// @ligne [restore] Délimite cette opération pour isoler sa validation et son résultat.
pub fn restore(store: &Store, path: &Path, password: &str) -> Result<usize> {
    // @ligne [restore] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let meta = std::fs::metadata(path)?;
    // @ligne [restore] Vérifie cette condition avant d’appliquer l’action protégée.
    if !meta.is_file() || meta.len() > 32 * 1024 * 1024 + 64 {
        // @ligne [restore] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Sauvegarde invalide ou trop volumineuse");
    }
    // @ligne [restore] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let bytes = std::fs::read(path)?;
    // @ligne [restore] Vérifie cette condition avant d’appliquer l’action protégée.
    if bytes.len() < 52 || !bytes.starts_with(MAGIC) {
        // @ligne [restore] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Format de sauvegarde DockForge non reconnu");
    }
    // @ligne [restore] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let key = derive(password, &bytes[8..24])?;
    // @ligne [restore] Ajoute un contexte lisible à l’échec sans l’ignorer.
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| anyhow::anyhow!("Clé invalide"))?;
    // @ligne [restore] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let clear = Zeroizing::new(
        // @ligne [restore] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        cipher
            // @ligne [restore] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .decrypt(
                // @ligne [restore] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                Nonce::from_slice(&bytes[24..36]),
                // @ligne [restore] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                Payload {
                    // @ligne [restore] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    msg: &bytes[36..],
                    // @ligne [restore] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    aad: MAGIC,
                },
            )
            // @ligne [restore] Ajoute un contexte lisible à l’échec sans l’ignorer.
            .map_err(|_| anyhow::anyhow!("Phrase incorrecte ou sauvegarde modifiée"))?,
    );
    // @ligne [restore] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let data: Value = serde_json::from_slice(&clear)?;
    // @ligne [restore] Vérifie cette condition avant d’appliquer l’action protégée.
    if data["schema"] != 1 {
        // @ligne [restore] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Version de sauvegarde non prise en charge");
    }
    // @ligne [restore] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let rows = data["rows"]
        // @ligne [restore] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .as_array()
        // @ligne [restore] Ajoute un contexte lisible à l’échec sans l’ignorer.
        .context("Fiches de sauvegarde absentes")?;
    // @ligne [restore] Vérifie cette condition avant d’appliquer l’action protégée.
    if rows.len() > 10000 {
        // @ligne [restore] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Plus de 10 000 fiches dans la sauvegarde");
    }
    // Tout est validé avant l’écriture ; la base locale conserve les identifiants déjà présents.
    // @ligne [restore] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut records = Vec::new();
    // @ligne [restore] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
    for row in rows {
        // @ligne [restore] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let entry: crate::store::VaultEntry = serde_json::from_value(row["entry"].clone())?;
        // @ligne [restore] Vérifie cette condition avant d’appliquer l’action protégée.
        if entry.id.is_empty()
            // @ligne [restore] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            || entry.id.len() > 128
            // @ligne [restore] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            || entry.name.len() > 255
            // @ligne [restore] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            || entry.image.len() > 255
            // @ligne [restore] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            || !row["document"].is_object()
        {
            // @ligne [restore] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Fiche malformée");
        }
        // @ligne [restore] Ajoute cette donnée à la collection en cours de préparation.
        records.push((entry, row["document"].clone()));
    }
    // @ligne [restore] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    store.unlock()?;
    // @ligne [restore] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    store.restore_records(records)
}

// @ligne [restore] Applique cet attribut au type ou à la fonction qui suit.
#[cfg(test)]
// @ligne [restore] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
mod tests {
    // @ligne [restore] Importe explicitement les types et fonctions nécessaires à ce module.
    use super::*;
    // @ligne [restore] Applique cet attribut au type ou à la fonction qui suit.
    #[test]
    // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Délimite cette opération pour isoler sa validation et son résultat.
    fn portable_backup_rejects_wrong_password_and_restores_into_another_key() {
        // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let tmp = tempfile::tempdir().unwrap();
        // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let original = Store::open(&tmp.path().join("first")).unwrap();
        // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        original.test_unlock();
        // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        original
            // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .save_credentials(
                // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "id",
                // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "demo",
                // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "postgres:17",
                // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                &json!({"password":"never-plain-on-disk"}),
            )
            // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap();
        // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let archive = tmp.path().join("backup.dfv");
        // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(
            // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            export(&original, &archive, "phrase de test confidentielle").unwrap(),
            // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            1
        );
        // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let restored = Store::open(&tmp.path().join("second")).unwrap();
        // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        restored.test_unlock();
        // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(restore(&restored, &archive, "phrase incorrecte mais assez longue").is_err());
        // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(
            // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            restore(&restored, &archive, "phrase de test confidentielle").unwrap(),
            // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            1
        );
        // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(
            // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            restore(&restored, &archive, "phrase de test confidentielle").unwrap(),
            // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            0
        );
        // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(
            // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            restored.credentials("id").unwrap()["password"],
            // @ligne [portable_backup_rejects_wrong_password_and_restores_into_another_key] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "never-plain-on-disk"
        );
    }
}
