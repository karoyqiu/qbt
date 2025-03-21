import { Galleria } from 'primereact/galleria';

import ReloadImage from './ReloadImage';

type GalleryProps = {
  images: string[];
};

export default function Gallery(props: GalleryProps) {
  const { images } = props;

  const itemTemplate = (src: string) => {
    return <ReloadImage className="w-full" imageClassName="w-full" src={src} />;
  };

  const thumbnailTemplate = (src: string) => {
    return (
      <ReloadImage
        className="max-w-32 max-h-32"
        imageClassName="max-w-32 max-h-32"
        src={src}
        preview={false}
        loading="lazy"
      />
    );
  };

  return (
    <Galleria value={images} item={itemTemplate} thumbnail={thumbnailTemplate} numVisible={5} />
  );
}
