import { PrimeIcons } from 'primereact/api';
import { Badge } from 'primereact/badge';
import { Column } from 'primereact/column';
import { DataTable } from 'primereact/datatable';
import { ProgressBar } from 'primereact/progressbar';
import { useEffect, useMemo, useRef } from 'react';

import { type TorrentState, commands } from '../lib/bindings';
import cn from '../lib/cn';
import { formatPercent, formatSize, formatSpeed } from '../lib/format';
import {
  type RequiredTorrentInfo,
  type TorrentFilter,
  matchTorrent,
} from '../lib/qBittorrentTypes';

const getStateIcon = (state: TorrentState) => {
  switch (state) {
    case 'allocating':
    case 'downloading':
    case 'metaDL':
    case 'queuedDL':
    case 'stalledDL':
    case 'checkingDL':
    case 'forcedDL':
      return PrimeIcons.ARROW_CIRCLE_DOWN;
    case 'uploading':
    case 'queuedUP':
    case 'stalledUP':
    case 'checkingUP':
    case 'forcedUP':
      return PrimeIcons.ARROW_CIRCLE_UP;
    case 'stoppedDL':
      return PrimeIcons.STOP_CIRCLE;
    case 'stoppedUP':
      return PrimeIcons.CHECK_CIRCLE;
    default:
      //console.warn(`Unknown state: ${state}`);
      return PrimeIcons.TIMES_CIRCLE;
  }
};

const filterTorrents = (torrents: RequiredTorrentInfo[], filter: TorrentFilter) =>
  torrents.filter((t) => matchTorrent(t, filter));

const adjustCompletionOn = (torrent: RequiredTorrentInfo) =>
  matchTorrent(torrent, 'completed') ? torrent.completion_on : Number.MAX_SAFE_INTEGER;

type TorrentTableProps = {
  loading?: boolean;
  filter: TorrentFilter;
  search?: string;
  torrents: RequiredTorrentInfo[];
  selection: RequiredTorrentInfo[];
  onSelectionChange: (value: RequiredTorrentInfo[]) => void;
  onClick: (hash: string) => void;
};

export default function TorrentTable(props: TorrentTableProps) {
  const { loading, filter, search, torrents, selection, onSelectionChange, onClick } = props;
  const filtered = filterTorrents(torrents, filter).sort((a, b) => {
    const ac = adjustCompletionOn(a);
    const bc = adjustCompletionOn(b);

    if (ac === bc) {
      return a.added_on - b.added_on;
    }

    return ac - bc;
  });

  const columns = useMemo(() => {
    const cols = [];

    if (filter !== 'completed') {
      cols.push(
        <Column
          field="dlspeed"
          header="Download speed"
          align="right"
          bodyClassName="font-mono"
          body={(torrent: RequiredTorrentInfo) => formatSpeed(torrent.dlspeed)}
        />,
        <Column
          field="progress"
          header="Progress"
          align="right"
          bodyClassName="font-mono"
          body={(torrent: RequiredTorrentInfo) => (
            <div className="flex flex-col">
              <span className={torrent.progress === 0 ? 'text-orange-500' : undefined}>
                {formatPercent(torrent.progress)}
              </span>
              <ProgressBar value={torrent.progress * 100} showValue={false} className="h-1" />
            </div>
          )}
        />,
        <Column
          field="added_on"
          header="Added at"
          body={(torrent: RequiredTorrentInfo) =>
            new Date(torrent.added_on * 1000).toLocaleString()
          }
        />,
      );
    }

    if (filter === 'completed') {
      cols.push(
        <Column
          field="completion_on"
          header="Completed at"
          body={(torrent: RequiredTorrentInfo) =>
            torrent.completion_on > 0
              ? new Date(torrent.completion_on * 1000).toLocaleString()
              : null
          }
        />,
      );
    }

    return cols;
  }, [filter]);

  const downloaded = useRef<Record<string, number | null>>({});

  useEffect(() => {
    for (const torrent of torrents) {
      if (matchTorrent(torrent, 'completed')) {
        if (!(torrent.name in downloaded.current)) {
          downloaded.current[torrent.name] = torrent.completion_on;
          commands.markAsDownloaded(torrent.name, torrent.infohash_v1, torrent.completion_on);
        }
      } else if (!(torrent.name in downloaded.current)) {
        commands.hasBeenDownloaded(torrent.name, torrent.infohash_v1).then((value) => {
          downloaded.current[torrent.name] = value;
        });
      }
    }
  }, [torrents]);

  return (
    <div className="min-h-0 grow">
      <DataTable
        loading={loading}
        value={filtered}
        dataKey="infohash_v1"
        stripedRows
        scrollable
        scrollHeight="flex"
        selectionMode="checkbox"
        selection={selection}
        onSelectionChange={(e) => onSelectionChange(e.value)}
        emptyMessage="No torrent"
        globalFilterFields={['name']}
        globalFilter={search}
      >
        <Column selectionMode="multiple" headerClassName="w-0" bodyClassName="w-0" />
        <Column
          field="name"
          header="Name"
          body={(torrent: RequiredTorrentInfo) => (
            <span
              className={cn(
                'cursor-pointer text-[--primary-color] underline-offset-4 hover:underline p-overlay-badge',
                getStateIcon(torrent.state),
              )}
              onClick={() => onClick(torrent.infohash_v1)}
            >
              &nbsp;
              {torrent.name}
              {downloaded.current[torrent.name] && (
                <Badge className="translate-x-3 -translate-y-1/2" severity="info" />
              )}
            </span>
          )}
        />
        <Column
          field="size"
          header="Size"
          align="right"
          bodyClassName="font-mono"
          body={(torrent: RequiredTorrentInfo) => formatSize(torrent.size)}
        />
        {...columns}
      </DataTable>
    </div>
  );
}
