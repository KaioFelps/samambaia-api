import { Link, usePage } from "@inertiajs/react";

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

function Unlogged() {
  return (
    <>
      <div className="flex items-center gap-2">
        <Link
          href="?register"
          className="btn-black"
        >
          <Sprite
            x={-66}
            y={-64}
            width={13}
            height={16}
          />

          Registrar
        </Link>

        <Link
          href="?login"
          className="btn-success btn-lg border-black to-black/25"
        >
          <Sprite
            x={-32}
            y={-65}
            width={15}
            height={15}
          />

          Login
        </Link>
      </div>

      <LoginForm />
      <RegisterForm />
    </>
  );
}

function Logged() {
  return <div />;
}
