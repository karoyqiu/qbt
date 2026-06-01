# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

qbt is a Tauri 2 desktop client for qBittorrent. Frontend is React 19 + TypeScript + PrimeReact + Tailwind CSS. Backend is Rust with Tauri commands. The app manages torrents and scrapes video metadata from external sites.

## Build Commands

```bash
yarn dev              # Vite dev server (port 1420), frontend only
yarn tauri dev        # Full Tauri dev (Rust + frontend hot reload)
yarn build            # TypeScript check + Vite production build (frontend only)
yarn tauri build      # Full production build (Rust + frontend, creates installer)
```

No test runner configured. No lint script in package.json — ESLint and Prettier are configured via `.eslintrc.cjs` and `.prettierrc.cjs` but must be run via editor or CLI directly.

## Architecture

### Tauri Command Binding (specta)

Rust commands (`#[tauri::command] #[specta::specta]`) are registered in `src-tauri/src/main.rs` via `collect_commands![]`. In debug builds, specta auto-generates TypeScript bindings to `src/lib/bindings.ts`. **Never edit `bindings.ts` manually** — it's regenerated on each `tauri dev` / `tauri build`.

To add a new command:
1. Write the Rust function with `#[tauri::command] #[specta::specta]` in the appropriate module
2. Add it to `collect_commands![]` in `main.rs`
3. Run `yarn tauri dev` to regenerate `bindings.ts`
4. Call `commands.yourFunction()` from frontend

### Backend Modules (`src-tauri/src/`)

| Module | Purpose |
|--------|---------|
| `main.rs` | Tauri app setup, command registration, plugin init, DB setup |
| `qbittorrent.rs` | qBittorrent WebUI API client (login, torrents, file priorities) |
| `db.rs` | SQLite via ormlite — stores `VideoInfo` records, schema migration |
| `scrape/` | Web scraping — movie code detection, metadata crawling, image download |
| `scrape/crawlers/` | Site-specific crawler implementations (each site = one file) |
| `error.rs` | `Error(anyhow::Error)` wrapper, `IntoResult` trait for conversion |

### Frontend Structure (`src/`)

| Path | Purpose |
|------|---------|
| `App.tsx` | Main component — login flow, torrent list, polling, auto-select, clipboard watch |
| `lib/bindings.ts` | Auto-generated Tauri command types (do not edit) |
| `lib/qBittorrentTypes.ts` | Torrent data types, filter logic, `mergeMainData` incremental update |
| `lib/useStore.ts` | Persistent settings via Tauri plugin-store |
| `lib/makeTree.ts` | Converts flat torrent file list to PrimeReact TreeTable nodes |
| `ui/TorrentTable.tsx` | Main torrent list view |
| `ui/TorrentDialog.tsx` | File selection tree for a single torrent |
| `ui/InfoDialog.tsx` | Video metadata panel with scraped info |
| `ui/AddDialog.tsx` | Add torrent by URL/magnet |

### Key Patterns

- **Incremental data sync**: `getMainData` returns deltas, merged client-side via `mergeMainData`. Polling interval comes from server state.
- **Auto-select**: When torrent metadata finishes downloading (`metaDL` state ends), files above `smallFileThreshold` are auto-selected, rest skipped.
- **Crawler architecture**: Two base traits — `Crawler` (HTTP-based via `reqwest`+`scraper`) and `CrawlerCDP` (headless Chrome via `headless_chrome` crate). Each site implements one of these. Results merge via `VideoInfo::apply`.
- **Image caching**: Scraped images stored as base64 data URIs in an in-memory `quick_cache` keyed by URL.
- **Error handling**: Rust uses custom `Error(anyhow::Error)` that serializes to string for Tauri. `IntoResult` trait converts any `Result<T, E>` where `E: Into<anyhow::Error>`.
- **State management**: Tauri managed state — `QBittorrentState` (reqwest client + cookies) and `DbState` (SQLite connection), both wrapped in `async_runtime::Mutex`.

## Development Notes

- Package manager: Yarn 4 (configured via `packageManager` in package.json)
- Rust edition 2024, MSRV follows Tauri 2 requirements
- Release profile uses `lto = "fat"`, `opt-level = "s"`, `strip = "symbols"` for small binary
- TypeScript strict mode with `noUnusedLocals` and `noUnusedParameters`
- UI comments and domain terminology are in Chinese (番号 = movie code)
