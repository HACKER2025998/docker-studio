#!/bin/sh
# Lance la version native déjà compilée ; ne déclenche aucune installation système.
set -eu
DOCKFORGE_ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
if [ -x "$DOCKFORGE_ROOT/dist-linux/dockforge.bin" ]; then
  exec "$DOCKFORGE_ROOT/dist-linux/dockforge.bin" "$@"
fi
if [ -x "$DOCKFORGE_ROOT/src-tauri/target/release/dockforge" ]; then
  exec "$DOCKFORGE_ROOT/src-tauri/target/release/dockforge" "$@"
fi
printf '%s\n' 'Aucun binaire local : utilisez ./dev.sh ou construisez un installateur selon le README.' >&2
exit 1
