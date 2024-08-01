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
import { formatPercent, formatSize } from '../lib/format';
import type { TorrentContent } from '../lib/qBittorrentTypes';

export type TorrentNode = Omit<TreeNode, 'data' | 'children'> & {
  data: TorrentContent;
  children?: TorrentNode[];
};

type TorrentDialogProps = {
  open: boolean;
  onOpenChanged: (value: boolean) => void;
  title: string;
  nodes: TorrentNode[];
  selected: TreeTableSelectionKeysType;
  expanded: TreeTableExpandedKeysType;
};

export default function TorrentDialog(props: TorrentDialogProps) {
  const { open, onOpenChanged, title, nodes, selected, expanded } = props;
  const [expandedKeys, setExpandedKeys] = useState<TreeTableExpandedKeysType>({});

  useEffect(() => setExpandedKeys(expanded), [expanded]);

  return (
    <Dialog
      header={title}
      visible={open}
      onHide={() => onOpenChanged(false)}
      className="w-[calc(100vw-16rem)]"
    >
      <TreeTable
        value={nodes}
        selectionMode="checkbox"
        selectionKeys={selected}
        expandedKeys={expandedKeys}
        onToggle={(e) => setExpandedKeys(e.value)}
        onSelectionChange={(e) => {
          console.log(e.value);
        }}
        sortField="size"
        sortOrder={-1}
      >
        <Column field="name" header="Name" bodyClassName="truncate" expander />
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
