import { useForm, usePage } from "@inertiajs/react";
import { type FormEvent, useEffect } from "react";

import { Alert } from "@/components/alert";
import Dialog from "@/components/dialog";
import { Input } from "@/components/form/input";
import type { SharedProps } from "@/inertiaShared";

import type { AuthenticationDialogProps } from "../userBox";

type LoginFormData = {
  nickname: string;
  password: string;
};

export function LoginForm({
  open,
  setOpen,
  setDialog,
  children: trigger,
}: AuthenticationDialogProps) {
  const props = usePage<SharedProps<{ loginSuccess?: string }>>().props;

  const { post, setData, data, errors, clearErrors, reset, processing } = useForm<LoginFormData>();

  function handleFormSubmit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();

    clearErrors();

    post("/sessions/login", {
      errorBag: "login",
      onSuccess() {
        setTimeout(() => {
          setOpen(false);
        }, 1000);
      },
    });
  }

  useEffect(() => {
    if (!open) {
      clearErrors();
      reset();
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
          title="Login"
          description="Acesse sua conta na Live Cosmic."
        />

        {props.flash.loginSuccess && (
          <Alert
            type="success"
            message={props.flash.loginSuccess}
            className="mb-4"
          />
        )}

        {(errors as Record<string, string>)["error"] && (
          <Alert
            type="error"
            message={(errors as Record<string, string>)["error"]}
            className="mb-4"
          />)}

        <form onSubmit={handleFormSubmit}>
          <Input
            name="login-nickname"
            containerClassName="mb-4"
            label="Nickname"
            validationError={errors.nickname}
            type="text"
            placeholder="FãDoFloricultor"
            className="text-input"
            onInput={(e) => setData({
              ...data,
              nickname: (e.target as HTMLInputElement).value,
            })}
          />

          <Input
            name="login-senha"
            containerClassName="mb-4"
            validationError={errors.password}
            label="Senha"
            type="password"
            placeholder="**********"
            className="text-input"
            onInput={(e) => setData({
              ...data,
              password: (e.target as HTMLInputElement).value,
            })}
          />

          <div className="flex items-center justify-end gap-2">
            <button
              type="button"
              className="btn-ghost-success"
              onClick={() => setDialog("register")}
            >
              Não tenho conta
            </button>
            <button
              className="btn-success btn-lg"
              type="submit"
              disabled={processing}
            >
              {processing
                ? "Logando..."
                : "Logar"}
            </button>
          </div>
        </form>
      </Dialog.Content>
    </Dialog.Root>
  );
}
