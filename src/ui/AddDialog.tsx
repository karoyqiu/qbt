import { PrimeIcons } from 'primereact/api';
import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';
import { InputTextarea } from 'primereact/inputtextarea';
import { useEffect, useRef } from 'react';

type AddDialogProps = {
  open: boolean;
  onClose: (urls?: string) => void;
};

export default function AddDialog(props: AddDialogProps) {
  const { open, onClose } = props;
  const ref = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (open && ref.current) {
      ref.current.value = '';
    }
  }, [open]);

  return (
    <Dialog
      header="Add torrents"
      visible={open}
      onHide={() => onClose()}
      className="w-full max-w-2xl"
      footer={
        <Button label="Add" icon={PrimeIcons.PLUS} onClick={() => onClose(ref.current?.value)} />
      }
      dismissableMask
    >
      <InputTextarea
        ref={ref}
        className="w-full font-mono"
        autoFocus
        autoResize
        rows={5}
        placeholder="Input one URL per line"
      />
    </Dialog>
  );
}
