import { InertiaLinkProps } from "@inertiajs/react";
import { ButtonHTMLAttributes, memo } from "react";

import { AdminIconButton, AdminIconButtonProps } from "./admin-icon-button";

export type LinkProps = Omit<InertiaLinkProps, "size" | "children" | "type" | "as">;

export type BaseIconButtonProps =
| { asLink: true } & LinkProps
| { asLink?: false } & Omit<ButtonHTMLAttributes<HTMLButtonElement>, "children"> & {};

type IconButtonProps =
| { admin: true } & AdminIconButtonProps
| { admin?: false } & {};

export const IconButton = memo(({ admin, ...buttonProps }: IconButtonProps) => {
  return admin
    ? <AdminIconButton {...buttonProps as AdminIconButtonProps} />
    : null;
});
