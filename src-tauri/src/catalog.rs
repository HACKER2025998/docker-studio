//! Catalogue embarqué : les différences entre services restent des données explicites.
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use serde::{Deserialize, Serialize};

// @ligne [module] Applique cet attribut au type ou à la fonction qui suit.
#[derive(Clone, Serialize, Deserialize, Debug)]
// @ligne [module] Définit les données échangées par ce module, sans comportement implicite.
pub struct Field {
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub key: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub label: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub secret: bool,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub default: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub required: bool,
}

// @ligne [module] Applique cet attribut au type ou à la fonction qui suit.
#[derive(Clone, Serialize, Deserialize, Debug)]
// @ligne [module] Définit les données échangées par ce module, sans comportement implicite.
pub struct Template {
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub id: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub name: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub category: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub description: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub image: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub versions: Vec<String>,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub port: u16,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub volume: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub fields: Vec<Field>,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub health: Vec<String>,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub notice: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub capabilities: Vec<String>,
}

// @ligne [catalog] Délimite cette opération pour isoler sa validation et son résultat.
pub fn catalog() -> Vec<Template> {
    // @ligne [catalog] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    serde_json::from_str(include_str!("catalog.json"))
        // @ligne [catalog] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .expect("catalogue embarqué validé par les tests")
}

// @ligne [find] Délimite cette opération pour isoler sa validation et son résultat.
pub fn find(id: &str) -> anyhow::Result<Template> {
    // @ligne [find] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    catalog()
        // @ligne [find] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .into_iter()
        // @ligne [find] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .find(|t| t.id == id)
        // @ligne [find] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        .ok_or_else(|| anyhow::anyhow!("Service inconnu"))
}

// @ligne [find] Applique cet attribut au type ou à la fonction qui suit.
#[cfg(test)]
// @ligne [find] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
mod tests {
    // @ligne [find] Applique cet attribut au type ou à la fonction qui suit.
    #[test]
    // @ligne [all_templates_have_unique_ids_and_valid_defaults] Délimite cette opération pour isoler sa validation et son résultat.
    fn all_templates_have_unique_ids_and_valid_defaults() {
        // @ligne [all_templates_have_unique_ids_and_valid_defaults] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
        let entries = super::catalog();
        // @ligne [all_templates_have_unique_ids_and_valid_defaults] Prépare une valeur locale qui sera complétée pendant cette opération.
        let mut ids = std::collections::HashSet::new();
        // @ligne [all_templates_have_unique_ids_and_valid_defaults] Vérifie cette propriété observable pour détecter une régression du contrat.
        assert!(entries.len() >= 35);
        // @ligne [all_templates_have_unique_ids_and_valid_defaults] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
        for t in entries {
            // @ligne [all_templates_have_unique_ids_and_valid_defaults] Vérifie cette propriété observable pour détecter une régression du contrat.
            assert!(ids.insert(t.id));
            // @ligne [all_templates_have_unique_ids_and_valid_defaults] Vérifie cette propriété observable pour détecter une régression du contrat.
            assert!(t.versions.contains(&t.image));
            // @ligne [all_templates_have_unique_ids_and_valid_defaults] Vérifie cette propriété observable pour détecter une régression du contrat.
            assert!(t.port > 0);
            // @ligne [all_templates_have_unique_ids_and_valid_defaults] Vérifie cette propriété observable pour détecter une régression du contrat.
            assert!(t.volume.is_empty() || t.volume.starts_with('/'));
        }
    }
}
