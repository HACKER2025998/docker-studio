#!/bin/bash
# Compile dans un conteneur Debian ; seules les sorties sous dist-linux sont écrites sur l’hôte.
set -euo pipefail
DOCKFORGE_ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
DOCKFORGE_OUTPUT="$DOCKFORGE_ROOT/dist-linux"
mkdir -p "$DOCKFORGE_OUTPUT"
# Le cache registry reste en lecture seule ; le conteneur construit dans son propre /build.
DOCKFORGE_CARGO_CACHE="${CARGO_HOME:-$HOME/.cargo}/registry"
DOCKFORGE_CACHE_ARGS=()
if [ "${DOCKFORGE_OFFLINE:-false}" = true ] && [ -d "$DOCKFORGE_CARGO_CACHE" ]; then
  DOCKFORGE_CACHE_ARGS=(--mount "type=bind,src=$DOCKFORGE_CARGO_CACHE,dst=/usr/local/cargo/registry,readonly")
fi
exec docker run --rm --env "DOCKFORGE_OFFLINE=${DOCKFORGE_OFFLINE:-false}" --label io.dockforge.purpose=packaging --memory=8g --cpus=4 \
  --mount "type=bind,src=$DOCKFORGE_ROOT,dst=/source,readonly" \
  --mount "type=bind,src=$DOCKFORGE_ROOT/scripts/container-build-linux.sh,dst=/builder.sh,readonly" \
  --mount "type=bind,src=$DOCKFORGE_OUTPUT,dst=/out" \
  "${DOCKFORGE_CACHE_ARGS[@]}" rust:1.88-bookworm bash /builder.sh
