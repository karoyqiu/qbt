// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_handle;
mod db;
mod error;
mod qbittorrent;
mod scrape;

use db::{get_video_info, has_been_downloaded, mark_as_downloaded, rescrape, DbState};
use log::{error, LevelFilter};
use tauri::{Manager, State};
use tauri_specta::{collect_commands, Builder, ErrorHandlingMode};

use qbittorrent::{
  add_files, add_urls, delete, get_main_data, get_torrent_contents, initialize, login, recheck,
  set_file_priority, start, stop, QBittorrentState,
};

fn main() {
  let builder = Builder::<tauri::Wry>::new()
    // Then register them (separated by a comma)
    .commands(collect_commands![
      add_files,
      add_urls,
      delete,
      get_main_data,
      get_torrent_contents,
      get_video_info,
      has_been_downloaded,
      initialize,
      login,
      mark_as_downloaded,
      recheck,
      rescrape,
      set_file_priority,
      start,
      stop,
    ])
    .error_handling(ErrorHandlingMode::Throw);

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
    .plugin(tauri_plugin_store::Builder::new().build())
    .plugin(
      tauri_plugin_log::Builder::new()
        .clear_targets()
        .target(tauri_plugin_log::Target::new(
          tauri_plugin_log::TargetKind::Stdout,
        ))
        .level(LevelFilter::Warn)
        .level_for("qbt", LevelFilter::Trace)
        .build(),
    )
    .plugin(tauri_plugin_clipboard::init())
    .plugin(tauri_plugin_shell::init())
    .manage(QBittorrentState::default())
    .manage(DbState::default())
    .invoke_handler(builder.invoke_handler())
    .setup(|app| {
      let handle = app.handle();
      app_handle::set_app_handle(handle);

      tauri::async_runtime::block_on(async move {
        let state: State<DbState> = handle.state();
        let mut state = state.lock().await;

        if let Err(e) = state.open(handle).await {
          error!("Failed to open database: {:?}", e);
        }
      });
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
