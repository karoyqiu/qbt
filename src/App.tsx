import { appWindow } from '@tauri-apps/api/window';
import { IconField } from 'primereact/iconfield';
import { InputIcon } from 'primereact/inputicon';
import { InputText } from 'primereact/inputtext';
import { Menubar } from 'primereact/menubar';
import type { MenuItem } from 'primereact/menuitem';
import { TabMenu } from 'primereact/tabmenu';
import { useEffect, useState } from 'react';
import { useInterval, useLocalStorage } from 'usehooks-ts';
import QBittorrent from './lib/QBittorrent';
import { TorrentInfo, type TorrentFilter } from './lib/qBittorrentTypes';
import LoginDialog, { type Credentials } from './ui/LoginDialog';
import TorrentTable from './ui/TorrentTable';

let qbt: QBittorrent;

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

  const buttons: MenuItem[] = [
    { label: 'Add', icon: 'pi pi-plus' },
    { label: 'Pause', icon: 'pi pi-pause' },
    { label: 'Resume', icon: 'pi pi-play' },
    { label: 'Delete', icon: 'pi pi-trash' },
    { label: 'Settings', icon: 'pi pi-cog' },
  ];
  const tabs: MenuItem[] = [
    {
      label: 'All',
      icon: 'pi pi-list',
      data: 'all',
    },
    {
      label: 'Downloading',
      icon: 'pi pi-download',
      data: 'downloading',
    },
    {
      label: 'Completed',
      icon: 'pi pi-check',
      data: 'completed',
    },
  ];

  useEffect(() => {
    appWindow.show();
  }, []);

  useEffect(() => {
    qbt = new QBittorrent(credentials.url);
    qbt
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
    if (qbt) {
      qbt
        .getTorrentList({ filter, sort: filter === 'completed' ? 'completion_on' : 'added_on' })
        .then(setTorrents)
        .catch(console.error);
    }
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
      />
      <LoginDialog
        open={showLogin}
        onLogin={(data) => {
          if (data) {
            setCredentials(data);
          }
        }}
      />
    </div>
  );
}

export default App;
