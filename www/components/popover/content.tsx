import { colors } from "@crate/tailwind.config";
import * as Popover from "@radix-ui/react-popover";
import clsx from "clsx";
import { ReactNode } from "react";

import { colorWithOpacity } from "@/lib/tailwind";

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
          boxShadow: `inset 0px 2px 0 0 ${colorWithOpacity(colors.white, 15)},
                    0 2px 0 0 ${colorWithOpacity(colors.black, 20)},
                    0 0 0 2px black`,
          ...style,
        }}
        className="group
            data-[state=open]:data-[side=bottom]:animate-slideDownAndFade
            data-[state=closed]:data-[side=bottom]:animate-slideDownAndFadeReverse

            data-[state=open]:data-[side=left]:animate-slideLeftAndFade
            data-[state=closed]:data-[side=left]:animate-slideLeftAndFadeReverse
            data-[state=open]:data-[side=right]:animate-slideRightAndFade
            data-[state=closed]:data-[side=right]:animate-slideRightAndFadeReverse

            data-[state=open]:data-[side=top]:animate-slideUpAndFade
            data-[state=closed]:data-[side=top]:animate-slideUpAndFadeReverse

            z-20 bg-gray-800 rounded-lg text-gray-200
            "
      >

        {children}

        <Popover.Arrow
          className="group-data-[side=bottom]:fill-[#4f4f55] fill-gray-800"
          width={20}
          height={10}
          style={{
            filter: `drop-shadow(-1px 1px 0 black)
                    drop-shadow(0 1px 0 black)
                    drop-shadow(1px 1px 0 black)`,
          }}
        />
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
