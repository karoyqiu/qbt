import { Image } from 'primereact/image';
import { Skeleton } from 'primereact/skeleton';
import { useEffect, useState } from 'react';

import { type VideoInfo, commands } from '../lib/bindings';
import ActressAvatar from './ActressAvatar';
import Gallery from './Gallery';

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
  downloadedAt?: number | null;
};

export default function VideoInfoPanel(props: VideoInfoPanelProps) {
  const { loading, videoInfo, downloadedAt } = props;
  const [imgSrc, setImgSrc] = useState('');

  useEffect(() => {
    setImgSrc(videoInfo?.poster ?? videoInfo?.cover ?? '');
  }, [videoInfo]);

  if (!videoInfo) {
    if (loading) {
      return (
        <div className="flex gap-8">
          <Skeleton width="360px" height="520px" />
          <div className="flex flex-col gap-8 grow">
            <div className="flex flex-col gap-4">
              <Skeleton height="1.75rem" />
              <Skeleton />
              <div className="flex gap-16 text-sm">
                <Skeleton width="8rem" />
                <Skeleton width="8rem" />
                <Skeleton width="8rem" />
                <Skeleton width="8rem" />
              </div>
            </div>
            <div className="flex flex-col gap-4">
              <Skeleton height="5rem" />
              <Skeleton height="5rem" />
            </div>
            <div className="grid grid-cols-6 gap-4">
              <div className="flex flex-col gap-2 items-center">
                <Skeleton size="4rem" borderRadius="0.75rem" />
                <Skeleton />
              </div>
              <div className="flex flex-col gap-2 items-center">
                <Skeleton size="4rem" borderRadius="0.75rem" />
                <Skeleton />
              </div>
            </div>
            <div className="grid grid-cols-[6rem_1fr] gap-x-16 gap-y-2 text-sm">
              <Skeleton />
              <Skeleton />
              <Skeleton />
              <Skeleton />
              <Skeleton />
              <Skeleton />
            </div>
          </div>
        </div>
      );
    }

    return <span>No video information.</span>;
  }

  return (
    <div className="flex flex-col gap-8">
      <div className="flex gap-8">
        <Image
          className="shrink-0"
          src={imgSrc}
          width="360"
          referrerPolicy="no-referrer"
          loading="lazy"
          onError={async () => {
            if (imgSrc.startsWith('http')) {
              try {
                const src = await commands.downloadImage(imgSrc);
                setImgSrc(src);
              } catch (e) {
                console.error('Failed to load image', e);
              }
            }
          }}
          preview
        />
        <div className="flex flex-col gap-8 grow">
          <div className="flex flex-col gap-4">
            <h3 className="text-xl font-bold">{videoInfo.title.text}</h3>
            <h3>{videoInfo.title.translated}</h3>
            <div className="flex gap-16 text-sm text-[--text-color-secondary]">
              <span>{videoInfo.code}</span>
              {videoInfo.release_date && <span>{formatDate(videoInfo.release_date)}</span>}
              {videoInfo.duration && <span>{formatMinutes(videoInfo.duration)}</span>}
              {downloadedAt && <span>{`Downloaded at ${formatDate(downloadedAt)}`}</span>}
            </div>
          </div>
          <div className="flex flex-col gap-4">
            <p>{videoInfo.outline?.text}</p>
            <p>{videoInfo.outline?.translated}</p>
          </div>
          {videoInfo.actresses && (
            <div className="grid grid-cols-6 gap-y-4">
              {videoInfo.actresses.map((actress) => (
                <div key={actress.name} className="flex flex-col gap-2 items-center">
                  <ActressAvatar photo={actress.photo} />
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
      {videoInfo.extra_fanart && <Gallery images={videoInfo.extra_fanart} />}
    </div>
  );
}
