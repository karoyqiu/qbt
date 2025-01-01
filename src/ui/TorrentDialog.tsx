import { join } from '@tauri-apps/api/path';
import { open as shellOpen } from '@tauri-apps/plugin-shell';
import { Button } from 'primereact/button';
import { Column } from 'primereact/column';
import { Dialog } from 'primereact/dialog';
import { ProgressBar } from 'primereact/progressbar';
import { TreeNode } from 'primereact/treenode';
import {
  TreeTable,
  type TreeTableExpandedKeysType,
  type TreeTableSelectionKeysType,
} from 'primereact/treetable';
import { useEffect, useState } from 'react';
import { useReadLocalStorage } from 'usehooks-ts';

import { type TorrentContent, commands } from '../lib/bindings';
import { formatPercent, formatSize } from '../lib/format';

export type TorrentNode = Omit<TreeNode, 'data' | 'children'> & {
  data: TorrentContent & { fullPath: string };
  children?: TorrentNode[];
};

type TorrentDialogProps = {
  open: boolean;
  onClose: () => void;
  loading?: boolean;
  nodes: TorrentNode[];
  selected: TreeTableSelectionKeysType;
  expanded: TreeTableExpandedKeysType;
  onSelectedChange: (value: TreeTableSelectionKeysType) => void;
  onSelect: (node: TorrentNode) => void;
  onUnselect: (node: TorrentNode) => void;
  onMagnetToTorrent: () => void;
  onAutoSelect: () => void;
};

export default function TorrentDialog(props: TorrentDialogProps) {
  const {
    open,
    onClose,
    loading,
    nodes,
    selected,
    expanded,
    onSelectedChange,
    onSelect,
    onUnselect,
    onMagnetToTorrent,
    onAutoSelect,
  } = props;
  const [expandedKeys, setExpandedKeys] = useState<TreeTableExpandedKeysType>({});
  const localDownloadDir = useReadLocalStorage<string>('localDownloadDir');

  useEffect(() => setExpandedKeys(expanded), [expanded]);

  useEffect(() => {
    if (nodes.length > 0) {
      commands.scrape(nodes[0].data.name);
    }
  }, [nodes]);

  return (
    <Dialog
      header="Torrent"
      visible={open}
      onHide={onClose}
      className="w-[calc(100vw-16rem)] max-w-screen-lg"
      footer={
        <div className="pt-6">
          {nodes.length === 0 ? (
            <Button label="Magnet to torrent" onClick={onMagnetToTorrent} />
          ) : (
            <Button label="Auto select" onClick={onAutoSelect} />
          )}
        </div>
      }
      dismissableMask
    >
      <TreeTable
        loading={loading}
        value={nodes}
        selectionMode="checkbox"
        selectionKeys={selected}
        expandedKeys={expandedKeys}
        onToggle={(e) => setExpandedKeys(e.value)}
        onSelectionChange={(e) => {
          if (typeof e.value !== 'string') {
            onSelectedChange(e.value);
          }
        }}
        onSelect={(e) => onSelect(e.node as TorrentNode)}
        onUnselect={(e) => onUnselect(e.node as TorrentNode)}
        sortField="size"
        sortOrder={-1}
        emptyMessage="No content"
      >
        <Column
          field="name"
          header="Name"
          bodyClassName="truncate"
          expander
          body={(node: TorrentNode) => {
            if (!localDownloadDir || node.data.progress < 1) {
              return node.data.name;
            }

            return (
              <span
                className="cursor-pointer text-[--primary-color] underline-offset-4 hover:underline"
                onClick={async () => {
                  const path = await join(localDownloadDir, node.data.fullPath);
                  await shellOpen(path);
                }}
              >
                {node.data.name}
              </span>
            );
          }}
        />
        <Column
          field="size"
          align="right"
          alignHeader="right"
          header="Size"
          headerClassName="w-32"
          bodyClassName="font-mono w-32"
          body={(node: TorrentNode) => formatSize(node.data.size)}
        />
        <Column
          field="progress"
          align="right"
          alignHeader="right"
          header="Progress"
          headerClassName="w-32"
          bodyClassName="font-mono w-32"
          body={(node: TorrentNode) => (
            <div className="flex flex-col">
              <span>{formatPercent(node.data.progress)}</span>
              <ProgressBar value={node.data.progress * 100} showValue={false} className="h-1" />
            </div>
          )}
        />
      </TreeTable>
    </Dialog>
  );
}
