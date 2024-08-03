import { open as openDialog } from '@tauri-apps/api/dialog';
import { PrimeIcons } from 'primereact/api';
import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';
import { InputNumber } from 'primereact/inputnumber';
import { InputSwitch } from 'primereact/inputswitch';
import { InputText } from 'primereact/inputtext';
import { useId } from 'react';
import { useLocalStorage } from 'usehooks-ts';

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

type SettingsDialogProps = {
  open: boolean;
  onClose: () => void;
};

export default function SettingsDialog(props: SettingsDialogProps) {
  const { open, onClose } = props;
  const [refreshInterval, setRefreshInterval] = useLocalStorage('refreshInterval', 1000);
  const [smallFileThreshold, setSmallFileThreshold] = useLocalStorage(
    'smallFileThreshold',
    200 * 1024 * 1024,
  );
  const [localDownloadDir, setLocalDownloadDir] = useLocalStorage('localDownloadDir', '');
  const [watchClipboard, setWatchClipboard] = useLocalStorage('watchClipboard', false);
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
          <label htmlFor={`${id}ri`}>Refresh interval</label>
          <InputNumber
            id={`${id}ri`}
            autoFocus
            allowEmpty={false}
            inputClassName="w-full font-mono text-end"
            inputMode="numeric"
            suffix={` ${getUnit('millisecond')}`}
            value={refreshInterval}
            onValueChange={(e) => setRefreshInterval(e.value ?? 1000)}
          />
        </div>
        <div className="flex flex-auto flex-col gap-1">
          <label htmlFor={`${id}smt`}>Small file threshold</label>
          <InputNumber
            id={`${id}smt`}
            allowEmpty={false}
            inputClassName="w-full font-mono text-end"
            inputMode="numeric"
            suffix={` ${getUnit('megabyte')}`}
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
