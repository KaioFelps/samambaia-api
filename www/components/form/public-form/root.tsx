import clsx from "clsx";
import type { FormHTMLAttributes } from "react";

type PublicFormRootProps = FormHTMLAttributes<HTMLFormElement>;

export const PublicFormRoot = ({ className, ...props }: PublicFormRootProps) => {
  return <form className={clsx("w-full flex flex-col gap-3", className && className)} {...props} />;
};
