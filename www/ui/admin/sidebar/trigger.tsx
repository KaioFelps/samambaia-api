import { type Icon } from "@phosphor-icons/react";
import { CaretDown } from "@phosphor-icons/react/dist/ssr/CaretDown";
import { Trigger } from "@radix-ui/react-accordion";
import clsx from "clsx";

type SidebarMenuTriggerProps = {
  title: string;
  icon: Icon;
};

export const SidebarMenuSectionTrigger = ({ title, icon: I }: SidebarMenuTriggerProps) => {
  return (
    <Trigger className={clsx(
      "transition-all duration-150 flex items-center gap-2 px-2 py-1 rounded-lg cursor-default",
      "group w-full data-[state=closed]:hover:bg-purple-500/10",
      "data-[state=closed]:active:bg-purple-500/20",
      "data-[state=closed]:active:cursor-pointer data-[state=open]:bg-purple-500/10",
      //
      "relative ml-1 before:absolute before:inset-y-full before:left-0 before:rounded-full",
      "before:w-0.5 before:bg-purple-500 before:-ml-1 data-[state=open]:before:inset-y-0",
      "before:transition-all before:will-change-[inset-y,_inset] before:duration-75",
    )}
    >
      <I
        size={20}
        weight="bold"
        className="text-purple-500"
      />

      {title}

      <CaretDown
        size={16}
        weight="bold"
        className={clsx(
          "absolute right-2 top-1/2 -translate-y-1/2 h-0 group-data-[state=open]:h-4",
          "group-data-[state=closed]:group-hover:h-4 transition-[height,transform] duration-300",
          "group-data-[state=open]:rotate-180",
        )}
      />
    </Trigger>
  );
};
