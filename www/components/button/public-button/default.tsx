import { Slot } from "@radix-ui/react-slot";
import clsx from "clsx";
import * as React from "react";

import type { CorePublicButtonProps } from ".";

type Props = {
  variant?: "default" | "black" | "success" | "yellow";
  ref?: React.Ref<HTMLButtonElement>;
} & CorePublicButtonProps;

export const DefaultPublicButton = React.forwardRef<HTMLButtonElement, Props>(
  ({ asChild, children, variant = "default", className, size = "md", ...props }, ref) => {
    const Button = asChild ? Slot : "button";
    return (
      <Button
        ref={ref}
        role="button"
        className={clsx(
          "flex items-center justify-center gap-3 leading-tight",
          "transition-all duration-150 will-change-[shadow,_filter]",
          "disabled:cursor-not-allowed [role='button']:not-enabled:cursor-not-allowed",

          variant === "default" && "btn",

          // sempre diminuir 1 nos paddings pra diminuir 4, o que
          // compensa os 4px extras ganhados pelas bordas de 2px em cada lado
          size === "md" && "px-5 py-2 font-medium",
          size === "lg" && "px-7 py-3 font-bold",

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
  },
);
