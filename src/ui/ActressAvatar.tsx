import { PrimeIcons } from 'primereact/api';
import { Avatar } from 'primereact/avatar';
import { useEffect, useState } from 'react';

import { commands } from '../lib/bindings';

type ActressAvatarProps = {
  photo: string | null;
};

export default function ActressAvatar(props: ActressAvatarProps) {
  const { photo } = props;
  const [src, setSrc] = useState(photo);

  useEffect(() => {
    setSrc(photo);
  }, [photo]);

  if (src) {
    return (
      <div className="shrink-0 size-16 p-avatar">
        <img
          className="rounded-xl"
          src={src}
          onError={async () => {
            setSrc(null);

            if (src.startsWith('http')) {
              try {
                setSrc(await commands.downloadImage(src));
              } catch (e) {}
            }
          }}
        />
      </div>
    );
  }

  return <Avatar className="shrink-0 size-16 text-2xl" icon={PrimeIcons.USER} size="xlarge" />;
}
