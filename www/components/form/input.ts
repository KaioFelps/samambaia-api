import type { DetailedHTMLProps, InputHTMLAttributes } from "react";

export type InputProps = DetailedHTMLProps<
  InputHTMLAttributes<HTMLInputElement>,
  HTMLInputElement
> & {
  label: string;
  hiddenLabel?: boolean;
  validationError?: string;
  containerClassName?: string;
  admin?: boolean;
};
