import { Slot } from "@radix-ui/react-slot";
import clsx from "clsx";
import type { CorePublicButtonProps } from ".";

type Props = {
  variant?: "default" | "black" | "success" | "yellow";
} & CorePublicButtonProps;

export function DefaultPublicButton({
  asChild,
  children,
  variant = "default",
  className,
  size = "md",
  ...props
}: Props) {
  const Button = asChild ? Slot : "button";
  return (
    <Button
      className={clsx(
        "flex items-center justify-center gap-3 leading-tight",
        "transition-all duration-150 will-change-[shadow,_filter]",
        "not-enabled:cursor-not-allowed",

        variant === "default" && "btn",
        size === "md" && "px-6 py-3 font-medium",
        size === "lg" && "px-8 py-4 font-bold",

        variant === "default" && "btn",
        variant === "success" && "btn-success",
        variant === "black" && "btn-black",
        variant === "yellow" && "btn-yellow",

        className && className,
      )}
      {...props}>
      {children}
    </Button>
  );
}
