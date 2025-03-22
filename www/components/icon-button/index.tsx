import { ButtonHTMLAttributes, memo } from "react";

import { AdminIconButton, AdminIconButtonProps } from "./admin-icon-button";

export type BaseIconButtonProps = Omit<ButtonHTMLAttributes<HTMLButtonElement>, "children"> & {};

type IconButtonProps =
| { admin: true } & AdminIconButtonProps
| { admin?: false } & {};

export const IconButton = memo(({ admin, ...buttonProps }: IconButtonProps) => {
  return admin
    ? <AdminIconButton {...buttonProps as AdminIconButtonProps} />
    : null;
});
