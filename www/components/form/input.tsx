import clsx from "clsx";
import { DetailedHTMLProps, InputHTMLAttributes } from "react";

type InputProps = DetailedHTMLProps<InputHTMLAttributes<HTMLInputElement>, HTMLInputElement> & {
  label: string;
  validationError?: string;
  containerClassName?: string;
};

export function Input({
  id,
  name,
  label,
  className,
  validationError,
  containerClassName,
  ...rest
}: InputProps) {
  return (
    <div className={containerClassName}>
      <label
        htmlFor={id ?? name}
        className="block text-sm mb-1 ml-1"
      >
        {label}
      </label>

      {validationError && (
        <span className="
          block text-red-700 font-medium mb-2 text-sm bg-red-700/20 py-1 px-2 rounded-md
          "
        >
          {validationError}
        </span>)}

      <input
        id={id ?? name}
        name={name}
        className={clsx("text-input", className && className)}
        data-invalid={validationError
          ? ""
          : null}
        {...rest}
      />
    </div>
  );
}
