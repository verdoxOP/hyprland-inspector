# Hyprland Inspector

<p align="center">A visual, local-first debugger for the Hyprland compositor.</p>

Hyprland Inspector turns the compositor's live IPC data into a fast diagnostic console: see where windows are, how monitor and workspace state fits together, which live clients match a prospective rule, and the state behind a confusing placement.

It is built for Linux desktops running Hyprland. The backend is Rust/Tauri; the interface is a compact, cyberpunk-styled TypeScript/Vite application.

## What it does

- **Live window inventory** — inspect class, title, address, workspace, monitor, geometry, and floating state.
- **Monitor / workspace topology** — map focused outputs, workspaces, and visible client activity in one view.
- **Rule Lab** — generate modern Hyprland `windowrule` match conditions from a live client and preview the matching clients.
- **Event stream** — watch Hyprland's socket2 activity, including focus changes, client opens, moves, and workspace transitions.
- **Placement diagnostics** — a clear state-based explanation for “why is this window here?”
- **Release ladder** — run a deliberately progressive v0.1 through v1.0 feature set from the same reliable backend.

> Hyprland's IPC reports final window state, but does not expose rule execution provenance. The placement view says exactly what it can infer and does not pretend to know which config line ran.

## Requirements

- A running Hyprland session with `hyprctl` available on `PATH`
- Rust and Cargo
- Node.js with npm
- Linux with WebKitGTK dependencies required by Tauri

On NVIDIA + Wayland, the app sets `WEBKIT_DISABLE_DMABUF_RENDERER=1` before the webview is created. This avoids the known DMA-BUF renderer crash on affected setups.

## Development

```bash
git clone https://github.com/verdoxOP/hyprland-inspector.git
cd hyprland-inspector
npm install
npx tauri dev
```

The normal development command runs the complete v1.0 experience.

## Progressive releases

Every release is runnable, not a mockup. Earlier releases deliberately hide later capabilities while continuing to use the real Rust Hyprland IPC backend.

| Version | Focus | Command |
| --- | --- | --- |
| v0.1 | Connection and active-window readout | `npm run dev:0.1` |
| v0.2 | Client inventory | `npm run dev:0.2` |
| v0.3 | Workspace map | `npm run dev:0.3` |
| v0.4 | Monitor discovery | `npm run dev:0.4` |
| v0.5 | Complete topology map | `npm run dev:0.5` |
| v0.6 | Live event stream | `npm run dev:0.6` |
| v0.7 | Rule Lab | `npm run dev:0.7` |
| v0.8 | Placement diagnostics | `npm run dev:0.8` |
| v0.9 | Settings surface | `npm run dev:0.9` |
| v1.0 | Full inspector | `npm run dev:1.0` |

See [RELEASES.md](RELEASES.md) for the full release scope.

## Build

```bash
npx tauri build
```

Use the project launcher while developing:

```bash
./bin/hyprland-inspector
```

It uses a release binary when one exists, otherwise it starts Tauri's development workflow.

## Desktop integration

The repository includes XDG entries for the Inspector and its settings app. Install them locally with:

```bash
install -Dm644 desktop/hyprland-inspector.desktop ~/.local/share/applications/hyprland-inspector.desktop
install -Dm644 desktop/hyprland-inspector-settings.desktop ~/.local/share/applications/hyprland-inspector-settings.desktop
install -Dm644 assets/hyprland-inspector.svg ~/.local/share/icons/hicolor/scalable/apps/hyprland-inspector.svg
update-desktop-database ~/.local/share/applications
```

They are then discoverable through standard XDG app launchers, including Kiroshi Scanner.

## Architecture

```text
Hyprland IPC socket2 ──┐
                        ├── Rust / Tauri backend ── Tauri events ── Inspector interface
hyprctl -j snapshots ──┘
```

The app is local-first. It reads Hyprland IPC data and executes `hyprctl -j` snapshot queries; it does not send compositor data to a remote service.

## Contributing

For a focused local commit, use:

```bash
./scripts/commit.sh "Describe the change"
```

The helper stages source and project metadata only; dependencies and build output remain excluded.
