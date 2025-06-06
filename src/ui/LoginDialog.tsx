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
  username: z.string().min(1),
  password: z.string().min(1),
});

export type Credentials = z.infer<typeof credentialsSchema>;

type LoginDialogProps = {
  open: boolean;
  onLogin: (data?: Credentials) => Promise<unknown> | unknown;
};

export default function LoginDialog(props: LoginDialogProps) {
  const { open, onLogin } = props;
  const form = useForm({
    resolver: zodResolver(credentialsSchema),
    defaultValues: {
      url: '',
      username: '',
      password: '',
    },
  });
  const id = useId();

  return (
    <Dialog header="Login" visible={open} onHide={() => onLogin()} closable={false}>
      <form className="flex min-w-96 flex-col gap-8 pt-6" onSubmit={form.handleSubmit(onLogin)}>
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
            id={`${id}u`}
            className="w-full"
            autoComplete="username"
            required
            {...form.register('username')}
          />
          <label htmlFor={`${id}u`}>Username</label>
        </FloatLabel>
        <FloatLabel>
          <InputText
            id={`${id}p`}
            className="w-full"
            type="password"
            autoComplete="current-password"
            required
            {...form.register('password')}
          />
          <label htmlFor={`${id}p`}>Password</label>
        </FloatLabel>
        <div className="flex flex-row-reverse">
          <Button
            label="Login"
            icon={PrimeIcons.SIGN_IN}
            type="submit"
            disabled={form.formState.isSubmitting}
          />
        </div>
      </form>
    </Dialog>
  );
}
