import * as Tooltip from "@radix-ui/react-tooltip";
import clsx from "clsx";
import { PublicDroppableArrow } from "../droppable-arrow";

export type TooltipContentProps = Omit<Tooltip.TooltipContentProps, "className">;

export function TooltipContentContainer({
  children,
  sideOffset = 4,
  style,
  ...rest
}: TooltipContentProps) {
  return (
    <Tooltip.Portal>
      <Tooltip.Content
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

          "data-[state=delayed-open]:data-[side=bottom]:animate-slide-down-and-fade",
          "data-[state=closed]:data-[side=bottom]:animate-slide-down-and-fade-reverse",

          "data-[state=delayed-open]:data-[side=left]:animate-slide-left-and-fade",
          "data-[state=closed]:data-[side=left]:animate-slide-left-and-fade-reverse",

          "data-[state=delayed-open]:data-[side=right]:animate-slide-right-and-fade",
          "data-[state=closed]:data-[side=right]:animate-slide-right-and-fade-reverse",

          "data-[state=delayed-open]:data-[side=top]:animate-slide-up-and-fade",
          "data-[state=closed]:data-[side=top]:animate-slide-up-and-fade-reverse",
        )}>
        {children}
        <PublicDroppableArrow component="tooltip" />
      </Tooltip.Content>
    </Tooltip.Portal>
  );
}
