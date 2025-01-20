import { router, usePage } from "@inertiajs/react";
import { ReactNode, useState } from "react";

import { Sprite } from "@/components/sprite";
import { SharedProps } from "@/inertiaShared";

import { LoginForm } from "./forms/login";
import { RegisterForm } from "./forms/register";

export function UserBox() {
  const { auth } = usePage<SharedProps>().props;

  return auth
    ? Logged()
    : Unlogged();
}

type AuthenticationDialog = "login" | "register";

export type AuthenticationDialogProps = {
  setDialog: (_: AuthenticationDialog) => void;
  children: ReactNode;
  open: boolean;
  setOpen: (_: boolean) => void;
};

function Unlogged() {
  const [dialog, setDialog] = useState<AuthenticationDialog>();

  return (
    <>
      <div className="flex items-center gap-2">
        <RegisterForm
          open={dialog === "register"}
          setDialog={setDialog}
          setOpen={(v) => setDialog(v
            ? "register"
            : undefined,
          )}
        >
          <button className="btn-black">
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
    </>
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
