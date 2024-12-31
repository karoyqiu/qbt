// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod qbittorrent;

use qbittorrent::{set_url, QBittorrentState};
use tauri_specta::{collect_commands, Builder};

fn main() {
  let builder = Builder::<tauri::Wry>::new()
    // Then register them (separated by a comma)
    .commands(collect_commands![set_url,]);

  #[cfg(debug_assertions)] // <- Only export on non-release builds
  {
    let lang = specta_typescript::Typescript::new()
      .bigint(specta_typescript::BigIntExportBehavior::Number)
      .header("// @ts-nocheck\n");

    builder
      .export(&lang, "../src/lib/bindings.ts")
      .expect("Failed to export typescript bindings");
  }

  tauri::Builder::default()
    .plugin(tauri_plugin_clipboard_manager::init())
    .manage(QBittorrentState::default())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
