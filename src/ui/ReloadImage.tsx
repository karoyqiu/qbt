import { Image, type ImageProps } from 'primereact/image';
import { useEffect, useState } from 'react';

import { commands } from '../lib/bindings';

export default function ReloadImage(props: ImageProps) {
  const { src, onError, ...rest } = props;
  const [realSrc, setRealSrc] = useState(src);

  useEffect(() => setRealSrc(src), [src]);

  return (
    <Image
      preview
      {...rest}
      src={realSrc}
      onError={async (event) => {
        if (realSrc?.startsWith('http')) {
          try {
            setRealSrc(await commands.downloadImage(realSrc));
          } catch (e) {}
        }

        onError?.(event);
      }}
    />
  );
}
