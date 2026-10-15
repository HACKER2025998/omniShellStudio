# Recette et résultats de validation

## Validation effectuée dans cet environnement

Date de référence utilisateur : **15 octobre 2026**. Plateforme de compilation : Linux, Node 24, Rust 1.99, WebKitGTK 4.1 disponible.

- Compilation TypeScript et assets de production Vite : réussie.
- Compilation du bureau Tauri Linux et génération d'un `.deb` : réussies lors du premier build ; paquet de 9,56 Mio.
- Dix tests unitaires Rust : réussis (formats d'historique, UTF-16LE, continuations, enveloppes, secrets, limites réseau, DDL et cardinalités).
- Un scénario d'intégration Rust : réussi avec vrai serveur HTTPS local, stockage SQLite et droits de participants.
- Quatre parcours Playwright dans Chrome : réussis, y compris navigation des cinq modules, favoris/recherche, absence du moteur et capture de référence.

Le test LAN requiert l'énumération d'une interface IPv4 non-loopback et l'autorisation d'écouter localement. Un environnement sandbox bloquant ces appels doit autoriser ce test ou le lancer dans un runner adapté. Le résultat du test n'implique pas une connexion à un serveur Internet.

Les tests UI emploient des fixtures isolées : la capture `screenshots/coffre.png` montre une commande de test et ne représente pas l'historique réel du propriétaire de la machine.

- Analyse statique Clippy du moteur et du bureau avec avertissements interdits : réussie.

## Reproduire les vérifications

```sh
npm ci
npm run build
cargo test --locked --manifest-path src-tauri/Cargo.toml
cargo clippy --locked --manifest-path src-tauri/Cargo.toml -- -D warnings
npm test
npm run tauri -- build --bundles deb
```

`npm test` attend Google Chrome installé. `CHROME_BIN` peut désigner son exécutable. La CI inclut une étape d'installation du navigateur. Le serveur Vite de test est temporaire et le pont IPC est injecté uniquement dans les pages du test.

## Recette manuelle du coffre

Utilisez un répertoire de données portable et des commandes de laboratoire. Vérifiez : import initial ; second import inchangé ; masque d'un faux token ; filtre de catégorie ; recherche courte et longue ; favori conservé après relance ; suppression retirée aussi de FTS ; pagination de plus de cent lignes ; copie ; remplacement de template ; fermeture de l'inspecteur avec Échap.

Testez chaque hook dans un shell réel distinct. Vérifiez le prompt, les commandes avec guillemets, les pipelines, le code de sortie, le dossier et la convention de confidentialité par espace initial quand le shell la prend en charge. Mesurez l'impact du hook sur la machine cible avant d'annoncer un budget temporel.

## Recette Docker

Avec un moteur de laboratoire, créez un conteneur possédant un port, un volume et une fausse variable sensible. Synchronisez ; contrôlez les informations masquées. Arrêtez le moteur puis relancez le bureau : la capture doit rester lisible et datée. Contrôlez chaque commande copiée avant de l'utiliser. Vérifiez qu'aucun bouton ne lance une opération Docker automatiquement.

## Recette du réseau

Sur un /24 de laboratoire autorisé, exposez un service connu puis sondez. Comparez le port observé. Arrêtez le service puis recommencez : son absence des nouvelles observations ne doit pas être décrite comme une preuve que l'équipement est éteint. Vérifiez que les préfixes publics et les sous-réseaux plus larges sont refusés. Ouvrez les exports JSON et l'impression PDF. Un test LLDP/SNMP n'est pas pertinent tant que ces fonctions ne sont pas implémentées.

## Recette de salle LAN

1. Utilisez deux appareils sur un LAN de test ; partagez un PDF et une archive.
2. Vérifiez que le QR contient seulement le lien et que le PIN reste séparé.
3. Établissez la confiance du certificat hôte avant de saisir le code.
4. Vérifiez qu'un visiteur anonyme ne voit pas la liste et qu'un PIN erroné est rejeté.
5. Rejoignez la salle. Vérifiez la lecture seule initiale et un téléchargement.
6. Accordez l'upload à un pair, envoyez plusieurs blocs, puis coupez son Wi-Fi.
7. Rétablissez le Wi-Fi et choisissez le même fichier : la reprise doit partir de la taille confirmée.
8. Retirez le droit pendant un transfert : le prochain bloc doit être refusé.
9. Expulsez ce jeton puis vérifiez le refus des nouvelles requêtes.
10. Fermez la salle ; l'URL ne doit plus servir de téléchargement.
11. Examinez le dossier de réception et sa politique de nettoyage.

Le test automatisé vérifie notamment `401`, `403`, `206` et `409`. Une recette complémentaire doit vérifier le remplissage disque, les corps trop gros et le comportement multi-pairs en charge. La session ne doit pas être annoncée comme restaurable après arrêt de l'hôte.

## Recette DBA

Testez un DDL simple puis : types avec virgules, PK composite, FK composite, FK nullable, FK obligatoire, contrainte UNIQUE sur FK et tables sans relations. Vérifiez les cardinalités proposées et les règles métier manuellement. Enregistrez, relancez, rechargez et exportez. Pour une base SQLite, vérifiez son empreinte avant/après introspection afin de confirmer l'absence de modification métier.

## Mesures à produire avant certification

Pour chaque OS, relevez le commit, le compilateur, le mode release, le matériel, la RAM au repos/après import, la taille du paquet, la taille de base et le temps de recherche médian/p95/p99 sur 100 000 commandes synthétiques. Séparez le coût du démarrage du processus, l'ouverture SQLite, la recherche et le rendu UI. Ne comparez pas une requête chaude en mémoire avec un lancement froid complet.

Les chiffres client restent des cibles tant que ce protocole n'est pas exécuté. La signature et l'installation Windows/macOS nécessitent leurs environnements et identités de signature ; elles ne sont pas validées par la création du `.deb` Linux.
