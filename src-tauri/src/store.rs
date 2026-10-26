//! Le disque ne contient jamais le JSON d’identifiants en clair : AES-GCM lie la fiche à son ID.
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
use base64::{Engine, engine::general_purpose::STANDARD};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use rand::RngCore;
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use rusqlite::{Connection, params};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use serde::{Deserialize, Serialize};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use std::{
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    path::{Path, PathBuf},
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    sync::Mutex,
};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use zeroize::Zeroizing;

// @ligne [module] Définit les données échangées par ce module, sans comportement implicite.
pub struct Store {
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    db: Mutex<Connection>,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    root: PathBuf,
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    key: Mutex<Option<Zeroizing<Vec<u8>>>>,
}

// @ligne [module] Applique cet attribut au type ou à la fonction qui suit.
#[derive(Serialize, Deserialize)]
// @ligne [module] Définit les données échangées par ce module, sans comportement implicite.
pub struct VaultEntry {
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub id: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub name: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub image: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub archived: bool,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub created_at: String,
}

// @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
impl Store {
    // @ligne [open] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn open(root: &Path) -> Result<Self> {
        // @ligne [open] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        std::fs::create_dir_all(root)?;
        // @ligne [open] Applique cet attribut au type ou à la fonction qui suit.
        #[cfg(unix)]
        {
            // @ligne [open] Importe explicitement les types et fonctions nécessaires à ce module.
            use std::os::unix::fs::PermissionsExt;
            // @ligne [open] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            std::fs::set_permissions(root, std::fs::Permissions::from_mode(0o700))?;
        }
        // @ligne [open] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let db = Connection::open(root.join("dockforge.sqlite3"))?;
        // @ligne [open] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        db.busy_timeout(std::time::Duration::from_secs(5))?;
        // @ligne [open] Passe par SQLite et ses paramètres pour séparer les données de la requête.
        db.execute_batch(
            // @ligne [open] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "PRAGMA journal_mode=WAL; PRAGMA secure_delete=ON;
          CREATE TABLE IF NOT EXISTS managed_containers_vault (
            id TEXT PRIMARY KEY, name TEXT NOT NULL, image TEXT NOT NULL,
            ciphertext BLOB NOT NULL, archived INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL);
          CREATE TABLE IF NOT EXISTS custom_built_images (
            tag TEXT PRIMARY KEY, recipe TEXT NOT NULL, created_at TEXT NOT NULL);
          CREATE TABLE IF NOT EXISTS history_sources (path TEXT PRIMARY KEY,shell TEXT NOT NULL);
          CREATE TABLE IF NOT EXISTS commands (
            id INTEGER PRIMARY KEY, command TEXT NOT NULL UNIQUE, shell TEXT NOT NULL,
            favorite INTEGER NOT NULL DEFAULT 0, imported_at TEXT NOT NULL);",
        // @ligne [open] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        )?;
        // @ligne [open] Renvoie une réussite explicite avec les données attendues par l’appelant.
        Ok(Self {
            // @ligne [open] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            db: Mutex::new(db),
            // @ligne [open] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            root: root.to_owned(),
            // @ligne [open] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            key: Mutex::new(None),
        })
    }

    // @ligne [root] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn root(&self) -> &Path {
        // @ligne [root] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        &self.root
    }

    /// L’absence de trousseau bloque le coffre : aucun fichier de clé de secours non chiffré.
    // @ligne [unlock] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn unlock(&self) -> Result<()> {
        // @ligne [unlock] Prépare une valeur locale qui sera complétée pendant cette opération.
        let mut guard = self
            // @ligne [unlock] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .key
            // @ligne [unlock] Sérialise l’accès à cet état partagé entre les tâches.
            .lock()
            // @ligne [unlock] Ajoute un contexte lisible à l’échec sans l’ignorer.
            .map_err(|_| anyhow::anyhow!("Coffre indisponible"))?;
        // @ligne [unlock] Vérifie cette condition avant d’appliquer l’action protégée.
        if guard.is_some() {
            // @ligne [unlock] Renvoie une réussite explicite avec les données attendues par l’appelant.
            return Ok(());
        }
        // La transaction sérialise aussi la première création de clé entre deux processus.
        // @ligne [unlock] Sérialise l’accès à cet état partagé entre les tâches.
        let mut db = self.db.lock().unwrap();
        // @ligne [unlock] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let tx = db.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        // @ligne [unlock] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let account = self.root.canonicalize()?.to_string_lossy().into_owned();
        // @ligne [unlock] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let entry = keyring::Entry::new("io.github.hacker2025998.dockforge", &account)?;
        // @ligne [unlock] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let raw = match entry.get_password() {
            // @ligne [unlock] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(value) => Zeroizing::new(value),
            // @ligne [unlock] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            Err(keyring::Error::NoEntry) => {
                // @ligne [unlock] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
                let count: i64 =
                    // @ligne [unlock] Passe par SQLite et ses paramètres pour séparer les données de la requête.
                    tx.query_row("SELECT COUNT(*) FROM managed_containers_vault", [], |r| {
                        // @ligne [unlock] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        r.get(0)
                    // @ligne [unlock] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    })?;
                // @ligne [unlock] Vérifie cette condition avant d’appliquer l’action protégée.
                if count > 0 {
                    // @ligne [unlock] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
                    bail!(
                        // @ligne [unlock] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        "Clé absente du trousseau : restaurez-la ; une nouvelle clé rendrait les fiches illisibles."
                    );
                }
                // @ligne [unlock] Prépare une valeur locale qui sera complétée pendant cette opération.
                let mut bytes = Zeroizing::new(vec![0u8; 32]);
                // @ligne [unlock] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                rand::rngs::OsRng.fill_bytes(&mut bytes);
                // @ligne [unlock] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
                let encoded = Zeroizing::new(STANDARD.encode(&*bytes));
                // @ligne [unlock] Ajoute un contexte lisible à l’échec sans l’ignorer.
                entry.set_password(&encoded).context(
                    // @ligne [unlock] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    "Trousseau verrouillé ou absent : ouvrez votre session de clés native",
                // @ligne [unlock] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                )?;
                // @ligne [unlock] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                encoded
            }
            // @ligne [unlock] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            Err(e) => return Err(e).context("Impossible de déverrouiller le trousseau système"),
        };
        // @ligne [unlock] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let decoded = STANDARD.decode(raw.as_bytes())?;
        // @ligne [unlock] Vérifie cette condition avant d’appliquer l’action protégée.
        if decoded.len() != 32 {
            // @ligne [unlock] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            bail!("Format de clé invalide");
        }
        // @ligne [unlock] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        tx.commit()?;
        *guard = Some(Zeroizing::new(decoded));
        // @ligne [unlock] Renvoie une réussite explicite avec les données attendues par l’appelant.
        Ok(())
    }

    // @ligne [lock] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn lock(&self) {
        *self.key.lock().unwrap() = None;
    }

    // @ligne [crypt] Délimite cette opération pour isoler sa validation et son résultat.
    fn crypt(&self, id: &str, input: &[u8], encrypt: bool) -> Result<Vec<u8>> {
        // @ligne [crypt] Sérialise l’accès à cet état partagé entre les tâches.
        let guard = self.key.lock().unwrap();
        // @ligne [crypt] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let key = guard
            // @ligne [crypt] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .as_ref()
            // @ligne [crypt] Ajoute un contexte lisible à l’échec sans l’ignorer.
            .context("Déverrouillez le coffre avant cette opération")?;
        // @ligne [crypt] Ajoute un contexte lisible à l’échec sans l’ignorer.
        let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| anyhow::anyhow!("Clé invalide"))?;
        // @ligne [crypt] Vérifie cette condition avant d’appliquer l’action protégée.
        if encrypt {
            // @ligne [crypt] Prépare une valeur locale qui sera complétée pendant cette opération.
            let mut nonce = [0u8; 12];
            // @ligne [crypt] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            rand::rngs::OsRng.fill_bytes(&mut nonce);
            // @ligne [crypt] Prépare une valeur locale qui sera complétée pendant cette opération.
            let mut bytes = nonce.to_vec();
            // @ligne [crypt] Ajoute cette donnée à la collection en cours de préparation.
            bytes.extend(
                // @ligne [crypt] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                cipher
                    // @ligne [crypt] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    .encrypt(
                        // @ligne [crypt] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        Nonce::from_slice(&nonce),
                        // @ligne [crypt] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        Payload {
                            // @ligne [crypt] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                            msg: input,
                            // @ligne [crypt] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                            aad: id.as_bytes(),
                        },
                    )
                    // @ligne [crypt] Ajoute un contexte lisible à l’échec sans l’ignorer.
                    .map_err(|_| anyhow::anyhow!("Chiffrement impossible"))?,
            );
            // @ligne [crypt] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(bytes)
        // @ligne [crypt] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        } else {
            // @ligne [crypt] Vérifie cette condition avant d’appliquer l’action protégée.
            if input.len() < 28 {
                // @ligne [crypt] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
                bail!("Fiche tronquée");
            }
            // @ligne [crypt] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            cipher
                // @ligne [crypt] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .decrypt(
                    // @ligne [crypt] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    Nonce::from_slice(&input[..12]),
                    // @ligne [crypt] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    Payload {
                        // @ligne [crypt] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        msg: &input[12..],
                        // @ligne [crypt] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                        aad: id.as_bytes(),
                    },
                )
                // @ligne [crypt] Ajoute un contexte lisible à l’échec sans l’ignorer.
                .map_err(|_| anyhow::anyhow!("Intégrité du coffre invalide ou mauvaise clé"))
        }
    }

    // @ligne [save_credentials] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn save_credentials(
        // @ligne [save_credentials] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        &self,
        // @ligne [save_credentials] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        id: &str,
        // @ligne [save_credentials] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        name: &str,
        // @ligne [save_credentials] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        image: &str,
        // @ligne [save_credentials] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        data: &serde_json::Value,
    // @ligne [save_credentials] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    ) -> Result<()> {
        // @ligne [save_credentials] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let clear = Zeroizing::new(serde_json::to_vec(data)?);
        // @ligne [save_credentials] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let encrypted = self.crypt(id, &clear, true)?;
        // @ligne [save_credentials] Sérialise l’accès à cet état partagé entre les tâches.
        self.db.lock().unwrap().execute("INSERT INTO managed_containers_vault(id,name,image,ciphertext,created_at) VALUES(?1,?2,?3,?4,?5) ON CONFLICT(id) DO UPDATE SET ciphertext=excluded.ciphertext", params![id,name,image,encrypted,chrono::Utc::now().to_rfc3339()])?;
        // @ligne [save_credentials] Renvoie une réussite explicite avec les données attendues par l’appelant.
        Ok(())
    }

    // @ligne [credentials] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn credentials(&self, id: &str) -> Result<serde_json::Value> {
        // @ligne [credentials] Sérialise l’accès à cet état partagé entre les tâches.
        let bytes: Vec<u8> = self.db.lock().unwrap().query_row(
            // @ligne [credentials] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "SELECT ciphertext FROM managed_containers_vault WHERE id=?1",
            // @ligne [credentials] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            [id],
            // @ligne [credentials] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            |r| r.get(0),
        // @ligne [credentials] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        )?;
        // @ligne [credentials] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let clear = Zeroizing::new(self.crypt(id, &bytes, false)?);
        // @ligne [credentials] Renvoie une réussite explicite avec les données attendues par l’appelant.
        Ok(serde_json::from_slice(&clear)?)
    }

    // @ligne [list] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn list(&self) -> Result<Vec<VaultEntry>> {
        // @ligne [list] Sérialise l’accès à cet état partagé entre les tâches.
        let db = self.db.lock().unwrap();
        // @ligne [list] Passe par SQLite et ses paramètres pour séparer les données de la requête.
        let mut q = db.prepare("SELECT id,name,image,archived,created_at FROM managed_containers_vault ORDER BY created_at DESC")?;
        // @ligne [list] Passe par SQLite et ses paramètres pour séparer les données de la requête.
        Ok(q.query_map([], |r| {
            // @ligne [list] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok(VaultEntry {
                // @ligne [list] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                id: r.get(0)?,
                // @ligne [list] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                name: r.get(1)?,
                // @ligne [list] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                image: r.get(2)?,
                // @ligne [list] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                archived: r.get(3)?,
                // @ligne [list] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                created_at: r.get(4)?,
            })
        // @ligne [list] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        })?
        // @ligne [list] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .collect::<rusqlite::Result<_>>()?)
    }

    // @ligne [restore_records] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn restore_records(&self, records: Vec<(VaultEntry, serde_json::Value)>) -> Result<usize> {
        // @ligne [restore_records] Prépare une valeur locale qui sera complétée pendant cette opération.
        let mut encrypted = Vec::new();
        // @ligne [restore_records] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
        for (entry, doc) in records {
            // @ligne [restore_records] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let clear = Zeroizing::new(serde_json::to_vec(&doc)?);
            // @ligne [restore_records] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let ciphertext = self.crypt(&entry.id, &clear, true)?;
            // @ligne [restore_records] Ajoute cette donnée à la collection en cours de préparation.
            encrypted.push((entry, ciphertext));
        }
        // @ligne [restore_records] Sérialise l’accès à cet état partagé entre les tâches.
        let mut db = self.db.lock().unwrap();
        // @ligne [restore_records] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let tx = db.transaction()?;
        // @ligne [restore_records] Prépare une valeur locale qui sera complétée pendant cette opération.
        let mut count = 0;
        // @ligne [restore_records] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
        for (entry, bytes) in encrypted {
            // @ligne [restore_records] Passe par SQLite et ses paramètres pour séparer les données de la requête.
            count+=tx.execute("INSERT OR IGNORE INTO managed_containers_vault(id,name,image,ciphertext,archived,created_at) VALUES(?1,?2,?3,?4,?5,?6)",params![entry.id,entry.name,entry.image,bytes,entry.archived,entry.created_at])?;
        }
        // @ligne [restore_records] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        tx.commit()?;
        // @ligne [restore_records] Renvoie une réussite explicite avec les données attendues par l’appelant.
        Ok(count)
    }

    // @ligne [retire] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn retire(&self, id: &str, archive: bool) -> Result<()> {
        // @ligne [retire] Vérifie cette condition avant d’appliquer l’action protégée.
        if archive {
            // @ligne [retire] Sérialise l’accès à cet état partagé entre les tâches.
            self.db.lock().unwrap().execute(
                // @ligne [retire] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                "UPDATE managed_containers_vault SET archived=1 WHERE id=?1",
                // @ligne [retire] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                [id],
            // @ligne [retire] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            )?;
        // @ligne [retire] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        } else {
            // @ligne [retire] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            self.db
                // @ligne [retire] Sérialise l’accès à cet état partagé entre les tâches.
                .lock()
                // @ligne [retire] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .unwrap()
                // @ligne [retire] Passe par SQLite et ses paramètres pour séparer les données de la requête.
                .execute("DELETE FROM managed_containers_vault WHERE id=?1", [id])?;
        }
        // @ligne [retire] Renvoie une réussite explicite avec les données attendues par l’appelant.
        Ok(())
    }

    // @ligne [save_image] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn save_image(&self, tag: &str, recipe: &str) -> Result<()> {
        // @ligne [save_image] Sérialise l’accès à cet état partagé entre les tâches.
        self.db.lock().unwrap().execute("INSERT INTO custom_built_images VALUES(?1,?2,?3) ON CONFLICT(tag) DO UPDATE SET recipe=excluded.recipe,created_at=excluded.created_at",params![tag,recipe,chrono::Utc::now().to_rfc3339()])?;
        // @ligne [save_image] Renvoie une réussite explicite avec les données attendues par l’appelant.
        Ok(())
    }

    // @ligne [images] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn images(&self) -> Result<Vec<serde_json::Value>> {
        // @ligne [images] Sérialise l’accès à cet état partagé entre les tâches.
        let db = self.db.lock().unwrap();
        // @ligne [images] Passe par SQLite et ses paramètres pour séparer les données de la requête.
        let mut q = db.prepare(
            // @ligne [images] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "SELECT tag,recipe,created_at FROM custom_built_images ORDER BY created_at DESC",
        // @ligne [images] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        )?;
        // @ligne [images] Passe par SQLite et ses paramètres pour séparer les données de la requête.
        Ok(q.query_map([],|r| Ok(serde_json::json!({"tag":r.get::<_,String>(0)?,"recipe":r.get::<_,String>(1)?,"created_at":r.get::<_,String>(2)?})))?.collect::<rusqlite::Result<_>>()?)
    }

    // @ligne [import_commands] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn import_commands(&self, commands: Vec<String>, shell: &str) -> Result<usize> {
        // @ligne [import_commands] Sérialise l’accès à cet état partagé entre les tâches.
        let mut db = self.db.lock().unwrap();
        // @ligne [import_commands] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let tx = db.transaction()?;
        // @ligne [import_commands] Prépare une valeur locale qui sera complétée pendant cette opération.
        let mut added = 0;
        // @ligne [import_commands] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
        for command in commands {
            // @ligne [import_commands] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let safe = crate::suite::redact(&command);
            // @ligne [import_commands] Vérifie cette condition avant d’appliquer l’action protégée.
            if !safe.trim().is_empty() {
                // @ligne [import_commands] Passe par SQLite et ses paramètres pour séparer les données de la requête.
                added += tx.execute(
                    // @ligne [import_commands] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    "INSERT OR IGNORE INTO commands(command,shell,imported_at) VALUES(?1,?2,?3)",
                    // @ligne [import_commands] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                    params![safe, shell, chrono::Utc::now().to_rfc3339()],
                // @ligne [import_commands] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                )?;
            }
        }
        // @ligne [import_commands] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        tx.commit()?;
        // @ligne [import_commands] Renvoie une réussite explicite avec les données attendues par l’appelant.
        Ok(added)
    }

    // @ligne [commands] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn commands(&self, search: &str, favorites: bool) -> Result<Vec<serde_json::Value>> {
        // @ligne [commands] Sérialise l’accès à cet état partagé entre les tâches.
        let db = self.db.lock().unwrap();
        // @ligne [commands] Passe par SQLite et ses paramètres pour séparer les données de la requête.
        let mut q=db.prepare("SELECT id,command,shell,favorite FROM commands WHERE instr(lower(command),lower(?1))>0 AND (?2=0 OR favorite=1) ORDER BY id DESC LIMIT 500")?;
        // @ligne [commands] Passe par SQLite et ses paramètres pour séparer les données de la requête.
        Ok(q.query_map(params![search,favorites],|r| Ok(serde_json::json!({"id":r.get::<_,i64>(0)?,"command":r.get::<_,String>(1)?,"shell":r.get::<_,String>(2)?,"favorite":r.get::<_,bool>(3)?})))?.collect::<rusqlite::Result<_>>()?)
    }

    // @ligne [history_sources] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn history_sources(&self) -> Result<Vec<(String, String)>> {
        // @ligne [history_sources] Sérialise l’accès à cet état partagé entre les tâches.
        let db = self.db.lock().unwrap();
        // @ligne [history_sources] Passe par SQLite et ses paramètres pour séparer les données de la requête.
        let mut q = db.prepare("SELECT path,shell FROM history_sources")?;
        // @ligne [history_sources] Passe par SQLite et ses paramètres pour séparer les données de la requête.
        Ok(q.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
            // @ligne [history_sources] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .collect::<rusqlite::Result<_>>()?)
    }
    // @ligne [watch_history] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn watch_history(&self, path: &str, shell: &str, enabled: bool) -> Result<()> {
        // @ligne [watch_history] Vérifie cette condition avant d’appliquer l’action protégée.
        if enabled {
            // @ligne [watch_history] Vérifie cette condition avant d’appliquer l’action protégée.
            if !["bash", "zsh", "fish", "powershell"].contains(&shell) {
                // @ligne [watch_history] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
                bail!("Shell inconnu");
            }
            // @ligne [watch_history] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let path = std::path::Path::new(path).canonicalize()?;
            // @ligne [watch_history] Sérialise l’accès à cet état partagé entre les tâches.
            self.db.lock().unwrap().execute("INSERT INTO history_sources VALUES(?1,?2) ON CONFLICT(path) DO UPDATE SET shell=excluded.shell",params![path.to_string_lossy(),shell])?;
        // @ligne [watch_history] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        } else {
            // @ligne [watch_history] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            self.db
                // @ligne [watch_history] Sérialise l’accès à cet état partagé entre les tâches.
                .lock()
                // @ligne [watch_history] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                .unwrap()
                // @ligne [watch_history] Passe par SQLite et ses paramètres pour séparer les données de la requête.
                .execute("DELETE FROM history_sources WHERE path=?1", [path])?;
        }
        // @ligne [watch_history] Renvoie une réussite explicite avec les données attendues par l’appelant.
        Ok(())
    }

    // @ligne [favorite] Délimite cette opération pour isoler sa validation et son résultat.
    pub fn favorite(&self, id: i64, value: bool) -> Result<()> {
        // @ligne [favorite] Sérialise l’accès à cet état partagé entre les tâches.
        self.db.lock().unwrap().execute(
            // @ligne [favorite] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "UPDATE commands SET favorite=?2 WHERE id=?1",
            // @ligne [favorite] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            params![id, value],
        // @ligne [favorite] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        )?;
        // @ligne [favorite] Renvoie une réussite explicite avec les données attendues par l’appelant.
        Ok(())
    }

    // @ligne [favorite] Applique cet attribut au type ou à la fonction qui suit.
    #[cfg(test)]
    // @ligne [test_unlock] Délimite cette opération pour isoler sa validation et son résultat.
    pub(crate) fn test_unlock(&self) {
        *self.key.lock().unwrap() = Some(Zeroizing::new(rand::random::<[u8; 32]>().to_vec()));
    }
}

/// Écriture exclusive et permissions restrictives : pas d’écrasement ni de suivi de symlink.
// @ligne [export_private] Délimite cette opération pour isoler sa validation et son résultat.
pub fn export_private(path: &Path, bytes: &[u8]) -> Result<()> {
    // @ligne [export_private] Importe explicitement les types et fonctions nécessaires à ce module.
    use std::io::Write;
    // @ligne [export_private] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut options = std::fs::OpenOptions::new();
    // @ligne [export_private] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    options.write(true).create_new(true);
    // @ligne [export_private] Applique cet attribut au type ou à la fonction qui suit.
    #[cfg(unix)]
    {
        // @ligne [export_private] Importe explicitement les types et fonctions nécessaires à ce module.
        use std::os::unix::fs::OpenOptionsExt;
        // @ligne [export_private] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        options.mode(0o600);
    }
    // @ligne [export_private] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut file = options
        // @ligne [export_private] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .open(path)
        // @ligne [export_private] Ajoute un contexte lisible à l’échec sans l’ignorer.
        .context("Choisissez un nouveau fichier ; l’écrasement est refusé")?;
    // @ligne [export_private] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    file.write_all(bytes)?;
    // @ligne [export_private] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    file.sync_all()?;
    // @ligne [export_private] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(())
}

// @ligne [export_private] Applique cet attribut au type ou à la fonction qui suit.
#[cfg(test)]
// @ligne [export_private] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
mod tests {
    // @ligne [export_private] Importe explicitement les types et fonctions nécessaires à ce module.
    use super::*;
    // @ligne [export_private] Applique cet attribut au type ou à la fonction qui suit.
    #[test]
    // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Délimite cette opération pour isoler sa validation et son résultat.
    fn disk_has_no_secret_and_ciphertext_is_bound_to_container() {
        // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let tmp = tempfile::tempdir().unwrap();
        // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let store = Store::open(tmp.path()).unwrap();
        // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        store.test_unlock();
        // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let data = serde_json::json!({"password":"super-secret-value-unique"});
        // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        store
            // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .save_credentials("alpha", "name", "postgres:17", &data)
            // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap();
        // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(store.credentials("alpha").unwrap(), data);
        // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Sérialise l’accès à cet état partagé entre les tâches.
        let db = store.db.lock().unwrap();
        // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let encrypted: Vec<u8> = db
            // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Passe par SQLite et ses paramètres pour séparer les données de la requête.
            .query_row("SELECT ciphertext FROM managed_containers_vault", [], |r| {
                // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
                r.get(0)
            })
            // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap();
        // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        drop(db);
        // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(store.crypt("beta", &encrypted, false).is_err());
        // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Prépare une valeur locale qui sera complétée pendant cette opération.
        let mut corrupted = encrypted.clone();
        // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        corrupted[20] ^= 1;
        // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(store.crypt("alpha", &corrupted, false).is_err());
        // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
        for entry in std::fs::read_dir(tmp.path()).unwrap() {
            // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let bytes = std::fs::read(entry.unwrap().path()).unwrap();
            // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
            let secret = b"super-secret-value-unique";
            // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Vérifie cette propriété observable pour détecter une régression du contrat.
            assert!(!bytes.windows(secret.len()).any(|x| x == secret));
        }
        // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Sérialise l’accès à cet état partagé entre les tâches.
        store.lock();
        // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(store.credentials("alpha").is_err());
    }
    // @ligne [disk_has_no_secret_and_ciphertext_is_bound_to_container] Applique cet attribut au type ou à la fonction qui suit.
    #[test]
    // @ligne [export_does_not_overwrite_existing_file] Délimite cette opération pour isoler sa validation et son résultat.
    fn export_does_not_overwrite_existing_file() {
        // @ligne [export_does_not_overwrite_existing_file] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let tmp = tempfile::tempdir().unwrap();
        // @ligne [export_does_not_overwrite_existing_file] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let p = tmp.path().join("access.json");
        // @ligne [export_does_not_overwrite_existing_file] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        export_private(&p, b"one").unwrap();
        // @ligne [export_does_not_overwrite_existing_file] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(export_private(&p, b"two").is_err());
        // @ligne [export_does_not_overwrite_existing_file] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert_eq!(std::fs::read(p).unwrap(), b"one");
    }
}
