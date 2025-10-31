import { useForm } from "@inertiajs/react";
import { type FormEvent, memo, useEffect } from "react";
import { toast } from "react-toastify";

import { Alert } from "@/components/alert";
import PublicButton from "@/components/button/public-button";
import Dialog from "@/components/dialog";
import PublicForm from "@/components/form/public-form";
import { appConfig } from "@/config/app";
import type { AuthenticationDialogProps } from "../user-box";

type RegisterFormData = {
  nickname: string;
  password: string;
  passwordRepetition: string;
  verification_code: string;
  error?: string;
};

type Props = {
  verificationMotto: string;
} & AuthenticationDialogProps;

export const RegisterForm = memo(
  ({ children: trigger, open, setDialog, setOpen, verificationMotto }: Props) => {
    const { post, errors, reset, clearErrors, data, setData, setError, processing } =
      useForm<RegisterFormData>();

    function handleFormSubmit(event: FormEvent<HTMLFormElement>) {
      event.preventDefault();
      clearErrors();

      if (data.password !== data.passwordRepetition) {
        setError("passwordRepetition", "As senhas precisam ser iguais.");
        return;
      }

      post("/sessions/register", {
        errorBag: "register",
        onSuccess() {
          setOpen(false);
        },
      });
    }

    useEffect(() => {
      if (!open) {
        reset();
        clearErrors();
      }
    }, [open, clearErrors, reset]);

    return (
      <Dialog.Root open={open} onOpenChange={setOpen}>
        <Dialog.Trigger asChild>{trigger}</Dialog.Trigger>
        <Dialog.Content>
          <Dialog.Header
            title="Registre-se"
            description={`Crie sua própria conta no ${appConfig.appName} de graça!`}
          />

          {errors.error && <Alert type="error" message={errors.error} className="mb-4" />}

          <form onSubmit={handleFormSubmit}>
            <PublicForm.Input
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

            <PublicForm.Input
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

            <PublicForm.Input
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

            <PublicForm.Input
              label="Cole na sua missão"
              type="text"
              value={verificationMotto}
              className="text-input bg-gray-200 cursor-pointer"
              containerClassName="mb-4"
              readOnly
              onClick={(e) => {
                e.preventDefault();

                window.navigator.clipboard.writeText(
                  (e.target as HTMLInputElement).value as string,
                );

                toast("Valor copiado!", { type: "info" });
              }}
              name="register-codigo-verificacao"
              validationError={errors.verification_code}
              onInput={(e) => {
                const value = (e.target as HTMLInputElement).value;
                setData({ ...data, verification_code: value });
              }}
            />

            <div className="flex items-center justify-end gap-2">
              <PublicButton.Ghost
                variant="success"
                size="lg"
                type="button"
                onClick={() => setDialog("login")}>
                Já tenho conta
              </PublicButton.Ghost>

              <PublicButton.Default
                variant="success"
                size="lg"
                type="submit"
                disabled={processing}
                aria-busy={processing}>
                {processing ? "Registrando..." : "Registre-me"}
              </PublicButton.Default>
            </div>
          </form>
        </Dialog.Content>
      </Dialog.Root>
    );
  },
);
