import * as Dialog from "@radix-ui/react-dialog";
import clsx from "clsx";
import { ReactNode } from "react";

export type DialogContentProps = {
  children: ReactNode;
  className?: string;
};

export function DialogContent({ children, className }: DialogContentProps) {
  return (
    <Dialog.Portal>
      <Dialog.Overlay className="fixed inset-0 bg-black/30 backdrop-blur-xs z-20" />
      <Dialog.Content
        style={{
          boxShadow:
            `inset 0 0 0 4px var(--color-white),
            0 2px 0 0 color-mix(in oklab, var(--color-black, 25%, transparent)`,
        }}
        className={clsx(
          "fixed z-20 top-1/2 left-1/2",
          "bg-purple-100 p-4 rounded-lg border-2 border-black w-[calc(100%_-_48px)]",
          "max-w-(--breakpoint-sm)",
          "data-[state=open]:animate-dialogScaleUpAndFade",
          "data-[state=closed]:animate-dialogScaleUpAndFadeReverse",
          className && className,
        )}
      >
        {children}
      </Dialog.Content>
    </Dialog.Portal>
  );
}
