import { join } from '@tauri-apps/api/path';
import { open as shellOpen } from '@tauri-apps/plugin-shell';
import { PrimeIcons } from 'primereact/api';
import { Button } from 'primereact/button';
import { Column } from 'primereact/column';
import { Dialog } from 'primereact/dialog';
import { ProgressBar } from 'primereact/progressbar';
import { TabPanel, TabView } from 'primereact/tabview';
import { TreeNode } from 'primereact/treenode';
import {
  TreeTable,
  type TreeTableExpandedKeysType,
  type TreeTableSelectionKeysType,
} from 'primereact/treetable';
import { useEffect, useState } from 'react';

import { type TorrentContent, VideoInfo, commands } from '../lib/bindings';
import { formatPercent, formatSize } from '../lib/format';
import { useStore } from '../lib/useStore';
import RenameDialog from './RenameDialog';
import VideoInfoPanel from './VideoInfoPanel';

export type TorrentNode = Omit<TreeNode, 'data' | 'children'> & {
  data: TorrentContent & { fullPath: string };
  children?: TorrentNode[];
};

const getSelectedNodes = (
  result: TorrentContent[],
  nodes: TorrentNode[],
  selected: TreeTableSelectionKeysType,
) => {
  for (const node of nodes) {
    if (node.children) {
      getSelectedNodes(result, node.children, selected);
    } else if (node.key) {
      const sel = selected[node.key];

      if (sel === true || (sel && 'checked' in sel && sel.checked)) {
        result.push(node.data);
      }
    }
  }
};

type TorrentDialogProps = {
  open: boolean;
  onClose: () => void;
  loading?: boolean;
  hash: string;
  name: string;
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
    hash,
    name,
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
  const [status, setStatus] = useState<'undone' | 'doing' | 'done'>('undone');
  const [videoInfo, setVideoInfo] = useState<VideoInfo | null>(null);
  const [downloadedAt, setDownloadedAt] = useState<number | null>(0);
  const [tabIndex, setTabIndex] = useState(0);
  const [renameOpen, setRenameOpen] = useState(false);
  const [code, setCode] = useState(name);
  const [localDownloadDir] = useStore<string>('localDownloadDir', '');

  const guessMovieCode = async () => {
    const checked: TorrentContent[] = [];
    getSelectedNodes(checked, nodes, selected);
    checked.sort((a, b) => b.size - a.size);

    for (const sel of checked) {
      const code = await commands.guessMovieCode(sel.name);

      if (code) {
        setCode(code);
        return;
      }
    }

    setCode('');
  };

  const autoRename = async () => {
    if (code) {
      await commands.rename(hash, code);
    }
  };

  useEffect(() => setExpandedKeys(expanded), [expanded]);

  useEffect(() => {
    setStatus('undone');
    setDownloadedAt(null);
    setVideoInfo(null);
    setTabIndex(0);
  }, [nodes]);

  useEffect(() => {
    guessMovieCode();
  }, [name, nodes, selected]);

  return (
    <Dialog
      header={name}
      visible={open}
      onHide={onClose}
      className="w-[calc(100vw-16rem)] max-w-screen-lg"
      footer={
        <div className="pt-6 space-x-4">
          {tabIndex === 0 &&
            (nodes.length === 0 ? (
              <Button label="Magnet to torrent" onClick={onMagnetToTorrent} />
            ) : (
              <>
                <Button label="Rename" onClick={() => setRenameOpen(true)} />
                <Button label="Auto rename" onClick={autoRename} />
                <Button label="Auto select" onClick={onAutoSelect} />
              </>
            ))}
          {tabIndex === 1 && (
            <Button
              label="Re-scrape"
              icon={PrimeIcons.REFRESH}
              loading={status === 'doing'}
              onClick={async () => {
                setStatus('doing');
                setVideoInfo(null);
                setVideoInfo(await commands.rescrape(name));
                setStatus('done');
              }}
            />
          )}
        </div>
      }
      dismissableMask
    >
      <TabView
        onBeforeTabChange={async (e) => {
          setTabIndex(e.index);

          if (e.index === 1 && status === 'undone' && !videoInfo) {
            setStatus('doing');
            const [d, v] = await Promise.all([
              commands.hasBeenDownloaded(name, hash),
              commands.getVideoInfo(name),
            ]);
            setDownloadedAt(d);
            setVideoInfo(v);
            setStatus('done');
          }
        }}
      >
        <TabPanel header="Contents">
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
          <RenameDialog
            open={renameOpen}
            onClose={async (name) => {
              if (name) {
                await commands.rename(hash, name);
              }

              setRenameOpen(false);
            }}
            originalName={name}
            suggestion={code}
          />
        </TabPanel>
        <TabPanel header="Information">
          <VideoInfoPanel
            loading={status === 'doing'}
            videoInfo={videoInfo}
            downloadedAt={downloadedAt}
          />
        </TabPanel>
      </TabView>
    </Dialog>
  );
}
