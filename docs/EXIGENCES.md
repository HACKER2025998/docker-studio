# Matrice de réception DockForge v2

Les documents client ont été lus intégralement et conservés dans `specifications/`. Leurs directives destinées à un agent sont des éléments du cahier des charges ; elles ne remplacent pas les instructions utilisateur ni les contrôles réels d’exécution. Les durées commerciales du CDC ne pilotent pas le développement.

| ID | Demande | Implémentation et preuve attendue |
| --- | --- | --- |
| FR-01 | OS, Docker, Compose, Buildx, WSL | Doctor natif, commandes bornées et ping Bollard ; diagnostic CLI et fenêtre |
| FR-02 | Assistant, instructions et refresh | Instructions adaptées et sondes relancées à chaud ; Windows winget/UAC ; installation Linux guidée pour préserver les dépôts du poste |
| FR-03 | Droits ponctuels Polkit/UAC | Remédiations à verbes fermés, confirmation produit et dialogue système ; pas d’élévation globale |
| FR-04 | Création guidée et catalogue | 35 modèles, formulaires, ports, volumes, variables, réseau existant/IP statique, GPU et ports supplémentaires ; PostgreSQL réel testé |
| FR-05 | Progression et diagnostic | Événements corrélés, lecture des derniers logs, état sain distinct de simple démarrage et corrections préparées sans suppression automatique |
| FR-06 | Export et coffre durable | Fiche entière AES-GCM, trousseau natif, JSON/URI/.env, archives et sauvegarde portable Argon2id ; tests d’intégrité et restauration |
| FR-07 | Images sans Dockerfile | Bases et paquets validés, extensions PHP, contexte borné, Dockerfile consultable, multi-stage Rust/.NET, vrai build BuildKit testé |
| FR-08 | Commandes, réseau et LAN | Import/suivi volontaire d’historiques, favoris, scan TCP privé, carte logique, QR/PIN et serveur HTTPS avec rôles |

## Corrections et variantes explicites

- Le champ `credentials_json_payload` en clair du schéma fourni contredirait le chiffrement du mot de passe. La fiche complète est donc chiffrée ; les métadonnées restent distinctes.
- L’édition Rust 2024 et les dépendances résolues exigent un Rust plus récent que 1.80. Le projet déclare un minimum de 1.88.
- Une variable arbitraire `ADMIN_PASSWORD` ne configure pas tous les éditeurs. Les modèles exposent seulement les variables prévues ; les assistants internes des applications restent nécessaires là où l’image l’exige.
- Un conteneur dont le processus tourne n’est pas automatiquement un service prêt. Les messages distinguent sonde saine, attente et absence de sonde.
- Un PIN de six chiffres n’encode pas une adresse complète et un authentificateur robuste ; il est séparé du lien/QR.
- Une cartographie physique FAI → switches → terminaux ne se déduit pas d’un scan TCP. Les observations réseau sont présentées comme logiques et partielles.
- Un binaire Linux n’est pas indépendant de l’architecture et des bibliothèques du système. Les formats de distribution sont construits sur des cibles définies.
- Aucun identifiant AUR/winget, domaine APT ou certificat de signature non publié n’est revendiqué comme disponible.

## Limites de la recette

La présence d’un modèle ne valide pas toutes les versions, architectures et configurations de l’image. Les stacks Wazuh, Milvus, Zabbix, GitLab et autres applications complexes nécessitent leurs dépendances et ressources réelles ; les notices expliquent ce qui n’est pas créé par la carte seule.

Le mode LAN actuel partage des fichiers, avec archivage manuel des dossiers et contribution en corps entier. Il n’implémente pas les uploads reprenables par blocs, la découverte mDNS ni la restauration de salle. Ces extensions ne sont pas présentées comme déjà livrées.

Le scan réseau actuel ne capture pas LLDP/CDP et ne collecte pas SNMP/WAN. Le Command Vault ne dispose pas de daemon capturant chaque nouvelle commande pendant que l’application est fermée ; il rattrape les historiques sélectionnés au lancement suivant.

La recette Windows/macOS, l’installation sur machines vierges, les images ARM/x86, les identités de signature et les essais d’une classe entière nécessitent leurs environnements cibles. Le workflow de build prépare ces contrôles ; seul son résultat exécuté est une preuve. La version reste alpha tant que ces points de réception ne sont pas validés.
