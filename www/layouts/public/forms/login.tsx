import { useForm } from "@inertiajs/react";
import { type FormEvent, memo, useEffect } from "react";

import { Alert } from "@/components/alert";
import PublicButton from "@/components/button/public-button";
import Dialog from "@/components/dialog";
import { Input } from "@/components/form/input";
import { appConfig } from "@/config/app";
import type { AuthenticationDialogProps } from "../userBox";

type LoginFormData = {
  nickname: string;
  password: string;
};

export const LoginForm = memo(
  ({ open, setOpen, setDialog, children: trigger }: AuthenticationDialogProps) => {
    const { post, setData, data, errors, clearErrors, reset, processing } =
      useForm<LoginFormData>();

    function handleFormSubmit(e: FormEvent<HTMLFormElement>) {
      e.preventDefault();

      clearErrors();

      post("/sessions/login", {
        errorBag: "login",
        onSuccess() {
          setOpen(false);
        },
      });
    }

    useEffect(() => {
      if (!open) {
        clearErrors();
        reset();
      }
    }, [open, clearErrors, reset]);

    return (
      <Dialog.Root open={open} onOpenChange={setOpen}>
        <Dialog.Trigger asChild>{trigger}</Dialog.Trigger>

        <Dialog.Content>
          <Dialog.Header title="Login" description={`Acesse sua conta no ${appConfig.appName}.`} />

          {(errors as Record<string, string>).error && (
            <Alert
              type="error"
              message={(errors as Record<string, string>).error}
              className="mb-4"
            />
          )}

          <form onSubmit={handleFormSubmit}>
            <Input
              name="login-nickname"
              containerClassName="mb-4"
              label="Nickname"
              validationError={errors.nickname}
              type="text"
              placeholder="FãDoFloricultor"
              className="text-input"
              onInput={(e) =>
                setData({
                  ...data,
                  nickname: (e.target as HTMLInputElement).value,
                })
              }
            />

            <Input
              name="login-senha"
              containerClassName="mb-4"
              validationError={errors.password}
              label="Senha"
              type="password"
              placeholder="**********"
              className="text-input"
              onInput={(e) =>
                setData({
                  ...data,
                  password: (e.target as HTMLInputElement).value,
                })
              }
            />

            <div className="flex items-center justify-end gap-2">
              <PublicButton.Ghost
                variant="success"
                size="lg"
                type="button"
                onClick={() => setDialog("register")}>
                Não tenho conta
              </PublicButton.Ghost>

              <PublicButton.Default variant="success" size="lg" type="submit" disabled={processing}>
                {processing ? "Logando..." : "Logar"}
              </PublicButton.Default>
            </div>
          </form>
        </Dialog.Content>
      </Dialog.Root>
    );
  },
);
