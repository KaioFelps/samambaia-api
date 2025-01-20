import { colors } from "@crate/tailwind.config";
import * as Dialog from "@radix-ui/react-dialog";
import clsx from "clsx";
import { ReactNode } from "react";

import { colorWithOpacity } from "@/lib/tailwind";

export type DialogContentProps = {
  children: ReactNode;
  className?: string;
};

export function DialogContent({ children, className }: DialogContentProps) {
  return (
    <Dialog.Portal>
      <Dialog.Overlay className="fixed inset-0 bg-black/30 backdrop-blur-sm z-20" />
      <Dialog.Content
        style={{
          boxShadow:
            `inset 0 0 0 4px ${colors.white},
            0 2px 0 0 ${colorWithOpacity(colors.black, 25)}`,
        }}
        className={clsx(
          "fixed z-20 top-1/2 left-1/2",
          "bg-purple-100 p-4 rounded-lg border-2 border-black w-[calc(100%_-_48px)]",
          "max-w-screen-sm",
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
