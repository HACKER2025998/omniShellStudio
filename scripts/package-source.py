#!/usr/bin/env python3
"""Produit une archive des sources suivies et son SHA-256, sans dépendances ni données locales."""
import hashlib
import json
import subprocess
import tarfile
from pathlib import Path
# Git définit exactement les fichiers publiables ; les builds ignorés ne sont jamais exportés.
root=Path(__file__).resolve().parents[1]
version=json.loads((root/'package.json').read_text())['version']
files=subprocess.check_output(['git','ls-files','-z'],cwd=root).decode().split('\0')
out=root/'release'
out.mkdir(exist_ok=True)
archive=out/f'omnishell-studio-{version}.tar.gz'
with tarfile.open(archive,'w:gz') as tar:
    for relative in files:
        if relative and (root/relative).is_file():
            tar.add(root/relative,arcname='omnishell-studio/'+relative,recursive=False)
# Le checksum est calculé sur l'archive effectivement produite, jamais laissé à SKIP dans une publication.
digest=hashlib.sha256(archive.read_bytes()).hexdigest()
(archive.with_suffix(archive.suffix+'.sha256')).write_text(f'{digest}  {archive.name}\n')
# Le manifeste AUR produit à côté de l’archive contient son nom et son hash réels.
manifest=(root/"packaging/PKGBUILD").read_text().replace("source=('omnishell-studio.tar.gz')",f"source=('{archive.name}')").replace("sha256sums=('SKIP')",f"sha256sums=('{digest}')")
(out/"PKGBUILD").write_text(manifest)
print(archive)
print(digest)
