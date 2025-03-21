import { PrimeIcons } from 'primereact/api';
import { Avatar } from 'primereact/avatar';

import ReloadImage from './ReloadImage';

type ActressAvatarProps = {
  photo: string | null;
};

export default function ActressAvatar(props: ActressAvatarProps) {
  const { photo } = props;

  if (photo) {
    return (
      <div className="shrink-0 size-16 p-avatar">
        <ReloadImage src={photo} />
      </div>
    );
  }

  return <Avatar className="shrink-0 size-16 text-2xl" icon={PrimeIcons.USER} size="xlarge" />;
}
