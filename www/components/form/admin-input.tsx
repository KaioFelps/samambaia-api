import clsx from "clsx";

import type { InputProps } from "./input";
import { ValidationErrorSpan } from "./validation-error-alert";

export function AdminInput({
  id,
  name,
  label,
  className,
  validationError,
  containerClassName,
  ...rest
}: Omit<InputProps, "admin">) {
  return (
    <div className={containerClassName}>
      <label htmlFor={id ?? name} className="block text-sm mb-1 ml-1">
        {label}
      </label>

      <ValidationErrorSpan validationError={validationError} />

      <input
        id={id ?? name}
        name={name}
        className={clsx(
          "w-full rounded-lg border border-black/20 text-sm leading-none py-1 px-2",
          "bg-white",
          "transition-all outline-hidden ring-inset ring-0 ring-purple-500 focus:ring-1",
          "focus:border-purple-500",
          className && className,
        )}
        data-invalid={validationError ? "" : null}
        {...rest}
      />
    </div>
  );
}
