#!/usr/bin/env python3
"""Recette SQL sur conteneurs éphémères ; nettoyage garanti et ports limités à loopback."""
import os
import subprocess
import time
from pathlib import Path
# Les noms de conteneurs identifient cette seule exécution et ne touchent pas les services existants.
containers=[]
root=Path(__file__).resolve().parents[1]
def command(*args):
    # Une liste d'arguments évite toute expansion shell des données de test.
    return subprocess.check_output(args,text=True).strip()
try:
    for engine,image,port,variables in [('postgres','postgres:17-alpine',5432,['POSTGRES_PASSWORD=omni-test-only']),('mysql','mariadb:11',3306,['MARIADB_ROOT_PASSWORD=omni-test-only','MARIADB_DATABASE=omni_test'])]:
        name=f'omnishell-test-{engine}-{os.getpid()}'
        # Docker choisit un port libre sur 127.0.0.1 ; aucun accès LAN à la base de test.
        args=['docker','run','--rm','-d','--name',name,'-p',f'127.0.0.1::{port}']
        for variable in variables:
            args.extend(['-e',variable])
        command(*args,image)
        containers.append(name)
        mapped=command('docker','port',name,f'{port}/tcp').rsplit(':',1)[1]
        # Les sondes internes attendent l'initialisation complète avant les tests SQL.
        for attempt in range(60):
            probe=['docker','exec',name]+(['pg_isready','-U','postgres'] if engine=='postgres' else ['healthcheck.sh','--connect','--innodb_initialized'])
            if subprocess.run(probe,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL).returncode==0:
                break
            time.sleep(1)
        else:
            raise RuntimeError(f'Le conteneur {engine} ne devient pas prêt')
        env=os.environ.copy()
        # Ces identifiants jetables sont détruits avec le conteneur, jamais conservés dans une configuration utilisateur.
        env['OMNI_TEST_PG_URL' if engine=='postgres' else 'OMNI_TEST_MYSQL_URL']=f'{engine}://{("postgres" if engine=="postgres" else "root")}:omni-test-only@127.0.0.1:{mapped}/{("postgres" if engine=="postgres" else "omni_test")}'
        subprocess.run(['cargo','test','--locked','--manifest-path',str(root/'src-tauri/Cargo.toml'),f'{engine}_live_schema','--','--ignored','--nocapture'],env=env,check=True,cwd=root)
finally:
    # Même une assertion en échec supprime les bases de laboratoire de cette exécution.
    for name in containers:
        subprocess.run(['docker','rm','-f',name],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
