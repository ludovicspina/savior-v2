#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# Fedora / broken IPv6: NuGet hangs ~100s then NU1301 if .NET tries IPv6 first
export DOTNET_SYSTEM_NET_DISABLEIPV6=1

dotnet publish sidecar/Savior.Sensord.csproj -c Release -r win-x64 \
  --self-contained true -p:PublishSingleFile=true \
  -p:IncludeNativeLibrariesForSelfExtract=true -p:EnableCompressionInSingleFile=true

mkdir -p src-tauri/binaries
cp sidecar/bin/Release/net8.0/win-x64/publish/Savior.Sensord.exe \
   src-tauri/binaries/savior-sensord-x86_64-pc-windows-msvc.exe

echo "Sidecar copied to src-tauri/binaries/savior-sensord-x86_64-pc-windows-msvc.exe"
