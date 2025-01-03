import { LazyStore } from '@tauri-apps/plugin-store';
import { useCallback, useEffect, useState } from 'react';

const store = new LazyStore('settings.json', { autoSave: 500 });

export const useStore = <T>(key: string, initialValue: T) => {
  const [value, setValue] = useState<T>(initialValue);

  type Value = { value: T };

  useEffect(() => {
    const migrate = async () => {
      const storedValue = await localStorage.getItem(key);

      if (storedValue) {
        const parsedValue = JSON.parse(storedValue) as T;
        setValue(parsedValue);
        await store.set(key, { value: parsedValue });

        // migrated
        localStorage.removeItem(key);
        return true;
      }

      // no need to migrated
      return false;
    };

    const fetchValue = async () => {
      const storedValue = await store.get<Value>(key);

      if (storedValue) {
        setValue(storedValue.value);
      }
    };

    migrate().then((migrated) => {
      if (!migrated) {
        fetchValue();
      }
    });
  }, [key]);

  const setStoredValue = useCallback(
    async (newValue: T) => {
      setValue(newValue);
      await store.set(key, { value: newValue });
    },
    [key],
  );

  return [value, setStoredValue] as const;
};
