import { InertiaLinkProps } from "@inertiajs/react";
import { ButtonHTMLAttributes, memo } from "react";

import { AdminButton, AdminButtonProps } from "./admin-button";
import { PublicButton, PublicButtonProps } from "./public-button";

export type BaseButtonProps =
| { asLink: true } & InertiaLinkProps
| { asLink?: false | undefined } & ButtonHTMLAttributes<HTMLButtonElement>;

type ButtonProps = { admin: true } & AdminButtonProps | { admin?: false } & PublicButtonProps;

const Button = memo(({ admin = false, ...buttonProps }: ButtonProps) => {
  return admin
    ? <AdminButton {...buttonProps as AdminButtonProps} />
    : <PublicButton {...buttonProps as PublicButtonProps} />;
});

export default Button;
