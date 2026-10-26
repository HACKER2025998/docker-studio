#!/bin/bash
set -euo pipefail
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq
apt-get install -y --no-install-recommends build-essential curl ca-certificates xz-utils pkg-config libssl-dev libgtk-3-dev libwebkit2gtk-4.1-dev librsvg2-dev libayatana-appindicator3-dev libsecret-1-dev libdbus-1-dev file patchelf xdg-utils
mkdir -p /opt/node /build
curl --fail --location --silent --show-error https://nodejs.org/dist/latest-v24.x/SHASUMS256.txt -o /tmp/node-checksums
NODE_ARCHIVE=$(awk '/linux-x64.tar.xz$/ {print $2;exit}' /tmp/node-checksums)
curl --fail --location --silent --show-error "https://nodejs.org/dist/latest-v24.x/$NODE_ARCHIVE" -o "/tmp/$NODE_ARCHIVE"
cd /tmp
grep "  $NODE_ARCHIVE$" /tmp/node-checksums | sha256sum -c -
tar -xJf "$NODE_ARCHIVE" -C /opt/node --strip-components=1
export PATH="/opt/node/bin:$PATH"
tar -C /source --exclude=node_modules --exclude=target --exclude=dist --exclude=.tools --exclude=.git --exclude=dist-linux --exclude=test-results --exclude=playwright-report --exclude=__pycache__ -cf - . | tar -C /build -xf -
cd /build
npm ci
export CARGO_NET_OFFLINE="${DOCKFORGE_OFFLINE:-false}"
export CARGO_BUILD_JOBS=4
export APPIMAGE_EXTRACT_AND_RUN=1
export NO_STRIP=true
npm run tauri -- build --bundles deb,appimage
cp src-tauri/target/release/bundle/deb/*.deb /out/
cp src-tauri/target/release/bundle/appimage/*.AppImage /out/
cp src-tauri/target/release/dockforge /out/dockforge-debian.bin
rustc --version > /out/toolchain.txt
node --version >> /out/toolchain.txt
