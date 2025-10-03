import clsx from "clsx";
import { memo, useMemo } from "react";

import { Sprite, type SpriteProps } from "../sprite";
import type { BaseAlertProps } from ".";

export type PublicAlertProps = BaseAlertProps & {
  type: "warning" | "error" | "success" | "info";
};

export const PublicAlert = memo(({ message, type, className }: PublicAlertProps) => {
  const sprites: Record<PublicAlertProps["type"], SpriteProps> = useMemo(
    () => ({
      warning: {
        height: 21,
        width: 23,
        x: -431,
        y: -5,
      },
      error: {
        width: 16,
        height: 16,
        x: -102,
        y: -128,
      },
      success: {
        width: 16,
        height: 16,
        x: -81,
        y: -128,
      },
      info: {
        width: 20,
        height: 20,
        x: -150,
        y: -126,
      },
    }),
    [],
  );

  return (
    <div
      className={clsx(
        "w-full flex items-center gap-3 p-2 rounded-lg mx-auto border-2 cursor-default",
        type === "warning" && "bg-yellow-500/25 border-yellow-900 text-yellow-900",
        type === "error" && "bg-red-700/25 border-red-700 text-red-800",
        type === "success" && "bg-green-500/25 border-green-700 text-green-700",
        type === "info" && "bg-blue-500/15 border-blue-500 text-blue-700",
        className && className,
      )}>
      <div
        className={clsx(
          "size-[40px] grid place-items-center rounded-md",
          type === "warning" && "bg-yellow-900",
          type === "error" && "bg-red-800",
          type === "success" && "bg-green-700",
          type === "info" && "bg-blue-700",
        )}>
        <Sprite {...sprites[type]} />
      </div>

      {message}
    </div>
  );
});
