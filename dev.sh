#!/bin/sh
# Développement natif : fonctionne également avec le bootstrap npm isolé livré sur ce poste.
set -eu
DOCKFORGE_ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cd "$DOCKFORGE_ROOT"
if command -v npm >/dev/null 2>&1; then
  DOCKFORGE_REAL_NPM=$(command -v npm)
  export DOCKFORGE_REAL_NPM
elif [ -f "$DOCKFORGE_ROOT/.tools/npm/bin/npm-cli.js" ]; then
  DOCKFORGE_NPM_CLI="$DOCKFORGE_ROOT/.tools/npm/bin/npm-cli.js"
  export DOCKFORGE_NPM_CLI
else
  printf '%s\n' 'Node est présent sans npm : installez npm puis relancez ce script.' >&2
  exit 1
fi
PATH="$DOCKFORGE_ROOT/scripts/tooling:$PATH"
export PATH
if [ ! -d node_modules ]; then
  npm ci
fi
exec npm run tauri -- dev
