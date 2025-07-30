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
  const [downloadedAt, setDownloadedAt] = useState<number | null>(0);
  const loading = status === 'doing';

  useEffect(() => {
    if (code) {
      setStatus('doing');
      Promise.all([commands.hasBeenDownloaded(code, null), commands.getVideoInfo(code)])
        .then(([d, v]) => {
          setDownloadedAt(d);
          setVideoInfo(v);
        })
        .finally(() => setStatus('done'));
    }
  }, [code]);

  return (
    <Dialog
      className="w-[calc(100vw-16rem)] max-w-screen-lg"
      header="Movie Information"
      visible={open}
      onHide={onClose}
      dismissableMask
      footer={
        <div className="mt-4 flex gap-4 justify-end">
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
            text
          />
          <Button
            label="Mark as Downloaded"
            icon={PrimeIcons.DOWNLOAD}
            disabled={!videoInfo || !!downloadedAt}
            onClick={async () => {
              if (videoInfo) {
                const t = Math.floor(Date.now() / 1000);
                await commands.markAsDownloaded(videoInfo.code, '', t);
                setDownloadedAt(t);
              }
            }}
          />
        </div>
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
      <VideoInfoPanel loading={loading} videoInfo={videoInfo} downloadedAt={downloadedAt} />
    </Dialog>
  );
}
