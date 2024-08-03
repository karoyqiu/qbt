import { PrimeIcons } from 'primereact/api';
import { Column } from 'primereact/column';
import { DataTable } from 'primereact/datatable';
import { ProgressBar } from 'primereact/progressbar';
import cn from '../lib/cn';
import { formatPercent, formatSize, formatSpeed } from '../lib/format';
import type { TorrentFilter, TorrentInfo, TorrentState } from '../lib/qBittorrentTypes';

const getStateIcon = (state: TorrentState) => {
  switch (state) {
    case 'allocating':
    case 'downloading':
    case 'metaDL':
    case 'queuedDL':
    case 'stalledDL':
    case 'checkingDL':
    case 'forcedDL':
      return PrimeIcons.DOWNLOAD;
    case 'uploading':
    case 'queuedUP':
    case 'stalledUP':
    case 'checkingUP':
    case 'forcedUP':
      return PrimeIcons.UPLOAD;
    case 'pausedDL':
      return PrimeIcons.PAUSE;
    case 'pausedUP':
      return PrimeIcons.CHECK;
    default:
      return PrimeIcons.TIMES_CIRCLE;
  }
};

type TorrentTableProps = {
  loading?: boolean;
  filter: TorrentFilter;
  torrents: TorrentInfo[];
  selection: TorrentInfo[];
  onSelectionChange: (value: TorrentInfo[]) => void;
  onClick: (hash: string) => void;
};

export default function TorrentTable(props: TorrentTableProps) {
  const { loading, filter, torrents, selection, onSelectionChange, onClick } = props;

  return (
    <div className="min-h-0 grow">
      <DataTable
        loading={loading}
        value={torrents}
        dataKey="hash"
        stripedRows
        scrollable
        scrollHeight="flex"
        selectionMode="checkbox"
        selection={selection}
        onSelectionChange={(e) => onSelectionChange(e.value)}
      >
        <Column selectionMode="multiple" headerClassName="w-0" bodyClassName="w-0" />
        <Column
          field="name"
          header="Name"
          body={(torrent: TorrentInfo) => (
            <span
              className={cn(
                'cursor-pointer text-[--primary-color] underline-offset-4 hover:underline',
                getStateIcon(torrent.state),
              )}
              onClick={() => onClick(torrent.hash)}
            >
              &nbsp;
              {torrent.name}
            </span>
          )}
        />
        <Column
          field="size"
          header="Size"
          align="right"
          bodyClassName="font-mono"
          body={(torrent: TorrentInfo) => formatSize(torrent.size)}
        />
        {filter === 'downloading' && (
          <Column
            field="dlspeed"
            header="Download speed"
            align="right"
            bodyClassName="font-mono"
            body={(torrent: TorrentInfo) => formatSpeed(torrent.dlspeed)}
          />
        )}
        <Column
          field="progress"
          header="Progress"
          align="right"
          bodyClassName="font-mono"
          body={(torrent: TorrentInfo) => (
            <div className="flex flex-col">
              <span>{formatPercent(torrent.progress)}</span>
              <ProgressBar value={torrent.progress * 100} showValue={false} className="h-1" />
            </div>
          )}
        />
        {filter !== 'completed' && (
          <Column
            field="added_on"
            header="Added at"
            body={(torrent: TorrentInfo) => new Date(torrent.added_on * 1000).toLocaleString()}
          />
        )}
        {filter !== 'downloading' && (
          <Column
            field="completion_on"
            header="Completed at"
            body={(torrent: TorrentInfo) =>
              torrent.completion_on > 0
                ? new Date(torrent.completion_on * 1000).toLocaleString()
                : null
            }
          />
        )}
      </DataTable>
    </div>
  );
}
