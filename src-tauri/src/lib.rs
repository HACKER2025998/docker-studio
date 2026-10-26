//! Moteur local : aucune opération Docker ne traverse un service applicatif distant.
// @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
pub mod backup;
// @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
pub mod catalog;
// @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
pub mod doctor;
// @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
pub mod engine;
// @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
pub mod images;
// @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
pub mod share;
// @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
pub mod store;
// @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
pub mod suite;

// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use serde::{Deserialize, Serialize};

/// Un événement ne contient jamais la configuration complète ni les identifiants.
// @ligne [module] Applique cet attribut au type ou à la fonction qui suit.
#[derive(Clone, Serialize, Deserialize, Debug)]
// @ligne [module] Définit les données échangées par ce module, sans comportement implicite.
pub struct Progress {
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub operation: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub phase: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub message: String,
}

/// Le même moteur sert la fenêtre native et les tests sans dépendre de WebKit.
// @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
pub type Reporter = std::sync::Arc<dyn Fn(Progress) + Send + Sync>;

// @ligne [report] Délimite cette opération pour isoler sa validation et son résultat.
pub fn report(out: &Reporter, operation: &str, phase: &str, message: impl Into<String>) {
    // @ligne [report] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    out(Progress {
        // @ligne [report] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        operation: operation.into(),
        // @ligne [report] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        phase: phase.into(),
        // @ligne [report] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        message: message.into(),
    });
}
