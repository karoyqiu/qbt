mod db_trait;

use log::debug;
use ormlite::{
  sqlite::{SqliteConnectOptions, SqliteConnection, SqliteJournalMode, SqliteSynchronous},
  Connection, Executor, Model, Row, TableMeta,
};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{async_runtime::Mutex, path::BaseDirectory, AppHandle, Manager};

use crate::{
  error::{IntoResult, Result},
  scrape::VideoInfo,
};

const CURRENT_DB_VERSION: u32 = 1;

#[derive(Default)]
pub struct DbStateInner {
  conn: Option<SqliteConnection>,
}

impl db_trait::DbStateTrait for DbStateInner {
  fn connection(&mut self) -> &mut Option<SqliteConnection> {
    &mut self.conn
  }
}

impl DbStateInner {
  pub async fn open(&mut self, app_handle: &AppHandle) -> Result<()> {
    self.conn = Some(open_db(app_handle, "videos").await?);
    self.upgrade().await
  }

  async fn upgrade(&mut self) -> Result<()> {
    let db = self.conn.as_mut().unwrap();
    let version = db.fetch_one("PRAGMA user_version").await.into_result()?;
    let version = version.try_get::<u32, usize>(0).into_result()?;
    debug!("Current db version {}", version);

    if version < CURRENT_DB_VERSION {
      db.transaction(|txn| {
        Box::pin(async move {
          // VideoInfoRecord 表
          let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} ({} TEXT PRIMARY KEY, info TEXT NOT NULL)",
            VideoInfoRecord::table_name(),
            VideoInfoRecord::primary_key().unwrap()
          );
          ormlite::query(&sql).fetch_optional(&mut **txn).await?;

          let sql = format!("PRAGMA user_version = {}", CURRENT_DB_VERSION);
          ormlite::query(&sql).fetch_optional(&mut **txn).await
        })
      })
      .await
      .into_result()?;
    }

    Ok(())
  }
}

pub type DbState = Mutex<DbStateInner>;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Type, Model)]
pub struct VideoInfoRecord {
  #[ormlite(primary_key)]
  pub code: String,
  #[ormlite(json)]
  pub info: VideoInfo,
}

async fn open_db(app_handle: &AppHandle, base_name: &str) -> Result<SqliteConnection> {
  let app_dir = app_handle
    .path()
    .resolve(".", BaseDirectory::AppLocalData)
    .expect("The app data directory should exist.");
  std::fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
  let sqlite_path = app_dir.join(format!("{}.db", base_name));

  let options = SqliteConnectOptions::new()
    .filename(sqlite_path)
    .journal_mode(SqliteJournalMode::Wal)
    .synchronous(SqliteSynchronous::Normal)
    .foreign_keys(true)
    .pragma("temp_store", "MEMORY")
    .pragma("optimize", "0x10002")
    .create_if_missing(true)
    .optimize_on_close(true, None);

  SqliteConnection::connect_with(&options).await.into_result()
}
