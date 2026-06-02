import { zodResolver } from '@hookform/resolvers/zod';
import { PrimeIcons } from 'primereact/api';
import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';
import { FloatLabel } from 'primereact/floatlabel';
import { InputText } from 'primereact/inputtext';
import { useId } from 'react';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

const credentialsSchema = z.object({
  url: z.string().url(),
  apiKey: z.string().min(1),
});

export type Credentials = z.infer<typeof credentialsSchema>;

type ConnectDialogProps = {
  open: boolean;
  onConnect: (data?: Credentials) => Promise<unknown> | unknown;
};

export default function ConnectDialog(props: ConnectDialogProps) {
  const { open, onConnect } = props;
  const form = useForm({
    resolver: zodResolver(credentialsSchema),
    defaultValues: {
      url: '',
      apiKey: '',
    },
  });
  const id = useId();

  return (
    <Dialog header="Connect" visible={open} onHide={() => onConnect()} closable={false}>
      <form className="flex min-w-96 flex-col gap-8 pt-6" onSubmit={form.handleSubmit(onConnect)}>
        <FloatLabel>
          <InputText
            id={`${id}url`}
            className="w-full"
            type="url"
            autoFocus
            required
            {...form.register('url')}
          />
          <label htmlFor={`${id}url`}>URL</label>
        </FloatLabel>
        <FloatLabel>
          <InputText
            id={`${id}key`}
            className="w-full"
            type="password"
            required
            {...form.register('apiKey')}
          />
          <label htmlFor={`${id}key`}>API Key</label>
        </FloatLabel>
        <div className="flex flex-row-reverse">
          <Button
            label="Connect"
            icon={PrimeIcons.SIGN_IN}
            type="submit"
            disabled={form.formState.isSubmitting}
          />
        </div>
      </form>
    </Dialog>
  );
}
