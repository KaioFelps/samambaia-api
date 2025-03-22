import { Icon } from "@phosphor-icons/react";
import clsx from "clsx";
import { memo, ReactElement } from "react";

import { BaseIconButtonProps } from ".";

export type AdminIconButtonProps = BaseIconButtonProps & {
  size: "sm";
  variant: "ghost";
  theme: "danger" | "warn" | "info";
  icon: Icon | ReactElement;
};

export const AdminIconButton = memo(({
  icon: _icon,
  className,
  size,
  variant,
  theme,
  ...props
}: AdminIconButtonProps) => {
  const icon = (() => {
    if ("render" in _icon) {
      const I = _icon as Icon;
      return (
        <I
          size={16}
          weight="bold"
        />
      );
    }
    return _icon as ReactElement;
  })();

  return (
    <button
      {...props}
      className={clsx(
        "outline-hidden ring-0 focus-visible:ring-4 transition-all will-change-[box-shadow]",

        size === "sm" && "p-[3px] rounded-[5px]",

        variant === "ghost" && theme === "danger" &&
            "bg-red-700/25 hover:bg-red-700/40 active:bg-red-700/60 ring-red-700/50",
        variant === "ghost" && theme === "warn" &&
            "bg-yellow-500/25 hover:bg-yellow-500/50 active:bg-yellow-500/70 ring-yellow-800/40",
        variant === "ghost" && theme === "info" &&
            "bg-blue-500/25 hover:bg-blue-500/40 active:bg-blue-500/50 ring-blue-500/40",

        className && className,
      )}
    >
      {icon}
    </button>
  );
});
