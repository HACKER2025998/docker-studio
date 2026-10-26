DOCUMENT 02 // CAHIER DES CHARGES COMPLET (CDC) & README OPEN-SOURCE OFFICIEL
CAHIER DES CHARGES & README OPEN-SOURCE DE RÉFÉRENCEProjet : DockForge Studio (v2.0)
Spécifications Fonctionnelles et Techniques de A à Z, Gestion des Privilèges Système, Workflows de Création d'Images/Conteneurs, Export d'Identifiants JSON et README GitHub prêt pour une publication Open-Source internationale.
Élément du Projet
Détail Officiel
Nom du Logiciel
DockForge Studio — The Instant Container, Custom Image & DevOps Workstation
Créateur & Lead Architect
Kossi Bernard Etonam ADJOGBLE (GitHub : HACKER2025998 / Terminal : xhackerip)
Formats d'Installation
Windows (.exe / .msi / winget), Debian/Ubuntu (.deb / apt), Arch Linux (yay / AUR), Linux Portable (.bin / AppImage)
Partie I de ce Document
Cahier des Charges Fonctionnel & Technique Complet (Analyse, Exigences FR/NFR, Sécurité, Packaging)
Partie II de ce Document
README.md Open-Source de niveau international (prêt à être poussé sur GitHub/GitLab)


PARTIE I : CAHIER DES CHARGES COMPLET (DE A à Z)
1. Vision du Produit & Justification de l'Architecture Desktop Native
Lorsqu'un développeur, un administrateur de bases de données (DBA), un ingénieur DevOps/DevSecOps ou un étudiant travaille sur un projet, il a constamment besoin de lancer une base de données (PostgreSQL, MySQL, MariaDB, Redis, Oracle), un serveur web ou un conteneur sur mesure. Aujourd'hui, ce processus oblige à :
Chercher la syntaxe exacte des variables d'environnement : Se rappeler si c'est `POSTGRES_PASSWORD`, `MYSQL_ROOT_PASSWORD` ou `MARIADB_PASSWORD`, configurer les ports et les volumes.
Perdre les identifiants après la création : Une fois le `docker run` exécuté dans un terminal, si le terminal est fermé, l'utilisateur oublie souvent quel mot de passe, quel nom d'utilisateur ou quel port il avait attribué au conteneur.
Subir des erreurs obscures : Quand un conteneur crash au démarrage (port déjà utilisé, conflit de nom, permissions de volume), Docker renvoie des erreurs brutes difficiles à interpréter rapidement.
Pourquoi DockForge Studio ne peut PAS être une application Web et DOIT être un exécutable Desktop (.exe, .deb, .bin) : Une application web dans un navigateur n'a aucun droit d'accès au système d'exploitation local. DockForge Studio a besoin de vérifier si Docker est installé sur l'OS, de demander l'élévation de privilèges à l'utilisateur (Polkit / UAC Administrateur), de communiquer directement avec le socket noyau `/var/run/docker.sock` ou `//./pipe/docker_engine`, de tester les ports TCP locaux et de créer des interfaces réseau.
2. Matrice Complète des Exigences Fonctionnelles (FR)
ID
Module
Spécification Fonctionnelle
Priorité
FR-01
OS & Docker Doctor
Au lancement de l'application, détection automatique de l'OS (Windows, Arch, Debian, Ubuntu) et vérification de la présence de Docker Engine, Docker Compose, Buildx et WSL2.
P0 (Critique)
FR-02
Assistant d'Installation & Refresh
Si Docker ou un composant manque, affichage d'un guide d'installation en 1 clic adapté à l'OS et d'un bouton [🔄 Rafraîchir l'application] qui re-teste le système à chaud sans redémarrer l'app.
P0 (Critique)
FR-03
Gestion des Privilèges
Vérification automatique des droits sur le socket Docker. Si l'utilisateur n'a pas les droits, demande d'autorisation native (Polkit/UAC) pour démarrer le démon ou configurer le groupe `docker`.
P0 (Critique)
FR-04
Création Conteneur Guidée
Formulaires intelligents pour créer n'importe quel conteneur (PostgreSQL, MySQL, Mongo, Redis, Nginx, GLPI, etc.) en demandant uniquement : Username, Password, Nom BD, Port et Mode Réseau (Localhost vs IP LAN).
P0 (Critique)
FR-05
Rapport de Logs & Diagnostic
Pendant et après la création, génération d'un rapport de logs clair. En cas d'échec, traduction de l'erreur technique en message clair en français avec bouton de correction en 1 clic.
P0 (Critique)
FR-06
Export JSON & Vault Persistant
Dès que le conteneur est créé, génération d'un fichier `.json` téléchargeable contenant tous les identifiants et chaînes de connexion (URI, JDBC, .env, CLI). Conservation dans l'application tant que le conteneur n'est pas supprimé.
P0 (Critique)
FR-07
Constructeur d'Images (Image Forge)
Création d'images Docker sur mesure sans coder de Dockerfile (choix de l'OS de base, cases à cocher pour les paquets/langages, build en direct) puis création immédiate de conteneurs basés sur ces images.
P0 (Critique)
FR-08
Suite Intégrée (Vault, NetMap, LAN)
Intégration native avec la rétro-ingestion des commandes du terminal (`sudo`, `docker`, `nmap`), la cartographie réseau automatique et le partage de fichiers en classe par QR Code / Code à 6 chiffres.
P1 (Haute)

3. Sécurité, Gestion des Droits & Stockage des Mots de Passe
Principe du Moindre Privilège : L'interface graphique tourne avec les droits de l'utilisateur courant. Seules les actions système critiques (installation initiale de Docker, démarrage de `docker.service`, attribution d'IP dédiée) déclenchent une demande d'élévation ponctuelle approuvée par l'utilisateur.
Chiffrement du Coffre-Fort d'Identifiants : Les mots de passe saisis lors de la création de conteneurs (PostgreSQL, MySQL, etc.) sont chiffrés sur le disque en **AES-256-GCM** via le trousseau natif de l'OS (`libsecret` / GNOME Keyring sous Linux, `DPAPI` Credential Manager sous Windows). Lorsque l'utilisateur clique sur **[📥 Télécharger le fichier JSON]** ou **[👁️ Voir mes identifiants]**, le déchiffrement s'opère instantanément en mémoire vive.

PARTIE II : README.MD OFFICIEL (PRÊT POUR GITHUB / OPEN-SOURCE)
Le contenu ci-dessous est rédigé selon les standards des plus grands projets Open-Source internationaux (avec badges, présentation visuelle, cas d'usage, guide d'installation multi-OS et guide de contribution).
🐳 DOCKFORGE STUDIO — CREATE IMAGES, SPIN UP CONTAINERS & MANAGE CREDENTIALS IN SECONDS[License: MIT]  [Built with: Rust + Tauri v2]  [OS: Linux (.deb, .bin, yay, apt) | Windows (.exe) | macOS (.dmg)]  [UI: Daylight Studio]Le studio desktop natif ultra-léger qui vérifie votre environnement Docker, construit vos images sur mesure, déploie n'importe quel conteneur (PostgreSQL, MySQL, Redis, Nginx, Stacks IA/Web) via des formulaires intelligents, diagnostique les erreurs en clair et sauvegarde automatiquement tous vos identifiants en JSON.

✨ Pourquoi DockForge Studio change la donne ?
Créer une base de données PostgreSQL ou MySQL pour un nouveau projet ne devrait jamais prendre 15 minutes de recherche sur Google ou ChatGPT, et vous ne devriez jamais perdre le mot de passe ou le port d'un conteneur créé il y a deux mois. **DockForge Studio** automatise tout le cycle de vie de A à Z :
🩺 1. Smart OS & Docker Doctor au Démarrage : Dès l'ouverture, DockForge Studio inspecte votre OS (Windows 10/11, Arch Linux, Debian, Ubuntu, Kali). Si Docker ou un composant manque, il vous guide pour l'installer et vous permet de rafraîchir l'application en 1 clic sans la relancer.
🚀 2. Création de Conteneurs à la Demande (Zéro Ligne de Commande) : Besoin d'un PostgreSQL, d'un MySQL, d'un Redis ou d'un serveur Web ? Sélectionnez le service, remplissez un formulaire clair (Username, Password, Nom de la base, Port, Exposition Localhost `127.0.0.1` ou IP LAN `192.168.x.x`) et cliquez sur **Lancer**.
📋 3. Rapport de Logs Intelligent & Diagnostic d'Erreurs en Clair : DockForge Studio surveille le démarrage réel du conteneur. Si tout se passe bien, vous obtenez un rapport vert de validation. Si un problème survient (port déjà occupé, conflit de nom, erreur de volume), il traduit l'erreur technique en français clair et propose un bouton pour la corriger en 1 clic.
🔐 4. Export JSON Instantané & Coffre-Fort d'Identifiants Persistant : Une fois le conteneur créé, téléchargez immédiatement un fichier `<conteneur>_credentials.json` contenant l'IP, le port, le username, le password et toutes les chaînes de connexion prêtes à copier (`URI`, `JDBC Java`, `.NET ConnectionString`, `.env Laravel/Node`, commande `psql`/`mysql`). Tant que le conteneur n'est pas supprimé, retrouvez ses accès à tout moment dans l'application !
🏗️ 5. Constructeur d'Images Visuel (Image Forge) : Créez vos propres images Docker personnalisées en sélectionnant une base (Ubuntu, Alpine, Arch, Python, Node, PHP, Rust) et vos paquets, puis lancez des conteneurs basés sur vos images en un clic.
⚡ 6. Intégration Complète DevOps (Command Vault, NetMap & ClassShare LAN) : Inclut la rétro-ingestion automatique de toutes vos commandes terminal passées, la cartographie réseau du FAI jusqu'aux terminaux et le partage de fichiers hors-ligne par QR Code / Code à 6 chiffres.
📥 Installation Rapide (Linux & Windows)
🐧 Arch Linux / Manjaro (AUR — yay / paru)
Installation en 1 commande via AUR$ yay -S dockforge-studio-bin# Ou compilation depuis les sources Rust :$ yay -S dockforge-studio

🌀 Debian / Ubuntu / Kali / Linux Mint (APT & Paquet .deb)
Installation via APT ou fichier .deb$ sudo apt update && sudo apt install ./dockforge-studio_2.0.0_amd64.deb# Ou via le binaire portable universel Linux (.bin) :$ chmod +x dockforge-studio-linux-x86_64.bin && ./dockforge-studio-linux-x86_64.bin

🪟 Windows 10 & Windows 11 (.exe / Winget)
Installation sous Windows# Téléchargez et exécutez DockForge-Studio-Setup-2.0.0.exe# Ou installez depuis PowerShell avec Winget :PS C:\> winget install HACKER2025998.DockForgeStudio

📖 Exemple d'Utilisation en 30 Secondes : Créer une Base PostgreSQL
Étape
Action de l'Utilisateur dans DockForge Studio
Ce que fait le Moteur Rust en Arrière-Plan
1. Check Initial
Ouvre DockForge Studio.
Vérifie l'OS, valide que le démon Docker tourne et que les permissions socket sont actives.
2. Sélection
Clique sur [🚀 Créer un Conteneur] puis choisit [🐘 PostgreSQL].
Charge le formulaire intelligent PostgreSQL et teste si le port `5432` est libre sur la machine.
3. Paramétrage
Entre l'utilisateur `dev_user`, clique sur [🎲 Générer Mot de Passe], choisit la base `ecommerce_db` et coche [📡 Accessible sur IP LAN 192.168.1.45].
Prépare la configuration du conteneur, crée le volume persistant et configure le binding réseau.
4. Lancement & Rapport
Clique sur [🚀 Lancer le Conteneur].
Télécharge l'image si absente, démarre le conteneur, exécute `pg_isready` et affiche : « ✅ Conteneur opérationnel en 2.8s ».
5. Export JSON & Stockage
Clique sur [📥 Télécharger les Identifiants (.json)].
Sauvegarde le fichier `pg-ecommerce_credentials.json` sur le PC et chiffre la fiche dans le Coffre-Fort interne pour toute consultation future.

🛠️ Compiler depuis le Code Source (Pour les Contributeurs)
Prérequis : Rust (1.80+), Node.js (20+), pnpm$ git clone https://github.com/HACKER2025998/dockforge-studio.git$ cd dockforge-studio$ pnpm install$ pnpm tauri dev        # Lance l'application Desktop en mode développement$ pnpm tauri build      # Génère les installateurs .exe, .deb, .bin et .AppImage

🤝 Contribuer & Licence
Les contributions (nouveaux templates de conteneurs, nouvelles règles de traduction d'erreurs Docker, traductions UI) sont les bienvenues via Pull Requests ! Distribué sous licence Open-Source MIT. Créé avec passion par Kossi Bernard Etonam ADJOGBLE.