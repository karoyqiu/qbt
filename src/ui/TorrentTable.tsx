import { Column } from 'primereact/column';
import { DataTable } from 'primereact/datatable';
import { ProgressBar } from 'primereact/progressbar';
import { formatPercent, formatSize } from '../lib/format';
import type { TorrentFilter, TorrentInfo } from '../lib/qBittorrentTypes';

type TorrentTableProps = {
  filter: TorrentFilter;
  torrents: TorrentInfo[];
  selection: TorrentInfo[];
  onSelectionChange: (value: TorrentInfo[]) => void;
  onClick: (hash: string) => void;
};

export default function TorrentTable(props: TorrentTableProps) {
  const { filter, torrents, selection, onSelectionChange, onClick } = props;

  return (
    <div className="min-h-0 grow">
      <DataTable
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
              className="cursor-pointer hover:text-[--primary-color] hover:underline"
              onClick={() => onClick(torrent.hash)}
            >
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
        <Column
          field="completed"
          header="Progress"
          align="right"
          bodyClassName="font-mono"
          body={(torrent: TorrentInfo) => (
            <div className="flex flex-col">
              <span>{formatPercent(torrent.completed / torrent.size)}</span>
              <ProgressBar
                value={(torrent.completed * 100) / torrent.size}
                showValue={false}
                className="h-1"
              />
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
