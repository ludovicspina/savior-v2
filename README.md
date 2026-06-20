# Savior v2

Desktop technician toolkit for real-time hardware monitoring (CPU, RAM, GPU, disks, SMART). Built with **Tauri 2**, **Vue 3**, and a **C# .NET 8 sidecar** (Windows) powered by LibreHardwareMonitor.

> **Important:** A single cross-platform binary is **not possible**. Savior produces **two portable single-file artifacts** — one per OS (Windows `.exe` + sidecar, Linux `.AppImage`).

## Stack

| Layer | Technology |
|-------|------------|
| Frontend | Vue 3, TypeScript, Vite, Tailwind CSS, Pinia |
| Shell | Tauri 2 (Rust) |
| Windows sensors | C# .NET 8 sidecar + LibreHardwareMonitorLib |
| Linux sensors | `/sys/class/hwmon` + `sysinfo` crate (phase 1) |

## Prerequisites

### All platforms (dev)

- Node.js 20+
- Rust stable (`rustup`)
- Tauri CLI: `cargo install tauri-cli --version "^2.0.0"`

### Linux (Fedora / dev target)

```bash
sudo dnf install webkit2gtk4.1-devel openssl-devel librsvg2-devel \
  libappindicator-gtk3-devel
```

### Windows build (cross-compile from Linux or CI)

- [.NET 8 SDK](https://dotnet.microsoft.com/download/dotnet/8.0)
- Si `NU1301` / timeout NuGet depuis Fedora : IPv6 cassé sur le réseau — les scripts exportent déjà `DOTNET_SYSTEM_NET_DISABLEIPV6=1`. Sinon : `export DOTNET_SYSTEM_NET_DISABLEIPV6=1` avant `dotnet restore`.

### Windows runtime (end users)

- **WebView2** — pre-installed on Windows 10/11; otherwise install the [Evergreen bootstrapper](https://developer.microsoft.com/en-us/microsoft-edge/webview2/).

## Quick start (Linux dev)

```bash
npm install
npm run tauri dev
```

This starts the app with `LinuxSysfsSource` — basic CPU load, RAM, disk usage, and hwmon temperatures where available.

## Build commands

### Linux AppImage (single-file, no sidecar)

```bash
bash scripts/build-linux.sh
# Output: src-tauri/target/release/bundle/appimage/Savior_*.AppImage
```

**Utilisation standalone (utilisateur final)** — un seul fichier, rien à installer dans `/usr` :

```bash
chmod +x Savior_0.1.0_amd64.AppImage
./Savior_0.1.0_amd64.AppImage
```

- Tu lances le **`.AppImage`**, pas `AppRun` ni le dossier `Savior.AppDir/`.
- Le dossier `Savior.AppDir/usr/` est un **artefact intermédiaire de build** (layout FHS *à l'intérieur* de l'AppImage). Ce n'est pas une installation système.
- WebKitGTK et les libs nécessaires sont **emballées dedans** (~100 Mo). Aucun `dnf install` requis sur la machine cible.
- Sur certaines distros sans FUSE monté : `APPIMAGE_EXTRACT_AND_RUN=1 ./Savior_*.AppImage`

### Windows portable (sidecar bundled)

```bash
bash scripts/build-windows.sh
```

**Depuis Fedora/Linux** : le script cross-compile le sidecar C# + le frontend, puis s'arrête (pas de `.exe` Tauri — impossible sans hôte Windows).

**Sur Windows ou CI `windows-latest`** : produit l'installateur NSIS + `savior.exe` avec le sidecar bundlé.

```bash
npm run build
cargo tauri build --bundles nsis
# Output: src-tauri/target/release/bundle/nsis/
#         src-tauri/target/release/savior.exe
```

For a **portable NSIS installer** (no install step), enable in `src-tauri/tauri.conf.json`:

```json
"bundle": {
  "windows": {
    "nsis": {
      "installerIcon": "icons/icon.ico",
      "displayLanguageSelector": false
    }
  }
}
```

See [Tauri 2 NSIS docs](https://v2.tauri.app/reference/config/#nsisconfig) for portable bundle options.

## Sidecar build (Windows only)

```bash
bash scripts/build-sidecar.sh
```

Publishes `Savior.Sensord.exe` as a self-contained single-file binary and copies it to:

```
src-tauri/binaries/savior-sensord-x86_64-pc-windows-msvc.exe
```

Tauri resolves `bundle.externalBin` via `src-tauri/tauri.windows.conf.json` (Windows-only merge) and appends the target triple automatically. Linux builds do not require the sidecar binary.

### Publish settings (sidecar/Savior.Sensord.csproj)

- `PublishSingleFile=true`, `SelfContained=true`
- `IncludeNativeLibrariesForSelfExtract=true`
- **`PublishTrimmed` is disabled** — LHM uses reflection and loads native drivers; trimming breaks it.
- **NativeAOT is not used** — incompatible with LHM driver/reflection.

## LibreHardwareMonitor & WinRing0 / PawnIO

By default, older LibreHardwareMonitor builds embed the **WinRing0** kernel driver, which **Windows Defender quarantines** since 2025 ([CVE-2020-14979](https://nvd.nist.gov/vuln/detail/CVE-2020-14979)).

**You must use the PawnIO build of LibreHardwareMonitor**, which provides a properly signed Ring0 mechanism:

1. Clone [LibreHardwareMonitor](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor) and switch to the branch/tag that uses **PawnIO** (check upstream releases/README).
2. Build `LibreHardwareMonitorLib` from that source.
3. If the NuGet package `LibreHardwareMonitorLib` still bundles WinRing0, replace the referenced DLL with your PawnIO build before publishing the sidecar.

### Graceful degradation (implemented)

| Mode | Driver | Admin | Trigger |
|------|--------|-------|---------|
| **basic** (default) | None | No | Sidecar starts automatically |
| **deep** | Ring0 (PawnIO) | May require admin | User clicks "Enable deep sensors" → `enable-deep` on stdin |

The sidecar `app.manifest` defaults to `asInvoker`. For deep mode with drivers that need elevation, change to `requireAdministrator` and re-build.

## JSON contract (sidecar → Rust → frontend)

One JSON object per line on stdout:

```json
{
  "ts": 1718800000,
  "mode": "basic",
  "cpu": { "name": "string", "loadPct": 0.0, "tempC": null, "clockMhz": null },
  "ram": { "usedMb": 0, "totalMb": 0 },
  "gpus": [{ "name": "string", "loadPct": null, "tempC": null, "memUsedMb": null }],
  "disks": [{ "name": "string", "tempC": null, "healthPct": null, "usedGb": 0, "totalGb": 0 }]
}
```

Stdin commands: `enable-deep`, `disable-deep`, `set-interval <ms>`, `quit`.

## CI / Windows testing from Fedora

- `dotnet publish -r win-x64` **works from Linux** (cross-publish).
- The Ring0/PawnIO driver **only loads on real Windows** — test deep mode in a **Windows VM** or **GitHub Actions** (`windows-latest`).
- Linux path is fully testable on Fedora.

Optional CI workflow: `.github/workflows/build.yml` (matrix `ubuntu-latest` + `windows-latest`).

## Project layout

```
src/                  Vue 3 frontend
src-tauri/            Tauri 2 Rust backend
  src/sensors/        SensorSource trait + OS backends
  binaries/           Sidecar exe (Windows, post-build)
sidecar/              C# .NET 8 sensord (Windows only)
scripts/              build-sidecar.sh, build-linux.sh, build-windows.sh
```

## TODO: Code signing

- [ ] Sign `Savior.Sensord.exe` (sidecar) with an Authenticode certificate
- [ ] Sign the final `savior.exe` / NSIS installer
- Reduces SmartScreen warnings and antivirus false positives
- Required for production distribution

## License

TBD
