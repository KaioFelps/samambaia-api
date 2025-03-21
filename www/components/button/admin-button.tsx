import { Link } from "@inertiajs/react";
import clsx from "clsx";
import { useMemo } from "react";

import { BaseButtonProps } from ".";
import { ButtonSize } from "./shared-types";

export type AdminButtonProps = BaseButtonProps & {
  variant?: "default";
  size?: ButtonSize;
};

export const AdminButton = ({
  asLink,
  className,
  variant = "default",
  size = "md",
  ...props
}: AdminButtonProps) => {
  const ButtonElement = useMemo(() => asLink
    ? Link
    : "button", [asLink]);

  return (
    // @ts-expect-error props are correct, but there ain't no way of casting it inside react jsx
    <ButtonElement
      {...props}
      className={clsx(
        "flex items-center justify-center gap-2.5",

        // default variant
        variant === "default" && "bg-blue-500 border border-black/10 rounded-lg text-white",

        // md size
        size === "md" && "px-2 py-1.5 text-sm",
        className && className,
      )}
    />
  );
};
