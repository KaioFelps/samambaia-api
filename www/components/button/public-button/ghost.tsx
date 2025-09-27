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
        size === "md" && "px-5 py-2 font-medium",
        size === "lg" && "px-7 py-3 font-bold",

        variant === "success" && "btn-ghost-success",
      )}
      {...props}>
      {children}
    </Button>
  );
}
