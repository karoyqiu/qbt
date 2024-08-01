import { appWindow } from '@tauri-apps/api/window';
import { IconField } from 'primereact/iconfield';
import { InputIcon } from 'primereact/inputicon';
import { InputText } from 'primereact/inputtext';
import { Menubar } from 'primereact/menubar';
import type { MenuItem } from 'primereact/menuitem';
import { TabMenu } from 'primereact/tabmenu';
import { useEffect } from 'react';
import LoginDialog from './LoginDialog';

function App() {
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
      <LoginDialog open onLogin={() => {}} />
    </>
  );
}

export default App;
