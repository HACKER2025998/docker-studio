#!/usr/bin/env python3
"""Commente les instructions Rust hors littéraux ; délègue le contrôle AST TypeScript."""
from pathlib import Path
import re
import subprocess
import sys

ROOT=Path(__file__).resolve().parents[1]
CHECK='--check' in sys.argv

def explain(text,context):
    rules=[
        (r'^use ',"Importe explicitement les types et fonctions nécessaires à ce module."),
        (r'^pub (struct|enum)|^(struct|enum) ',"Définit les données échangées par ce module, sans comportement implicite."),
        (r'^#\[',"Applique cet attribut au type ou à la fonction qui suit."),
        (r'\bfn ',"Délimite cette opération pour isoler sa validation et son résultat."),
        (r'\.lock\(',"Sérialise l’accès à cet état partagé entre les tâches."),
        (r'\.await',"Attend l’opération asynchrone avant d’utiliser son résultat."),
        (r'\b(bail!|return Err)',"Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide."),
        (r'^if |^else if ',"Vérifie cette condition avant d’appliquer l’action protégée."),
        (r'^for |^while |^loop',"Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle."),
        (r'\.execute|\.prepare|\.query',"Passe par SQLite et ses paramètres pour séparer les données de la requête."),
        (r'assert',"Vérifie cette propriété observable pour détecter une régression du contrat."),
        (r'\.context\(|\.map_err',"Ajoute un contexte lisible à l’échec sans l’ignorer."),
        (r'let mut ',"Prépare une valeur locale qui sera complétée pendant cette opération."),
        (r'^let ',"Calcule cette valeur avant de l’utiliser dans la suite de l’opération."),
        (r'Ok\(',"Renvoie une réussite explicite avec les données attendues par l’appelant."),
        (r'pub \w+:',"Expose ce champ du contrat sérialisé avec son type explicite."),
        (r'\.insert|\.push|\.extend',"Ajoute cette donnée à la collection en cours de préparation."),
    ]
    for pattern,message in rules:
        if re.search(pattern,text):return f'[{context}] {message}'
    return f'[{context}] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.'

def annotate(source):
    lines=[line for line in source.splitlines() if not re.match(r'^\s*// @ligne ',line)]
    output=[];string=False;raw=None;block=0;context='module'
    for line in lines:
        text=line.strip();inside=string or raw is not None or block>0
        match=re.search(r'\bfn\s+(\w+)',text)
        if match and not inside:context=match[1]
        if text and not inside and not text.startswith(('//','/*','*')) and not re.fullmatch(r'[\[\]{}();,:]+',text):
            indent=line[:len(line)-len(line.lstrip())]
            output.append(indent+'// @ligne '+explain(text,context))
        output.append(line)
        i=0
        while i<len(line):
            if raw is not None:
                end='"'+raw;position=line.find(end,i)
                if position<0:break
                i=position+len(end);raw=None;continue
            if string:
                if line[i]=='\\':i+=2;continue
                if line[i]=='"':string=False
                i+=1;continue
            if block:
                if line.startswith('/*',i):block+=1;i+=2
                elif line.startswith('*/',i):block-=1;i+=2
                else:i+=1
                continue
            if line.startswith('//',i):break
            if line.startswith('/*',i):block=1;i+=2;continue
            match=re.match(r'r(#+)?"',line[i:])
            if match:raw=match[1] or '';i+=len(match[0]);continue
            match=re.match(r"'(?:\\.|[^'\\])'",line[i:])
            if match:i+=len(match[0]);continue
            if line[i]=='"':string=True
            i+=1
    return '\n'.join(output)+'\n'

for path in [*sorted((ROOT/'src-tauri/src').glob('*.rs')),ROOT/'src-tauri/build.rs']:
    original=path.read_text();result=annotate(original)
    if CHECK and original!=result:raise SystemExit(f'Annotations à régénérer : {path}')
    if not CHECK and original!=result:path.write_text(result)
# Les règles CSS restent commentables sans transformer des valeurs en texte affiché.
css=ROOT/'src/style.css'
original=css.read_text()
lines=[line for line in original.splitlines() if not re.match(r'^\s*/\* @ligne ',line)]
properties={
 'color':'Choisit la couleur de premier plan pour préserver la lisibilité sur le fond associé.',
 'background':'Définit le fond de cet élément dans le thème clair.',
 'background-color':'Définit le fond de cet élément dans le thème clair.',
 'display':'Choisit le modèle de disposition des enfants de cet élément.',
 'padding':'Réserve l’espace intérieur entre le contenu et son contour.',
 'margin':'Règle l’espacement extérieur par rapport aux éléments voisins.',
 'gap':'Sépare régulièrement les éléments de cette grille ou de ce groupe flex.',
 'border':'Délimite le composant pour rendre sa frontière perceptible.',
 'border-radius':'Arrondit les angles selon la hiérarchie visuelle du studio.',
 'font-size':'Règle la taille du texte en cohérence avec le niveau d’information.',
 'font-weight':'Hiérarchise visuellement le texte sans dépendre seulement de la couleur.',
 'line-height':'Garde suffisamment d’espace entre les lignes pour faciliter la lecture.',
 'position':'Définit le repère de placement de cet élément dans la fenêtre.',
 'overflow':'Contrôle le contenu dépassant du cadre pour maintenir la vue utilisable.',
 'width':'Fixe ou borne la largeur dans la disposition courante.',
 'height':'Fixe ou borne la hauteur dans la disposition courante.',
 'z-index':'Définit l’ordre de superposition, notamment pour les dialogues et notifications.',
 'transition':'Adoucit ce changement visuel ; la préférence de réduction des animations est traitée plus bas.',
 'grid-template-columns':'Répartit les colonnes selon la largeur disponible.',
 'align-items':'Aligne les enfants sur l’axe transversal du groupe.',
 'justify-content':'Répartit les enfants sur l’axe principal du groupe.',
}
output=[]
for line in lines:
 text=line.strip()
 if text and text not in ['}', '};'] and not text.startswith(('/*','*','*/')):
  key=text.split(':',1)[0].strip()
  message=properties.get(key)
  if message is None:
   if text.startswith('@media'):message='Adapte cette disposition à la largeur ou aux préférences du système.'
   elif text.startswith('@import'):message='Charge les outils de style compilés localement avec l’application.'
   elif text.endswith('{'):message='Applique le groupe de règles suivant à ce composant ou à cette condition.'
   else:message='Complète cette propriété de mise en page, sans changer le contenu métier.'
  indent=line[:len(line)-len(line.lstrip())]
  output.append(indent+'/* @ligne '+message+' */')
 output.append(line)
result='\n'.join(output)+'\n'
if CHECK and original!=result:raise SystemExit('Annotations CSS à régénérer')
if not CHECK and original!=result:css.write_text(result)
subprocess.run(['node',str(ROOT/'scripts/comment-typescript.mjs'),*(['--check'] if CHECK else [])],cwd=ROOT,check=True)
print('Annotations Rust contrôlées hors chaînes et commentaires existants.')
