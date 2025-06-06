import { open as openDialog } from '@tauri-apps/plugin-dialog';
import { PrimeIcons } from 'primereact/api';
import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';
import { Dropdown } from 'primereact/dropdown';
import { InputNumber } from 'primereact/inputnumber';
import { InputSwitch } from 'primereact/inputswitch';
import { InputText } from 'primereact/inputtext';
import { useId } from 'react';

import { useStore } from '../lib/useStore';

const getUnit = (unit: string) => {
  const formatter = new Intl.NumberFormat(undefined, {
    style: 'unit',
    unit,
    unitDisplay: 'short',
  });

  const parts = formatter.formatToParts();
  const u = parts.find((p) => p.type === 'unit');
  return u ? u.value : unit;
};
const MB = ` ${getUnit('megabyte')}`;

type SettingsDialogProps = {
  open: boolean;
  onClose: () => void;
};

export default function SettingsDialog(props: SettingsDialogProps) {
  const { open, onClose } = props;
  const [smallFileThreshold, setSmallFileThreshold] = useStore(
    'smallFileThreshold',
    200 * 1024 * 1024,
  );
  const [localDownloadDir, setLocalDownloadDir] = useStore('localDownloadDir', '');
  const [watchClipboard, setWatchClipboard] = useStore('watchClipboard', false);
  const [proxy, setProxy] = useStore('proxy', '<system>');
  const id = useId();

  return (
    <Dialog
      header="Settings"
      visible={open}
      onHide={onClose}
      className="w-full max-w-xl"
      dismissableMask
    >
      <div className="flex flex-col gap-4">
        <div className="flex flex-auto flex-col gap-1">
          <label htmlFor={`${id}smt`}>Small file threshold</label>
          <InputNumber
            id={`${id}smt`}
            allowEmpty={false}
            inputClassName="w-full font-mono text-end"
            inputMode="numeric"
            suffix={MB}
            value={smallFileThreshold / 1024 / 1024}
            onValueChange={(e) => setSmallFileThreshold((e.value ?? 200) * 1024 * 1024)}
          />
        </div>
        <div className="flex flex-auto flex-col gap-1">
          <label htmlFor={`${id}ldd`}>Local download directory</label>
          <div className="p-inputgroup flex-1">
            <InputText
              placeholder="Select a directory"
              value={localDownloadDir}
              onChange={(e) => setLocalDownloadDir(e.target.value)}
            />
            <Button
              icon={PrimeIcons.FOLDER_OPEN}
              onClick={async () => {
                const dir = await openDialog({ directory: true, defaultPath: localDownloadDir });

                if (dir) {
                  setLocalDownloadDir(Array.isArray(dir) ? dir[0] : dir);
                }
              }}
            />
          </div>
        </div>
        <div className="flex flex-auto flex-col gap-1">
          <label htmlFor={`${id}proxy`}>Proxy</label>
          <Dropdown
            value={proxy}
            onChange={(e) => setProxy(e.value)}
            options={[
              { label: 'Use system proxy', value: '<system>' },
              { label: 'No proxy', value: '<direct>' },
            ]}
            optionLabel="label"
            optionValue="value"
            dataKey="value"
            editable
          />
        </div>
        <div className="flex flex-auto items-center justify-between">
          <label htmlFor={`${id}wc`}>Watch clipboard</label>
          <InputSwitch
            id={`${id}wc`}
            checked={watchClipboard}
            onChange={(e) => setWatchClipboard(e.value)}
          />
        </div>
      </div>
    </Dialog>
  );
}
