#!/usr/bin/env python3
"""Mesure RSS/PSS du daemon et du bureau, sur un profil synthétique isolé."""
import json
import os
import signal
import subprocess
import tempfile
import time
from pathlib import Path
# La mesure vise Linux /proc ; aucune donnée du compte réel n'est nécessaire.
root=Path(__file__).resolve().parents[1]
binary=root/'src-tauri/target/release/omnishell'
def descendants(pid):
    # Les renderers WebKit sont comptés, afin de ne pas sous-estimer la mémoire desktop.
    children=[]
    try:
        for child in (Path('/proc')/str(pid)/'task'/str(pid)/'children').read_text().split():
            children.extend(descendants(int(child)))
    except (FileNotFoundError,PermissionError):
        pass
    return [pid,*children]
def sample(pid):
    result={'rss_kib':0,'pss_kib':0,'processes':0}
    for process in descendants(pid):
        try:
            fields=dict(line.split(':',1) for line in (Path('/proc')/str(process)/'smaps_rollup').read_text().splitlines() if ':' in line)
            result['rss_kib']+=int(fields['Rss'].split()[0])
            result['pss_kib']+=int(fields['Pss'].split()[0])
            result['processes']+=1
        except (FileNotFoundError,PermissionError,KeyError):
            pass
    return result
report={'platform':'Linux','scope':'Processus parent et descendants, profil synthétique de 100 commandes, huit secondes ; RSS et PSS ne sont pas interchangeables.'}
with tempfile.TemporaryDirectory(prefix='omnishell-resources-') as folder:
    sandbox=Path(folder)
    # Les chemins XDG, HOME et le stockage applicatif sont tous redirigés.
    env=os.environ.copy()
    for key,suffix in [('HOME','home'),('XDG_CONFIG_HOME','config'),('XDG_CACHE_HOME','cache'),('XDG_DATA_HOME','share'),('OMNISHELL_DATA_DIR','data')]:
        path=sandbox/suffix
        path.mkdir()
        env[key]=str(path)
    # Docker est volontairement indisponible : la mesure ne lit aucun conteneur réel.
    tools=sandbox/'tools'
    tools.mkdir()
    (tools/'docker').write_text('#!/bin/sh\nexit 1\n')
    (tools/'docker').chmod(0o755)
    env['PATH']=str(tools)+':'+env.get('PATH','')
    (sandbox/'home/.bash_history').write_text('\n'.join(f'git show synthetic-{i}' for i in range(100)))
    for mode,args in [('daemon',['daemon']),('desktop',[])]:
        selected=binary.with_name("omnishell-daemon") if mode=="daemon" else binary
        process=subprocess.Popen([str(selected),*([] if mode=="daemon" else args)],env=env,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,start_new_session=True)
        peaks={'rss_kib':0,'pss_kib':0,'processes':0}
        try:
            for index in range(32):
                time.sleep(.25)
                if process.poll() is not None:
                    peaks['early_exit']=process.returncode
                    break
                current=sample(process.pid)
                for key in ['rss_kib','pss_kib','processes']:
                    peaks[key]=max(peaks[key],current[key])
            report[mode]=peaks
        finally:
            try:
                os.killpg(process.pid,signal.SIGTERM)
                process.wait(timeout=5)
            except ProcessLookupError:
                pass
            except subprocess.TimeoutExpired:
                os.killpg(process.pid,signal.SIGKILL)
                process.wait()
print(json.dumps(report,indent=2,ensure_ascii=False))
