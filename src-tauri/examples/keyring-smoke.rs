//! Recette Linux volontaire : profil et clé synthétiques, nettoyés à la fin du programme.
// Le test importe uniquement le stockage local, sans lancer de conteneur ni de fenêtre.
use dockforge_core::store::Store;
// Le JSON synthétique vérifie la persistance sans lire une fiche utilisateur.
use serde_json::json;
// Cette garde supprime exclusivement l’entrée liée au dossier temporaire de ce test.
struct Cleanup(keyring::Entry);
// Le nettoyage s’exécute aussi si une étape renvoie une erreur Rust.
impl Drop for Cleanup {
    // Le trousseau réel ne conserve pas cette clé de recette après la sortie normale.
    fn drop(&mut self) {
        // L’erreur de nettoyage ne remplace pas le résultat initial du test.
        let _ = self.0.delete_credential();
    }
}
// Cette recette doit être exécutée dans une session dont le trousseau est déjà déverrouillé.
fn main() -> anyhow::Result<()> {
    // Le chemin aléatoire distingue strictement ce compte de toute donnée existante.
    let directory = tempfile::tempdir()?;
    // Le Store utilise ce même chemin canonique comme identifiant de trousseau.
    let account = directory.path().canonicalize()?.to_string_lossy().into_owned();
    // L’entrée est créée pour ce test seulement, avec l’identifiant applicatif de production.
    let _cleanup = Cleanup(keyring::Entry::new("io.github.hacker2025998.dockforge", &account)?);
    // Une première ouverture initialise la base vide et sa clé native.
    let first = Store::open(directory.path())?;
    // Le déverrouillage teste ici le vrai backend Secret Service de la session.
    first.unlock()?;
    // La fiche synthétique doit être chiffrée par la même voie que les accès de production.
    first.save_credentials("test-id", "recette", "test-image", &json!({"password":"synthetic-keyring-test"}))?;
    // Fermer le Store retire sa clé en mémoire : la lecture suivante doit utiliser le trousseau.
    drop(first);
    // Le second Store représente une nouvelle ouverture de l’application sur le même profil.
    let second = Store::open(directory.path())?;
    // Cette étape récupère la clé persistée, sans fonction de déverrouillage dédiée aux tests.
    second.unlock()?;
    // Une égalité confirme le vrai aller-retour entre SQLite chiffré et le trousseau natif.
    assert_eq!(second.credentials("test-id")?["password"], "synthetic-keyring-test");
    // Aucune valeur de secret ni chemin privé n’est imprimé dans le rapport de recette.
    println!("Trousseau natif : persistance et réouverture vérifiées, entrée de test nettoyée.");
    // Les gardes détruisent le Store, la clé synthétique et le dossier temporaire.
    Ok(())
}
