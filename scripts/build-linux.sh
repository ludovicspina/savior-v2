#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

npm run build
# Fedora 44+ / recent binutils: bundled linuxdeploy strip fails on .relr.dyn sections
export NO_STRIP=1
cargo tauri build --bundles appimage

echo ""
echo "Linux AppImage: src-tauri/target/release/bundle/appimage/"
