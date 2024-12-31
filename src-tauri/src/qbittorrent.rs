use std::collections::HashMap;

use serde::Deserialize;
use specta::Type;
use tauri::{async_runtime::Mutex, State};

use crate::error::Result;

#[derive(Debug, Clone, Deserialize, Type)]
pub struct TorrentInfo {
  /// Time (Unix Epoch) when the torrent was added to the client
  added_on: i64,
  /// Amount of data left to download (bytes)
  amount_left: u64,
  /// Whether this torrent is managed by Automatic Torrent Management
  auto_tmm: bool,
  /// Percentage of file pieces currently available
  availability: f64,
  /// Category of the torrent
  category: String,
  /// Amount of transfer data completed (bytes)
  completed: u64,
  /// Time (Unix Epoch) when the torrent completed
  completion_on: i64,
  /// Torrent download speed limit (bytes/s). -1 if unlimited.
  dl_limit: i64,
  /// Torrent download speed (bytes/s)
  dlspeed: u64,
  /// Amount of data downloaded
  downloaded: u64,
  /// Amount of data downloaded this session
  downloaded_session: u64,
  /// Torrent ETA (seconds)
  eta: i64,
  /// True if first last piece are prioritized
  f_l_piece_prio: bool,
  /// True if force start is enabled for this torrent
  force_start: bool,
  /// Torrent hash
  infohash_v1: String,
  infohash_v2: String,
  /// Last time (Unix Epoch) when a chunk was downloaded/uploaded
  last_activity: i64,
  /// Magnet URI corresponding to this torrent
  magnet_uri: String,
  /// Maximum share ratio until torrent is stopped from seeding/uploading
  max_ratio: f64,
  /// Maximum seeding time (seconds) until torrent is stopped from seeding
  max_seeding_time: i64,
  /// Torrent name
  name: String,
  /// Number of seeds in the swarm
  num_complete: u64,
  /// Number of leechers in the swarm
  num_incomplete: u64,
  /// Number of leechers connected to
  num_leechs: u64,
  /// Number of seeds connected to
  num_seeds: u64,
  /// Torrent priority. Returns -1 if queuing is disabled or torrent is in seed mode
  priority: i64,
  /// Torrent progress (percentage/100)
  progress: f64,
  /// Torrent share ratio. Max ratio value: 9999.
  ratio: f64,
  /// TODO (what is different from max_ratio?)
  ratio_limit: f64,
  /// Path where this torrent's data is stored
  save_path: String,
  /// TODO (what is different from max_seeding_time?)
  seeding_time_limit: i64,
  /// Time (Unix Epoch) when this torrent was last seen complete
  seen_complete: i64,
  /// True if sequential download is enabled
  seq_dl: bool,
  /// Total size (bytes) of files selected for download
  size: u64,
  /// Torrent state
  state: String,
  /// True if super seeding is enabled
  super_seeding: bool,
  /// Comma-concatenated tag list of the torrent
  tags: String,
  /// Total active time (seconds)
  time_active: i64,
  /// Total size (bytes) of all file in this torrent (including unselected ones)
  total_size: u64,
  /// The first tracker with working status. Returns empty string if no tracker is working.
  tracker: String,
  /// Torrent upload speed limit (bytes/s). -1 if unlimited.
  up_limit: i64,
  /// Amount of data uploaded
  uploaded: u64,
  /// Amount of data uploaded this session
  uploaded_session: u64,
  /// Torrent upload speed (bytes/s)
  upspeed: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Type)]
pub enum TorrentContentPriority {
  /// Do not download
  DoNotDownload = 0,
  /// Normal priority
  Normal = 1,
  /// High priority
  High = 6,
  /// Maximal priority
  Maximum = 7,
}

#[derive(Debug, Clone, Type)]
pub struct TorrentContent {
  /// File index
  index: usize,
  /// File name (including relative path)
  name: String,
  /// File size (bytes)
  size: u64,
  /// File progress (percentage/100)
  progress: f32,
  /// File priority
  priority: TorrentContentPriority,
  /// True if file is seeding/complete
  is_seed: bool,
  /// The first number is the starting piece index and the second number is the ending piece index (inclusive)
  piece_range: Vec<usize>,
  /// Percentage of file pieces currently available
  availability: f32,
}

// pub const TORRENT_STATES: &[&str] = &[
//   "error",
//   "missingFiles",
//   "uploading",
//   "stoppedUP",
//   "queuedUP",
//   "stalledUP",
//   "checkingUP",
//   "forcedUP",
//   "allocating",
//   "downloading",
//   "metaDL",
//   "stoppedDL",
//   "queuedDL",
//   "stalledDL",
//   "checkingDL",
//   "forcedDL",
//   "checkingResumeData",
//   "moving",
//   "unknown",
// ];

// pub type TorrentState = &'static str;

#[derive(Debug, Clone, Deserialize, Type)]
pub struct MainData {
  /// Whether the response contains all the data or partial data
  pub full_update: bool,
  /// Response ID
  pub rid: u32,
  /// Global transfer info
  pub server_state: ServerState,
  /// Property: torrent hash, value: same as torrent list
  pub torrents: HashMap<String, TorrentInfo>,
  /// List of hashes of torrents removed since last request
  pub torrents_removed: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Type)]
pub struct ServerState {
  pub alltime_dl: u64,
  pub alltime_ul: u64,
  pub average_time_queue: u64,
  /// Connection status
  pub connection_status: ConnectionStatus,
  /// DHT nodes connected to
  pub dht_nodes: u32,
  /// Data downloaded this session (bytes)
  pub dl_info_data: u64,
  /// Global download rate (bytes/s)
  pub dl_info_speed: u64,
  /// Download rate limit (bytes/s)
  pub dl_rate_limit: u64,
  pub free_space_on_disk: u64,
  pub global_ratio: String,
  pub queued_io_jobs: u32,
  /// True if torrent queueing is enabled
  pub queueing: bool,
  pub read_cache_hits: String,
  pub read_cache_overload: String,
  /// Transfer list refresh interval (milliseconds)
  pub refresh_interval: u64,
  pub total_buffers_size: u64,
  pub total_peer_connections: u32,
  pub total_queued_size: u64,
  pub total_wasted_session: u64,
  /// Data uploaded this session (bytes)
  pub up_info_data: u64,
  /// Global upload rate (bytes/s)
  pub up_info_speed: u64,
  /// Upload rate limit (bytes/s)
  pub up_rate_limit: u64,
  /// True if alternative speed limits are enabled
  pub use_alt_speed_limits: bool,
  pub use_subcategories: bool,
  pub write_cache_overload: String,
}

#[derive(Debug, Clone, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionStatus {
  Connected,
  Firewalled,
  Disconnected,
}

#[derive(Default)]
pub struct QBittorrentStateInner {
  url: String,
}

pub type QBittorrentState = Mutex<QBittorrentStateInner>;

/// 设置 URL
#[tauri::command]
#[specta::specta]
pub async fn set_url(state: State<'_, QBittorrentState>, value: String) -> Result<()> {
  let mut state = state.lock().await;
  state.url = value;
  Ok(())
}
