import { Link, router, useForm, usePage } from "@inertiajs/react";
import React, { useEffect, useState } from "react";

import { Alert } from "@/components/alert";
import Dialog from "@/components/dialog";
import { Sprite } from "@/components/sprite";

export function LoginForm() {
  const [backUrl, setBackUrl] = useState("/");
  const [open, setOpen] = useState(false);

  const props = usePage().props;

    type LoginForm = {
      nickname: string;
      password: string;
    };

    const { post, setData, data, errors, transform } = useForm<LoginForm>();

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

    useEffect(() => {
      const handleNavigationChange = () => {
        const url = new URL(window.location.href);
        setOpen(!!url.searchParams.has("login"));

        const _backUrl = url;
        _backUrl.searchParams.delete("login");
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
            title="Login"
            description="Acesse sua conta na Live Cosmic."
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
              <Link
                href="?register"
                className="btn-ghost-success"
              >
                Não tenho conta
              </Link>
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
