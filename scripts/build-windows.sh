#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

bash scripts/build-sidecar.sh
npm run build

# Cross-compile sidecar + frontend works on Linux; Tauri Windows bundle needs a Windows host (or CI).
is_windows_host() {
  case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*) return 0 ;;
  esac
  [[ -n "${WINDIR:-}" || -n "${OS:-}" && "${OS}" == "Windows_NT" ]]
}

if is_windows_host; then
  cargo tauri build --no-bundle
  bash scripts/package-windows-portable.sh
  echo ""
  echo "Requires WebView2 runtime on target machine (pre-installed on Windows 10/11)."
else
  echo ""
  echo "Sidecar + frontend Windows prêts (cross-compile depuis Linux)."
  echo ""
  echo "  Sidecar : src-tauri/binaries/savior-sensord-x86_64-pc-windows-msvc.exe"
  echo "  Frontend: dist/"
  echo ""
  echo "Le bundle Tauri portable ne peut pas être produit nativement sous Linux."
  echo "Sur Windows ou CI windows-latest :"
  echo ""
  echo "  bash scripts/build-windows.sh"
  echo ""
  echo "Artefact : src-tauri/target/release/bundle/Savior_*_x64-portable.zip"
fi
