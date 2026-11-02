import { Slot } from "@radix-ui/react-slot";
import clsx from "clsx";
import type { LabelHTMLAttributes } from "react";

type PublicLabelProps = LabelHTMLAttributes<HTMLLabelElement> & {
  asChild?: boolean;
};

export function PublicLabel({
  className,
  htmlFor,
  children,
  asChild = false,
  hidden,
  ...props
}: PublicLabelProps) {
  const Element = asChild ? Slot : "label";
  return (
    <Element
      htmlFor={htmlFor}
      {...props}
      className={clsx("block text-sm mb-1 ml-1", hidden && "sr-only", className && className)}>
      {children}
    </Element>
  );
}
