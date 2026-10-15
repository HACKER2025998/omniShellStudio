# Recette et résultats de validation

## Validation effectuée dans cet environnement

Date de référence utilisateur : **15 octobre 2026**. Plateforme de compilation : Linux, Node 24, Rust 1.99, WebKitGTK 4.1 disponible.

- Compilation TypeScript et assets de production Vite : réussie.
- Compilation du bureau Tauri Linux et génération d'un `.deb` : réussies lors du premier build ; paquet de 12,97 Mio, service autonome inclus.
- Seize tests unitaires Rust : réussis (formats d'historique, UTF-16LE, continuations, enveloppes, secrets, limites réseau, DDL et cardinalités).
- Deux scénarios d’intégration Rust : serveur HTTPS et stockage/permissions/restauration, ainsi que redémarrage d’un vrai daemon sur son transport natif.
- Deux recettes SQL supplémentaires exécutées : PostgreSQL 17 et MariaDB 11 sur conteneurs éphémères nettoyés.
- Cinq parcours Playwright dans Chrome : réussis, y compris navigation des cinq modules, favoris/recherche, absence du moteur et capture de référence.

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

Sur un /24 de laboratoire autorisé, exposez un service connu puis sondez. Comparez le port observé. Arrêtez le service puis recommencez : son absence des nouvelles observations ne doit pas être décrite comme une preuve que l'équipement est éteint. Vérifiez que les préfixes publics et les sous-réseaux plus larges sont refusés. Ouvrez les exports JSON et l'impression PDF. Recettez LLDP/CDP sur un équipement émettant effectivement ces annonces et SNMPv2c sur un équipement de laboratoire configuré ; les parseurs de trames sont testés sans exiger ce matériel.

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

Le test automatisé vérifie notamment `401`, `403`, `206` et `409`. Le scénario vérifie aussi un hash invalide, un dépassement de taille, la publication d’un fichier vide et la restauration. Une recette complémentaire doit vérifier le remplissage disque et le comportement multi-pairs en charge. La restauration doit réutiliser le port précédent et conserver les rôles et expulsions ; elle est testée automatiquement.

## Recette DBA

Testez un DDL simple puis : types avec virgules, PK composite, FK composite, FK nullable, FK obligatoire, contrainte UNIQUE sur FK et tables sans relations. Vérifiez les cardinalités proposées et les règles métier manuellement. Enregistrez, relancez, rechargez et exportez. Pour une base SQLite, vérifiez son empreinte avant/après introspection afin de confirmer l'absence de modification métier.

## Mesures à produire avant certification

Pour chaque OS, relevez le commit, le compilateur, le mode release, le matériel, la RAM au repos/après import, la taille du paquet, la taille de base et le temps de recherche médian/p95/p99 sur 100 000 commandes synthétiques. Séparez le coût du démarrage du processus, l'ouverture SQLite, la recherche et le rendu UI. Ne comparez pas une requête chaude en mémoire avec un lancement froid complet.

Les chiffres client restent des cibles tant que ce protocole n'est pas exécuté. La signature et l'installation Windows/macOS nécessitent leurs environnements et identités de signature ; elles ne sont pas validées par la création du `.deb` Linux.

## Mesures enregistrées

Le rapport `benchmarks/search-linux.json` utilise cent mille commandes synthétiques et deux cents recherches complètes : médiane 1,37 ms, p95 1,61 ms, p99 1,73 ms. Le rapport `benchmarks/resources-linux.json` inclut les descendants du processus : daemon dédié 7 996 Kio RSS / 5 378 Kio PSS ; bureau 691 396 Kio RSS cumulés / 314 450 Kio PSS. Les pages partagées sont comptées plusieurs fois dans la somme RSS ; PSS ne doit pas être présenté comme RSS. Le bureau atteint la cible révisée de 400 Mio PSS dans ce profil : 314 450 Kio, soit 307,1 Mio. Le budget initial de 55 Mo n’est pas atteint et a été remplacé par décision utilisateur du 15 octobre 2026 pour conserver Rust/Tauri/React.

Le script de mesure utilise un HOME/XDG privé, cent commandes synthétiques et un Docker indisponible ; il ne lit ni l’historique réel ni les conteneurs du compte. Les captures d’interface utilisent des fixtures, pas des données utilisateur.

## Critère mémoire révisé

L’utilisateur a choisi de conserver Rust/Tauri/React et d’accepter une cible réaliste mesurée. Le plafond de réception retenu pour le bureau est **400 Mio PSS (409 600 Kio)** sur Linux, soit environ 30 % de marge par rapport au pic enregistré. Il porte sur la somme PSS du processus principal et de tous ses descendants, relevée toutes les 250 ms pendant les huit premières secondes par `python scripts/measure-resources.py`, en build release, avec cent commandes synthétiques et Docker indisponible. Le daemon conserve son budget initial, vérifié séparément.

Pour une nouvelle livraison, reconstruisez les deux binaires release, exécutez ce script dans une session graphique Linux et conservez son JSON avec le commit, les versions système et le matériel. Le bureau satisfait ce critère si `desktop.pss_kib` est positif et inférieur ou égal à 409 600, sans `early_exit`, et si les processus WebKit sont inclus. Une valeur absente ou un démarrage interrompu invalide la mesure. La somme RSS est informative : elle compte plusieurs fois certaines pages partagées et ne se compare pas au plafond PSS.

Ce profil mesure le démarrage ; il ne certifie ni une journée d’utilisation, ni une salle complète, ni un import massif. Les mesures sous charge et sur Windows/macOS restent à produire avec un protocole et une métrique adaptés. Les documents client originaux conservent leur seuil historique de 55 Mo pour assurer la traçabilité.
