# Construire et distribuer une version

Le dépôt distant autorisé est `https://github.com/HACKER2025998/omniShellStudio`. Le code est publié avec ses lockfiles et ses recettes. Les archives, installateurs et données utilisateurs sont ignorés par Git.

## Version candidate

La version courante est `1.0.0-rc.1`. Les fonctionnalités sont soumises aux tests locaux et à la matrice GitHub Actions. Le suffixe RC indique que les contraintes non validables sur cette machine, en particulier la mémoire du bureau et les signatures de plateforme, ne sont pas transformées en promesses de production.

## Sources

Après avoir ajouté les nouveaux fichiers au suivi Git :

```sh
python scripts/package-source.py
```

L'archive de `release/` contient uniquement les fichiers suivis, sous la racine `omnishell-studio/`. Son fichier `.sha256` contient le hash réel. Le script génère aussi `release/PKGBUILD` avec le nom d’archive et le checksum réels : lancez `makepkg` depuis ce répertoire sur Arch. Le manifeste de préparation suivi dans `packaging/` reste un modèle, pas un paquet AUR publié. Le dépôt ne se présente pas comme un paquet déjà publié dans l'AUR.

## Installateurs

```sh
npm ci
npm run tauri -- build --bundles deb
```

Le script compile d'abord `omnishell-daemon` sans interface graphique et prépare le sidecar portant le triple de plateforme. Tauri l'embarque avec le bureau. La CI construit également NSIS sous Windows et DMG sous macOS ; les artefacts réels sont à récupérer dans l'exécution correspondante.

Pour une AppImage, utilisez la même commande avec `--bundles appimage` sur Linux. Pour une construction croisée, configurez explicitement le compilateur, le target Cargo et le triple ; le script de sidecar fourni vise les builds natifs de la matrice.

## Dépôt APT signé

Le script `scripts/build-apt-repository.sh` génère les index Packages/Release et leurs signatures avec une **clé GPG du mainteneur déjà disponible** :

```sh
OMNISHELL_GPG_KEY=IDENTIFIANT_DE_VOTRE_CLE bash scripts/build-apt-repository.sh 'chemin/OmniShell Studio_1.0.0-rc.1_amd64.deb'
```

Installez préalablement `dpkg-dev`, `apt-utils` et GnuPG. Le dossier `release/apt/` doit être hébergé sur une adresse que vous contrôlez ; communiquez sa clé publique par un canal vérifiable. Aucun domaine fictif n'est ajouté au README. Ce script prépare actuellement l'index amd64 ; une publication arm64 doit produire son index d'architecture correspondant.

## Signatures Windows et macOS

Les builds ne disposent pas par défaut d'une identité de signature de code. Le certificat Windows, les secrets de notarisation Apple et les droits sur un registre AUR/winget doivent appartenir au responsable du projet. Les installateurs non signés restent des artefacts de recette. Aucune signature fictive, contournement de SmartScreen ou désactivation de la vérification système n'est proposé.

Le push de code et la génération des artefacts ne créent pas automatiquement une release publique ou une publication dans un registre de paquets. Les résultats de CI doivent être consultés avant de promouvoir une RC en version stable.
