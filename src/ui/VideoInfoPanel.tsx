import { PrimeIcons } from 'primereact/api';
import { Avatar } from 'primereact/avatar';
import { Image } from 'primereact/image';
import { ProgressSpinner } from 'primereact/progressspinner';

import type { VideoInfo } from '../lib/bindings';

const formatDate = (seconds: number) => {
  const date = new Date(seconds * 1000);
  return date.toLocaleDateString();
};

const minutes = new Intl.NumberFormat(undefined, {
  style: 'unit',
  unit: 'minute',
  maximumFractionDigits: 0,
});

const formatMinutes = (seconds: number) => minutes.format(seconds / 60);

type VideoInfoPanelProps = {
  loading: boolean;
  videoInfo: VideoInfo | null;
};

export default function VideoInfoPanel(props: VideoInfoPanelProps) {
  const { loading, videoInfo } = props;

  if (!videoInfo) {
    if (loading) {
      return <ProgressSpinner />;
    }

    return <span>No video information.</span>;
  }

  return (
    <div className="flex gap-8">
      <Image
        className="shrink-0"
        src={videoInfo.poster ?? videoInfo.cover ?? undefined}
        width="360"
        referrerPolicy="no-referrer"
        loading="lazy"
      />
      <div className="flex flex-col gap-8">
        <div className="flex flex-col gap-4">
          <h3 className="text-xl font-bold">{videoInfo.title.text}</h3>
          <h3>{videoInfo.title.translated}</h3>
          <div className="flex gap-16 text-sm text-[--text-color-secondary]">
            {videoInfo.release_date && <span>{formatDate(videoInfo.release_date)}</span>}
            {videoInfo.duration && <span>{formatMinutes(videoInfo.duration)}</span>}
          </div>
        </div>
        <div className="flex flex-col gap-4">
          <p>{videoInfo.outline?.text}</p>
          <p>{videoInfo.outline?.translated}</p>
        </div>
        {videoInfo.actresses && (
          <div className="flex gap-4">
            {videoInfo.actresses.map((actress) => (
              <div key={actress.name} className="flex flex-col gap-2">
                <Avatar
                  className="shrink-0 size-16"
                  icon={PrimeIcons.USER}
                  image={actress.photo ?? undefined}
                  size="xlarge"
                />
                <span>{actress.name}</span>
              </div>
            ))}
          </div>
        )}
        <div className="grid grid-cols-[6rem_1fr] gap-x-16 gap-y-2 text-sm">
          {videoInfo.tags && (
            <>
              <span className="text-[--text-color-secondary]">Tags</span>
              <span>{videoInfo.tags.join(', ')}</span>
            </>
          )}
          {videoInfo.series && (
            <>
              <span className="text-[--text-color-secondary]">Series</span>
              <span>{videoInfo.series}</span>
            </>
          )}
          {videoInfo.director && (
            <>
              <span className="text-[--text-color-secondary]">Director</span>
              <span>{videoInfo.director}</span>
            </>
          )}
          {videoInfo.studio && (
            <>
              <span className="text-[--text-color-secondary]">Studio</span>
              <span>{videoInfo.studio}</span>
            </>
          )}
          {videoInfo.publisher && (
            <>
              <span className="text-[--text-color-secondary]">Publisher</span>
              <span>{videoInfo.publisher}</span>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
