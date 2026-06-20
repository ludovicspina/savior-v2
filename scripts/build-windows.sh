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
  cargo tauri build --bundles nsis
  echo ""
  echo "Windows build complete."
  echo "Installer: src-tauri/target/release/bundle/nsis/"
  echo "Binary:    src-tauri/target/release/savior.exe"
  echo "Requires WebView2 runtime on target machine (pre-installed on Windows 10/11)."
else
  echo ""
  echo "Sidecar + frontend Windows prêts (cross-compile depuis Linux)."
  echo ""
  echo "  Sidecar : src-tauri/binaries/savior-sensord-x86_64-pc-windows-msvc.exe"
  echo "  Frontend: dist/"
  echo ""
  echo "Le bundle Tauri (.exe / NSIS) ne peut pas être produit nativement sous Linux."
  echo "Sur une machine Windows (ou GitHub Actions windows-latest), lancer :"
  echo ""
  echo "  npm run build"
  echo "  cargo tauri build --bundles nsis"
  echo ""
  echo "Copier le sidecar déjà buildé si besoin, ou relancer scripts/build-sidecar.sh sous Windows."
fi
