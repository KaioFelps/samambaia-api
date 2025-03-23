import {
  Content as PContent,
  Portal,
  SelectContentProps as PContentProps,
} from "@radix-ui/react-select";
import clsx from "clsx";

import { AdminDroppableArrow } from "@/components/droppable-arrow";

type SelectContentProps = PContentProps;

export const SelectContent = ({
  children,
  className,
  position = "popper",
  align = "center",
  side = "bottom",
  sideOffset = 2,
  collisionPadding = 24,
  ...props
}: SelectContentProps) => {
  return (
    <Portal>
      <PContent
        {...props}
        position={position}
        align={align}
        side={side}
        sideOffset={sideOffset}
        collisionPadding={collisionPadding}
        className={clsx(
          "admin-dropdown-content flex flex-col gap-1",
          "group will-change-[opacity,transform]",
          "data-[state=open]:data-[side=bottom]:animate-slide-down-and-fade",
          "data-[state=closed]:data-[side=bottom]:animate-slide-down-and-fade-reverse",
          //
          "data-[state=open]:data-[side=left]:animate-slide-left-and-fade",
          "data-[state=closed]:data-[side=left]:animate-slide-left-and-fade-reverse",
          "data-[state=open]:data-[side=right]:animate-slide-right-and-fade",
          "data-[state=closed]:data-[side=right]:animate-slide-right-and-fade-reverse",
          //
          "data-[state=open]:data-[side=top]:animate-slide-up-and-fade",
          "data-[state=closed]:data-[side=top]:animate-slide-up-and-fade-reverse",

          className && className,
        )}
      >
        {children}
        <AdminDroppableArrow component="select" />
      </PContent>
    </Portal>
  );
};
