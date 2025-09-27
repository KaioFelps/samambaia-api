import type { PageProps } from "@inertiajs/core/types";
import { Link, router, usePage } from "@inertiajs/react";
import { type ReactNode, useEffect, useMemo, useState } from "react";
import { toast } from "react-toastify";
import PublicButton from "@/components/button/public-button/index";
import Popover from "@/components/popover";
import { Sprite } from "@/components/sprite";
import { FaceGesture, Imager } from "@/utils/imager";
import { LoginForm } from "./forms/login";
import { RegisterForm } from "./forms/register";

type AuthenticationDialog = "login" | "register";

export type AuthenticationDialogProps = {
  setDialog: (_: AuthenticationDialog) => void;
  children: ReactNode;
  open: boolean;
  setOpen: (_: boolean) => void;
};

export function UserBox() {
  const { auth, flash, verificationMotto } = usePage().props;
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

  return auth ? (
    <Logged user={auth.user} />
  ) : (
    <Unlogged setDialog={setDialog} dialog={dialog} verificationMotto={verificationMotto} />
  );
}

function Unlogged({
  dialog,
  setDialog,
  verificationMotto,
}: {
  dialog?: AuthenticationDialog;
  setDialog: (_: AuthenticationDialog | undefined) => void;
  verificationMotto: string;
}) {
  return (
    <div className="flex items-center gap-2">
      <RegisterForm
        open={dialog === "register"}
        setDialog={setDialog}
        setOpen={(v) => setDialog(v ? "register" : undefined)}
        verificationMotto={verificationMotto}>
        <PublicButton.Default
          variant="black"
          size="lg"
          type="button"
          style={{
            "--btn-bottom-text-shadow":
              "color-mix(in oklab, var(--color-green-300) 25%, var(--color-transparent))",
          }}
          className="text-green-500 ring-green-500/40">
          <Sprite x={-66} y={-64} width={13} height={16} />
          Registrar
        </PublicButton.Default>
      </RegisterForm>

      <LoginForm
        open={dialog === "login"}
        setDialog={setDialog}
        setOpen={(v) => setDialog(v ? "login" : undefined)}>
        <PublicButton.Default
          variant="success"
          size="lg"
          type="button"
          className="border-black"
          style={{
            "--bottom-inner-shadow": "color-mix(in oklab, var(--color-black) 50%, transparent)",
          }}>
          <Sprite x={-32} y={-65} width={15} height={15} />
          Login
        </PublicButton.Default>
      </LoginForm>
    </div>
  );
}

type LoggedProps = {
  user: Exclude<PageProps["auth"], undefined>["user"];
};

function Logged({ user }: LoggedProps) {
  const habboAvatar = useMemo(
    () =>
      Imager.getUserImage(user.nickname, {
        direction: "3",
        head_direction: "3",
        gesture: FaceGesture.smile,
      }),
    [user],
  );

  function handleLogout() {
    router.post("/sessions/logout");
  }
  return (
    <div className="relative">
      <div
        inert
        style={{ backgroundImage: `url("${habboAvatar}")` }}
        className="pixelated w-[90px] h-[110px] absolute left-1/2 -translate-x-1/2 bottom-4"
      />
      <Popover.Root>
        <Popover.Trigger asChild>
          <PublicButton.Default
            variant="black"
            type="button"
            className="group gap-0 ring-purple-500">
            <Sprite x={-152} y={-90} width={14} height={14} className="mr-1.5" />
            <span className="font-rowdies text-sm font-normal leading-none">{user?.nickname}</span>
            <Popover.Indicator className="ml-3" />
          </PublicButton.Default>
        </Popover.Trigger>
        <Popover.Content side="bottom" collisionPadding={24}>
          <div className="p-3 text-sm flex gap-3">
            <div className="p-2 bg-white/10 rounded-md grid place-items-center">
              <img
                className="pixelated object-center"
                width="90px"
                height="130px"
                src={habboAvatar}
                alt="avatar de habbo"
              />
            </div>

            <div className="flex flex-col gap-1">
              <div
                className="
                px-3 py-2 bg-black/15 text-white rounded-md border-2 border-black
                ">
                <b className="select-none">ID:</b> {user.id}
              </div>

              <div
                className="
                px-3 py-2 bg-black/15 text-white rounded-md border-2 border-black
                ">
                <b className="select-none">Cargo:</b> {user.role}
              </div>

              <div className="w-full h-0.5 bg-black my-2 shadow-white/10 shadow-[0_2px_0_0]" />

              <PublicButton.Default asChild variant="black" className="justify-start font-normal">
                <Link href="/">
                  <Sprite x={-201} y={-62} width={20} height={20} />
                  Configurações
                </Link>
              </PublicButton.Default>

              <PublicButton.Default
                variant="black"
                className="justify-start font-normal"
                type="button"
                onClick={handleLogout}>
                <Sprite x={-224} y={-62} width={20} height={20} />
                Logout
              </PublicButton.Default>
            </div>
          </div>
        </Popover.Content>
      </Popover.Root>
    </div>
  );
}
