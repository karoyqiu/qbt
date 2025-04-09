use std::collections::HashMap;

use reqwest::{Client, Proxy, multipart};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_with::skip_serializing_none;
use specta::Type;
use tauri::{State, async_runtime::Mutex};
use url::Url;

use crate::error::{IntoResult, Result};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct TorrentInfo {
  /// Time (Unix Epoch) when the torrent was added to the client
  added_on: Option<i64>,
  /// Amount of data left to download (bytes)
  amount_left: Option<i64>,
  /// Whether this torrent is managed by Automatic Torrent Management
  auto_tmm: Option<bool>,
  /// Percentage of file pieces currently available
  availability: Option<f32>,
  /// Category of the torrent
  category: Option<String>,
  /// Amount of transfer data completed (bytes)
  completed: Option<i64>,
  /// Time (Unix Epoch) when the torrent completed
  completion_on: Option<i64>,
  /// Torrent download speed limit (bytes/s). -1 if unlimited.
  dl_limit: Option<i64>,
  /// Torrent download speed (bytes/s)
  dlspeed: Option<i64>,
  /// Amount of data downloaded
  downloaded: Option<i64>,
  /// Amount of data downloaded this session
  downloaded_session: Option<i64>,
  /// Torrent ETA (seconds)
  eta: Option<i64>,
  /// True if first last piece are prioritized
  f_l_piece_prio: Option<bool>,
  /// True if force start is enabled for this torrent
  force_start: Option<bool>,
  /// Torrent hash
  infohash_v1: Option<String>,
  infohash_v2: Option<String>,
  /// Last time (Unix Epoch) when a chunk was downloaded/uploaded
  last_activity: Option<i64>,
  /// Magnet URI corresponding to this torrent
  magnet_uri: Option<String>,
  /// Maximum share ratio until torrent is stopped from seeding/uploading
  max_ratio: Option<f32>,
  /// Maximum seeding time (seconds) until torrent is stopped from seeding
  max_seeding_time: Option<i64>,
  /// Torrent name
  name: Option<String>,
  /// Number of seeds in the swarm
  num_complete: Option<i64>,
  /// Number of leechers in the swarm
  num_incomplete: Option<i64>,
  /// Number of leechers connected to
  num_leechs: Option<i64>,
  /// Number of seeds connected to
  num_seeds: Option<i64>,
  /// Torrent priority. Returns -1 if queuing is disabled or torrent is in seed mode
  priority: Option<i64>,
  /// Torrent progress (percentage/100)
  progress: Option<f32>,
  /// Torrent share ratio. Max ratio value: 9999.
  ratio: Option<f32>,
  /// TODO (what is different from max_ratio?)
  ratio_limit: Option<f32>,
  /// Path where this torrent's data is stored
  save_path: Option<String>,
  /// TODO (what is different from max_seeding_time?)
  seeding_time_limit: Option<i64>,
  /// Time (Unix Epoch) when this torrent was last seen complete
  seen_complete: Option<i64>,
  /// True if sequential download is enabled
  seq_dl: Option<bool>,
  /// Total size (bytes) of files selected for download
  size: Option<i64>,
  /// Torrent state
  state: Option<TorrentState>,
  /// True if super seeding is enabled
  super_seeding: Option<bool>,
  /// Comma-concatenated tag list of the torrent
  tags: Option<String>,
  /// Total active time (seconds)
  time_active: Option<i64>,
  /// Total size (bytes) of all file in this torrent (including unselected ones)
  total_size: Option<i64>,
  /// The first tracker with working status. Returns empty string if no tracker is working.
  tracker: Option<String>,
  /// Torrent upload speed limit (bytes/s). -1 if unlimited.
  up_limit: Option<i64>,
  /// Amount of data uploaded
  uploaded: Option<i64>,
  /// Amount of data uploaded this session
  uploaded_session: Option<i64>,
  /// Torrent upload speed (bytes/s)
  upspeed: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct TorrentContent {
  /// File index
  index: usize,
  /// File name (including relative path)
  name: String,
  /// File size (bytes)
  size: i64,
  /// File progress (percentage/100)
  progress: f32,
  /// File priority
  priority: i32,
  /// True if file is seeding/complete
  //is_seed: bool,
  /// The first number is the starting piece index and the second number is the ending piece index (inclusive)
  piece_range: Vec<usize>,
  /// Percentage of file pieces currently available
  availability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum TorrentState {
  Error,
  MissingFiles,
  Uploading,
  StoppedUP,
  QueuedUP,
  StalledUP,
  CheckingUP,
  ForcedUP,
  Allocating,
  Downloading,
  MetaDL,
  StoppedDL,
  QueuedDL,
  StalledDL,
  CheckingDL,
  ForcedDL,
  CheckingResumeData,
  Moving,
  Unknown,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ServerState {
  pub alltime_dl: Option<i64>,
  pub alltime_ul: Option<i64>,
  pub average_time_queue: Option<i64>,
  /// Connection status
  pub connection_status: Option<ConnectionStatus>,
  /// DHT nodes connected to
  pub dht_nodes: Option<u32>,
  /// Data downloaded this session (bytes)
  pub dl_info_data: Option<i64>,
  /// Global download rate (bytes/s)
  pub dl_info_speed: Option<i64>,
  /// Download rate limit (bytes/s)
  pub dl_rate_limit: Option<i64>,
  pub free_space_on_disk: Option<i64>,
  pub global_ratio: Option<String>,
  pub queued_io_jobs: Option<u32>,
  /// True if torrent queueing is enabled
  pub queueing: Option<bool>,
  pub read_cache_hits: Option<String>,
  pub read_cache_overload: Option<String>,
  /// Transfer list refresh interval (milliseconds)
  pub refresh_interval: Option<i64>,
  pub total_buffers_size: Option<i64>,
  pub total_peer_connections: Option<u32>,
  pub total_queued_size: Option<i64>,
  pub total_wasted_session: Option<i64>,
  /// Data uploaded this session (bytes)
  pub up_info_data: Option<i64>,
  /// Global upload rate (bytes/s)
  pub up_info_speed: Option<i64>,
  /// Upload rate limit (bytes/s)
  pub up_rate_limit: Option<i64>,
  /// True if alternative speed limits are enabled
  pub use_alt_speed_limits: Option<bool>,
  pub use_subcategories: Option<bool>,
  pub write_cache_overload: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionStatus {
  Connected,
  Firewalled,
  Disconnected,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct MainData {
  /// Whether the response contains all the data or partial data
  #[serde(default)]
  pub full_update: bool,
  /// Response ID
  pub rid: u32,
  /// Global transfer info
  pub server_state: Option<ServerState>,
  /// Property: torrent hash, value: same as torrent list
  pub torrents: Option<HashMap<String, TorrentInfo>>,
  /// List of hashes of torrents removed since last request
  pub torrents_removed: Option<Vec<String>>,
}

#[derive(Default)]
pub struct QBittorrentStateInner {
  url: Option<Url>,
  /// HTTP 客户端
  client: Option<Client>,
  rid: u32,
}

impl QBittorrentStateInner {
  fn get_url(&self, api_name: &str, method_name: &str) -> Result<Url> {
    let url = self.url.as_ref().unwrap();
    let path = format!("/api/v2/{}/{}", api_name, method_name);
    url.join(&path).into_result()
  }

  async fn get<F, T>(&self, api_name: &str, method_name: &str, query: Option<&F>) -> Result<T>
  where
    F: Serialize + ?Sized,
    T: DeserializeOwned,
  {
    let mut url = self.get_url(api_name, method_name)?;

    if let Some(query) = query {
      let query = serde_urlencoded::to_string(query).into_result()?;
      url.set_query(Some(&query));
    }

    let client = self.client.as_ref().unwrap();
    let res = client.get(url).send().await.into_result()?;

    #[cfg(debug_assertions)]
    {
      let text = res.text().await.into_result()?;
      return match serde_json::from_str(&text) {
        Ok(data) => Ok(data),
        Err(e) => {
          log::debug!("Error: {}", e);
          log::debug!("Text: {}", text);
          anyhow::anyhow!(e).into_result()
        }
      };
    }

    #[cfg(not(debug_assertions))]
    res.json().await.into_result()
  }

  async fn post<F: Serialize + ?Sized>(
    &self,
    api_name: &str,
    method_name: &str,
    body: &F,
  ) -> Result<String> {
    let client = self.client.as_ref().unwrap();
    let res = client
      .post(self.get_url(api_name, method_name)?)
      .form(body)
      .send()
      .await
      .into_result()?;
    res.text().await.into_result()
  }
}

pub type QBittorrentState = Mutex<QBittorrentStateInner>;

/// 设置 URL
#[tauri::command]
#[specta::specta]
pub async fn initialize(
  state: State<'_, QBittorrentState>,
  url: String,
  proxy: Option<String>,
) -> Result<()> {
  let mut state = state.lock().await;
  state.url = Some(Url::parse(&url).into_result()?);

  let mut builder = Client::builder().cookie_store(true);

  if let Some(proxy) = proxy {
    if proxy.is_empty() {
      builder = builder.no_proxy();
    } else {
      builder = builder.proxy(Proxy::all(proxy).into_result()?);
    }
  }

  state.client = Some(builder.build().into_result()?);
  Ok(())
}

/// 登录
#[tauri::command]
#[specta::specta]
pub async fn login(
  state: State<'_, QBittorrentState>,
  username: String,
  password: String,
) -> Result<bool> {
  let state = state.lock().await;
  let result = state
    .post(
      "auth",
      "login",
      &[
        ("username", username.as_str()),
        ("password", password.as_str()),
      ],
    )
    .await?;
  Ok(result == "Ok.")
}

/// 获取主要数据
#[tauri::command]
#[specta::specta]
pub async fn get_main_data(state: State<'_, QBittorrentState>) -> Result<MainData> {
  let mut state = state.lock().await;
  let data: MainData = state
    .get(
      "sync",
      "maindata",
      Some(&[("rid", state.rid.to_string().as_str())]),
    )
    .await?;
  state.rid = data.rid;
  Ok(data)
}

/// 获取种子内容
#[tauri::command]
#[specta::specta]
pub async fn get_torrent_contents(
  state: State<'_, QBittorrentState>,
  hash: String,
) -> Result<Vec<TorrentContent>> {
  let state = state.lock().await;
  state
    .get("torrents", "files", Some(&[("hash", hash.as_str())]))
    .await
}

/// 添加链接
#[tauri::command]
#[specta::specta]
pub async fn add_urls(state: State<'_, QBittorrentState>, urls: String) -> Result<()> {
  let state = state.lock().await;
  state
    .post(
      "torrents",
      "add",
      &[("urls", urls.as_str()), ("root_folder", "true")],
    )
    .await?;
  Ok(())
}

/// 添加文件
#[tauri::command]
#[specta::specta]
pub async fn add_files(state: State<'_, QBittorrentState>, paths: Vec<String>) -> Result<()> {
  let mut form = multipart::Form::new()
    .text("paused", "true")
    .text("root_folder", "true");

  for path in paths {
    form = form.file("torrents", path).await.into_result()?;
  }

  let state = state.lock().await;
  let client = state.client.as_ref().unwrap();
  client
    .post(state.get_url("torrents", "add")?)
    .multipart(form)
    .send()
    .await
    .into_result()?;
  Ok(())
}

/// 开始
#[tauri::command]
#[specta::specta]
pub async fn start(state: State<'_, QBittorrentState>, hashes: Vec<String>) -> Result<()> {
  let state = state.lock().await;
  state
    .post(
      "torrents",
      "start",
      &[("hashes", hashes.join("|").as_str())],
    )
    .await?;
  Ok(())
}

/// 停止
#[tauri::command]
#[specta::specta]
pub async fn stop(state: State<'_, QBittorrentState>, hashes: Vec<String>) -> Result<()> {
  let state = state.lock().await;
  state
    .post("torrents", "stop", &[("hashes", hashes.join("|").as_str())])
    .await?;
  Ok(())
}

/// 重新校验
#[tauri::command]
#[specta::specta]
pub async fn recheck(state: State<'_, QBittorrentState>, hashes: Vec<String>) -> Result<()> {
  let state = state.lock().await;
  state
    .post(
      "torrents",
      "recheck",
      &[("hashes", hashes.join("|").as_str())],
    )
    .await?;
  Ok(())
}

/// 删除
#[tauri::command]
#[specta::specta]
pub async fn delete(state: State<'_, QBittorrentState>, hashes: Vec<String>) -> Result<()> {
  let state = state.lock().await;
  state
    .post(
      "torrents",
      "delete",
      &[
        ("hashes", hashes.join("|").as_str()),
        ("deleteFiles", "true"),
      ],
    )
    .await?;
  Ok(())
}

/// 重命名
#[tauri::command]
#[specta::specta]
pub async fn rename(state: State<'_, QBittorrentState>, hash: String, name: String) -> Result<()> {
  let state = state.lock().await;
  state
    .post(
      "torrents",
      "rename",
      &[("hash", hash.as_str()), ("name", name.as_str())],
    )
    .await?;
  Ok(())
}

/// 设置文件优先级
#[tauri::command]
#[specta::specta]
pub async fn set_file_priority(
  state: State<'_, QBittorrentState>,
  hash: String,
  indexes: Vec<usize>,
  priority: i32,
) -> Result<()> {
  let id = indexes
    .into_iter()
    .map(|i| i.to_string())
    .collect::<Vec<String>>()
    .join("|");

  let state = state.lock().await;
  state
    .post(
      "torrents",
      "filePrio",
      &[
        ("hash", hash.as_str()),
        ("id", id.as_str()),
        ("priority", priority.to_string().as_str()),
      ],
    )
    .await?;
  Ok(())
}
