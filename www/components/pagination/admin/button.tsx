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
        "data-[state=deactivated]:bg-purple-700/20 data-[state=deactivated]:hover:bg-purple-700/30",
        "data-[state=deactivated]:active:bg-purple-700/40 data-[state=deactivated]:underline",
        "data-[state=deactivated]:decoration-dotted data-[state=deactivated]:decoration-2",
        "data-[state=active]:bg-purple-700/5",
      )}>
      <CorePaginationButton {...props} />
    </Slot>
  );
});
