# Traçabilité de la version 1.0.0-rc.1

Les deux documents intégraux sont conservés dans `specifications/`. La réalisation distingue le code disponible, sa recette locale et les conditions de déploiement. Une fonction implémentée ne signifie pas qu’un équipement absent ou un système non testé a fourni une preuve réelle.

| ID | Exigence | Réalisation | Validation et limites |
| --- | --- | --- | --- |
| FR-01 | Import Bash/Zsh/Fish/PSReadLine | Quatre formats, dates, continuations, UTF-8/UTF-16LE et relecture idempotente | Tests unitaires ; les commandes effacées du shell ne sont pas récupérables |
| FR-02 | Capture asynchrone multi-shell | Quatre hooks, socket Unix, Named Pipe Windows authentifié, fallback SQLite et retries idempotents | Scénario de stockage testé ; latence <1,5 ms non certifiée sur les quatre OS/shells |
| FR-03 | Analyse AST et sudo | Tree-sitter Bash, Fish et PowerShell ; normalisation d’enveloppes et explications d’options | Tests des trois grammaires et du masquage ; extensions Zsh hors sous-ensemble Bash signalées |
| FR-04 | Taxonomie, recherche, copie | Douze familles, favoris, date, élévation, pagination et templates ; FTS5 | Tests UI/moteur ; 100 000 lignes : médiane 1,37 ms, p99 1,73 ms, rendu exclu |
| FR-05 | Cache Docker et origine | Événements Docker avec reconnexion, ports, volumes, environnement/logs masqués, Compose, actions et sauvegardes | Cache persistant ; commande capturée corrélée au nom/image, distincte de la reconstruction ; Docker ne fournit pas toujours l’origine exacte |
| FR-06 | Cartographie multi-couches | TCP et bannières, routes et voisins système, mDNS, ARP/LLDP/CDP facultatifs, SNMPv2c, WAN sur demande ; canevas et exports | Parseurs testés ; raw Unix soumis aux permissions, repli sur Windows sans pilote dédié ; preuves et liens logiques distingués, pas d’OS ou de câblage inventé |
| FR-07 | Partage LAN, PIN/QR et dossiers | HTTPS/ALPN, WebSocket authentifié, découverte mDNS, lien de repli, dialogues natifs, archives tar.zst bornées | Vrai serveur TLS testé ; certificat auto-signé à valider ; le PIN ne remplace pas une adresse IP arbitraire |
| FR-08 | Droits maître et suivi | Lecture seule, contribution, retrait et expulsion, octets servis/reçus, droits relus côté serveur et durant les flux | Scénario HTTP réel ; un pair connaissant encore le PIN peut demander une autre identité déclarative |
| FR-09 | Reconnexion et reprise | Jeton navigateur, Range, blocs BLAKE3 de 4 Mio, offsets durables, salle/permissions/empreintes de jetons persistées | Reprise sur même port après arrêt de l’hôte testée ; expiration 24 h ; effacer le stockage client retire ses identifiants |
| FR-10 | DBA, MCD/MLD/UML | DDL avec FK composites et contraintes ALTER, introspection SQLite/PG/MySQL, canevas déplaçable, cardinalités, sauvegardes et exports | PostgreSQL 17 et MariaDB 11 réellement recettés sur conteneurs éphémères ; règles métier à valider, pas de cardinalité inventée |

## Exigences transversales et mesures

| Objectif | État vérifiable |
| --- | --- |
| Daylight clair | Réalisé, navigation et inspecteur contextuel |
| Accessibilité | Audit Axe des cinq vues avec règles WCAG A/AA/contrastes AAA : aucun défaut automatisé relevé ; cela ne remplace pas un audit humain complet |
| Local-first / aucune télémétrie | Fonctionnement local autonome ; WAN et bases SQL accessibles uniquement par actions explicites |
| Secrets avant persistance | Filtre commun des commandes/snapshots et AST ; jetons LAN persistés sous forme d’empreintes ; reconnaissance non universelle |
| TLS et permissions | Réalisés et testés avec vrai serveur local ; confiance initiale du certificat à déployer |
| Traversée de chemins / disque | Allowlist, noms séparés des chemins, limites par bloc/fichier/salle et réserve disque ; quotas configurables et fichiers vides couverts |
| Recherche <5 ms /100 000 lignes | Atteinte dans le protocole local documenté, p99 1,73 ms |
| Paquet <14 Mo | Paquet Debian local : 12,97 Mio (environ 13,60 Mo), service autonome inclus |
| Daemon <12 Mo | Atteint dans le profil local : 7,8 Mio RSS / 5,3 Mio PSS, binaire distinct sans WebView ; rapport dans `benchmarks/resources-linux.json` |
| Bureau <55 Mo | **Non atteint sur la machine de recette** avec Tauri/WebKit et ses processus enfants ; aucun chiffre parent seul n’est utilisé pour masquer cet écart |
| Trois plateformes | Matrice de tests et installateurs GitHub Actions ; le succès réel des jobs fait foi, pas la simple présence du YAML |
| Publication APT/AUR/winget et signature | Outils de préparation et paquets ; registres, hébergement et identités de signature non créés fictivement |
| Commentaires | Explications métier françaises et annotation de lignes des sources TS/TSX/Rust ; comparateur de sortie compilée sans changement de comportement |
| Documentation approfondie | README, architecture, sécurité, recette, distribution, mesures et sources client |

## Corrections d’exigences techniquement ambiguës

- Un PIN à six chiffres ne peut encoder toute IPv4, tout port et un authentificateur sûr. La découverte et le lien résolvent l’adresse ; le PIN admet un participant.
- La configuration Docker ne prouve pas une ligne d’origine exacte : les commandes capturées et les reconstructions sont distinguées.
- Les liens du sous-réseau sont logiques tant qu’une preuve ne décrit pas le lien physique. Un switch silencieux n’est pas inventé.
- `time`, `watch`, `env` et `nohup` ne sont pas une élévation de privilèges.
- Une révocation est relue sur le serveur ; un ancien rôle déclaré par le client ne suffit jamais.
- Les profils, capacités raw, pare-feu et magasins de confiance ne sont pas modifiés silencieusement par l’application.

## Conditions pour annoncer une conformité stricte à 100 %

Le code des fonctions ci-dessus est présent. La cible mémoire du bureau reste un écart mesuré à arbitrer avec la pile imposée. Les latences de hooks, la charge d’une classe complète, les équipements LLDP/CDP/SNMP réels, les versions des trois OS et les signatures de distribution doivent être réceptionnés dans leur environnement cible. Ces validations ne sont pas déclarées réussies sans preuve.
