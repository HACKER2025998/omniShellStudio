# Sécurité et confidentialité

## Modèle de menace de cette livraison

Le poste et le compte de l'hôte sont supposés administrés par leur propriétaire. L'application réduit les risques d'un pair LAN non autorisé, d'une saisie de chemin malveillante, d'un rôle périmé et d'une fuite de secrets courants dans l'historique. Elle n'est pas une sandbox pour du code malveillant exécuté par le propriétaire du compte, ni un outil d'identification forte des étudiants.

## Données sensibles

Le moteur masque les affectations et paramètres courants de mots de passe, secrets, tokens, clés, Bearer, identifiants d'URL et mots de passe MySQL compacts. Les règles ne peuvent reconnaître un mot de passe arbitraire passé à une fonction personnelle sans nom explicite. N'utilisez pas ce filtre comme justification pour saisir des secrets en clair dans un terminal.

Les historiques source ne sont pas modifiés. Même si OmniShell masque une commande, le shell peut avoir conservé l'original. Les variables d'environnement, noms de fichiers, chemins, logs et modèles SQL peuvent contenir des informations privées non reconnues par un filtre automatique.

La base est locale mais non chiffrée. Sous Unix, le répertoire de données dédié est 0700 et le socket 0600. Les ACL Windows et le chiffrement au repos relèvent du déploiement. Sauvegardes, captures d'écran et exports doivent recevoir la même protection que l'historique.

## Surface IPC

Le bureau autorise une liste fermée d'actions. Aucun endpoint ne reçoit une chaîne à exécuter dans `sh -c` ou PowerShell. Les appels Docker utilisent des tableaux d'arguments avec `Command`, pas un shell. Les snippets affichés sont copiables uniquement.

Le mode Vite n'ajoute pas d'API système ouverte pour faciliter le développement. Le serveur LAN ne connaît pas le coffre de commandes et n'expose aucune route de contrôle maître.

## Salle LAN

- TLS avec certificat local auto-signé ; la confiance initiale doit être établie hors du canal inconnu.
- Jetons aléatoires longs distincts des identifiants publics et du PIN.
- Lecture seule par défaut, permissions relues sur chaque requête.
- Jetons absents de la liste des pairs et des URL de téléchargement.
- Cookie de téléchargement `HttpOnly`, `Secure`, `SameSite=Strict`.
- WebSocket authentifié par son premier message, avec délai de dix secondes.
- PIN limité à trente tentatives par minute pour toute la salle.
- Fichiers source choisis explicitement, téléchargements forcés en pièces jointes, `nosniff`.
- Uploads limités à 4 Mio par bloc, 2 Gio par transfert et 10 Gio par salle.
- Réserve disque de 64 Mio ; décalage durable et BLAKE3 vérifiés avant acquittement.

Le nom d'un participant est déclaratif. Une expulsion invalide son jeton mais n'empêche pas une réinscription avec un autre nom s'il connaît le PIN. Le service n’effectue pas une authentification institutionnelle, une analyse antivirus ou une interdiction matérielle. Les contributions terminées sont publiées dans la salle après contrôle de nom, taille et blocs ; elles ne sont jamais exécutées.

La révocation interdit la requête ou le bloc suivant. Chaque segment émis du téléchargement relit aussi les permissions ; le flux peut donc être interrompu après une expulsion. Le code ne prétend pas récupérer des octets déjà reçus par le participant. Fermer la salle arrête le serveur et ses connexions.

Le contrôle d'espace libre est une photographie : un autre processus peut consommer le disque après la vérification. L'échec d'écriture reste donc géré. Les quotas se configurent avant ouverture ; la politique de rétention des contributions reste une responsabilité de déploiement.

## Réseau actif

Les sondes sont déclenchées par l'utilisateur et limitées à un réseau privé /24. Elles n'ont pas besoin de privilèges raw et ne lancent aucun service externe de reconnaissance. Une adresse privée n'est pas à elle seule une autorisation : l'administrateur définit le périmètre de son laboratoire.

Aucune capacité `cap_net_admin`, appartenance au groupe Docker, règle de pare-feu ou installation de certificat racine n'est accordée automatiquement. L'accès au moteur Docker doit rester conforme à la politique du poste.

## Signalement d'un problème

Préparez un scénario minimal, la version, le système, les journaux expurgés et l'impact observé. Ne placez pas de jeton, certificat privé ou historique brut dans un ticket public. Le dépôt ne déclare pas d'adresse de contact de sécurité non vérifiée.

Avant une diffusion institutionnelle : revue des permissions, test de charge multi-pairs, revue TLS/confiance, gestion des expirations de session, tests sur les plateformes cibles, audit de dépendances et politique de conservation doivent être réalisés par le responsable du déploiement.

## Accès Internet facultatif

Le fonctionnement local n’émet aucun appel WAN. Le bouton de collecte FAI/IP publique utilise HTTPS vers ipapi.co, dont le service observe l’adresse de la requête. Cette action est expliquée dans le panneau et distincte du scan LAN. Les connexions SQL sont également explicites : les URLs et identifiants ne sont ni enregistrés avec les modèles ni renvoyés dans les erreurs brutes de pilotes.
