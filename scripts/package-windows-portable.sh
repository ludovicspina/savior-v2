#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

VERSION=$(node -p "require('./package.json').version")
RELEASE="$ROOT/src-tauri/target/release"
STAGING="$ROOT/src-tauri/target/release/bundle/portable/Savior"
ZIP="$ROOT/src-tauri/target/release/bundle/Savior_${VERSION}_x64-portable.zip"
SIDECAR="$RELEASE/savior-sensord-x86_64-pc-windows-msvc.exe"

if [[ ! -f "$RELEASE/savior.exe" ]]; then
  echo "Missing $RELEASE/savior.exe — run cargo tauri build --no-bundle first." >&2
  exit 1
fi

if [[ ! -f "$SIDECAR" ]]; then
  echo "Missing sidecar $SIDECAR — run scripts/build-sidecar.sh first." >&2
  exit 1
fi

rm -rf "$ROOT/src-tauri/target/release/bundle/portable"
mkdir -p "$STAGING"

cp "$RELEASE/savior.exe" "$STAGING/Savior.exe"
cp "$SIDECAR" "$STAGING/"

mkdir -p "$(dirname "$ZIP")"
rm -f "$ZIP"

if command -v zip >/dev/null 2>&1; then
  (cd "$ROOT/src-tauri/target/release/bundle/portable" && zip -rq "$ZIP" Savior)
elif command -v powershell.exe >/dev/null 2>&1; then
  powershell.exe -NoProfile -Command \
    "Compress-Archive -Path '$STAGING' -DestinationPath '$ZIP' -Force"
else
  echo "Need zip or PowerShell to create the portable archive." >&2
  exit 1
fi

echo ""
echo "Portable Windows package (no installer): $ZIP"
echo "Extract on USB, then run Savior.exe — nothing is written to Program Files."
