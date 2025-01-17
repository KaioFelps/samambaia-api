import clsx from "clsx";

import { Sprite, SpriteProps } from "./sprite";

type AlertProps = {
  type: "warning";
  message: string;
  className?: string;
};

export function Alert({ message, type, className }: AlertProps) {
  const sprites: Record<string, SpriteProps> = {
    warning: {
      x: -431,
      y: -5,
      width: 23,
      height: 21,
    },
  };

  return (
    <div className={clsx(
      "w-full flex items-center gap-3 p-2 rounded-lg mx-auto border-2 cursor-default",
      type === "warning" && "bg-yellow-500/25 border-yellow-900 text-yellow-900",
      className && className,
    )}
    >
      <div className={clsx(
        "size-[40px] grid place-items-center rounded-md",
        type === "warning" && "bg-yellow-900",
      )}
      >
        <Sprite {...sprites[type]} />
      </div>

      {message}
    </div>
  );
}
