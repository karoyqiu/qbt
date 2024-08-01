import { appWindow } from '@tauri-apps/api/window';
import { IconField } from 'primereact/iconfield';
import { InputIcon } from 'primereact/inputicon';
import { InputText } from 'primereact/inputtext';
import { Menubar } from 'primereact/menubar';
import type { MenuItem } from 'primereact/menuitem';
import { TabMenu } from 'primereact/tabmenu';
import { useEffect, useState } from 'react';
import { useInterval, useLocalStorage } from 'usehooks-ts';
import LoginDialog, { type Credentials } from './LoginDialog';
import QBittorrent from './lib/QBittorrent';

let qbt: QBittorrent;

function App() {
  const [credentials, setCredentials] = useLocalStorage<Credentials>('credentials', {
    url: '',
    username: '',
    password: '',
  });
  const [showLogin, setShowLogin] = useState(false);
  const [refreshInterval, setRefreshInterval] = useState<number | null>(null);

  const buttons: MenuItem[] = [
    { label: 'Add', icon: 'pi pi-plus' },
    { label: 'Pause', icon: 'pi pi-pause' },
    { label: 'Resume', icon: 'pi pi-play' },
    { label: 'Delete', icon: 'pi pi-trash' },
    { label: 'Settings', icon: 'pi pi-cog' },
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
        .getTorrentList({ filter: 'downloading', sort: 'added_on' })
        .then((torrents) => {
          console.debug(torrents);
        })
        .catch(console.error);
    }
  }, refreshInterval);

  return (
    <>
      <div className="card flex gap-4">
        <Menubar className="border-none bg-transparent" model={buttons} />
        <IconField className="grow self-center" iconPosition="left">
          <InputIcon className="pi pi-search" />
          <InputText className="w-full" type="search" placeholder="Search" />
        </IconField>
        <TabMenu
          className="pb-1"
          model={[
            {
              label: 'All',
              icon: 'pi pi-list',
            },
            {
              label: 'Downloading',
              icon: 'pi pi-download',
            },
            {
              label: 'Completed',
              icon: 'pi pi-check',
            },
          ]}
        />
      </div>
      <LoginDialog
        open={showLogin}
        onLogin={(data) => {
          if (data) {
            setCredentials(data);
          }
        }}
      />
    </>
  );
}

export default App;
