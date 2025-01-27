import { router, usePage } from "@inertiajs/react";
import { ReactNode, useEffect, useState } from "react";
import { toast } from "react-toastify";

import { Sprite } from "@/components/sprite";
import { SharedProps } from "@/inertiaShared";

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
    ? Logged()
    : Unlogged({ dialog, setDialog });
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

function Logged() {
  function handleLogout() {
    router.post("/sessions/logout");
  }
  return (
    <div>
      <button onClick={handleLogout}>logout</button>
    </div>
  );
}
