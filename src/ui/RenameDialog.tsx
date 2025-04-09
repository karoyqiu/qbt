import { PrimeIcons } from 'primereact/api';
import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';
import { InputText } from 'primereact/inputtext';
import { useEffect, useState } from 'react';

type RenameDialogProps = {
  open: boolean;
  onClose: (name?: string) => void;
  originalName: string;
  suggestion?: string;
};

export default function RenameDialog(props: RenameDialogProps) {
  const { open, onClose, originalName, suggestion } = props;
  const [name, setName] = useState(suggestion || originalName);

  useEffect(() => {
    if (open) {
      setName(suggestion || originalName);
    }
  }, [open]);

  return (
    <Dialog
      header="Rename"
      visible={open}
      onHide={onClose}
      footer={
        <div className="pt-6 space-x-4">
          <Button className="p-button-text" label="Cancel" onClick={() => onClose()} />
          <Button label="OK" icon={PrimeIcons.CHECK} onClick={() => onClose(name)} />
        </div>
      }
      dismissableMask
    >
      <InputText
        value={name}
        onChange={(e) => setName(e.target.value)}
        autoFocus
        placeholder={originalName}
      />
    </Dialog>
  );
}
