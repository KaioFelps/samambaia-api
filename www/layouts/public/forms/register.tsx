import { useForm, usePage } from "@inertiajs/react";
import { FormEvent, useEffect } from "react";
import { toast } from "react-toastify";

import { Alert } from "@/components/alert";
import Dialog from "@/components/dialog";
import { Input } from "@/components/form/input";
import { SharedProps } from "@/inertiaShared";

import { AuthenticationDialogProps } from "../userBox";

const confirmationCode = "@livecosmicfs";

type RegisterFormData = {
  nickname: string;
  password: string;
  passwordRepetition: string;
  verification_code: string;
  error?: string;
};

export function RegisterForm({
  children: trigger,
  open,
  setDialog,
  setOpen,
}: AuthenticationDialogProps) {
  const props = usePage<SharedProps<{ registerSuccess?: string }>>().props;
  const { post, errors, reset, clearErrors, data, setData, setError } = useForm<RegisterFormData>();

  function handleFormSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    clearErrors();

    if (data.password !== data.passwordRepetition) {
      setError("passwordRepetition", "As senhas precisam ser iguais.");
      return;
    }

    fetch(`${import.meta.env.VITE_USER_INFO_API}${data.nickname}`)
      .then(res => res.json()
        .then(({ motto }) => {
          if (motto !== confirmationCode) {
            setError(
              "verification_code",
          `Mude sua missão para ${confirmationCode} para registrar-se (atual: "${motto}").`,
            );
          } else {
            post("/sessions/register");
          }
        }));
  }

  useEffect(() => {
    if (!open) {
      reset();
      clearErrors();
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [open]);

  return (
    <Dialog.Root
      open={open}
      onOpenChange={setOpen}
    >
      <Dialog.Trigger>
        {trigger}
      </Dialog.Trigger>
      <Dialog.Content>
        <Dialog.Header
          title="Registre-se"
          description="Crie sua própria conta na Live Cosmic de graça!"
        />

        {props.flash.registerSuccess && (
          <Alert
            type="warning"
            message={props.flash.registerSuccess}
          />
        )}

        {errors.error && (
          <Alert
            type="warning"
            message={errors.error}
          />)}

        <form onSubmit={handleFormSubmit}>
          <Input
            label="Nickname"
            type="text"
            placeholder="FãDoFloricultor"
            className="text-input"
            containerClassName="mb-4"
            name="register-nickname"
            validationError={errors.nickname}
            onInput={(e) => {
              const value = (e.target as HTMLInputElement).value;
              setData({ ...data, nickname: value });
            }}
          />

          <Input
            label="Senha"
            type="password"
            placeholder="**********"
            className="text-input"
            containerClassName="mb-4"
            name="register-senha"
            validationError={errors.password}
            onInput={(e) => {
              const value = (e.target as HTMLInputElement).value;
              setData({ ...data, password: value });
            }}
          />

          <Input
            label="Repita sua senha"
            type="password"
            placeholder="**********"
            className="text-input"
            containerClassName="mb-4"
            name="register-repita-a-senha"
            validationError={errors.passwordRepetition}
            onInput={(e) => {
              const value = (e.target as HTMLInputElement).value;
              setData({ ...data, passwordRepetition: value });
            }}
          />

          <Input
            label="Cole na sua missão"
            type="text"
            value={confirmationCode}
            className="text-input bg-gray-200 cursor-pointer"
            containerClassName="mb-4"
            readOnly
            onClick={(e) => {
              e.preventDefault();

              window.navigator.clipboard.writeText((
                e.target as HTMLInputElement).value as string,
              );

              toast("Valor copiado!", {
                type: "info",
              });
            }}
            name="register-codigo-verificacao"
            validationError={errors.verification_code}
            onInput={(e) => {
              const value = (e.target as HTMLInputElement).value;
              setData({ ...data, verification_code: value });
            }}
          />

          <div className="flex items-center justify-end gap-2">
            <button
              type="button"
              onClick={() => setDialog("login")}
              className="btn-ghost-success"
            >
              Já tenho conta
            </button>
            <button className="btn-success btn-lg">Registre-me</button>
          </div>
        </form>
      </Dialog.Content>
    </Dialog.Root>
  );
}