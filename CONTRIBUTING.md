# Contribuer à DockForge

Commencez par décrire un cas utilisateur concret : quel service, quel OS, quelle configuration et quel résultat devrait changer ? Une contribution facile à relire reste centrée sur ce cas et montre sa validation.

## Préparer son environnement

Suivez les prérequis du README, puis `npm ci` et `npm run tauri -- dev`. Pour travailler sur le moteur seul, `cargo test --manifest-path src-tauri/Cargo.toml --no-default-features` évite les dépendances graphiques à l’exécution. Le test HTTPS nécessite un port loopback temporaire.

Gardez les versions minor npm et Rust de Tauri synchronisées. Les deux lockfiles font partie de la modification lorsque les dépendances changent. Une mise à jour d’une moitié seulement peut passer `cargo check` puis échouer au bundling.

## Ajouter un service

1. Vérifiez la documentation de l’éditeur et la licence de l’image.
2. Ajoutez une entrée unique dans le catalogue commun : variables réellement interprétées, ports, volume, versions et notice de dépendances.
3. Marquez les secrets ; ne créez pas un champ de mot de passe que l’image ignore.
4. Fournissez une sonde lorsqu’elle peut attester le service ; sinon gardez l’état « démarré sans sonde ».
5. Testez la création, un redémarrage, les données persistantes et l’export d’accès avec des valeurs synthétiques.
6. Indiquez les architectures et tags effectivement vérifiés. Une variante non testée ne doit pas hériter d’un badge de certification.

## Modifier du code commenté

Les annotations sont régénérées par `python3 scripts/comment-source.py`. Écrivez d’abord les explications métier utiles à la fonction, puis régénérez les commentaires de ligne. La vérification TypeScript compare le JavaScript émis ; elle refuse un changement de rendu provoqué par un commentaire JSX mal placé.

Ne reformatez pas tout le dépôt pour une correction locale. Si un formatage d’une source annotée est nécessaire, retirez ses seuls commentaires générés `@ligne`, effectuez le formatage puis régénérez. Ne supprimez pas une ligne contenant du code sous prétexte qu’elle porte un commentaire en fin de ligne.

## Avant une proposition

- `npm run build` et `npm run check:comments`.
- Tests Rust et Clippy appropriés au changement.
- Tests Playwright pour un parcours ou une modification de style/accessibilité.
- Tests Docker réels pour un changement de provisioning ou de build.

Expliquez dans la proposition le problème, le comportement obtenu, les tests exécutés et les limites matérielles non vérifiées. N’inventez pas de résultat de test manquant.

## Données à partager

Utilisez uniquement des secrets synthétiques. Masquez les chemins personnels, domaines privés et informations client non nécessaires. Un JSON d’identifiants ou un `.env` réel n’a pas sa place dans une issue, une capture ou une pull request.

Pour un problème de sécurité, consultez `docs/SECURITE.md`. Aucune adresse de contact privée n’est inventée tant que le mainteneur n’a pas établi son canal de signalement.
