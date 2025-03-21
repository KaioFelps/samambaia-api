import { InertiaLinkProps, Link } from "@inertiajs/react";
import { ButtonHTMLAttributes, useMemo } from "react";

import { BaseButtonProps } from ".";

export type PublicButtonProps = BaseButtonProps & {
  variant: string;
};

export const PublicButton = ({ asLink, ...props }: PublicButtonProps) => {
  const ButtonElement = useMemo(() => asLink
    ? Link
    : "button", [asLink]);

  return (
    // @ts-expect-error props are correct, but there ain't no way of casting it inside react jsx
    <ButtonElement {...(asLink
      ? props as InertiaLinkProps
      : props as ButtonHTMLAttributes<HTMLButtonElement>)}
    />
  );
};
