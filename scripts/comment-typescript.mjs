// Annote les sources sans accepter une modification du JavaScript réellement émis.
import fs from 'node:fs';
import path from 'node:path';
import ts from 'typescript';

const check = process.argv.includes('--check');
const roots = ['src', 'src-tauri/src/share.js'];
function files(dir) { if(fs.statSync(dir).isFile()) return [dir]; return fs.readdirSync(dir, { withFileTypes: true }).flatMap(e => e.isDirectory() ? files(path.join(dir, e.name)) : /\.(tsx?|js)$/.test(e.name) ? [path.join(dir, e.name)] : []); }
function clean(source) { return source.split('\n').filter(line => !/^\s*(?:\/\/ @ligne |\{\/\* @ligne .*\*\/\})/.test(line)).join('\n'); }
function emitted(source, file) {
  const result = ts.transpileModule(source, { fileName: file, compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, jsx: ts.JsxEmit.ReactJSX }, reportDiagnostics: true });
  if (result.diagnostics.some(d => d.category === ts.DiagnosticCategory.Error)) return null;
  const tree = ts.createSourceFile('output.js', result.outputText, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  function shape(node) {
    const literal = ts.isIdentifier(node) || ts.isStringLiteralLike(node) || ts.isNumericLiteral(node) || ts.isRegularExpressionLiteral(node) || ts.isTemplateHead(node) || ts.isTemplateMiddle(node) || ts.isTemplateTail(node);
    const children = []; ts.forEachChild(node, n => { children.push(shape(n)); });
    return [node.kind, literal ? (node.text ?? node.escapedText) : null, children];
  }
  return JSON.stringify(shape(tree));
}
function explain(text, file) {
  const name = path.basename(file, path.extname(file));
  if (/^import /.test(text)) return 'Importe les dépendances explicites utilisées par cet espace.';
  if (/useEffect/.test(text)) return 'Déclare un effet lié au cycle de vie ; son nettoyage évite les abonnements persistants.';
  if (/useState/.test(text)) return 'Conserve cet état local pour mettre à jour le formulaire sans perdre la saisie.';
  if (/useRef/.test(text)) return 'Garde une référence stable entre rendus, sans provoquer un rendu supplémentaire.';
  if (/^const \[/.test(text)) return 'Associe la valeur affichée à sa fonction de mise à jour React.';
  if (/\bawait\b/.test(text)) return 'Attend le résultat de cette action avant de poursuivre le parcours utilisateur.';
  if (/\bcall[<(]/.test(text)) return 'Passe par une commande métier native, avec validation côté Rust.';
  if (/^if\b/.test(text)) return 'Vérifie cette condition avant d’autoriser la suite du traitement.';
  if (/^catch\b|catch\s*\(/.test(text)) return 'Transforme l’échec en retour lisible pour l’utilisateur.';
  if (/^return\b/.test(text)) return 'Renvoie le résultat ou la vue correspondant à l’état courant.';
  if (/onClick=/.test(text)) return 'Relie ce bouton à une action explicite de l’utilisateur.';
  if (/onChange=/.test(text)) return 'Répercute la saisie dans l’état du formulaire.';
  if (/disabled=/.test(text)) return 'Bloque une action indisponible ou déjà en cours.';
  if (/aria-|role=/.test(text)) return 'Expose le sens de ce contrôle aux technologies d’assistance.';
  if (/^<label|^<input|^<select|^<textarea/.test(text)) return 'Présente un champ de configuration associé à son libellé.';
  if (/^<button/.test(text)) return 'Présente une action volontaire, accessible au clavier.';
  if (/^<\//.test(text)) return 'Termine ce groupe visuel en conservant la hiérarchie de la vue.';
  if (/^</.test(text)) return `Compose cet élément de la vue ${name} à partir de l’état courant.`;
  if (/\.map\(/.test(text)) return 'Construit une entrée de vue pour chaque élément de cette collection.';
  if (/\.filter\(/.test(text)) return 'Ne conserve que les éléments répondant au filtre demandé.';
  if (/set[A-Z]\w*\(/.test(text)) return 'Met à jour l’état concerné pour refléter le résultat dans l’interface.';
  if (/^export (type|interface)/.test(text)) return 'Définit le contrat partagé entre le moteur et les composants.';
  if (/^export .*function|^function /.test(text)) return 'Délimite cette responsabilité afin de garder le parcours testable.';
  if (/^const |^let /.test(text)) return 'Prépare une valeur locale utilisée par les étapes suivantes.';
  if (/^\w+[?:]?:/.test(text)) return 'Renseigne ce champ du contrat avec son type ou sa valeur explicite.';
  if (/^[A-Z][A-Za-z0-9_]*,?$/.test(text)) return `Importe ou référence ${text.replace(',', '')}, repère visuel ou type partagé par cette vue.`;
  if (/^className=/.test(text)) return 'Associe les styles Daylight à cet élément sans modifier ses données.';
  if (/^value=/.test(text)) return 'Affiche la valeur courante de l’état, qui reste la source de vérité du champ.';
  if (/^checked=/.test(text)) return 'Reflète le choix courant dans la case à cocher.';
  if (/^key=/.test(text)) return 'Donne à React une identité stable pour conserver correctement cet élément de liste.';
  if (/^type=/.test(text)) return 'Choisit le comportement natif du contrôle, notamment le masquage des secrets.';
  if (/^placeholder=/.test(text)) return 'Fournit un exemple de saisie sans remplacer le libellé accessible.';
  if (/^try/.test(text)) return 'Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.';
  if (/^throw/.test(text)) return 'Signale une entrée ou une réponse invalide avant de poursuivre.';
  if (/^finally/.test(text)) return 'Exécute le nettoyage aussi bien après réussite qu’après échec.';
  if (/^\w+:/.test(text)) return 'Transmet cette option explicite au composant ou au contrat natif.';
  if (/^}/.test(text)) return `Termine cette expression de ${name} et rattache sa suite au groupe englobant.`;
  return `Fournit cet argument à l’expression de ${name} ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.`;
}
let annotated = 0, skipped = 0;
for (const file of roots.flatMap(files)) {
  const original = fs.readFileSync(file, 'utf8');
  const source = clean(original);
  const baseline = emitted(source, file);
  if (!baseline) throw new Error(`Source invalide : ${file}`);
  const tree = ts.createSourceFile(file, source, ts.ScriptTarget.Latest, true, file.endsWith('.tsx') ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const lines = source.split('\n'); let offset = 0; const candidates = [];
  for (let index = 0; index < lines.length; index++) {
    const line = lines[index], text = line.trim(), position = offset + line.search(/\S|$/); offset += line.length + 1;
    if (!text || /^\/\/|^\/\*|^\*|^[\]\[{}();,:]+$/.test(text)) continue;
    let ancestors = [];
    function visit(node, chain) {
      if (position < node.getFullStart() || position >= node.end) return;
      ancestors = [...chain, node];
      ts.forEachChild(node, child => visit(child, [...chain, node]));
    }
    visit(tree, []);
    const deepest = ancestors.at(-1);
    if (!deepest) continue;
    if ((ts.isStringLiteralLike(deepest) || ts.isTemplateLiteralToken(deepest)) && deepest.getStart(tree) < position) continue;
    if (ts.isJsxText(deepest) && text && !text.startsWith('<')) continue;
    let jsx = false;
    for (let n = ancestors.length - 1; n >= 0; n--) {
      const node = ancestors[n], parent = ancestors[n - 1];
      if ((ts.isJsxElement(node) || ts.isJsxSelfClosingElement(node) || ts.isJsxExpression(node) || ts.isJsxFragment(node)) && node.getStart(tree) === position && parent && (ts.isJsxElement(parent) || ts.isJsxFragment(parent))) { jsx = true; break; }
      if ((ts.isJsxClosingElement(node) || ts.isJsxClosingFragment(node)) && node.getStart(tree) === position) { jsx = true; break; }
    }
    const indent = line.match(/^\s*/)[0]; const comment = explain(text, file);
    candidates.push({ index, value: jsx ? `${indent}{/* @ligne ${comment} */}` : `${indent}// @ligne ${comment}` });
  }
  function render(items) { const map = new Map(items.map(c => [c.index, c.value])); return lines.flatMap((line, i) => map.has(i) ? [map.get(i), line] : [line]).join('\n'); }
  function safe(items) {
    if (!items.length) return [];
    if (emitted(render(items), file) === baseline) return items;
    if (items.length === 1) { skipped++; return []; }
    const middle = Math.floor(items.length / 2); return [...safe(items.slice(0, middle)), ...safe(items.slice(middle))];
  }
  let accepted = safe(candidates);
  if (emitted(render(accepted), file) !== baseline) throw new Error(`Interactions de commentaires non neutres : ${file}`);
  const output = render(accepted);
  if (check && output !== original) throw new Error(`Annotations à régénérer : ${file}`);
  if (!check) fs.writeFileSync(file, output);
  annotated += accepted.length;
}
console.log(`${annotated} lignes TypeScript annotées ; ${skipped} positions conservées intactes pour préserver la sémantique JSX.`);
