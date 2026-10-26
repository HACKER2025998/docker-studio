#!/usr/bin/env python3
"""Mesure Linux du démarrage natif, avec profil local isolé et descendants WebKit inclus."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import signal
import subprocess
import tempfile
import time

ROOT=Path(__file__).resolve().parents[1]
parser=argparse.ArgumentParser()
parser.add_argument('--binary',type=Path,default=ROOT/'src-tauri/target/release/dockforge')
BINARY=parser.parse_args().binary.resolve()

def descendants(pid,seen=None):
    # Les processus lancés par un worker Tokio peuvent appartenir à un autre thread que le chef.
    if seen is None:seen=set()
    if pid in seen:return []
    seen.add(pid)
    result=[pid]
    children=set()
    try:
        for task in (Path('/proc')/str(pid)/'task').iterdir():
            try:children.update(int(child) for child in (task/'children').read_text().split())
            except (FileNotFoundError,PermissionError):pass
        for child in children:result.extend(descendants(child,seen))
    except (FileNotFoundError,PermissionError):pass
    return result

def sample(pid):
    result={'rss_kib':0,'pss_kib':0,'processes':0}
    for child in descendants(pid):
        try:
            fields=dict(line.split(':',1) for line in (Path('/proc')/str(child)/'smaps_rollup').read_text().splitlines())
            result['rss_kib']+=int(fields['Rss'].split()[0])
            result['pss_kib']+=int(fields['Pss'].split()[0])
            result['processes']+=1
        except (FileNotFoundError,PermissionError,KeyError):pass
    return result

with tempfile.TemporaryDirectory(prefix='dockforge-memory-') as folder:
    env=os.environ.copy();env['DOCKFORGE_DATA_DIR']=folder
    process=subprocess.Popen([str(BINARY)],env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
    report={'binary_sha256':hashlib.sha256(BINARY.read_bytes()).hexdigest(),'platform':'Linux','scope':'Pics échantillonnés toutes les 250 ms durant huit secondes de démarrage, écran Doctor, profil vide ; descendants de tous les threads inclus sans double compte. Pas une mesure de charge ni une garantie Windows/macOS.','desktop':{'rss_kib':0,'pss_kib':0,'processes':0}}
    try:
        for _ in range(32):
            time.sleep(.25)
            if process.poll() is not None:
                report['early_exit']=process.returncode
                break
            current=sample(process.pid)
            for metric,value in current.items():report['desktop'][metric]=max(report['desktop'][metric],value)
    finally:
        if process.poll() is None:
            os.killpg(process.pid,signal.SIGTERM)
            try:process.wait(timeout=5)
            except subprocess.TimeoutExpired:os.killpg(process.pid,signal.SIGKILL);process.wait()
    if 'early_exit' in report:
        report['stderr']=process.stderr.read(4096).decode(errors='replace')
    print(json.dumps(report,ensure_ascii=False,indent=2))
