import clsx from "clsx";
import { memo, useMemo } from "react";

import { Sprite, SpriteProps } from "./sprite";

type AlertProps = {
  type: "warning" | "error" | "success";
  message: string;
  className?: string;
};

export const Alert = memo(({ message, type, className }: AlertProps) => {
  const sprites: Record<AlertProps["type"], SpriteProps> = useMemo(() => ({
    warning: {
      x: -431,
      y: -5,
      width: 23,
      height: 21,
    },
    error: {
      height: 16,
      width: 16,
      x: -102,
      y: -128,
    },
    success: {
      height: 16,
      width: 16,
      x: -81,
      y: -128,
    },
  }), []);

  return (
    <div className={clsx(
      "w-full flex items-center gap-3 p-2 rounded-lg mx-auto border-2 cursor-default",
      type === "warning" && "bg-yellow-500/25 border-yellow-900 text-yellow-900",
      type === "error" && "bg-red-700/25 border-red-700 text-red-800",
      type === "success" && "bg-green-500/25 border-green-700 text-green-700",
      className && className,
    )}
    >
      <div className={clsx(
        "size-[40px] grid place-items-center rounded-md",
        type === "warning" && "bg-yellow-900",
        type === "error" && "bg-red-800",
        type === "success" && "bg-green-700",
      )}
      >
        <Sprite {...sprites[type]} />
      </div>

      {message}
    </div>
  );
});
