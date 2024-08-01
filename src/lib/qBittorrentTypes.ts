/** Torrent states */
export const torrentStates = [
  /** Some error occurred, applies to paused torrents */
  'error',
  /** Torrent data files is missing */
  'missingFiles',
  /** Torrent is being seeded and data is being transferred */
  'uploading',
  /** Torrent is paused and has finished downloading */
  'pausedUP',
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
  /** Torrent is paused and has NOT finished downloading */
  'pausedDL',
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
  'paused',
  'active',
  'inactive',
  'resumed',
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
  hash: string;
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
