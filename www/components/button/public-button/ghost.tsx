import { Slot } from "@radix-ui/react-slot";
import clsx from "clsx";
import React from "react";
import type { CorePublicButtonProps } from ".";

type Props = {
  variant?: "success" | "blue";
  size?: "md" | "lg";
} & CorePublicButtonProps;

export const GhostPublicButton = React.forwardRef<HTMLButtonElement, Props>(
  ({ asChild, children, variant = "success", size = "md", disabled, ...props }, ref) => {
    const Button = asChild ? Slot : "button";
    return (
      <Button
        ref={ref}
        disabled={disabled}
        data-disabled={disabled ? "disabled" : "enabled"}
        role="button"
        className={clsx(
          "flex items-center justify-center gap-3 leading-tight",
          "transition-all duration-100 will-change-[shadow,_filter]",
          "button-like-disabled:cursor-not-allowed",

          size === "md" && "px-6 py-2.5 font-medium",
          size === "lg" && "px-8 py-3.5 font-bold",

          variant === "success" && "btn-ghost-success",
          variant === "blue" && "btn-ghost-blue"
        )}
        {...props}
      >
        {children}
      </Button>
    );
  },
);
