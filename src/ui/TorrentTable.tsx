import { Column } from 'primereact/column';
import { DataTable } from 'primereact/datatable';
import { ProgressBar } from 'primereact/progressbar';
import type { TorrentFilter, TorrentInfo } from '../lib/qBittorrentTypes';

const threshold = 1024 as const;
const sizeUnits = ['byte', 'kilobyte', 'megabyte', 'gigabyte', 'terabyte', 'petabyte'] as const;
const sizeFormatters = Object.freeze(
  sizeUnits.map((unit) =>
    Intl.NumberFormat(undefined, {
      style: 'unit',
      unit,
      maximumFractionDigits: 2,
    }),
  ),
);
const speedFormatters = Object.freeze(
  sizeUnits.map((unit) =>
    Intl.NumberFormat(undefined, {
      style: 'unit',
      unit: `${unit}-per-second`,
      maximumFractionDigits: 2,
    }),
  ),
);
const percentFormatter = new Intl.NumberFormat(undefined, {
  style: 'percent',
  minimumFractionDigits: 2,
});
const formatSize = (bytes: number, formatters: readonly Intl.NumberFormat[]) => {
  let i = 0;
  let n = bytes;

  while (n > threshold && i < formatters.length) {
    n /= threshold;
    i += 1;
  }

  const formatter = formatters[i];
  return formatter.format(n);
};

type TorrentTableProps = {
  filter: TorrentFilter;
  torrents: TorrentInfo[];
  selection: TorrentInfo[];
  onSelectionChange: (value: TorrentInfo[]) => void;
};

export default function TorrentTable(props: TorrentTableProps) {
  const { filter, torrents, selection, onSelectionChange } = props;

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
        <Column selectionMode="multiple" />
        <Column
          field="name"
          header="Name"
          body={(torrent: TorrentInfo) => (
            <span
              className="hover:cursor-pointer hover:text-[--primary-color] hover:underline"
              onClick={() => console.log(torrent.name)}
            >
              {torrent.name}
            </span>
          )}
        />
        <Column
          field="size"
          header="Size"
          headerClassName="text-end"
          bodyClassName="font-mono text-end"
          body={(torrent: TorrentInfo) => formatSize(torrent.size, sizeFormatters)}
        />
        <Column
          field="completed"
          header="Progress"
          headerClassName="text-end"
          bodyClassName="font-mono text-end"
          body={(torrent: TorrentInfo) => (
            <div className="flex flex-col">
              <span>{percentFormatter.format(torrent.completed / torrent.size)}</span>
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
