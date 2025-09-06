import clsx from "clsx";
import type { FormHTMLAttributes } from "react";

type FormRootProps = FormHTMLAttributes<HTMLFormElement>;

export const FormRoot = ({ className, ...props }: FormRootProps) => {
  return (
    <form
      className={clsx(
        "bg-white p-6 rounded-xl shadow-md shadow-black/5 w-full",
        "border border-gray-300",
        className && className,
      )}
      {...props}
    />
  );
};
