#!/usr/bin/env bash
# Prépare un dépôt APT signé à héberger sur un serveur choisi par le mainteneur.
set -euo pipefail
# L'identité de signature doit appartenir au mainteneur ; aucune clé de confiance n'est inventée.
: "${OMNISHELL_GPG_KEY:?Indiquez un identifiant de clé GPG de signature}"
# Le répertoire ne contient que les paquets explicitement transmis au script.
repo_dir="${OMNISHELL_APT_DIR:-release/apt}"
# Le script refuse de publier un dépôt vide.
(( $# > 0 )) || { echo 'Usage : build-apt-repository.sh paquet.deb ...' >&2; exit 1; }
# Les commandes sont vérifiées avant de modifier le contenu du dépôt.
for dependency in dpkg-scanpackages apt-ftparchive gpg gzip; do command -v "$dependency" >/dev/null; done
# Le dépôt est construit localement ; sa mise en ligne n'est pas faite automatiquement.
mkdir -p "$repo_dir/pool/main" "$repo_dir/dists/stable/main/binary-amd64"
# Les chemins sont toujours cités pour accepter les noms de paquets contenant des espaces.
cp -- "$@" "$repo_dir/pool/main/"
# Les index sont relatifs à la racine qui sera exposée par HTTP ou HTTPS.
cd "$repo_dir"
# Les sommes des paquets sont calculées par les outils Debian officiels.
dpkg-scanpackages --multiversion pool /dev/null > dists/stable/main/binary-amd64/Packages
# Les deux formats conviennent aux clients apt modernes.
gzip -n -c dists/stable/main/binary-amd64/Packages > dists/stable/main/binary-amd64/Packages.gz
# Release contient les sommes de contrôle des index réellement générés.
apt-ftparchive -o APT::FTPArchive::Release::Origin=OmniShell -o APT::FTPArchive::Release::Suite=stable release dists/stable > dists/stable/Release
# Les signatures sont faites avec une clé existante, sans exporter sa partie privée.
gpg --batch --yes --local-user "$OMNISHELL_GPG_KEY" --clearsign --output dists/stable/InRelease dists/stable/Release
# L'ancienne signature détachée reste disponible pour les clients qui la demandent.
gpg --batch --yes --local-user "$OMNISHELL_GPG_KEY" --detach-sign --output dists/stable/Release.gpg dists/stable/Release
# Seule la clé publique est distribuée avec le dépôt.
gpg --armor --export "$OMNISHELL_GPG_KEY" > omnishell-signing.asc
