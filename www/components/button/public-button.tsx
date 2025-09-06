import { type InertiaLinkProps, Link } from "@inertiajs/react";
import React, { type ButtonHTMLAttributes, useMemo } from "react";

import type { BaseButtonProps } from ".";

export type PublicButtonProps = BaseButtonProps & {
  variant: string;
};

export const PublicButton = React.forwardRef(({ asLink, ...props }: PublicButtonProps, ref) => {
  const ButtonElement = useMemo(() => (asLink ? Link : "button"), [asLink]);

  return (
    <ButtonElement
      {...(asLink
        ? (props as InertiaLinkProps)
        : (props as ButtonHTMLAttributes<HTMLButtonElement>))}
      // @ts-expect-error Cant infer type of ref
      ref={ref}
    />
  );
});
