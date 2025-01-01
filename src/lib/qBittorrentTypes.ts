import { assign } from 'radashi';

import type { MainData, ServerState, TorrentInfo, TorrentState } from './bindings';

type NonNullableFields<T> = {
  [P in keyof T]: NonNullable<T[P]>;
};

export type RequiredServerState = NonNullableFields<Required<ServerState>>;
export type RequiredTorrentInfo = NonNullableFields<Required<TorrentInfo>>;

export type RequiredMainData = {
  full_update: boolean;
  rid: number;
  server_state: RequiredServerState;
  torrents: Record<string, RequiredTorrentInfo>;
  torrents_removed: string[];
};

export const defaultServerState = Object.freeze<RequiredServerState>({
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

export const defaultMainData = Object.freeze<RequiredMainData>({
  full_update: false,
  rid: 0,
  server_state: defaultServerState,
  torrents: {},
  torrents_removed: [],
});

export const TorrentContentPriority = {
  DO_NOT_DOWNLOAD: 0,
  NORMAL: 1,
  HIGH: 6,
  MAXIMUM: 7,
} as const;

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

export const matchTorrent = (torrent: RequiredTorrentInfo, filter: TorrentFilter) => {
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

export const mergeMainData = (data: RequiredMainData, delta: MainData) => {
  let merged = assign(data, delta) as RequiredMainData;
  merged.torrents_removed = delta.torrents_removed ?? [];
  return merged;
};

export const getInfoHash = (torrent: RequiredTorrentInfo) => torrent.infohash_v1;
export const getInfoHashes = (torrents: RequiredTorrentInfo[]) => torrents.map(getInfoHash);
