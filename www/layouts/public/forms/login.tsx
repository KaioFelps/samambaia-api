import { useForm, usePage } from "@inertiajs/react";
import React, { useEffect } from "react";

import { Alert } from "@/components/alert";
import Dialog from "@/components/dialog";

import { AuthenticationDialogProps } from "../userBox";

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
  const props = usePage().props;

  const { post, setData, data, errors, transform } = useForm<LoginFormData>();

  function handleFormSubmit(e: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
    e.preventDefault();
    transform(data => {
      console.log(data);
      return data;
    });

    post("/sessions/login", {
      errorBag: "login",
    });
  }

  useEffect(() => {
    console.log(props);
  }, [props]);

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

        {(errors as Record<string, string>)["error"] && (
          <Alert
            type="warning"
            message={(errors as Record<string, string>)["error"]}
          />)}
        <form>
          <div className="mb-2">
            <label
              htmlFor=""
              className="text-sm mb-2 ml-1"
            >
              Nickname
            </label>

            <input
              type="text"
              placeholder="FãDoFloricultor"
              className="text-input"
              onInput={(e) => setData({
                ...data,
                nickname: (e.target as HTMLInputElement).value,
              })}
            />
          </div>

          <div className="mb-4">
            <label
              htmlFor=""
              className="text-sm mb-2 ml-1"
            >
              Senha
            </label>

            <input
              type="text"
              placeholder="**********"
              className="text-input"
              onInput={(e) => setData({
                ...data,
                password: (e.target as HTMLInputElement).value,
              })}
            />
          </div>

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
              onClick={handleFormSubmit}
            >Logar
            </button>
          </div>
        </form>
      </Dialog.Content>
    </Dialog.Root>
  );
}
