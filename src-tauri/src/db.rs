use log::debug;
use ormlite::{
  query_builder::OnConflict,
  sqlite::{SqliteConnectOptions, SqliteConnection, SqliteJournalMode, SqliteSynchronous},
  Connection, Executor, Model, Row, TableMeta,
};
use tauri::{async_runtime::Mutex, path::BaseDirectory, AppHandle, Manager, State};

use crate::{
  error::{Error, IntoResult, Result},
  scrape::{crawl, get_movie_code, VideoInfo},
};

const CURRENT_DB_VERSION: u32 = 1;

#[derive(Default)]
pub struct DbStateInner {
  conn: Option<SqliteConnection>,
}

impl DbStateInner {
  pub async fn open(&mut self, app_handle: &AppHandle) -> Result<()> {
    self.conn = Some(open_db(app_handle, "videos").await?);
    self.upgrade().await
  }

  async fn upgrade(&mut self) -> Result<()> {
    let db = self
      .conn
      .as_mut()
      .ok_or(Error(anyhow::anyhow!("No connection")))?;
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

  async fn query_one(&mut self, code: &str) -> Result<Option<VideoInfoRecord>> {
    let db = self
      .conn
      .as_mut()
      .ok_or(Error(anyhow::anyhow!("No connection")))?;
    VideoInfoRecord::select()
      .where_bind("code = ?", code)
      .fetch_optional(db)
      .await
      .into_result()
  }

  async fn upsert_one(&mut self, video_info: VideoInfo) -> Result<()> {
    let db = self
      .conn
      .as_mut()
      .ok_or(Error(anyhow::anyhow!("No connection")))?;

    VideoInfoRecord {
      code: video_info.code.clone(),
      info: video_info,
    }
    .insert(db)
    .on_conflict(OnConflict::do_update_on_pkey(
      VideoInfoRecord::primary_key().unwrap(),
    ))
    .await
    .into_result()?;

    Ok(())
  }
}

pub type DbState = Mutex<DbStateInner>;

#[derive(Debug, Default, Clone, Model)]
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

async fn find_video_info(state: &State<'_, DbState>, code: &String) -> Result<Option<VideoInfo>> {
  let mut state = state.lock().await;
  Ok(state.query_one(code).await?.map(|info| info.info))
}

async fn insert_video_info(state: &State<'_, DbState>, video_info: VideoInfo) -> Result<()> {
  let mut state = state.lock().await;
  state.upsert_one(video_info).await
}

#[tauri::command]
#[specta::specta]
pub async fn get_video_info(state: State<'_, DbState>, name: String) -> Result<Option<VideoInfo>> {
  if let Some(code) = get_movie_code(&name) {
    debug!("Movie code: {}", code);

    if let Some(info) = find_video_info(&state, &code).await? {
      return Ok(Some(info));
    }

    let info = crawl(&code).await?;

    if !info.title.text.is_empty() {
      insert_video_info(&state, info.clone()).await?;
      return Ok(Some(info));
    }
  }

  Ok(None)
}
