# Traçabilité des exigences client

Les textes intégraux des deux documents sont dans `specifications/`. Cette table distingue une exigence fonctionnelle satisfaite dans le périmètre annoncé, une réalisation partielle et une absence. Une compilation locale réussie ne valide pas toutes les plateformes.

| ID | Exigence | État de la livraison 0.1.0 | Preuve / limites |
| --- | --- | --- | --- |
| FR-01 | Import Bash/Zsh/Fish/PSReadLine | Réalisé pour les formats documentés | `history.rs`, tests formats, multiligne, UTF-16LE ; fichiers réels Windows à recetter sur Windows |
| FR-02 | Capture asynchrone multi-shell <1,5 ms | Partiel | Quatre hooks fournis, écriture directe ou daemon Unix ; latence non certifiée, pas de daemon Named Pipe Windows |
| FR-03 | Analyse AST et sudo | Partiel | Analyse lexicale et guillemets, enveloppes courantes et badge sudo testés ; pas d'AST complet multi-shell |
| FR-04 | Classement, recherche et copie | Réalisé dans la taxonomie disponible | Huit familles client + autres ; FTS5, favoris, pagination, copie et templates d'IP |
| FR-05 | Cache Docker, ports, volumes, origine exacte | Partiel | Snapshots persistants, polling optionnel, actions, logs ; origine reconstruite partiellement et étiquetée, pas d'écoute d'événements native |
| FR-06 | Carte FAI → routeur → switch → terminaux | Partiel | Sondes TCP /24 et cache réels ; pas de WAN, ARP actif, LLDP/CDP, SNMP, mDNS d'hôtes, OUI ou graphes physiques |
| FR-07 | Serveur LAN HTTP/2 + WS, PIN et QR | Réalisé pour fichiers | HTTPS rustls avec ALPN, WebSocket authentifié, PIN et QR, annonce mDNS ; dossiers à archiver manuellement, pas de parcours natif « rejoindre par PIN seul » |
| FR-08 | Rôles et révocation par le maître | Réalisé dans la salle active | Serveur autoritaire, lecture seule initiale, contribution, retrait et expulsion par jeton ; tests HTTP réels ; pas d'anti-réinscription par empreinte matérielle |
| FR-09 | Reconnexion et reprise | Réalisé dans une salle toujours ouverte | Jeton navigateur, Range, blocs 4 Mio BLAKE3 et offset durable ; pas de reprise après arrêt de l'hôte ni persistance des rôles en SQLite |
| FR-10 | SQL, MCD/MLD/UML et coffre DBA | Partiel | Parseur SQL, tables/FK/cardinalités, exports, sauvegarde modèles, introspection SQLite API ; pas de connexions directes PG/MySQL ni édition graphique métier complète |

## Exigences transversales

| Exigence | État |
| --- | --- |
| Interface claire Daylight | Réalisée : palette demandée, quatre zones, navigation, inspecteur, statuts et erreurs |
| Accessibilité WCAG AAA | Non certifiée : focus visible, labels et réduction du mouvement présents ; certains textes secondaires nécessitent une revue de contraste complète |
| Zéro cloud et télémétrie | Réalisé pour le fonctionnement du produit ; npm/Cargo nécessitent Internet à l'installation initiale des dépendances |
| Secrets masqués avant persistance | Réalisé pour les familles reconnues et testé ; pas de garantie de détection universelle |
| TLS et jetons LAN | Réalisé avec certificat auto-signé à valider localement et jetons 256 bits |
| Protection des chemins | Allowlist de fichiers côté téléchargement, identifiants stricts côté réception ; pas de chemin arbitraire fourni par un pair |
| Contrôle de capacité disque | Limites par bloc, fichier, salle et réserve disque ; erreur d'écriture propagée |
| RAM <55 Mo / daemon <12 Mo | À mesurer ; aucune garantie inscrite sans benchmark |
| Recherche <5 ms /100 000 lignes | FTS5 trigram présent ; budget à mesurer sur un corpus documenté |
| Installateur <14 Mo | Premier `.deb` local : 9,56 Mio ; résultat à recontrôler après chaque build et pour chaque plateforme |
| Linux / Windows / macOS | Code conditionnel et CI de compilation fournis ; seul le build Linux a été exécuté dans cet environnement |
| APT / AUR / winget publics | Non publiés ; PKGBUILD local et paquets Tauri disponibles, aucun registre fictif annoncé |
| Commentaires pédagogiques en français | Présents sur les décisions et opérations ; pas une annotation littérale de chaque ligne structurelle ou bundle généré |
| README très détaillé | Fourni avec installation, parcours, limites, administration, architecture et recette complémentaires |

## Corrections apportées aux propositions techniques

1. **PIN :** six chiffres ne peuvent contenir une IPv4 arbitraire, un port et un authentificateur suffisamment robuste. Le lien explicite résout l'adresse ; le PIN est un secret d'admission limité en essais.
2. **Docker :** une inspection n'est pas une preuve de la commande d'origine exacte. La reconstruction est annoncée partielle.
3. **Réseau :** le scan TCP observe des services, pas une topologie physique complète. Les preuves sont affichées et aucun switch n'est inventé.
4. **Privilèges :** `env`, `time`, `watch` et `nohup` ne signifient pas `sudo`. Le badge reflète les enveloppes d'élévation reconnues.
5. **Permissions :** un rôle inclus dans un ancien JWT ne suffit pas à une révocation immédiate. Les droits sont relus dans l'état serveur à chaque requête.
6. **Installation :** pas de `setcap cap_net_admin` automatique sur tout le binaire desktop. Les scans actuels fonctionnent sans raw socket.
7. **Profils shell :** pas de réécriture silencieuse de `.bashrc`, `.zshrc`, `config.fish` ou `$PROFILE`. Le chargement est explicite et réversible.
8. **Stockage :** pas de mmap arbitraire de 30 Go ; les paramètres doivent suivre les mesures de charge.
9. **Publication :** pas d'URL APT ou badge CI annoncé comme actif sans publication ou exécution réelle.

## Reste nécessaire pour une réception stricte du CDC 1.0

Implémenter et recetter les éléments indiqués partiels ci-dessus ; mesurer les budgets sur Linux, Windows et macOS ; auditer sécurité et accessibilité ; signer et publier les installateurs ; valider les parcours de classe sur des réseaux isolés et avec plusieurs dizaines de pairs. Cette liste est un écart connu, pas un ensemble de fonctionnalités prétendues présentes.
