import { Slot } from "@radix-ui/react-slot";
import clsx from "clsx";
import type { PropsWithChildren } from "react";

type Props = PropsWithChildren & {
  asChild?: boolean;
};

export function Chip({ asChild, children }: Props) {
  const Element = asChild ? Slot : "span";

  return (
    <Element
      className={clsx(
        "text-white font-medium text-sm leading-normal px-2 py-0.5 rounded-sm cursor-default",
        "bg-blue-500 ring-inset ring-2 ring-blue-800 shadow-[inset_0_4px_0_0] shadow-white/50",
        "text-shadow-[0px_-1px_0] text-shadow-blue-800",
      )}>
      {children}
    </Element>
  );
}
