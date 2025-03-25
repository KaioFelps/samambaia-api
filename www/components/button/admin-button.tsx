import { InertiaLinkProps, Link } from "@inertiajs/react";
import clsx from "clsx";
import React, { ButtonHTMLAttributes, useMemo } from "react";

import { BaseButtonProps } from ".";
import { getAdminButtonStyles, OptionalVariantAndTheme, ThemeKey } from "./admin-button-themes";
import { ButtonSize } from "./shared-types";

export type AdminButtonProps = BaseButtonProps & {
  size?: ButtonSize;
} & OptionalVariantAndTheme;

export const AdminButton = React.forwardRef(({
  asLink = false,
  className,
  variant = "default",
  theme = "default",
  size = "md",
  ...props
}: AdminButtonProps, ref) => {
  const variantAndThemeStyle = useMemo(() => {
    return getAdminButtonStyles({ variant, theme: theme as ThemeKey });
  }, [variant, theme]);

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

        variantAndThemeStyle,
        // md size
        size === "md" && "px-2 py-1.5 text-sm",
        size === "lg" && "px-4 py-2 text-md",
        className && className,
      )}
      // @ts-expect-error can't infer type of forwarded ref
      ref={ref}
    />
  );
});
