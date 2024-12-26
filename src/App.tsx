import { appWindow } from '@tauri-apps/api/window';
import { PrimeIcons } from 'primereact/api';
import { Button } from 'primereact/button';
import { IconField } from 'primereact/iconfield';
import { InputIcon } from 'primereact/inputicon';
import { InputText } from 'primereact/inputtext';
import { Menubar } from 'primereact/menubar';
import type { MenuItem } from 'primereact/menuitem';
import { Sidebar } from 'primereact/sidebar';
import { TabMenu } from 'primereact/tabmenu';
import type { TreeTableExpandedKeysType, TreeTableSelectionKeysType } from 'primereact/treetable';
import { diff, fork, max, unique } from 'radashi';
import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { useInterval, useLocalStorage, useReadLocalStorage } from 'usehooks-ts';
import { formatSize, formatSpeed } from './lib/format';
import makeTree from './lib/makeTree';
import QBittorrent from './lib/QBittorrent';
import {
  defaultServerState,
  TorrentContentPriority,
  type TorrentFilter,
  type TorrentInfo,
} from './lib/qBittorrentTypes';
import useClipboard from './lib/useClipboard';
import AddDialog from './ui/AddDialog';
import LoginDialog, { type Credentials } from './ui/LoginDialog';
import SettingsDialog from './ui/SettingsDialog';
import TorrentDialog, { TorrentNode } from './ui/TorrentDialog';
import TorrentTable from './ui/TorrentTable';

const collectChildIndexes = (node: TorrentNode) => {
  const indexes: number[] = [];

  if (node.children) {
    for (const child of node.children) {
      if (child.data.index === -1) {
        indexes.push(...collectChildIndexes(child));
      } else {
        indexes.push(child.data.index);
      }
    }
  }

  return indexes;
};

function remove<T>(list: T[], value: T, toKey: (item: T) => number | string | symbol) {
  const key = toKey(value);
  const index = list.findIndex((v) => toKey(v) === key);

  if (index >= 0) {
    list.splice(index, 1);
  }

  return list;
}

function App() {
  const [credentials, setCredentials] = useLocalStorage<Credentials>('credentials', {
    url: '',
    username: '',
    password: '',
  });
  const [showLogin, setShowLogin] = useState(false);
  const [loading, setLoading] = useState(true);
  const [filter, setFilter] = useState<TorrentFilter>('downloading');
  const [search, setSearch] = useState('');
  const [torrents, setTorrents] = useState<TorrentInfo[]>([]);
  const [selected, setSelected] = useState<TorrentInfo[]>([]);
  const [currentHash, setCurrentHash] = useState('');
  const [nodes, setNodes] = useState<TorrentNode[]>([]);
  const [selectedNodes, setSelectedNodes] = useState<TreeTableSelectionKeysType>({});
  const [expanded, setExpanded] = useState<TreeTableExpandedKeysType>({});
  const [showAdd, setShowAdd] = useState(false);
  const [showTorrent, setShowTorrent] = useState(false);
  const [contentLoading, setContentLoading] = useState(true);
  const [showSettings, setShowSettings] = useState(false);
  const [showSidebar, setShowSidebar] = useState(false);
  const [mainData, setMainData] = useState(defaultServerState);
  const refreshInterval = useReadLocalStorage<number>('refreshInterval') ?? 1000;
  const smallFileThreshold = useReadLocalStorage<number>('smallFileThreshold') ?? 200 * 1024 * 1024;
  const watchClipboard = useReadLocalStorage<boolean>('watchClipboard') ?? false;

  const qbt = useRef<QBittorrent>();
  const metas = useRef<TorrentInfo[]>([]);

  const maxRefreshInterval = Math.max(refreshInterval, mainData.refresh_interval);

  const buttons = useMemo<MenuItem[]>(
    () => [
      { label: 'Add', icon: PrimeIcons.PLUS, command: () => setShowAdd(true) },
      {
        label: 'Stop',
        icon: PrimeIcons.STOP,
        disabled: selected.length === 0,
        command: () => qbt.current?.pause(selected.map((s) => s.hash)),
      },
      {
        label: 'Start',
        icon: PrimeIcons.PLAY,
        disabled: selected.length === 0,
        command: () => qbt.current?.resume(selected.map((s) => s.hash)),
      },
      {
        label: 'Delete',
        icon: PrimeIcons.TRASH,
        disabled: selected.length === 0,
        command: () => {
          for (const sel of selected) {
            remove(metas.current, sel, (item) => item.hash);
          }

          qbt.current?.delete(selected.map((s) => s.hash));
        },
      },
      { label: 'Settings', icon: PrimeIcons.COG, command: () => setShowSettings(true) },
    ],
    [selected],
  );
  const tabs = useMemo<MenuItem[]>(
    () => [
      {
        label: 'All',
        icon: PrimeIcons.LIST,
        data: 'all',
      },
      {
        label: 'Downloading',
        icon: PrimeIcons.ARROW_CIRCLE_DOWN,
        data: 'downloading',
      },
      {
        label: 'Completed',
        icon: PrimeIcons.CHECK_CIRCLE,
        data: 'completed',
      },
      {
        label: 'Error',
        icon: PrimeIcons.TIMES_CIRCLE,
        data: 'errored',
      },
    ],
    [],
  );

  const autoSelect = useCallback(
    async (hash: string) => {
      if (!qbt.current) {
        return [];
      }

      const content = await qbt.current.getTorrentContent(hash);
      const [larges, smalls] = fork(content, (item) => item.size >= smallFileThreshold);

      if (larges.length === 0) {
        const largest = max(smalls, (item) => item.size);

        if (largest) {
          larges.push(largest);
          remove(smalls, largest, (item) => item.index);
        }
      }

      const promises = [
        qbt.current.setFilePriority(
          hash,
          larges.map((item) => item.index),
          TorrentContentPriority.NORMAL,
        ),
      ];

      if (smalls.length > 0) {
        promises.push(
          qbt.current.setFilePriority(
            hash,
            smalls.map((item) => item.index),
            TorrentContentPriority.DO_NOT_DOWNLOAD,
          ),
        );
      }

      await Promise.all(promises);

      return [
        ...larges.map((c) => ({ ...c, priority: TorrentContentPriority.NORMAL })),
        ...smalls.map((c) => ({ ...c, priority: TorrentContentPriority.DO_NOT_DOWNLOAD })),
      ];
    },
    [smallFileThreshold],
  );

  const refresh = useCallback(async () => {
    if (!qbt.current?.hasLoggedIn) {
      return;
    }

    const ts = await qbt.current.getTorrentList({
      filter,
      sort: filter === 'completed' ? 'completion_on' : 'added_on',
    });

    setTorrents(ts);
    setLoading(false);

    const hashes = ts.map((item) => item.hash);
    setSelected((old) => old.filter((item) => hashes.includes(item.hash)));

    const newMetas = ts.filter((item) => item.state === 'metaDL');
    const noLongers = diff(metas.current, newMetas, (item) => item.hash);
    const rest = diff(metas.current, noLongers, (item) => item.hash);
    metas.current = unique([...rest, ...newMetas], (item) => item.hash);

    if (noLongers.length > 0) {
      await Promise.all(noLongers.map((m) => autoSelect(m.hash)));
    }
  }, [filter, smallFileThreshold]);

  const select = useCallback(
    async (node: TorrentNode) => {
      let indexes = node.data.index === -1 ? collectChildIndexes(node) : node.data.index;
      await qbt.current?.setFilePriority(currentHash, indexes, TorrentContentPriority.NORMAL);
    },
    [currentHash],
  );

  const unselect = useCallback(
    async (node: TorrentNode) => {
      let indexes = node.data.index === -1 ? collectChildIndexes(node) : node.data.index;
      await qbt.current?.setFilePriority(
        currentHash,
        indexes,
        TorrentContentPriority.DO_NOT_DOWNLOAD,
      );
    },
    [currentHash],
  );

  useEffect(() => {
    appWindow.show();
    appWindow.maximize();
  }, []);

  useEffect(() => {
    qbt.current = new QBittorrent(credentials.url);
    qbt.current
      .login(credentials.username, credentials.password)
      .then((ok) => {
        setShowLogin(!ok);
      })
      .catch((e) => {
        console.error('Failed to login', e);
        setShowLogin(true);
      });
  }, [credentials]);

  useInterval(
    () => {
      refresh().catch(console.error);
    },
    showLogin ? null : maxRefreshInterval,
  );

  useInterval(
    async () => {
      if (qbt.current) {
        const data = await qbt.current.getMainData();
        setMainData(data);
      }
    },
    showSidebar ? maxRefreshInterval : null,
  );

  const onClipboard = useCallback((text: string) => {
    qbt.current?.add(text);
  }, []);

  useClipboard({
    enabled: watchClipboard,
    onTextChange: onClipboard,
  });

  return (
    <div className="flex h-full w-full flex-col">
      <div className="card flex gap-4">
        <Menubar
          className="border-none bg-transparent"
          model={buttons}
          start={<Button icon={PrimeIcons.BARS} text plain onClick={() => setShowSidebar(true)} />}
        />
        <IconField className="grow self-center" iconPosition="left">
          <InputIcon className={PrimeIcons.SEARCH} />
          <InputText
            className="w-full"
            type="search"
            placeholder="Search"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </IconField>
        <TabMenu
          className="pb-1"
          model={tabs}
          activeIndex={tabs.findIndex((tab) => tab.data === filter)}
          onTabChange={(e) => {
            setLoading(true);
            setFilter(e.value.data as TorrentFilter);
          }}
        />
      </div>
      <TorrentTable
        {...{ loading, filter, search, torrents }}
        selection={selected}
        onSelectionChange={setSelected}
        onClick={async (hash) => {
          if (!qbt.current) {
            return;
          }

          setCurrentHash(hash);
          setContentLoading(true);
          setNodes([]);
          setShowTorrent(true);

          const content = await qbt.current.getTorrentContent(hash);
          const { nodes, selected, expanded } = makeTree(content);

          setNodes(nodes);
          setSelectedNodes(selected);
          setExpanded(expanded);
          setContentLoading(false);
        }}
      />
      <LoginDialog
        open={showLogin}
        onLogin={(data) => {
          if (data) {
            setCredentials(data);
          }
        }}
      />
      <AddDialog
        open={showAdd}
        onClose={(urls) => {
          setShowAdd(false);

          if (urls) {
            qbt.current?.add(urls);
          }
        }}
      />
      <TorrentDialog
        open={showTorrent}
        onClose={() => setShowTorrent(false)}
        loading={contentLoading}
        nodes={nodes}
        selected={selectedNodes}
        expanded={expanded}
        onSelectedChange={setSelectedNodes}
        onSelect={select}
        onUnselect={unselect}
        onMagnetToTorrent={async () => {
          setShowTorrent(false);
          await qbt.current?.delete(currentHash);
          await qbt.current?.add(`https://itorrents.org/torrent/${currentHash}.torrent`);
        }}
        onAutoSelect={async () => {
          if (qbt.current) {
            const content = await autoSelect(currentHash);

            const { nodes, selected, expanded } = makeTree(content);
            setNodes(nodes);
            setSelectedNodes(selected);
            setExpanded(expanded);
          }
        }}
      />
      <SettingsDialog open={showSettings} onClose={() => setShowSettings(false)} />
      <Sidebar visible={showSidebar} onHide={() => setShowSidebar(false)}>
        <div className="grid grid-cols-2 gap-y-2">
          <span>DHT nodes</span>
          <span className="text-end font-mono">{mainData.dht_nodes}</span>
          <span>Data downloaded</span>
          <span className="text-end font-mono">{formatSize(mainData.dl_info_data)}</span>
          <span>Download speed</span>
          <span className="text-end font-mono">{formatSpeed(mainData.dl_info_speed)}</span>
          <span>Data uploaded</span>
          <span className="text-end font-mono">{formatSize(mainData.up_info_data)}</span>
          <span>Upload speed</span>
          <span className="text-end font-mono">{formatSpeed(mainData.up_info_speed)}</span>
          <span>Free disk space</span>
          <span className="text-end font-mono">{formatSize(mainData.free_space_on_disk)}</span>
          <span>Speed limited</span>
          <span className="text-end">{mainData.use_alt_speed_limits ? 'Yes' : 'No'}</span>
        </div>
      </Sidebar>
    </div>
  );
}

export default App;
