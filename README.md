# DockForge Studio

**Un atelier Docker local pour créer des conteneurs, construire des images et retrouver leurs accès.**

Créé par **Kossi Bernard Etonam ADJOGBLE** — HACKER2025998 / xhackerip. Code du projet sous [licence MIT](LICENSE). Version de développement : **2.0.0-alpha.1**.

DockForge est une application **desktop Rust 2024 / Tauri 2 / React 19**. Son moteur communique directement avec le démon Docker local par Bollard. Le navigateur sert à développer et consulter l’interface ; il ne remplace pas le moteur natif et ne simule pas des conteneurs actifs.

![Catalogue DockForge, interface claire Daylight Studio](docs/screenshots/catalogue.png)

Ce dépôt contient du code exécutable, les deux spécifications client intégrales dans [docs/specifications](docs/specifications), les règles de sécurité et des tests. Les exemples commerciaux du README fourni par le client ne constituent pas des preuves de publication : un paquet AUR, une référence winget ou un dépôt APT ne sont pas annoncés comme disponibles sans publication réelle. La [matrice de réception](docs/EXIGENCES.md) distingue les fonctions livrées et les validations restantes.

## Se repérer

- [Installer les prérequis et lancer le logiciel](#démarrer-depuis-les-sources).
- [Comprendre Docker Doctor](#docker-doctor--préparer-la-machine).
- [Créer son premier PostgreSQL](#créer-son-premier-postgresql).
- [Choisir les ports, le réseau et la persistance](#réseau-et-stockage--faire-le-bon-choix).
- [Retrouver, copier et exporter des identifiants](#le-coffre-des-conteneurs-et-des-accès).
- [Sauvegarder et restaurer le coffre](#sauvegarde-portable-chiffrée).
- [Construire une image personnelle](#image-forge--votre-image-sans-écrire-le-dockerfile).
- [Command Vault, NetMap et ClassShare](#les-outils-intégrés).
- [Développer, tester et contribuer](#développer-et-tester).
- [Architecture](docs/ARCHITECTURE.md), [sécurité](docs/SECURITE.md), [recette](docs/RECETTE.md), [distribution](docs/DISTRIBUTION.md).

## Ce que vous pouvez faire

| Espace | Parcours disponible |
| --- | --- |
| Docker Doctor | Détection OS/architecture, sondes Docker/Compose/Buildx/WSL, ping du démon et rafraîchissement à chaud |
| Container Forge | 35 modèles embarqués, formulaires, génération de mots de passe, contrôle de ports, réseau, volumes, GPU et suivi de création |
| Image Forge | Bases Linux/langages, paquets autorisés, extensions PHP, contexte projet filtré, aperçu Dockerfile, build BuildKit et lancement guidé |
| Conteneurs & accès | État live, arrêt/démarrage, suppression confirmée, archives, affichage des accès, export JSON/URI/.env et sauvegarde portable chiffrée |
| Command Vault | Import Bash/Zsh/Fish/PowerShell, suivi volontaire des historiques, masquage, recherche, favoris et copie sans exécution |
| NetMap Studio | Découverte TCP explicite sur un /24 privé, observations exportables et carte logique |
| ClassShare LAN | Serveur HTTPS local, QR d’adresse et PIN séparé, lecture, contribution contrôlée, révocation et téléchargements partiels |

Aucun compte cloud ni télémétrie applicative n’est nécessaire. **Le téléchargement d’images et l’installation des dépendances nécessitent néanmoins un accès aux registres et dépôts concernés** ; une image déjà présente peut être utilisée hors ligne. Les applications lancées dans Docker ont leur propre comportement réseau, indépendant de DockForge.

## Ouvrir la livraison locale

Dans ce dossier, `./run.sh` ouvre le binaire Linux déjà compilé lorsqu’il est présent dans `dist-linux/`. `./run.sh --doctor` produit le diagnostic sans fenêtre. `./dev.sh` ouvre le mode développement natif et prépare le PATH de npm pour cet environnement. Ces scripts n’installent pas Docker et ne modifient pas vos privilèges.

## Démarrer depuis les sources

### Prérequis communs

- Rust **1.88 ou supérieur**, Cargo et une chaîne C/C++ native. L’édition Rust 2024 n’est pas compatible avec le minimum Rust 1.80 suggéré dans le README client.
- Node.js **22.12+ ou 24 LTS**, avec npm. Les versions exactes JavaScript sont verrouillées dans `package-lock.json`, celles de Rust dans `src-tauri/Cargo.lock`.
- Docker Engine ou Docker Desktop, Docker Compose v2 et Buildx. Les droits doivent permettre d’accéder au démon **local**.
- Un trousseau natif disponible pour les identifiants : Secret Service sous Linux, Credential Manager sous Windows, Keychain sous macOS.
- Les bibliothèques de développement Tauri propres à votre système.

L’application sait expliquer un environnement absent. Elle ne vous oblige pas à installer Docker pour simplement consulter les formulaires ou utiliser les fonctions locales indépendantes.

### Debian / Ubuntu / Kali

Installez les prérequis de compilation, en adaptant les noms des paquets à votre version :

```sh
sudo apt update
sudo apt install build-essential curl pkg-config libssl-dev \
  libgtk-3-dev libwebkit2gtk-4.1-dev librsvg2-dev libayatana-appindicator3-dev \
  libsecret-1-dev
```

Installez Docker avec les instructions de votre distribution sur [la documentation officielle Docker](https://docs.docker.com/engine/install/). Évitez de mélanger sans vérification les paquets `docker.io` et ceux du dépôt Docker CE. Le logiciel n’exécute pas un script téléchargé avec les droits administrateur.

### Arch Linux / Manjaro

```sh
sudo pacman -S --needed base-devel rust nodejs npm webkit2gtk-4.1 gtk3 \
  librsvg libayatana-appindicator libsecret docker docker-buildx docker-compose
```

Démarrez le démon avec le mécanisme de votre système. Docker Doctor peut ouvrir une demande Polkit ponctuelle pour `systemctl start docker.service` après votre clic.

### Windows

Installez Visual Studio Build Tools avec les outils C++, WebView2, Rust et Node.js. Installez Docker Desktop et configurez WSL2 si votre installation l’utilise. Les images Linux du catalogue exigent un moteur **Linux containers**. La simple existence de `wsl.exe` ne garantit pas que la virtualisation et votre distribution WSL soient opérationnelles.

L’application utilise le Named Pipe local Docker ; elle ne publie pas le socket sur un port HTTP. Le bouton d’installation lance l’identifiant winget exact de Docker Desktop via une demande UAC ; l’interface entière ne devient pas administrateur.

### macOS

Installez les Command Line Tools Xcode, Rust, Node.js et Docker Desktop. Une chaîne de compilation macOS et les identités Apple correspondantes sont nécessaires pour une distribution signée/notarisée. L’interface et le moteur ont une configuration portable ; une compilation Linux n’est pas une recette macOS.

### Installer les dépendances du projet

Ouvrez un terminal dans le dossier contenant ce README :

```sh
npm ci
npm run tauri -- dev
```

Le second lancement ouvre **une vraie fenêtre native**. Tauri lance Vite pour l’interface et compile le moteur Rust. Docker Doctor contrôle ensuite l’environnement. Aucun service Docker n’est démarré et aucun groupe utilisateur n’est modifié automatiquement.

Pour explorer seulement l’interface :

```sh
npm run dev
```

Ouvrez l’adresse Vite affichée, généralement `http://127.0.0.1:1420`. La bannière « Aperçu navigateur » indique explicitement que les actions système sont indisponibles. Les formulaires peuvent être consultés ; la création reste désactivée sans diagnostic natif valide.

Si npm est absent de votre installation de Node, installez-le via votre gestionnaire de paquets ou utilisez le lanceur fourni pour cet environnement décrit dans [DISTRIBUTION.md](docs/DISTRIBUTION.md). Ne remplacez pas Node au hasard pendant qu’un build est en cours.

### Construire un installateur

```sh
npm ci
npm run build
npm run tauri -- build --bundles deb
```

Les paquets apparaissent dans `src-tauri/target/release/bundle/`. Exemples de cibles natives : `deb,appimage` sous Linux ; `nsis,msi` sous Windows ; `dmg` sous macOS. Construisez sur l’OS correspondant ou préparez une chaîne de cross-compilation explicite. Un `.bin` Linux dépend toujours de son architecture et des bibliothèques système ; il n’est pas « universel » au sens d’un exécutable sans dépendance.

Pour un diagnostic sans ouvrir la fenêtre, après compilation :

```sh
./src-tauri/target/release/dockforge --doctor
```

Cette commande imprime un rapport JSON. Le diagnostic ne contient pas les identifiants du coffre.

## Docker Doctor : préparer la machine

Au démarrage, le logiciel examine les éléments suivants :

1. Le système d’exploitation, son architecture et les informations de distribution disponibles.
2. `docker --version`, `docker compose version` et `docker buildx version`.
3. Sous Windows, `wsl --status`.
4. L’accessibilité du démon via Bollard et le transport local de l’OS.

Une case verte indique le succès de **cette sonde**, pas la certification de tout Docker. Un ping réussi prouve que votre compte peut parler au démon ; il ne prouve pas qu’une image donnée supporte votre processeur ni que le pare-feu acceptera un port publié.

Le bouton **Rafraîchir le diagnostic** relance les sondes sans redémarrer la fenêtre. Il est utile après installation de composants ou démarrage du démon.

Sous Linux, deux remédiations sont prévues :

- **Démarrer Docker via Polkit** ouvre une autorisation native pour démarrer `docker.service`.
- **Configurer le groupe Docker** ajoute le compte courant au groupe `docker`, après confirmation dans l’application puis autorisation système.

Le groupe Docker permet généralement de prendre le contrôle de l’hôte. Pour un poste administré, respectez sa politique de sécurité. L’ajout au groupe ne recharge pas les groupes du processus déjà ouvert : fermez **votre session système complète**, reconnectez-vous puis relancez DockForge. Un rafraîchissement de l’interface ne remplace pas cette reconnexion.

Sous Windows, les actions prévues sont l’installation de Docker Desktop avec winget/UAC et son lancement depuis le chemin standard. Si vous avez choisi un chemin personnalisé, lancez Docker Desktop depuis votre système puis rafraîchissez.

Le bouton d’installation Linux appelle les paquets connus via Polkit : Arch utilise pacman ; Debian/Ubuntu utilise Docker CE et ses plugins **après configuration préalable du dépôt officiel**. Il ne configure pas un dépôt tiers à votre place. Les instructions Linux d’installation restent explicites : le programme ne modifie pas silencieusement vos dépôts APT, vos clés de signature, votre configuration WSL ou vos paramètres noyau.

## Créer son premier PostgreSQL

### 1. Choisir le modèle

Ouvrez **Container Forge**, cherchez PostgreSQL et cliquez sur **Créer**. Le catalogue est embarqué : aucune recherche distante n’est nécessaire pour afficher les champs.

Les versions proposées sont PostgreSQL 14, 15, 16 et 17 sur Alpine. Le champ image reste modifiable. Si vous choisissez une version dont les chemins de données changent, vous devez adapter le montage : un changement de tag ne garantit pas une migration automatique des données.

### 2. Identifier le service

Donnez un nom simple, par exemple `campus-db`. Les noms acceptent lettres, chiffres, point, tiret et soulignement, sur 1 à 63 caractères. Un nom ne peut pas contenir une commande shell.

Complétez :

- `POSTGRES_USER` : par exemple `campus_admin`.
- `POSTGRES_PASSWORD` : saisissez une valeur ou générez 24 caractères avec le dé.
- `POSTGRES_DB` : par exemple `campus`.

La génération utilise une source aléatoire système cryptographique. Le mot de passe reste masqué par défaut. Les variables supplémentaires acceptent une entrée `NOM=valeur` par ligne ; elles peuvent contenir d’autres secrets et feront partie de la fiche chiffrée.

### 3. Choisir le réseau

Pour un premier essai, gardez `127.0.0.1` et le port hôte `5432`. Le port interne reste `5432`. Si le port hôte est déjà occupé, le formulaire propose un port voisin disponible, par exemple `5433`.

Ce contrôle effectue un bind local puis libère immédiatement la socket. Un autre processus peut encore occuper le port ensuite : **la réponse finale de Docker fait foi**. Aucune promesse de contrôle en 0,2 ms n’est faite sans mesure sur votre matériel.

### 4. Choisir la persistance

Gardez **Nouveau volume Docker géré**, avec `/var/lib/postgresql/data`. Chaque création reçoit un nouveau volume nommé et étiqueté DockForge. Un retry ne réutilise pas silencieusement un volume potentiellement incompatible.

Un volume reste présent après l’arrêt ou la suppression du conteneur. Ce comportement évite une perte de données involontaire ; il faut gérer et sauvegarder les volumes séparément.

### 5. Lancer et lire le résultat

Cliquez sur **Créer & lancer**. Le moteur :

1. Revalide tous les champs côté Rust.
2. Vérifie l’image locale et la télécharge si nécessaire, avec événements de progression.
3. Prépare le montage et les publications de ports.
4. Crée le conteneur, sans encore le démarrer.
5. Enregistre sa fiche d’accès chiffrée.
6. Démarre le conteneur puis attend sa sonde de santé lorsqu’elle existe.
7. Met à jour la fiche avec l’adresse interne observée et le résultat du démarrage.

Pour PostgreSQL, la sonde appelle `pg_isready` sur TCP local, avec l’utilisateur et la base configurés. Le serveur temporaire d’initialisation ne suffit donc pas à valider la disponibilité finale.

Trois résultats sont distingués :

- **healthy** : la sonde de l’image ou du modèle a réussi.
- **health_pending** : le conteneur tourne, mais la sonde n’a pas encore confirmé sa disponibilité dans le délai d’attente.
- **running_without_healthcheck** : le processus tourne, sans preuve de disponibilité applicative.

Un service Web sans sonde n’est jamais présenté comme une base validée. Un échec conserve les accès dès lors qu’ils ont été enregistrés. Retrouvez le conteneur dans **Conteneurs & accès**, puis consultez ses logs avant de relancer.

### 6. Se connecter

Téléchargez le JSON, copiez l’URI ou retrouvez la fiche dans le coffre. Pour un port hôte `5433`, l’adresse à utiliser depuis ce PC sera `127.0.0.1:5433`, même si PostgreSQL écoute sur `5432` dans son conteneur.

## Réseau et stockage : faire le bon choix

### Localhost, LAN et IP interne

| Choix | Signification |
| --- | --- |
| `127.0.0.1` | Le port publié n’est accessible que depuis cet ordinateur, sous réserve du comportement réseau de l’OS/Docker |
| Une IP LAN de l’hôte | Le port est attaché à cette interface ; l’accès distant dépend du routage et du pare-feu |
| `0.0.0.0` | Écoute sur toutes les interfaces IPv4 ; l’URI locale utilise 127.0.0.1, les autres machines doivent utiliser une IP réelle |
| IP statique Docker | Adresse à l’intérieur d’un réseau Docker personnalisé ; elle n’est pas automatiquement routable sur le LAN |

L’exposition hors localhost nécessite une confirmation dans le formulaire. Ne rendez pas accessible un service non authentifié en supposant que Docker ajoutera un mot de passe.

Pour une IP interne statique, choisissez un réseau existant dont le sous-réseau contient cette IP. Docker refuse les adresses incohérentes et les chevauchements. DockForge ne crée pas une interface macvlan et ne modifie pas les routes de l’hôte à votre insu.

Vous pouvez publier jusqu’à seize ports supplémentaires sous forme de tableau JSON :

```json
[
  { "host": 8081, "container": 80, "protocol": "tcp" },
  { "host": 5353, "container": 53, "protocol": "udp" }
]
```

Le port principal est TCP, sauf pour le modèle WireGuard qui utilise UDP. Chaque port est contrôlé, puis Docker effectue l’allocation finale. Le champ avancé est volontairement explicite pour les services multiports ; il ne laisse pas croire que le seul port Web de Pi-hole suffit à fournir le DNS.

### Lier une application à une base existante

Les modèles GLPI, WordPress, Odoo, Nextcloud et Zabbix proposent de sélectionner une base déjà enregistrée dans DockForge. Après votre sélection, le moteur déverrouille la fiche de cette base et préremplit les variables compatibles.

Si la base utilise un réseau Docker personnalisé, le formulaire reprend ce réseau et son nom DNS de conteneur. Sinon, il propose l’IP interne actuellement observée avec un avertissement : cette IP peut changer à la recréation. Un réseau personnalisé est préférable pour une liaison stable.

Cette aide **réutilise la base, son utilisateur et ses droits existants**. Elle ne crée pas automatiquement une base isolée par application et ne remplace pas une politique de privilèges SQL. Pour un déploiement sérieux, créez un utilisateur et un schéma dédiés dans le moteur concerné.

### Volume ou dossier local

Un **volume Docker** est géré par le démon ; son emplacement peut se trouver dans la VM Docker Desktop. Il ne faut pas le traiter comme un simple dossier du poste Windows.

Un **bind mount** expose le dossier local que vous avez choisi au conteneur. Vérifiez le chemin interne et les UID/GID attendus par l’image. Un dossier vide peut masquer les fichiers initialement présents à cet emplacement. Le programme ne lance pas de `chmod -R 777` pour contourner une erreur.

**Aucun montage supplémentaire** laisse le comportement de l’image s’appliquer. Certains Dockerfiles déclarent leurs propres volumes ; « aucun montage supplémentaire » ne promet donc pas qu’aucun volume anonyme ne sera créé par Docker.

### GPU, capacités et architecture

Le GPU nécessite un runtime et des pilotes Docker compatibles. Cocher la case transmet une demande de périphérique ; cela n’installe pas le pilote NVIDIA.

WireGuard demande `NET_ADMIN`, avec confirmation. Cette capacité ne configure pas à elle seule votre VPN, vos clés ou les règles de routage. Les images qui nécessitent plusieurs capacités ou montages spécifiques doivent être adaptées à leur documentation.

Choisir `linux/amd64` sur ARM nécessite une émulation déjà configurée dans Docker. L’application explique cette condition ; elle n’installe pas silencieusement des handlers binfmt privilégiés.

## Le coffre des conteneurs et des accès

### Ce qui est conservé

La fiche chiffrée contient le nom, l’image, les adresses, les ports, les variables configurées et les chaînes de connexion disponibles. Le mot de passe n’est pas conservé en clair dans une seconde colonne JSON.

Le chiffrement est **AES-256-GCM**, avec un nonce aléatoire par écriture et l’identifiant du conteneur comme donnée authentifiée. Une fiche ne peut pas être transplantée vers un autre identifiant sans faire échouer la vérification d’intégrité.

La clé maître aléatoire est stockée dans le trousseau natif. Si celui-ci est absent ou verrouillé, le logiciel refuse de créer une fiche non protégée. Aucun fichier `master.key` en clair n’est créé pour « faire marcher quand même » le coffre.

La base SQLite conserve en clair certaines métadonnées utiles : noms des conteneurs, images, dates, archives, recettes d’images et commandes déjà filtrées. **Il ne s’agit pas d’un chiffrement intégral SQLCipher de la base**. La protection porte sur les fiches de secrets ; chiffrez aussi le disque si votre contexte exige la confidentialité de toutes les métadonnées.

### Consulter et masquer

Sélectionnez une fiche puis **Déverrouiller les accès**. Le mot de passe reste masqué ; l’œil permet de voir la valeur et les variables du service. **Masquer & verrouiller** retire le document de l’interface et efface la clé mise en cache par le moteur.

Le passage de l’application en arrière-plan masque également la fiche affichée et demande le verrouillage. Le presse-papier est celui de votre OS ; un gestionnaire de presse-papiers peut en conserver l’historique. DockForge ne l’efface pas arbitrairement après copie.

### Export JSON

Le dialogue d’enregistrement propose `<nom>_credentials.json`. Il faut choisir un **nouveau fichier** : l’écriture exclusive refuse d’écraser un fichier existant ou de suivre un lien à sa place. Sous Unix, le fichier est créé avec les permissions `0600`.

Un export JSON contient volontairement les secrets **en clair**, pour être utilisé dans vos outils. Il n’est pas une sauvegarde chiffrée. Ne le placez pas dans un dépôt Git, un partage public ou un dossier de synchronisation non maîtrisé.

Exemple réduit, avec valeurs fictives :

```json
{
  "generated_by": "DockForge Studio v2.0.0-alpha.1",
  "container": { "id": "exemple", "name": "campus-db", "image": "postgres:17-alpine", "status": "running", "readiness": "healthy" },
  "network": { "bind_ip": "127.0.0.1", "host_port": 5433, "container_port": 5432 },
  "credentials": { "engine": "postgres", "username": "campus_admin", "password": "EXEMPLE_A_REMPLACER", "database": "campus" },
  "connection_strings": { "uri_standard": "postgresql://campus_admin:EXEMPLE_A_REMPLACER@127.0.0.1:5433/campus" }
}
```

Le document réel comporte aussi les variables et les formats JDBC, .NET, `.env` et CLI applicables. Tous les services n’utilisent pas une URI de base de données : pour une interface Web, l’URI est son adresse HTTP et les variables d’accès restent dans `credentials.environment`.

La fiche décrit la configuration capturée lors de la création. Un changement manuel de mot de passe **à l’intérieur** du service ne peut pas être deviné par DockForge. L’état live est affiché séparément dans la liste ; une fiche ancienne ne doit pas être prise pour une mesure de santé actuelle.

### Arrêter, supprimer ou archiver

Vous pouvez démarrer et arrêter les conteneurs étiquetés comme gérés par DockForge. Le moteur refuse d’appliquer ces actions métier à un autre conteneur non géré.

La suppression n’est possible qu’après arrêt. Une confirmation permet soit d’archiver la fiche d’accès, soit de l’effacer. Les volumes ne sont pas supprimés. Une fiche d’un conteneur effacé depuis un autre outil peut être archivée manuellement ; aucune purge silencieuse n’efface vos accès lorsque le démon est momentanément indisponible.

## Sauvegarde portable chiffrée

Dans le coffre, ouvrez **Sauvegarde chiffrée & restauration**. Saisissez une phrase de douze caractères minimum, répétez-la puis choisissez le fichier `.dfv` à créer.

La sauvegarde déverrouille les fiches, les sérialise en mémoire puis les protège avec une clé dérivée de votre phrase par **Argon2id**, un sel aléatoire et AES-256-GCM. La clé du trousseau n’est pas nécessaire sur l’autre poste pour lire cette sauvegarde : sa propre clé locale protégera les fiches restaurées.

Conservez la phrase dans un autre emplacement que le fichier. Sans elle, la sauvegarde ne peut pas être déchiffrée. La restauration vérifie l’intégrité avant de modifier le coffre, importe les nouvelles fiches dans une transaction et **ne remplace pas les identifiants déjà présents**.

Limites explicites :

- La sauvegarde porte sur les fiches d’accès, pas sur les volumes, les images Docker, l’historique de commandes ni les dossiers partagés.
- Elle ne recrée pas les conteneurs à la restauration.
- Elle est limitée à 32 Mio et 10 000 fiches à la lecture.
- Le changement de chemin du répertoire applicatif change le compte du trousseau associé : effectuez une sauvegarde portable avant une migration de profil.
- Perdre à la fois le trousseau et la phrase de sauvegarde ne permet pas de déchiffrer la base. Une nouvelle clé n’est pas créée silencieusement par-dessus un coffre existant.

## Image Forge : votre image sans écrire le Dockerfile

### Parcours standard

1. Choisissez une base parmi Ubuntu 24.04, Debian 12 Slim, Alpine 3.20, Arch, Python 3.12, Node 22, PHP 8.3 Apache, Rust, .NET 8 SDK ou Java 21.
2. Cochez les outils nécessaires : curl, git, vim, nmap, iproute2, ffmpeg, supervisor. Les extensions `pdo_pgsql`, `pdo_mysql` et `redis` sont réservées à la base PHP.
3. Choisissez éventuellement votre dossier projet. Il sera incorporé sous `/app`.
4. Donnez un nom et un tag, par exemple `campus-api:v1`.
5. Indiquez la commande sous forme de tableau JSON, par exemple `["python", "main.py"]`, et éventuellement un port exposé.
6. Consultez le Dockerfile puis lancez la construction.

La commande JSON utilise la forme exec de Docker. `npm run start` doit donc être saisi `["npm", "run", "start"]`. Si votre application nécessite volontairement un shell, indiquez-le explicitement : `["sh", "-c", "votre commande"]`. Ce shell s’exécute **dans l’image**, pas dans le processus hôte DockForge.

L’installation de paquets utilise le gestionnaire adapté à la base et nettoie les caches APT/Pacman. L’image est construite par **BuildKit via l’API Bollard**, avec une session de build propre. Les images privées qui nécessitent une authentification de registre ne disposent pas encore d’un formulaire de login ; rendez leur base accessible au démon selon votre configuration avant le build.

### Builds multi-stage Rust et .NET

Pour Rust, choisissez la base Rust, un dossier contenant `Cargo.toml`/`Cargo.lock`, décochez les outils supplémentaires et renseignez le nom du binaire dans **Artefact multi-stage**. Le builder lance `cargo build --release --locked`, puis copie le binaire dans Debian Slim avec les bibliothèques de base prévues. Le runtime utilise l’UID non privilégié 65532.

Pour .NET, choisissez le SDK 8, un projet dont `dotnet publish` peut résoudre la cible, et le nom de la DLL principale, par exemple `Campus.Api.dll`. Le runtime utilise l’image ASP.NET 8 et l’UID non privilégié 1654.

Ces deux recettes ne remplacent pas l’analyse des bibliothèques natives de votre projet. Une application Rust liée à une bibliothèque C supplémentaire doit prévoir cette dépendance ; un dépôt .NET avec plusieurs projets peut exiger un contexte plus précis. Les arguments du champ commande deviennent le `CMD` associé à l’entrypoint généré.

### Ce qui entre dans le contexte

Le contexte est préparé dans une archive temporaire **sur disque**, sans charger l’ensemble du projet en mémoire. Il est limité à 512 Mio, 20 000 fichiers et 32 niveaux de parcours.

Les dossiers `.git`, `.ssh`, `.aws`, `.gnupg`, `node_modules`, `target`, les noms commençant par `.env`, les liens symboliques et les fichiers spéciaux sont exclus. Cette liste n’est pas un détecteur universel de secrets et ne remplace pas une revue de votre projet. Le programme ne prétend pas implémenter toute la sémantique de `.dockerignore` : préparez un dossier de contexte dédié si vous avez d’autres exclusions.

Les `RUN` du Dockerfile peuvent télécharger des paquets pendant la construction. Le programme n’invente pas un build hors ligne lorsqu’un paquet manque dans les caches.

Après succès, **Créer un conteneur avec cette image** ouvre le formulaire prérempli. Le port du conteneur reste à confirmer : `EXPOSE` dans l’image n’ouvre pas automatiquement un port sur l’hôte.

## Les outils intégrés

### Command Vault

Choisissez le shell puis importez son fichier d’historique : `.bash_history`, `.zsh_history`, `fish_history` ou l’historique PSReadLine PowerShell. L’import est local, limité à 32 Mio par fichier et déduplique les commandes exactes.

La case de suivi permet de ne suivre **que les fichiers que vous choisissez**. Pendant que DockForge reste ouvert, leur taille/date est contrôlée toutes les quinze secondes ; les changements déclenchent un nouvel import dédupliqué. À la prochaine ouverture, les historiques sélectionnés sont rattrapés. Il n’y a pas de daemon caché installé pour surveiller tous les terminaux lorsque l’application est fermée.

Les mots de passe, tokens et URI d’accès reconnaissables sont filtrés avant persistance. Le filtre ne reconnaît pas une phrase secrète dépourvue de contexte. Vérifiez une commande avant de la copier ou de la partager. La recherche renvoie au maximum 500 résultats ; les favoris permettent de garder vos commandes les plus utiles.

Le bouton de copie ne lance jamais la commande. Aucun endpoint d’exécution arbitraire du terminal hôte n’est exposé au frontend.

### NetMap Studio

Saisissez un sous-réseau IPv4 privé `/24`, confirmez votre autorisation puis lancez la découverte. Le scanner teste des ports TCP usuels avec un nombre borné de tâches et des délais courts. Il n’accepte ni cible publique ni masque `/16`.

Une réponse TCP est une observation, pas une identification fiable du constructeur ou de l’OS. Un poste qui filtre les ports peut rester invisible. Les liens dessinés représentent l’appartenance au sous-réseau logique. Sans données des équipements, il est impossible de reconstituer honnêtement chaque câble, switch silencieux ou route FAI.

Le JSON exporté conserve les hôtes observés, les ports et leur preuve. Aucune requête de géolocalisation WAN, aucun ajout de route et aucune élévation réseau automatique ne sont effectués.

### ClassShare LAN

1. Ajoutez les fichiers que vous souhaitez rendre accessibles. Pour un dossier, créez d’abord une archive puis ajoutez-la.
2. Choisissez une IP LAN de votre ordinateur et un port HTTPS disponible, par exemple 9443.
3. Ouvrez la salle. Le logiciel affiche l’adresse, son QR, le PIN à six chiffres et l’empreinte SHA-256 du certificat.
4. Transmettez l’adresse/QR et le PIN séparément aux participants.
5. Le participant ouvre l’adresse dans son navigateur, compare le certificat avec l’hôte, indique son nom et le PIN.

`127.0.0.1` ne permet qu’un essai sur le même ordinateur. Le programme ne modifie pas le pare-feu de votre poste ; autorisez le port selon votre politique. Deux appareils connectés au même Wi-Fi peuvent être isolés par le réseau.

Le certificat est auto-signé et propre à la salle. TLS chiffre le transport, mais la première confiance doit être établie auprès de l’hôte. **Ne désactivez pas globalement la validation HTTPS du navigateur ou de votre système.**

Le QR contient l’adresse, sans le PIN. Un PIN de six chiffres ne peut pas représenter simultanément toute adresse IP, un port et un authentificateur robuste : il admet le participant une fois l’adresse connue.

Chaque participant reçoit un cookie **HttpOnly, Secure, SameSite=Strict** spécifique à la salle. Le serveur ne conserve que l’empreinte du jeton. Le PIN est limité à six essais par minute par IP et la salle à cent participants.

Les invités sont en lecture par défaut. L’hôte peut accorder **Contribution**, revenir à Lecture ou Expulser. Les droits sont revérifiés à chaque requête. Une expulsion interdit les requêtes suivantes ; elle ne retire pas une copie déjà téléchargée et ne garantit pas l’interruption d’un flux déjà ouvert.

Les contributions sont limitées à 64 Mio par envoi, quatre envois simultanés et 1 Gio cumulé par salle. Les noms sont séparés des chemins réels, les fichiers sont créés exclusivement avec des noms aléatoires et un invité ne peut pas fournir un chemin système. Le quota est réservé atomiquement pour éviter les dépassements par concurrence.

Les téléchargements acceptent une plage HTTP `bytes=début-fin` unique. Le mode de contribution actuel envoie un fichier complet ; il ne promet pas la reprise par blocs d’un upload interrompu. Les salles ne sont pas restaurées après fermeture de l’application. La salle expire au bout de vingt-quatre heures et peut être fermée à tout moment par l’hôte.

## Diagnostic : que faire si cela échoue ?

| Symptôme | Explication et action |
| --- | --- |
| Docker non trouvé | Installez le client, Compose et Buildx adaptés à votre OS, puis rafraîchissez |
| Socket `Permission denied` | Vérifiez votre politique d’accès et les groupes actifs ; n’ouvrez pas le socket à tout le monde |
| Groupe ajouté mais erreur persistante | Reconnectez la session système complète |
| Port occupé | Acceptez le port voisin proposé ; Docker effectue encore une vérification finale |
| Nom déjà utilisé | Renommez le nouveau conteneur ; aucun ancien conteneur n’est supprimé automatiquement |
| Volume incompatible/non vide | Créez un nouveau volume ; ne supprimez pas les données existantes pour faire disparaître le message |
| `no matching manifest` / `exec format error` | Choisissez une image native ou configurez une émulation avant de sélectionner amd64 |
| Trousseau absent/verrouillé | Ouvrez le service Secret Service/Keychain/Credential Manager de votre session ; pas de fallback en clair |
| Mauvaise phrase de sauvegarde | Vérifiez la phrase et l’intégrité du `.dfv` ; aucun import partiel n’est effectué avant validation |
| Image construite mais processus quitte | Vérifiez sa commande ; `echo` ou un shell non interactif peuvent se terminer normalement |
| Service démarré sans sonde | Le processus tourne, mais la disponibilité n’a pas été certifiée |
| QR accessible seulement sur le PC hôte | Remplacez localhost par une IP LAN et vérifiez le pare-feu/isolation Wi-Fi |
| Contribution refusée | L’hôte doit accorder le rôle Contribution ; vérifiez aussi taille et quota |
| Paquet Tauri incompatible | Gardez les dépendances npm/Rust et leurs lockfiles synchronisés ; ne montez pas une seule moitié des plugins |

Le traducteur d’erreurs propose de modifier port, nom, volume ou plateforme. Une correction prépare le formulaire ; le lancement suivant reste explicite. Il ne supprime pas les données et ne remplace pas un ancien service à votre insu.

## Développer et tester

### Vérification rapide

```sh
npm ci
npm run build
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features
cargo check --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
```

Les tests sans frontend couvrent le coffre, la sauvegarde, la génération d’URI, les entrées invalides, l’exclusion de secrets, le contexte de build, le filtrage d’historique et un vrai serveur HTTPS local. Le test HTTPS ouvre un port loopback temporaire ; un bac à sable qui interdit toute socket doit lui accorder ce droit.

### Tests d’interface

```sh
npx playwright install chromium
npm test
```

Les tests vérifient le mode navigateur, le formulaire avec un adaptateur IPC exclusivement injecté par les tests, les filtres, le clavier et Axe sur les sept espaces. Cette fixture n’est pas un mode de démonstration activable dans le logiciel livré. L’audit automatisé ne remplace pas une vérification humaine avec lecteur d’écran.

### Tests Docker réels

Ces tests sont ignorés par défaut parce qu’ils nécessitent un démon Docker et un accès au registre. Ils ne doivent pas utiliser vos conteneurs métier : leurs noms et labels sont dédiés, puis leurs propres ressources sont nettoyées.

```sh
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features \
  postgres_provisioning -- --ignored --nocapture
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features \
  real_buildkit -- --ignored --nocapture
```

Le premier teste la création réelle d’un PostgreSQL, sa sonde et les accès conservés. Le second construit une image Alpine par BuildKit, vérifie sa présence et supprime l’image de test. Il ne suffit pas qu’un mock renvoie `success` pour valider ces parcours.

### Structure du dépôt

```text
src/                         React, types stricts, API IPC, formulaires et styles
src-tauri/src/catalog.json   35 modèles embarqués, champs, sondes et avertissements
src-tauri/src/doctor.rs      sondes OS/Docker et remédiations à verbes fermés
src-tauri/src/engine.rs      validation, provisioning, accès, logs et cycle de vie
src-tauri/src/images.rs      recettes, contexte tar filtré et build BuildKit
src-tauri/src/store.rs       SQLite, chiffrement local, archives et historique
src-tauri/src/backup.rs      sauvegarde portable Argon2id + AES-GCM
src-tauri/src/share.rs       serveur HTTPS, PIN, rôles, quotas et téléchargements
src-tauri/src/suite.rs       imports de commandes, suivi et cartographie locale
src-tauri/src/main.rs        adaptateur Tauri et commandes métier
scripts/                    outils de commentaire, lancement et distribution
tests/                      parcours Playwright et audit Axe
docs/                       architecture, sécurité, réception et sources client
```

### Commentaires pédagogiques

Les sources métier sont documentées en français : le commentaire explique l’action et son rôle dans le parcours, notamment les validations, les frontières de privilèges et les conditions d’échec. L’outil d’annotation complète les commentaires de fonctions par des explications de lignes. Les littéraux multilignes, le texte rendu en JSX, les données JSON et les fichiers générés ne reçoivent pas de commentaires insérés dans leur contenu : cela changerait leur sens ou invaliderait le format.

Le contrôle TypeScript compare les arbres du JavaScript émis avant et après annotation, afin de vérifier que les commentaires JSX ne modifient pas l’interface. Les tests Rust et la compilation revérifient le backend après annotation. Préférez toujours un commentaire métier précis à une répétition du nom de la fonction.

### Modifier un modèle

Éditez `src-tauri/src/catalog.json`, indiquez ses champs réellement supportés, son port, son volume et une sonde appropriée. Un secret doit être marqué `secret: true`. Si une variable n’est pas réellement interprétée par l’image, n’ajoutez pas un champ « admin password » qui donnerait une fausse impression de configuration.

Le frontend et le backend lisent le même catalogue embarqué. Ajoutez des tests de validation, documentez les dépendances multi-conteneurs et testez l’image réelle sur les architectures revendiquées. Une fiche de catalogue ne constitue pas une certification de toutes les variantes d’un éditeur.

### Contribuer

Une contribution doit décrire le problème concret, le comportement attendu et la manière de le vérifier. Les changements touchant secrets, privilèges, montages ou exposition réseau doivent expliquer leur frontière de confiance. Ne joignez pas de véritables identifiants ni d’exports clients dans les issues ou captures.

Les paquets tiers, images, modèles IA et services conteneurisés conservent leurs licences. La licence MIT du code DockForge ne vous accorde pas des droits supplémentaires sur SQL Server, Oracle ou les images distribuées par d’autres éditeurs.

## État de réception

La recette locale valide **11 tests Rust, 2 scénarios Docker réels et 4 tests Playwright**, ainsi que Clippy strict. Le paquet Debian produit fait **6,57 Mio** ; le démarrage natif a été vérifié dans un profil isolé.

Les résultats reproductibles et leurs limites sont consignés dans [RECETTE.md](docs/RECETTE.md). Les formats d’installation multi-OS sont préparés ; l’installation et la signature doivent être validées sur chaque cible réelle. Ni la présence d’un fichier CI ni celle d’un template de paquet ne prouve sa publication.

Les limites connues comprennent la topologie réseau physique non déduisible d’un scan TCP, les catalogues nécessitant des dépendances externes, les registres privés sans assistant d’authentification, la contribution LAN sans reprise d’upload par blocs et les salles non persistantes. Elles sont exposées pour que la réception repose sur le comportement réel, et non sur les promesses du README client.
