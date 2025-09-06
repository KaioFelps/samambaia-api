import { Viewport as PViewport, ScrollDownButton, ScrollUpButton } from "@radix-ui/react-select";
import clsx from "clsx";
import { memo, type ReactNode } from "react";

type ViewportProps = {
  children: ReactNode;
  className?: string;
};

export const SelectViewport = memo(({ children, className }: ViewportProps) => {
  return (
    <>
      <ScrollUpButton className="h-[25px] grid place-items-center text-purple-700" />
      <PViewport
        className={clsx("p-1 gap-0.5 flex flex-col min-w-48 text-sm", className && className)}>
        {children}
      </PViewport>
      <ScrollDownButton className="h-[25px] grid place-items-center text-purple-700" />
    </>
  );
});
