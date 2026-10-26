# Protocole de recette

## Mesures et traçabilité

Pour toute livraison, conserver le commit ou l’empreinte des sources, le système, la version Rust/Node, les lockfiles, le démon Docker, les commandes exécutées et leur résultat. Les rapports ne doivent contenir aucun vrai mot de passe.

Les tests locaux couvrent le moteur sans WebKit, le contrat Tauri, l’interface et des services réels. Un test ignoré n’est pas un test réussi. Les deux tests Docker doivent être déclenchés explicitement.

## Coffre et sauvegarde

1. Créer une fiche avec un secret synthétique reconnaissable.
2. Vérifier qu’aucun fichier SQLite/WAL ne contient ce secret en clair.
3. Déchiffrer la fiche avec sa bonne clé et son ID.
4. Modifier un octet du ciphertext, puis l’ID authentifié : les deux lectures doivent échouer.
5. Verrouiller et vérifier le refus de lecture.
6. Exporter une sauvegarde `.dfv`, refuser une mauvaise phrase et restaurer dans un profil avec une autre clé locale.
7. Restaurer une seconde fois : aucun doublon ni remplacement des fiches existantes.
8. Vérifier qu’un export ne remplace pas un fichier déjà présent.

Une recette manuelle doit compléter ces tests par un redémarrage de session système avec le trousseau natif de chaque OS. Les tests de cryptographie n’attestent pas à eux seuls le comportement d’un trousseau verrouillé de Windows ou macOS.

## Docker réel

Le test PostgreSQL crée un conteneur `dockforge-test-*`, publie un port loopback libre, crée un volume dédié, attend `pg_isready`, vérifie la fiche puis nettoie ses seules ressources. Le test BuildKit construit `dockforge-test-image:*`, vérifie la présence de l’image puis la retire.

Avant une recette manuelle, faire une sauvegarde des données utiles et utiliser des noms dédiés. Tester ensuite port occupé, nom occupé, image sans manifeste de la bonne architecture, permissions de montage, variable obligatoire vide et image dont la commande quitte immédiatement. Ne pas effacer un volume utilisateur pour transformer un test rouge en test vert.

## Interface

Playwright vérifie le catalogue, les filtres, le clavier, le formulaire en trois étapes, le contrôle du port proposé et les paramètres transmis au moteur. L’adaptateur natif est simulé uniquement dans un test d’interface ; la création réelle est couverte séparément par Docker.

Axe est exécuté avec les règles WCAG 2 A/AA et WCAG 2.1 AA sur les sept espaces et le formulaire. Les animations doivent avoir atteint leur état final avant la mesure du contraste. Tester aussi humainement le lecteur d’écran, le zoom 200 %, le focus de fermeture de modale et la préférence de réduction des animations.

## LAN réel

Un test lance un serveur TLS loopback temporaire et contrôle : refus anonyme, mauvais PIN, admission, cookie sécurisé, téléchargement partiel exact, refus de contribution en lecture, droit Contribution, refus de traversée de chemins et expulsion.

La validation multi-machine doit vérifier l’empreinte du certificat, le pare-feu, le réseau Wi-Fi isolé, l’expiration, les quotas, la perte du fichier source et le comportement d’une contribution interrompue. Le mode actuel n’annonce pas une reprise d’upload par blocs.

## Plateformes et paquets

Vérifier sur chaque OS l’ouverture de la fenêtre, les dialogues natifs, les emplacements de données, le trousseau, l’installation/désinstallation et le transport Docker. Inspecter le paquet produit : binaire, icônes, launcher et dépendances système. Une signature n’est validée qu’avec un certificat de mainteneur réel.

La taille du paquet, le RSS et le PSS doivent être mesurés, avec leur unité. Le RSS du seul parent ne représente pas la consommation complète de WebKit. Aucune mesure du projet OmniShell antérieur ne doit être attribuée à DockForge : les dépendances et le profil diffèrent.

Les résultats de cette livraison sont enregistrés dans `validation.json` à la fin des vérifications. Les captures de `screenshots/` montrent uniquement des données synthétiques ou le catalogue embarqué.

## Résultats locaux des 25–26 octobre 2026

- 11 tests moteur/HTTPS/sauvegarde réussis ; les 2 scénarios Docker ignorés par défaut ont ensuite été exécutés et réussis sur le vrai démon.
- 4 tests Playwright réussis, avec Axe sur les 7 espaces et le formulaire.
- Compilation TypeScript/Vite et Clippy tous targets avec `-D warnings` réussis.
- Binaire natif démarré dans un profil isolé ; diagnostic Docker prêt après installation utilisateur du plugin Buildx officiel manquant.
- Paquet Debian construit sur Bookworm de 7 092 424 octets (6,76 Mio), binaire, launcher et icônes vérifiés dans l’archive.
- Pics échantillonnés sur huit secondes : 304 588 Kio PSS (297,4 Mio), 699 396 Kio RSS cumulés (683,0 Mio), jusqu’à 9 processus en incluant les enfants de tous les threads. Ce profil vide de démarrage n’est pas une mesure de charge.

Les détails structurés, empreintes et versions figurent dans `validation.json`, `package-linux.json`, `resources-linux.json` et `tooling-linux.json`. La CI multi-OS est désormais exécutée et entièrement verte ; sa preuve est conservée dans `ci-result.json`.

Le test volontaire `cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --example keyring-smoke` a également validé le vrai trousseau Secret Service de la session Linux, déjà déverrouillé : création d’une clé temporaire, enregistrement d’une fiche synthétique, fermeture du Store puis réouverture et lecture. L’entrée de trousseau de ce profil temporaire est nettoyée à la fin ; aucune fiche utilisateur n’est lue.

Le build Debian Bookworm a aussi produit une AppImage. Son diagnostic natif est prêt sur le poste de recette avec `APPIMAGE_EXTRACT_AND_RUN=1`. Les premières mesures du build Arch restent conservées sous `reference-arch/` et ne sont pas attribuées au binaire Debian final.

Un paquet Arch `.pkg.tar.zst` a été construit avec makepkg/fakeroot à partir du binaire Debian. Le binaire, le launcher et l’icône sont vérifiés dans l’archive. Aucune installation de paquet n’a été effectuée sur le poste de recette ; le binaire direct reste lancé par `run.sh`.

La CI de référence du commit `9716e1f` génère aussi les installateurs Windows NSIS/MSI et macOS ARM64. Sous Windows, un contrôle du PE confirme le sous-système GUI et le vrai exécutable produit un JSON `--doctor` valide avec stdout redirigé. Docker Desktop n’étant pas prêt sur ce runner, ce test ne certifie pas un provisionnement Docker Windows. Les installateurs restent sans identité commerciale de signature fournie.
