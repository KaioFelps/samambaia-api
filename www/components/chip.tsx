import { Slot } from "@radix-ui/react-slot";
import clsx from "clsx";
import type { PropsWithChildren, RefCallback, SlotHTMLAttributes } from "react";

type Props<T = SlotHTMLAttributes<HTMLSpanElement>> = PropsWithChildren & {
  asChild?: boolean;
  ref?: RefCallback<HTMLElement>;
} & T;

export function Chip({ asChild, ref, children, ...props }: Props) {
  const Element = asChild ? Slot : "span";

  return (
    <Element
      ref={ref}
      {...props}
      className={clsx(
        "text-white font-medium text-sm leading-normal px-1.5 -py-1.5 rounded-sm cursor-default",
        "bg-blue-500 border-2 border-blue border-blue-800 shadow-[inset_0_2px_0_0] shadow-white/50",
        "text-shadow-[0px_-1px_0] text-shadow-blue-800",
        "transition-all duration-150 ring-0 ring-blue-500/40 focus-within:ring-4 outline-none",
      )}>
      {children}
    </Element>
  );
}
