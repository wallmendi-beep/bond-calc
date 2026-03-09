# Copilot Instructions for Tauri Bond Calculator

This is a **Tauri 2** desktop application built with Rust backend and Vanilla JavaScript frontend.

## Architecture Overview

**Frontend** (`src/`): Vanilla HTML/CSS/JavaScript with dark mode theme support via CSS custom properties. Entry point: `src/index.html`.

**Backend** (`src-tauri/`): Rust-based service layer using Tauri 2 framework. Two implementations exist:
- `main.rs`: Desktop-specific (used for build), contains `toggle_pin` command for window pinning
- `lib.rs`: Mobile-compatible version, contains `greet` example command

**Communication**: Tauri command system bridges frontend-backend:
```javascript
// Frontend (JavaScript)
const { invoke } = window.__TAURI__.core;
await invoke("command_name", { param: value });
```

```rust
// Backend (Rust)
#[tauri::command]
fn command_name(param: String) -> String { ... }
// Register: .invoke_handler(tauri::generate_handler![command_name])
```

## Key Development Patterns

**Adding Rust Commands**: 
1. Define function with `#[tauri::command]` in `src-tauri/src/main.rs`
2. Register in `.invoke_handler(tauri::generate_handler![...])` 
3. Call from JS via `invoke("command_name", { arg: value })`

**Frontend Theme System**:
- Uses CSS variables (`:root` and `[data-theme="dark"]`)
- Colors: `--bg-body`, `--primary-color`, `--accent-color`, etc.
- Applied consistently across calculator interface

**Tauri Configuration**:
- Config file: `src-tauri/tauri.conf.json`
- Frontend build output: `../src` (relative path to dist)
- Permissions managed via `capabilities/default.json` using Tauri 2 ACL system
- Current permissions: window management, opener plugin, webview zoom

## Build & Development Commands

```bash
# Development server
npm run tauri dev

# Production build
npm run tauri build

# Build targets: all platforms (Windows, macOS, Linux, iOS, Android)
```

The build system:
- Runs `tauri-build` during Rust compilation (`build.rs`)
- Compiles Rust library with crate-types: `["staticlib", "cdylib", "rlib"]`
- Windows-specific: lib name suffix `_lib` to avoid name conflicts with binary

## Project-Specific Conventions

- **Korean Localization**: UI text is Korean; app identifier uses reversed domain naming (`com.jhlee.bondcalc`)
- **Window Configuration**: 1000x800 fixed size with zoom hotkeys enabled, resizable
- **Security**: CSP is disabled (`null`), window is pinned via `toggle_pin` command
- **Dependencies**: Minimal - only `tauri`, `serde`/`serde_json`, and `tauri-plugin-opener`

## File Locations Reference

- Frontend logic: `src/main.js` (event listeners, command invocation)
- Frontend templates: `src/index.html` (450 lines, contains embedded styles)
- Rust backend: `src-tauri/src/main.rs`
- Build config: `src-tauri/build.rs`, `src-tauri/Cargo.toml`
- App settings: `src-tauri/tauri.conf.json`
- Permissions: `src-tauri/capabilities/default.json`

## Integration Points

- **Tauri Plugin**: `tauri-plugin-opener` for opening URLs/files (requires `opener:default` permission)
- **Global Tauri API**: Enabled in config (`withGlobalTauri: true`), accessible as `window.__TAURI__`
- **Window Management**: Commands can control always-on-top state via `window.set_always_on_top()`
