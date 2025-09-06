import type { DetailedHTMLProps, InputHTMLAttributes } from "react";

import { AdminInput } from "./admin-input";
import { PublicInput } from "./public-input";

export type InputProps = DetailedHTMLProps<
  InputHTMLAttributes<HTMLInputElement>,
  HTMLInputElement
> & {
  label: string;
  validationError?: string;
  containerClassName?: string;
  admin?: boolean;
};

export function Input({ admin, ...props }: InputProps) {
  return admin ? <AdminInput {...props} /> : <PublicInput {...props} />;
}
