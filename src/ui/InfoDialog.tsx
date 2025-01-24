import { PrimeIcons } from 'primereact/api';
import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';
import { useDebounce } from 'primereact/hooks';
import { IconField } from 'primereact/iconfield';
import { InputIcon } from 'primereact/inputicon';
import { InputText } from 'primereact/inputtext';
import { useEffect, useState } from 'react';

import { type VideoInfo, commands } from '../lib/bindings';
import VideoInfoPanel from './VideoInfoPanel';

type InfoDialogProps = {
  open: boolean;
  onClose: () => void;
};

export default function InfoDialog(props: InfoDialogProps) {
  const { open, onClose } = props;
  const [input, code, setInput] = useDebounce('', 500);
  const [status, setStatus] = useState<'undone' | 'doing' | 'done'>('undone');
  const [videoInfo, setVideoInfo] = useState<VideoInfo | null>(null);
  const loading = status === 'doing';

  useEffect(() => {
    if (code) {
      setStatus('doing');
      commands
        .getVideoInfo(code)
        .then(setVideoInfo)
        .then(() => setStatus('done'));
    }
  }, [code]);

  return (
    <Dialog
      className="w-[calc(100vw-16rem)] max-w-screen-lg"
      header="Movie information"
      visible={open}
      onHide={onClose}
      dismissableMask
      footer={
        <Button
          label="Re-scrape"
          icon={PrimeIcons.REFRESH}
          disabled={!code}
          loading={loading}
          onClick={async () => {
            setStatus('doing');
            setVideoInfo(null);
            setVideoInfo(await commands.rescrape(code));
            setStatus('done');
          }}
        />
      }
    >
      <IconField className="grow self-center mb-4" iconPosition="left">
        <InputIcon className={PrimeIcons.SEARCH} />
        <InputText
          autoFocus
          className="w-full"
          type="search"
          placeholder="Input movie code"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          disabled={loading}
        />
      </IconField>
      <VideoInfoPanel loading={loading} videoInfo={videoInfo} />
    </Dialog>
  );
}
