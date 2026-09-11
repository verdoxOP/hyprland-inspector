# Release ladder

Every release below is a runnable Tauri build. The release selector is compiled into the frontend, so earlier versions intentionally expose only the capabilities earned at that point; they are not mockups.

| Version | Working scope | Run it |
| --- | --- | --- |
| v0.1 | Hyprland connectivity status and active-window readout | `npm run dev:0.1` |
| v0.2 | Live client inventory and floating-state counters | `npm run dev:0.2` |
| v0.3 | Workspace map and per-workspace window indicators | `npm run dev:0.3` |
| v0.4 | Monitor discovery, output geometry, focused-display status | `npm run dev:0.4` |
| v0.5 | Unified monitor/workspace topology view | `npm run dev:0.5` |
| v0.6 | Socket2 compositor event stream with pause and clear controls | `npm run dev:0.6` |
| v0.7 | Rule Lab: generate Hyprland v2 window-rule match previews | `npm run dev:0.7` |
| v0.8 | “Why is this window here?” placement diagnostics | `npm run dev:0.8` |
| v0.9 | In-app settings surface and catalog settings launcher | `npm run dev:0.9` |
| v1.0 | Complete polished inspector: all views, live refresh, desktop integration | `npm run dev:1.0` |

Use the equivalent `npm run build:<version>` command for a frontend build. `v1.0` is the default when no release selector is supplied.

## Scope guarantee

The source of truth is shared deliberately: every milestone invokes the same Rust Hyprland IPC backend and therefore remains a functional diagnostic tool. Version gates only prevent unreleased controls from being rendered, avoiding ten copies that could drift or silently break.
