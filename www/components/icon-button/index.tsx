import type { InertiaLinkProps } from "@inertiajs/react";
import React, { type ButtonHTMLAttributes } from "react";

import { AdminIconButton, type AdminIconButtonProps } from "./admin-icon-button";

export type LinkProps = Omit<InertiaLinkProps, "size" | "children" | "type" | "as">;

export type BaseIconButtonProps =
  | ({ asLink: true } & LinkProps)
  | ({ asLink?: false } & Omit<ButtonHTMLAttributes<HTMLButtonElement>, "children"> & {});

type IconButtonProps = ({ admin: true } & AdminIconButtonProps) | ({ admin?: false } & {});

export const IconButton = React.forwardRef(({ admin, ...buttonProps }: IconButtonProps, ref) => {
  return admin ? <AdminIconButton {...(buttonProps as AdminIconButtonProps)} ref={ref} /> : null;
});
