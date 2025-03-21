import * as Popover from "@radix-ui/react-popover";
import clsx from "clsx";
import { ReactNode } from "react";

import { PublicDroppableArrow } from "../droppable-arrow";

export type PopoverContentProps = Omit<Popover.PopoverContentProps, "className">;

export function PopoverContent({
  children,
  sideOffset = 4,
  style,
  ...rest
}: PopoverContentProps) {
  return (
    <Popover.Portal>
      <Popover.Content
        sideOffset={sideOffset}
        {...rest}
        style={{
          boxShadow: `inset 0px 2px 0 0 color-mix(in oklab, var(--color-white) 15%, transparent),
                    0 2px 0 0 color-mix(in oklab, var(--color-black) 20%, transparent),
                    0 0 0 2px var(--color-black)`,
          ...style,
        }}
        className={clsx(
          "group z-20 bg-gray-800 rounded-lg text-gray-200",

          "data-[state=open]:data-[side=bottom]:animate-slide-down-and-fade",
          "data-[state=closed]:data-[side=bottom]:animate-slide-down-and-fade-reverse",

          "data-[state=open]:data-[side=left]:animate-slide-left-and-fade",
          "data-[state=closed]:data-[side=left]:animate-slide-left-and-fade-reverse",
          "data-[state=open]:data-[side=right]:animate-slide-right-and-fade",
          "data-[state=closed]:data-[side=right]:animate-slide-right-and-fade-reverse",

          "data-[state=open]:data-[side=top]:animate-slide-up-and-fade",
          "data-[state=closed]:data-[side=top]:animate-slide-up-and-fade-reverse",
        )}
      >
        {children}
        <PublicDroppableArrow component="popover" />
      </Popover.Content>
    </Popover.Portal>
  );
}

type PopoverContentContainerProps = { children: ReactNode; className?: string };
export function PopoverContentContainer({ children, className }: PopoverContentContainerProps) {
  return (
    <div className={clsx("px-3 pb-3 text-sm", className && className)}>
      {children}
    </div>
  );
}
