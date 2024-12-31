/** Main data */
export interface MainData {
  /** Whether the response contains all the data or partial data */
  full_update: boolean;
  /** Response ID */
  rid: number;
  /** Global transfer info */
  server_state: ServerState;
  /** Property: torrent hash, value: same as torrent list */
  torrents: Record<string, TorrentInfo>;
  /** List of hashes of torrents removed since last request */
  torrents_removed?: string[];
}

export interface ServerState {
  alltime_dl: number;
  alltime_ul: number;
  average_time_queue: number;
  /** Connection status */
  connection_status: 'connected' | 'firewalled' | 'disconnected';
  /** DHT nodes connected to */
  dht_nodes: number;
  /** Data downloaded this session (bytes) */
  dl_info_data: number;
  /** Global download rate (bytes/s) */
  dl_info_speed: number;
  /** Download rate limit (bytes/s) */
  dl_rate_limit: number;
  free_space_on_disk: number;
  global_ratio: string;
  queued_io_jobs: number;
  /** True if torrent queueing is enabled */
  queueing: boolean;
  read_cache_hits: string;
  read_cache_overload: string;
  /** Transfer list refresh interval (milliseconds) */
  refresh_interval: number;
  total_buffers_size: number;
  total_peer_connections: number;
  total_queued_size: number;
  total_wasted_session: number;
  /** Data uploaded this session (bytes) */
  up_info_data: number;
  /** Global upload rate (bytes/s) */
  up_info_speed: number;
  /** Upload rate limit (bytes/s) */
  up_rate_limit: number;
  /** True if alternative speed limits are enabled */
  use_alt_speed_limits: boolean;
  use_subcategories: boolean;
  write_cache_overload: string;
}

export const defaultServerState = Object.freeze<ServerState>({
  alltime_dl: 0,
  alltime_ul: 0,
  average_time_queue: 0,
  connection_status: 'disconnected',
  dht_nodes: 0,
  dl_info_data: 0,
  dl_info_speed: 0,
  dl_rate_limit: 0,
  free_space_on_disk: 0,
  global_ratio: '',
  queued_io_jobs: 0,
  queueing: false,
  read_cache_hits: '',
  read_cache_overload: '',
  refresh_interval: 1500,
  total_buffers_size: 0,
  total_peer_connections: 0,
  total_queued_size: 0,
  total_wasted_session: 0,
  up_info_data: 0,
  up_info_speed: 0,
  up_rate_limit: 0,
  use_alt_speed_limits: false,
  use_subcategories: false,
  write_cache_overload: '',
});

export const defaultMainData = Object.freeze<MainData>({
  full_update: false,
  rid: 0,
  server_state: defaultServerState,
  torrents: {},
});

/** Torrent states */
export const torrentStates = [
  /** Some error occurred, applies to paused torrents */
  'error',
  /** Torrent data files is missing */
  'missingFiles',
  /** Torrent is being seeded and data is being transferred */
  'uploading',
  /** Torrent is stopped and has finished downloading */
  'stoppedUP',
  /** Queuing is enabled and torrent is queued for upload */
  'queuedUP',
  /** Torrent is being seeded, but no connection were made */
  'stalledUP',
  /** Torrent has finished downloading and is being checked */
  'checkingUP',
  /** Torrent is forced to uploading and ignore queue limit */
  'forcedUP',
  /** Torrent is allocating disk space for download */
  'allocating',
  /** Torrent is being downloaded and data is being transferred */
  'downloading',
  /** Torrent has just started downloading and is fetching metadata */
  'metaDL',
  /** Torrent is stopped and has NOT finished downloading */
  'stoppedDL',
  /** Queuing is enabled and torrent is queued for download */
  'queuedDL',
  /** Torrent is being downloaded, but no connection were made */
  'stalledDL',
  /** Same as `checkingUP`, but torrent has NOT finished downloading */
  'checkingDL',
  /** Torrent is forced to downloading to ignore queue limit */
  'forcedDL',
  /** Checking resume data on qBt startup */
  'checkingResumeData',
  /** Torrent is moving to another location */
  'moving',
  /** Unknown status */
  'unknown',
] as const;
/** Torrent state */
export type TorrentState = (typeof torrentStates)[number];

export const torrentFilters = [
  'all',
  'downloading',
  'seeding',
  'completed',
  'stopped',
  'active',
  'inactive',
  'running',
  'stalled',
  'stalled_uploading',
  'stalled_downloading',
  'errored',
] as const;
export type TorrentFilter = (typeof torrentFilters)[number];

export interface TorrentInfo {
  /** Time (Unix Epoch) when the torrent was added to the client */
  added_on: number;
  /** Amount of data left to download (bytes) */
  amount_left: number;
  /** Whether this torrent is managed by Automatic Torrent Management */
  auto_tmm: boolean;
  /** Percentage of file pieces currently available */
  availability: number;
  /** Category of the torrent */
  category: string;
  /** Amount of transfer data completed (bytes) */
  completed: number;
  /** Time (Unix Epoch) when the torrent completed */
  completion_on: number;
  /** Torrent download speed limit (bytes/s). -1 if unlimited. */
  dl_limit: number;
  /** Torrent download speed (bytes/s) */
  dlspeed: number;
  /** Amount of data downloaded */
  downloaded: number;
  /** Amount of data downloaded this session */
  downloaded_session: number;
  /** Torrent ETA (seconds) */
  eta: number;
  /** True if first last piece are prioritized */
  f_l_piece_prio: boolean;
  /** True if force start is enabled for this torrent */
  force_start: boolean;
  /** Torrent hash */
  infohash_v1: string;
  infohash_v2: string;
  /** Last time (Unix Epoch) when a chunk was downloaded/uploaded */
  last_activity: number;
  /** Magnet URI corresponding to this torrent */
  magnet_uri: string;
  /** Maximum share ratio until torrent is stopped from seeding/uploading */
  max_ratio: number;
  /** Maximum seeding time (seconds) until torrent is stopped from seeding */
  max_seeding_time: number;
  /** Torrent name */
  name: string;
  /** Number of seeds in the swarm */
  num_complete: number;
  /** Number of leechers in the swarm */
  num_incomplete: number;
  /** Number of leechers connected to */
  num_leechs: number;
  /** Number of seeds connected to */
  num_seeds: number;
  /** Torrent priority. Returns -1 if queuing is disabled or torrent is in seed mode */
  priority: number;
  /** Torrent progress (percentage/100) */
  progress: number;
  /** Torrent share ratio. Max ratio value: 9999. */
  ratio: number;
  /** TODO (what is different from max_ratio?) */
  ratio_limit: number;
  /** Path where this torrent's data is stored */
  save_path: string;
  /** TODO (what is different from max_seeding_time?) */
  seeding_time_limit: number;
  /** Time (Unix Epoch) when this torrent was last seen complete */
  seen_complete: number;
  /** True if sequential download is enabled */
  seq_dl: boolean;
  /** Total size (bytes) of files selected for download */
  size: number;
  /** Torrent state */
  state: TorrentState;
  /** True if super seeding is enabled */
  super_seeding: boolean;
  /** Comma-concatenated tag list of the torrent */
  tags: string;
  /** Total active time (seconds) */
  time_active: number;
  /** Total size (bytes) of all file in this torrent (including unselected ones) */
  total_size: number;
  /** The first tracker with working status. Returns empty string if no tracker is working. */
  tracker: string;
  /** Torrent upload speed limit (bytes/s). -1 if unlimited. */
  up_limit: number;
  /** Amount of data uploaded */
  uploaded: number;
  /** Amount of data uploaded this session */
  uploaded_session: number;
  /** Torrent upload speed (bytes/s) */
  upspeed: number;
}

/* File priority */
export enum TorrentContentPriority {
  /** Do not download */
  DO_NOT_DOWNLOAD = 0,
  /** Normal priority */
  NORMAL = 1,
  /** High priority */
  HIGH = 6,
  /** Maximal priority */
  MAXIMUM = 7,
}

/** Torrent content */
export interface TorrentContent {
  /* File index */
  index: number;
  /* File name (including relative path) */
  name: string;
  /* File size (bytes) */
  size: number;
  /* File progress (percentage/100) */
  progress: number;
  /* File priority */
  priority: TorrentContentPriority;
  /* True if file is seeding/complete */
  is_seed: boolean;
  /* The first number is the starting piece index and the second number is the ending piece index (inclusive) */
  piece_range: number[];
  /* Percentage of file pieces currently available */
  availability: number;
}

export const matchTorrent = (torrent: TorrentInfo, filter: TorrentFilter) => {
  let states: TorrentState[] = [];

  switch (filter) {
    case 'downloading':
      states = [
        'downloading',
        'metaDL',
        'allocating',
        'forcedDL',
        'queuedDL',
        'stalledDL',
        'stoppedDL',
        'checkingDL',
      ];
      break;
    case 'completed':
      states = ['uploading', 'forcedUP', 'queuedUP', 'stalledUP', 'stoppedUP', 'checkingUP'];
      break;
    case 'errored':
      states = ['error', 'missingFiles'];
      break;
    default:
      return true;
  }

  return states.includes(torrent.state);
};
