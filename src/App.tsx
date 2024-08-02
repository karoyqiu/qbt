import { appWindow } from '@tauri-apps/api/window';
import { PrimeIcons } from 'primereact/api';
import { IconField } from 'primereact/iconfield';
import { InputIcon } from 'primereact/inputicon';
import { InputText } from 'primereact/inputtext';
import { Menubar } from 'primereact/menubar';
import type { MenuItem } from 'primereact/menuitem';
import { TabMenu } from 'primereact/tabmenu';
import type { TreeTableExpandedKeysType, TreeTableSelectionKeysType } from 'primereact/treetable';
import { diff, fork } from 'radashi';
import { useCallback, useEffect, useRef, useState } from 'react';
import { useInterval, useLocalStorage } from 'usehooks-ts';
import makeTree from './lib/makeTree';
import QBittorrent from './lib/QBittorrent';
import {
  TorrentContentPriority,
  type TorrentFilter,
  type TorrentInfo,
} from './lib/qBittorrentTypes';
import LoginDialog, { type Credentials } from './ui/LoginDialog';
import TorrentDialog, { TorrentNode } from './ui/TorrentDialog';
import TorrentTable from './ui/TorrentTable';

function App() {
  const [credentials, setCredentials] = useLocalStorage<Credentials>('credentials', {
    url: '',
    username: '',
    password: '',
  });
  const [showLogin, setShowLogin] = useState(false);
  const [filter, setFilter] = useState<TorrentFilter>('downloading');
  const [refreshInterval, setRefreshInterval] = useState<number | null>(null);
  const [torrents, setTorrents] = useState<TorrentInfo[]>([]);
  const [selected, setSelected] = useState<TorrentInfo[]>([]);
  const [nodes, setNodes] = useState<TorrentNode[]>([]);
  const [selectedNodes, setSelectedNodes] = useState<TreeTableSelectionKeysType>({});
  const [expanded, setExpanded] = useState<TreeTableExpandedKeysType>({});
  const [showTorrent, setShowTorrent] = useState(false);

  const qbt = useRef<QBittorrent>();
  const metas = useRef<TorrentInfo[]>([]);

  const buttons: MenuItem[] = [
    { label: 'Add', icon: PrimeIcons.PLUS },
    {
      label: 'Pause',
      icon: PrimeIcons.PAUSE,
      disabled: selected.length === 0,
      command: () => qbt.current?.pause(selected.map((s) => s.hash)),
    },
    {
      label: 'Resume',
      icon: PrimeIcons.PLAY,
      disabled: selected.length === 0,
      command: () => qbt.current?.resume(selected.map((s) => s.hash)),
    },
    {
      label: 'Delete',
      icon: PrimeIcons.TRASH,
      disabled: selected.length === 0,
      command: () => qbt.current?.delete(selected.map((s) => s.hash)),
    },
    { label: 'Settings', icon: PrimeIcons.COG },
  ];
  const tabs: MenuItem[] = [
    {
      label: 'All',
      icon: PrimeIcons.LIST,
      data: 'all',
    },
    {
      label: 'Downloading',
      icon: PrimeIcons.DOWNLOAD,
      data: 'downloading',
    },
    {
      label: 'Completed',
      icon: PrimeIcons.CHECK,
      data: 'completed',
    },
  ];

  const refresh = useCallback(async () => {
    if (!qbt.current) {
      return;
    }

    const ts = await qbt.current.getTorrentList({
      filter,
      sort: filter === 'completed' ? 'completion_on' : 'added_on',
    });

    setTorrents(ts);

    const hashes = ts.map((item) => item.hash);
    setSelected((old) => old.filter((item) => hashes.includes(item.hash)));

    const newMetas = ts.filter((v) => v.state === 'metaDL');
    const noLongers = diff(metas.current, newMetas);
    metas.current = newMetas;

    await Promise.all(
      noLongers.map(async (m) => {
        if (!qbt.current) {
          return;
        }

        const content = await qbt.current.getTorrentContent(m.hash);
        const [larges, smalls] = fork(content, (item) => item.size >= 200 * 1024 * 1024);

        await Promise.all([
          qbt.current.setFilePriority(
            m.hash,
            larges.map((item) => item.index),
            TorrentContentPriority.NORMAL,
          ),
          qbt.current.setFilePriority(
            m.hash,
            smalls.map((item) => item.index),
            TorrentContentPriority.DO_NOT_DOWNLOAD,
          ),
        ]);
      }),
    );
  }, [filter]);

  useEffect(() => {
    appWindow.show();
  }, []);

  useEffect(() => {
    qbt.current = new QBittorrent(credentials.url);
    qbt.current
      .login(credentials.username, credentials.password)
      .then((ok) => {
        setShowLogin(!ok);
        setRefreshInterval(1000);
      })
      .catch((e) => {
        console.error('Failed to login', e);
        setShowLogin(true);
        setRefreshInterval(null);
      });
  }, [credentials]);

  useInterval(() => {
    refresh().catch(console.error);
  }, refreshInterval);

  return (
    <div className="flex h-full w-full flex-col">
      <div className="card flex gap-4">
        <Menubar className="border-none bg-transparent" model={buttons} />
        <IconField className="grow self-center" iconPosition="left">
          <InputIcon className="pi pi-search" />
          <InputText className="w-full" type="search" placeholder="Search" />
        </IconField>
        <TabMenu
          className="pb-1"
          model={tabs}
          activeIndex={tabs.findIndex((tab) => tab.data === filter)}
          onTabChange={(e) => setFilter(e.value.data as TorrentFilter)}
        />
      </div>
      <TorrentTable
        filter={filter}
        torrents={torrents}
        selection={selected}
        onSelectionChange={setSelected}
        onClick={async (hash) => {
          if (!qbt.current) {
            return;
          }

          setNodes([]);
          setShowTorrent(true);

          const content = await qbt.current.getTorrentContent(hash);
          const { nodes, selected, expanded } = makeTree(content);

          setNodes(nodes);
          setSelectedNodes(selected);
          setExpanded(expanded);
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
      <TorrentDialog
        open={showTorrent}
        onOpenChanged={setShowTorrent}
        title="Torrent"
        nodes={nodes}
        selected={selectedNodes}
        expanded={expanded}
      />
    </div>
  );
}

export default App;
