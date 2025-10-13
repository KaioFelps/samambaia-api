import clsx from "clsx";
import { type DetailedHTMLProps, type TextareaHTMLAttributes, useId } from "react";
import { ValidationErrorSpan } from "../validation-error-alert";

type HtmlTextareaProps = DetailedHTMLProps<
  TextareaHTMLAttributes<HTMLTextAreaElement>,
  HTMLTextAreaElement
>;

export type PublicInputProps = HtmlTextareaProps & {
  label: string;
  validationError?: string;
  containerClassName?: string;
};

export function PublicTextArea({
  containerClassName,
  label,
  name,
  id,
  validationError,
  className,
  disabled,
  ...props
}: PublicInputProps) {
  const textareaId = useId();
  return (
    <div className={containerClassName}>
      <label htmlFor={id ?? `${label}-${textareaId}`} className="block text-sm mb-1 ml-1">
        {label}
      </label>

      <ValidationErrorSpan validationError={validationError} />

      <textarea
        id={id ?? `${label}-${textareaId}`}
        name={name}
        disabled={disabled}
        className={clsx("text-input", disabled && "resize-none", className && className)}
        data-invalid={validationError ? "" : null}
        {...props}
      />
    </div>
  );
}
