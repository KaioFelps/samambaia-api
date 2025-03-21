import { memo } from "react";

import { AdminAlert, AdminAlertProps } from "./admin-alert";
import { PublicAlert, PublicAlertProps } from "./public-alert";

export type BaseAlertProps = {
  message: string;
  className?: string;
};

type AlertProps = {
  admin: true;
} & AdminAlertProps | { admin?: false } & PublicAlertProps;

export const Alert = memo(({ admin, ...alertProps }: AlertProps) => {
  return admin
    ? <AdminAlert {...alertProps as AdminAlertProps} />
    : <PublicAlert {...alertProps as PublicAlertProps} />;
});
