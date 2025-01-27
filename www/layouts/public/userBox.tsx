import { Link, router, usePage } from "@inertiajs/react";
import { ReactNode, useEffect, useState } from "react";
import { toast } from "react-toastify";

import Popover from "@/components/popover";
import { Sprite } from "@/components/sprite";
import { SharedProps } from "@/inertiaShared";
import { FaceGesture, Imager } from "@/utils/imager";

import { LoginForm } from "./forms/login";
import { RegisterForm } from "./forms/register";

type AuthenticationDialog = "login" | "register";
type SuccessfulAuthenticationPossibleFlash = { loginSuccess?: string; registerSuccess?: string };

export type AuthenticationDialogProps = {
  setDialog: (_: AuthenticationDialog) => void;
  children: ReactNode;
  open: boolean;
  setOpen: (_: boolean) => void;
};

export function UserBox() {
  const { auth, flash } = usePage<SharedProps<SuccessfulAuthenticationPossibleFlash>>().props;
  const [dialog, setDialog] = useState<AuthenticationDialog>();

  useEffect(() => {
    if (flash.loginSuccess || flash.registerSuccess) {
      toast(flash.loginSuccess ?? flash.registerSuccess, {
        type: "success",
      });

      delete flash.loginSuccess;
      delete flash.registerSuccess;
    }
  }, [flash]);

  return auth
    ? <Logged user={auth.user} />
    : <Unlogged
        setDialog={setDialog}
        dialog={dialog}
      />;
}

function Unlogged({ dialog, setDialog }: {
  dialog?: AuthenticationDialog;
  setDialog: (_: AuthenticationDialog | undefined) => void;
}) {
  return (
    <div className="flex items-center gap-2">
      <RegisterForm
        open={dialog === "register"}
        setDialog={setDialog}
        setOpen={(v) => setDialog(v
          ? "register"
          : undefined,
        )}
      >
        <button className="
          btn-black text-green-500 text-shadow-extra-1-green-300/25 ring-green-500/40
          "
        >
          <Sprite
            x={-66}
            y={-64}
            width={13}
            height={16}
          />
          Registrar
        </button>

      </RegisterForm>

      <LoginForm
        open={dialog === "login"}
        setDialog={setDialog}
        setOpen={(v) => setDialog(v
          ? "login"
          : undefined,
        )}
      >
        <button className="btn-success btn-lg border-black to-black/25">
          <Sprite
            x={-32}
            y={-65}
            width={15}
            height={15}
          />

          Login
        </button>
      </LoginForm>
    </div>
  );
}

type LoggedProps = {
  user: Exclude<SharedProps["auth"], undefined>["user"];
};

function Logged({ user }: LoggedProps) {
  const habboAvatar = Imager.getUserImage(user.nickname, {
    direction: "3",
    head_direction: "3",
    gesture: FaceGesture.smile,
  });

  function handleLogout() {
    router.post("/sessions/logout");
  }
  return (
    <div className="relative">
      <div
        inert
        style={{ backgroundImage: `url("${habboAvatar}")` }}
        className="w-[64px] h-[110px] absolute left-1/2 -translate-x-1/2 bottom-4"
      />
      <Popover.Root>
        <Popover.Trigger className="
          group flex items-center gap-0 btn-black ring-purple-500 relative
          "
        >
          <Sprite
            x={-152}
            y={-90}
            width={14}
            height={14}
            className="mr-1.5"
          />

          <span className="font-rowdies text-sm font-normal leading-none">
            {user?.nickname}
          </span>

          <Popover.Indicator className="ml-3" />
        </Popover.Trigger>
        <Popover.Content
          side="bottom"
          collisionPadding={24}
        >
          <div className="p-3 text-sm flex gap-3">
            <div className="p-2 bg-white/10 rounded-md grid place-items-center">
              <img
                width="64px"
                height="110px"
                src={habboAvatar}
              />
            </div>

            <div className="flex flex-col gap-1">
              <div className="
                px-3 py-2 bg-black/15 text-white rounded-md border-2 border-black
                "
              >
                <b className="select-none">ID:</b> {user.id}
              </div>

              <div className="
                px-3 py-2 bg-black/15 text-white rounded-md border-2 border-black
                "
              >
                <b className="select-none">Cargo:</b> {user.role}
              </div>

              <div className="w-full h-0.5 bg-black my-2 shadow-white/10 shadow-[0_2px_0_0]" />

              <Link
                href="/"
                className="btn-black font-normal"
              >
                <Sprite
                  x={-201}
                  y={-62}
                  width={20}
                  height={20}
                />
                Configurações
              </Link>
              <button
                className="btn-black font-normal"
                onClick={handleLogout}
              >
                <Sprite
                  x={-224}
                  y={-62}
                  width={20}
                  height={20}
                />
                Logout
              </button>
            </div>
          </div>
        </Popover.Content>
      </Popover.Root>
    </div>
  );
}
