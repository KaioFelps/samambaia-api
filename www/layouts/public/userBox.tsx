import { usePage } from "@inertiajs/react";

import { Sprite } from "@/components/sprite";
import { SharedProps } from "@/inertiaShared";

export function UserBox() {
  const { auth } = usePage<SharedProps>().props;

  return auth
    ? Logged()
    : Unlogged();
}

function Unlogged() {
  return (
    <div className="flex items-center gap-2">
      <button className="btn-black">
        <Sprite
          x={-66}
          y={-64}
          width={13}
          height={16}
        />

        Registrar
      </button>

      <button className="btn-success btn-lg">
        <Sprite
          x={-32}
          y={-65}
          width={15}
          height={15}
        />

        Login
      </button>
    </div>
  );
}

function Logged() {
  return <div />;
}
