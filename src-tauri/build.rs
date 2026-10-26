// @ligne [main] Délimite cette opération pour isoler sa validation et son résultat.
fn main() {
    // @ligne [main] Vérifie cette condition avant d’appliquer l’action protégée.
    if std::env::var_os("CARGO_FEATURE_DESKTOP").is_some() {
        // @ligne [main] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        tauri_build::build();
    }
}
