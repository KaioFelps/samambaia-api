import { toast } from "react-toastify";

import Dialog from "@/components/dialog";

import { AuthenticationDialogProps } from "../userBox";

export function RegisterForm({
  children: trigger,
  open,
  setDialog,
  setOpen,
}: AuthenticationDialogProps) {
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
            />
          </div>

          <div className="mb-4">
            <label
              htmlFor=""
              className="text-sm mb-2 ml-1"
            >
              Repita sua senha
            </label>

            <input
              type="text"
              placeholder="**********"
              className="text-input"
            />
          </div>

          <div className="mb-4">
            <label
              htmlFor=""
              className="text-sm mb-2 ml-1"
            >
              Cole na sua missão
            </label>

            <input
              type="text"
              value="ovjsdovjsoosduhvkusdn2497tgy284hfw@$G#$@fd"
              className="text-input bg-gray-200 cursor-pointer"
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
            />
          </div>

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
