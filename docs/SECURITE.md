# Sécurité et modèle de confiance

## Ce que protège le coffre

Le chiffrement des fiches protège leur lecture directe depuis les fichiers SQLite et les backups non déverrouillés. La clé locale provient du trousseau natif. Les sauvegardes `.dfv` dérivent une clé distincte d’une phrase utilisateur par Argon2id ; AES-GCM protège également leur intégrité.

La clé en mémoire ne remplace pas l’isolation du compte système. Un processus disposant du contrôle du compte, un débogueur autorisé, une extension système malveillante ou un administrateur peut accéder aux secrets affichés et aux ressources Docker. Docker conserve généralement les variables d’environnement du conteneur dans ses propres métadonnées : **le coffre ne chiffre pas le démon Docker**.

Les métadonnées de conteneurs, recettes et commandes filtrées ne sont pas intégralement chiffrées dans SQLite. Le répertoire applicatif est restreint à 0700 sous Unix ; les exports sont créés à 0600. Sous Windows, la protection s’appuie aussi sur les ACL du profil. Le chiffrement du disque reste utile.

## Chemins et exports

Les exports utilisent `create_new`, refusent l’écrasement et n’acceptent pas qu’un lien symbolique préexistant remplace la destination finale. Les parents du chemin sont ceux choisis par l’utilisateur : ne choisissez pas un répertoire contrôlé par un tiers.

Les chemins de montage sont validés côté Rust, doivent être absolus et désigner des dossiers existants. Une fois exposés à un conteneur, leurs données peuvent être modifiées selon les droits de l’image. Aucun changement de permissions récursif n’est appliqué automatiquement.

Les contextes de build n’incluent pas les liens symboliques et excluent plusieurs emplacements sensibles. Les fichiers sont ouverts sans suivre un lien final sur Unix. Une arborescence modifiée simultanément par un processus du même compte n’est pas une frontière d’isolation hostile ; préparez un contexte stable et revu avant un build contenant des données sensibles.

## Exposition réseau

Localhost est le défaut de création. LAN et toutes interfaces nécessitent une confirmation explicite. Les capacités supplémentaires et l’acceptation de licence SQL Server ont également leur propre consentement produit. Le test de port ne réserve pas la socket ; Docker arbitre le binding définitif.

Le groupe Docker est assimilable à un droit administrateur. Polkit/UAC sont limités à des actions fixes et ne servent pas de canal d’exécution de texte transmis par le frontend. La fenêtre principale reste sous le compte utilisateur.

NetMap refuse les IP publiques et les masques plus larges que /24. Il s’agit d’un scan TCP actif ; n’utilisez ce module que sur un réseau autorisé. Le logiciel n’ajoute pas des capacités raw à son binaire et ne change pas votre pare-feu.

## Partage LAN

Le serveur n’est ouvert que sur demande, sur une adresse locale choisie. Le certificat est auto-signé : comparez son empreinte directement avec l’hôte. TLS sans vérification de cette première confiance ne protège pas contre une interception active lors de la découverte initiale.

Le PIN à six chiffres ne remplace pas un secret fort ; son usage est limité à l’admission, avec six essais par minute par IP. Les sessions utilisent des jetons aléatoires plus longs, des cookies HttpOnly/Secure/SameSite=Strict et une révocation côté serveur. Le PIN, les participants et la salle restent en mémoire et expirent au bout de vingt-quatre heures.

Les contributions sont authentifiées avant lecture, limitées à quatre requêtes simultanées, 64 Mio par corps et 1 Gio cumulé. Leur quota est réservé atomiquement. Le nom de stockage est généré côté serveur ; un nom d’affichage `../...` est rejeté. Une contribution reçue peut être dangereuse en tant que document : le programme la stocke, sans exécuter son contenu et sans promettre un antivirus.

Une révocation ne peut pas supprimer des octets déjà téléchargés. Un transfert déjà ouvert peut se terminer avant la requête suivante. La salle n’est pas conçue pour être exposée sur Internet ni derrière une redirection de port WAN.

## Logs et historiques

Les secrets reconnaissables sont masqués avant stockage d’une commande ou affichage de logs. Les valeurs connues du formulaire sont aussi retirées des erreurs de création. Un texte arbitraire ne peut pas être classifié infailliblement comme secret. Évitez d’imprimer vos secrets dans les applications conteneurisées et relisez tout extrait avant partage.

Les seuls historiques suivis sont ceux choisis explicitement. Le programme ne lit pas les historiques d’autres comptes, n’installe pas un hook dans les shells à votre insu et ne lance pas les commandes copiées.

## Réponse à un incident

1. Fermez la salle LAN et retirez l’exposition réseau des services concernés.
2. Si un export en clair a été partagé, faites tourner les secrets **dans le service** ; effacer le fichier ne révoque pas un mot de passe.
3. Arrêtez l’usage d’un trousseau compromis et restaurez les accès via une sauvegarde dont la confidentialité est établie.
4. Examinez les conteneurs, volumes et images étiquetés DockForge. Ne supprimez pas les données avant de disposer d’une sauvegarde cohérente.
5. Préparez un rapport reproductible sans vrais mots de passe, tokens, chemins privés ou export client.

Le dépôt ne prétend pas fournir une certification de sécurité ou une procédure de disclosure privée avant qu’une adresse de contact du mainteneur ait été définie.
