import { Slot } from "@radix-ui/react-slot";
import clsx from "clsx";
import type { PropsWithChildren, RefCallback, SlotHTMLAttributes } from "react";

type Props<T = SlotHTMLAttributes<HTMLSpanElement>> = PropsWithChildren & {
  asChild?: boolean;
  ref?: RefCallback<HTMLElement>;
  variant?: "default" | "gray";
} & T;

export function Chip({ asChild, ref, children, variant = "default", ...props }: Props) {
  const Element = asChild ? Slot : "span";

  return (
    <Element
      ref={ref}
      {...props}
      className={clsx(
        "font-medium text-sm leading-normal px-1.5 -py-1.5 rounded-sm cursor-default",
        "border-2 shadow-[inset_0_2px_0_0]",
        "transition-all duration-150 ring-0 focus-within:ring-4 outline-none",

        variant === "default" && [
          "text-white bg-blue-500 border-blue border-blue-800 shadow-white/50",
          "text-shadow-[0px_-1px_0] text-shadow-blue-800 ring-blue-500/40",
        ],

        variant === "gray" &&
          "text-gray-800 shadow-white bg-gray-300 border-gray-600 ring-gray-700/30 text-shadow-[0px_1px_0] text-shadow-white/50",
      )}>
      {children}
    </Element>
  );
}
