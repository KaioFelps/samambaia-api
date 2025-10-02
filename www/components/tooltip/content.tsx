import clsx from "clsx";
import type { ReactNode } from "react";

type PopoverContentContainerProps = { children: ReactNode; className?: string };

export function TooltipContent({ children, className }: PopoverContentContainerProps) {
  return <div className={clsx("px-3 py-1 text-sm", className && className)}>{children}</div>;
}
