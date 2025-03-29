import { Content, type DialogContentProps, Overlay, Portal } from "@radix-ui/react-dialog";
import clsx from "clsx";
import { memo } from "react";

export const AdminDialogContent = memo(({ className, ...props }: DialogContentProps) => {
  return (
    <Portal>
      <Overlay className="fixed inset-0 bg-black/30 backdrop-blur-xs z-20" />
      <Content
        {...props}
        className={clsx(
          "fixed z-20 top-1/2 left-1/2 shadow-sm shadow-black/10",
          "bg-purple-100 p-2 rounded-lg border border-black/20 w-[calc(100%_-_48px)]",
          "max-w-(--breakpoint-sm)",
          "data-[state=open]:animate-dialog-scale-up-and-fade",
          "data-[state=closed]:animate-dialog-scale-up-and-fade-reverse",
          className && className,
        )}
      />
    </Portal>
  );
});
