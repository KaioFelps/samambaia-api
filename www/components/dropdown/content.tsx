import * as Dropdown from "@radix-ui/react-dropdown-menu";
import clsx from "clsx";
import { ReactNode } from "react";

export type DropdownContentProps = Dropdown.DropdownMenuContentProps & {
  children: ReactNode;
};

export function DropdownContent({ children, className, ...rest }: DropdownContentProps) {
  return (
    <Dropdown.Portal>
      <Dropdown.Content
        collisionPadding={24}
        {...rest}
        className={clsx(
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
      </Dropdown.Content>
    </Dropdown.Portal>
  );
}
