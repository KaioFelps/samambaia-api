import { Slot } from "@radix-ui/react-slot";
import clsx from "clsx";
import type { CorePublicButtonProps } from ".";

type Props = {
  variant?: "success";
  size?: "md" | "lg";
} & CorePublicButtonProps;

export function GhostPublicButton({
  asChild,
  children,
  variant = "success",
  size = "md",
  ...props
}: Props) {
  const Button = asChild ? Slot : "button";
  return (
    <Button
      className={clsx(
        "flex items-center justify-center gap-3 leading-tight",
        "transition-all duration-100 will-change-[shadow,_filter]",
        "not-enabled:cursor-not-allowed",

        size === "md" && "px-6 py-2.5 font-medium",
        size === "lg" && "px-8 py-3.5 font-bold",

        variant === "success" && "btn-ghost-success",
      )}
      {...props}>
      {children}
    </Button>
  );
}
