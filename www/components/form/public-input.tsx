import clsx from "clsx";

import type { InputProps } from "./input";
import { ValidationErrorSpan } from "./validation-error-alert";

export const PublicInput = ({
  id,
  name,
  label,
  className,
  validationError,
  containerClassName,
  ...props
}: Omit<InputProps, "admin">) => {
  return (
    <div className={containerClassName}>
      <label htmlFor={id ?? name} className="block text-sm mb-1 ml-1">
        {label}
      </label>

      <ValidationErrorSpan validationError={validationError} />

      <input
        id={id ?? name}
        name={name}
        className={clsx("text-input", className && className)}
        data-invalid={validationError ? "" : null}
        {...props}
      />
    </div>
  );
};
