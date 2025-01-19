import { router } from "@inertiajs/react";
import { Link } from "@inertiajs/react";
import { useEffect, useState } from "react";
import { toast } from "react-toastify";

import Dialog from "@/components/dialog";
import { Sprite } from "@/components/sprite";

export function RegisterForm() {
  const [backUrl, setBackUrl] = useState("/");
  const [open, setOpen] = useState(false);

  useEffect(() => {
    const handleNavigationChange = () => {
      const url = new URL(window.location.href);
      setOpen(!!url.searchParams.has("register"));

      const _backUrl = url;
      _backUrl.searchParams.delete("register");
      setBackUrl(_backUrl.pathname);
    };

    router.on("navigate", handleNavigationChange);
  }, []);

  return (
    <Dialog.Root
      open={open}
      onOpenChange={() => router.visit(backUrl)}
    >
      <Dialog.Content>
        <Dialog.Header
          title="Registre-se"
          description="Crie sua própria conta na Live Cosmic de graça!"
          customClose={
            <Link
              className="dialog-close-btn"
              href={backUrl}
            >
              <Sprite
                x={-160}
                y={-63}
                width={20}
                height={20}
              />
            </Link>
          }
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
            <Link
              href="?login"
              className="btn-ghost-success"
            >
              Já tenho conta
            </Link>
            <button className="btn-success btn-lg">Registre-me</button>
          </div>
        </form>
      </Dialog.Content>
    </Dialog.Root>
  );
}
