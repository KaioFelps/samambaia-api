import { InertiaLinkProps, Link } from "@inertiajs/react";
import clsx from "clsx";
import React, { ButtonHTMLAttributes, useMemo } from "react";

import { BaseButtonProps } from ".";
import { ButtonSize } from "./shared-types";

export type AdminButtonProps = BaseButtonProps & {
  variant?: "default" | "ghost";
  theme?: "default";
  size?: ButtonSize;
};

export const AdminButton = React.forwardRef(({
  asLink = false,
  className,
  variant = "default",
  theme = "default",
  size = "md",
  ...props
}: AdminButtonProps, ref) => {
  const ButtonElement = useMemo(() => asLink
    ? Link
    : "button", [asLink]);

  return (
    <ButtonElement
      {...(asLink
        ? props as InertiaLinkProps
        : props as ButtonHTMLAttributes<HTMLButtonElement>)}
      className={clsx(
        "flex items-center justify-center gap-2.5 outline-hidden",
        "ring-0 focus-visible:ring-4 transition-all duration-100",
        "rounded-lg",

        // default variant
        variant === "default" && theme === "default" &&
          "bg-blue-500 border border-black/10 text-white",
        variant === "default" && theme === "default" &&
          "ring-blue-500/40 hover:bg-blue-600 active:brightness-90",

        variant === "ghost" && theme === "default" &&
          "border border-black/20 ring-purple-500/40 bg-transparent",
        variant === "ghost" && theme === "default" &&
          "hover:bg-black/5 active:bg-black/10",

        // md size
        size === "md" && "px-2 py-1.5 text-sm",
        className && className,
      )}
      // @ts-expect-error can't infer type of forwarded ref
      ref={ref}
    />
  );
});
