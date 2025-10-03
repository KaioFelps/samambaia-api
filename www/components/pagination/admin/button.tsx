import { Slot } from "@radix-ui/react-slot";
import clsx from "clsx";
import { memo } from "react";
import { CorePaginationButton, type CorePaginationButtonProps } from "../button";

export const AdminPaginationButton = memo((props: CorePaginationButtonProps) => {
  return (
    <Slot
      className={clsx(
        "text-sm transition-all self-stretch h-8 px-2 aspect-square grid place-items-center",
        "font-rowdies rounded-sm text-purple-700 select-none",
        "outline-hidden ring-0 ring-purple-500/40 focus-visible:ring-4",
        "button-like-enabled:bg-purple-700/20 button-like-enabled:hover:bg-purple-700/30",
        "button-like-enabled:active:bg-purple-700/40 button-like-enabled:underline",
        "button-like-enabled:decoration-dotted button-like-enabled:decoration-2",
        "button-like-disabled:bg-purple-700/5",
      )}>
      <CorePaginationButton {...props} />
    </Slot>
  );
});
