import type { UnlistenFn } from '@tauri-apps/api/event';
import { useEffect } from 'react';
import { onTextUpdate, startListening } from 'tauri-plugin-clipboard-api';
import { useDebounceCallback } from 'usehooks-ts';

export const maybeMagnet = (text: string) => /[a-z0-9]{20,50}/i.test(text);

type UseClipboardOptions = {
  enabled?: boolean;
  onTextChange: (text: string) => void;
};

const useClipboard = (options: UseClipboardOptions) => {
  const { enabled = true, onTextChange } = options;
  let unlistenTextUpdate: UnlistenFn;
  let unlistenClipboard: () => Promise<void>;

  const change = useDebounceCallback((text: string) => {
    if (maybeMagnet(text)) {
      onTextChange(text);
    }
  }, 100);

  useEffect(() => {
    if (enabled) {
      console.debug('Start watching clipboard');

      const unlistenFunctions = async () => {
        unlistenTextUpdate = await onTextUpdate(change);
        unlistenClipboard = await startListening();
      };
      unlistenFunctions().catch(console.error);

      return () => {
        console.debug('Stop watching clipboard');

        if (unlistenTextUpdate) {
          unlistenTextUpdate();
        }

        if (unlistenClipboard) {
          unlistenClipboard();
        }
      };
    }
  }, [enabled, onTextChange]);
};

export default useClipboard;
