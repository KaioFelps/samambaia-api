import clsx from "clsx";
import { useId } from "react";
import type { InputProps } from "../input";
import { ValidationErrorSpan } from "../validation-error-alert";
import { PublicLabel } from "./label";

export const PublicInput = ({
  id,
  name,
  label,
  className,
  validationError,
  containerClassName,
  ...props
}: Omit<InputProps, "admin">) => {
  const inputId = useId();
  return (
    <div className={containerClassName}>
      <PublicLabel htmlFor={id ?? `${label}-${inputId}`}>{label}</PublicLabel>

      <ValidationErrorSpan validationError={validationError} />

      <input
        id={id ?? `${label}-${inputId}`}
        name={name}
        className={clsx("text-input", className && className)}
        data-invalid={validationError ? "" : null}
        {...props}
      />
    </div>
  );
};
