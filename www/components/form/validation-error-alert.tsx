import clsx from "clsx";

type ValidationErrorSpanProps = {
  validationError?: string;
};

export const ValidationErrorSpan = ({ validationError }: ValidationErrorSpanProps) => {
  if (!validationError) return null;

  return (
    <span className={clsx(
      "block text-red-700 font-medium mb-2 text-sm bg-red-700/20 py-1 px-2 rounded-md",
    )}
    >
      {validationError}
    </span>
  );
};
