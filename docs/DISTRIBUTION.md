# Distribution et exploitation

## Construire sur l’OS cible

Le projet utilise Tauri 2. Les cibles recommandées sont `deb,appimage` sous Linux, `nsis,msi` sous Windows et `dmg` sous macOS. Les icônes PNG/ICO/ICNS sont générées à partir du SVG du projet, sans emprunt à un logo tiers.

```sh
npm ci
npm run tauri -- build --bundles deb
```

L’exécutable release Linux est `src-tauri/target/release/dockforge`. Sa copie portable peut recevoir le suffixe `.bin` ; cette copie ne supprime pas ses dépendances GTK/WebKit/Secret Service. L’AppImage embarque davantage de bibliothèques, sans garantir la compatibilité avec toutes les distributions.

Les installateurs ne contiennent pas Docker Engine. L’utilisateur choisit et installe le moteur adapté à son système ; Docker Doctor en contrôle ensuite la disponibilité.

## Dossier de données

Le chemin provient de `app_data_dir` Tauri, sous le profil de l’utilisateur et l’identifiant `io.github.hacker2025998.dockforge`. Sous Linux, il est généralement sous `$XDG_DATA_HOME` ou `~/.local/share`. Windows utilise le répertoire applicatif du profil, macOS Application Support.

Pour un profil de test isolé, définissez `DOCKFORGE_DATA_DIR` avec un **chemin absolu** avant de lancer l’application. La clé locale est associée au chemin canonique du profil dans le trousseau. Pour migrer, utilisez une sauvegarde portable `.dfv` plutôt que déplacer la base en espérant conserver une association de clé implicite.

Les contributions LAN sont dans `contributions/` sous ce profil ; fermer la salle ne supprime pas automatiquement les fichiers déjà reçus. Les volumes Docker se trouvent dans le stockage du démon, pas dans le profil DockForge.

## Démarrage sur cet environnement de développement

Si Node est présent sans npm, l’outil utilisé pendant la construction est isolé dans `.tools/npm`, non ajouté au système. Le lanceur `./dev.sh` préfère npm lorsqu’il existe, puis ce bootstrap local. Ce répertoire est un outil de travail ignoré par Git, pas une dépendance runtime de l’installateur.

Le lanceur configure le PATH pour les commandes avant build de Tauri. Sur une autre machine, installez normalement Node et npm puis utilisez `npm ci`.

## APT, Arch et winget

Le template `packaging/PKGBUILD` montre la structure d’un paquet Arch binaire. Avant publication, fournissez une URL de release réelle, une archive immuable et son SHA-256. Ne publiez pas un PKGBUILD avec `SKIP` ni un nom AUR qui n’a pas été enregistré.

Pour APT, hébergez les `.deb` avec des index `Packages`/`Release` signés et publiez votre clé publique par un canal vérifiable. Le logiciel n’invente pas de domaine de dépôt ou de clé GPG.

Pour winget, publiez d’abord un installateur versionné à URL stable, calculez son SHA-256 puis soumettez un manifeste conforme aux règles du dépôt winget. Le nom donné dans le README client n’est pas une preuve d’enregistrement.

## Signature

Windows Authenticode, certificat Apple Developer ID/notarisation et signatures de dépôt Linux demandent des identités réelles du mainteneur. Aucun secret de signature ne doit être stocké dans Git. La matrice CI produit des artefacts non signés tant que les identités et secrets correspondants n’ont pas été configurés.

## Désinstallation

Supprimer le logiciel ne supprime pas les conteneurs, volumes, exports JSON, sauvegardes ou contributions. Exportez/sauvegardez ce qui doit être conservé puis nettoyez explicitement les ressources correspondantes. Un volume de base peut contenir les seules données d’un projet : le désinstalleur ne doit pas le supprimer pour rendre le poste « propre ».

## Construction Linux isolée

`./scripts/build-portable-linux.sh` construit sur Debian Bookworm avec Rust 1.88 et Node 24, dans un conteneur temporaire dont le socket Docker n’est pas monté. Les sources sont en lecture seule et les résultats vont sous `dist-linux/`. Le script télécharge les dépendances de build et vérifie l’archive Node avec les empreintes officielles. Le mode optionnel `DOCKFORGE_OFFLINE=true` nécessite un cache Cargo complet et déjà résolu ; il ne rend pas les étapes APT/npm hors ligne.

Sur Arch récent, linuxdeploy peut échouer avec son ancien outil strip et les nouveaux chargeurs GDK/Glycin. La construction Debian évite de fabriquer une arborescence de bibliothèques fictive sur l’hôte. La compatibilité glibc du système de construction définit toujours une borne : testez le résultat sur vos distributions cibles. Voir la [documentation Tauri AppImage](https://v2.tauri.app/distribute/appimage/).

## Version MSI et console Windows

L’application garde la version sémantique `2.0.0-alpha.1`. `tauri.windows.conf.json` fixe séparément la version MSI numérique `2.0.0.1`, car WiX n’accepte pas le suffixe alphanumérique de la préversion. Le code d’upgrade est stable et doit être conservé lors des évolutions. Le binaire release utilise le sous-système GUI ; le mode `--doctor` préserve une redirection stdout existante et n’attache la console parente que si nécessaire. La CI vérifie le sous-système du PE et le JSON obtenu par redirection.

## Installateurs de la livraison CI

Le dossier local `dist-linux/` contient également le setup Windows x64 NSIS, le MSI Windows x64 et le DMG macOS ARM64, récupérés de la CI verte. Le nom de ce dossier n’implique pas que ces fichiers s’exécutent sous Linux. La version et l’architecture figurent dans chaque nom ; leurs SHA-256 sont dans `SHA256SUMS`. Les builds sont vérifiés ; l’installation interactive et les signatures de publication restent à réceptionner.
