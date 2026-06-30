#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

VERSION=$(node -p "require('./package.json').version")
RELEASE="$ROOT/src-tauri/target/release"
STAGING="$ROOT/src-tauri/target/release/bundle/portable/Savior"
ZIP="$ROOT/src-tauri/target/release/bundle/Savior_${VERSION}_x64-portable.zip"
SIDECAR_RELEASE="$RELEASE/savior-sensord-x86_64-pc-windows-msvc.exe"
SIDECAR_BINARIES="$ROOT/src-tauri/binaries/savior-sensord-x86_64-pc-windows-msvc.exe"

if [[ ! -f "$RELEASE/savior.exe" ]]; then
  echo "Missing $RELEASE/savior.exe — run cargo tauri build --no-bundle first." >&2
  exit 1
fi

if [[ -f "$SIDECAR_RELEASE" ]]; then
  SIDECAR="$SIDECAR_RELEASE"
elif [[ -f "$SIDECAR_BINARIES" ]]; then
  SIDECAR="$SIDECAR_BINARIES"
else
  echo "Missing sidecar — run scripts/build-sidecar.sh first." >&2
  echo "  Expected: $SIDECAR_BINARIES" >&2
  exit 1
fi

rm -rf "$ROOT/src-tauri/target/release/bundle/portable"
mkdir -p "$STAGING"

cp "$RELEASE/savior.exe" "$STAGING/Savior.exe"
cp "$SIDECAR" "$STAGING/"

mkdir -p "$(dirname "$ZIP")"
rm -f "$ZIP"

PORTABLE_PARENT="$ROOT/src-tauri/target/release/bundle/portable"
PORTABLE_NAME="Savior"

create_portable_zip() {
  if command -v zip >/dev/null 2>&1; then
    (cd "$PORTABLE_PARENT" && zip -rq "$ZIP" "$PORTABLE_NAME")
    return 0
  fi

  # Windows runners ship tar; Git Bash paths work and avoid PowerShell /d/... issues.
  if command -v tar >/dev/null 2>&1; then
    tar -a -cf "$ZIP" -C "$PORTABLE_PARENT" "$PORTABLE_NAME"
    return 0
  fi

  if command -v powershell.exe >/dev/null 2>&1; then
    local staging_win zip_win
    if command -v cygpath >/dev/null 2>&1; then
      staging_win=$(cygpath -w "$STAGING")
      zip_win=$(cygpath -w "$ZIP")
    else
      staging_win="$STAGING"
      zip_win="$ZIP"
    fi
    powershell.exe -NoProfile -Command \
      "Compress-Archive -LiteralPath '$staging_win' -DestinationPath '$zip_win' -Force"
    return 0
  fi

  return 1
}

if ! create_portable_zip; then
  echo "Need zip, tar, or PowerShell to create the portable archive." >&2
  exit 1
fi

echo ""
echo "Portable Windows package (no installer): $ZIP"
echo "Extract on USB, then run Savior.exe — nothing is written to Program Files."
