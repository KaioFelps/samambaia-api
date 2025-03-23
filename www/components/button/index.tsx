import { InertiaLinkProps } from "@inertiajs/react";
import React, { ButtonHTMLAttributes, ForwardedRef, ReactElement } from "react";

import { AdminButton, AdminButtonProps } from "./admin-button";
import { PublicButton, PublicButtonProps } from "./public-button";

export type BaseButtonProps =
| { asLink: true } & InertiaLinkProps
| { asLink?: false | undefined } & ButtonHTMLAttributes<HTMLButtonElement>;

type ButtonProps = { admin: true } & AdminButtonProps | { admin?: false } & PublicButtonProps;

const Button = React.forwardRef((
  { admin = false, ...buttonProps }: ButtonProps,
  ref: ForwardedRef<HTMLButtonElement | ReactElement>,
) => {
  return admin
    ? <AdminButton
        {...buttonProps as AdminButtonProps}
        ref={ref}
      />
    : <PublicButton
        {...buttonProps as PublicButtonProps}
        ref={ref as ForwardedRef<PublicButtonProps>}
      />;
});

export default Button;
