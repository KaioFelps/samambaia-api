import type { InertiaLinkProps } from "@inertiajs/react";
import React, { type ButtonHTMLAttributes, type ForwardedRef, type ReactElement } from "react";

import { AdminButton, type AdminButtonProps } from "./admin-button";

export type BaseButtonProps =
  | ({ asLink: true } & Omit<InertiaLinkProps, "size">)
  | ({ asLink?: false | undefined } & ButtonHTMLAttributes<HTMLButtonElement>);

type ButtonProps = ({ admin: true } & AdminButtonProps) | ({ admin?: false } & never);

const Button = React.forwardRef(
  (
    { admin = true, ...buttonProps }: ButtonProps,
    ref: ForwardedRef<HTMLButtonElement | ReactElement>,
  ) => {
    return admin ? (
      <AdminButton {...(buttonProps as AdminButtonProps)} ref={ref} />
    ) : (
      null
    );
  },
);

export default Button;
