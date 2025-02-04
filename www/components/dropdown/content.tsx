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
          "data-[state=open]:data-[side=bottom]:animate-slideDownAndFade",
          "data-[state=closed]:data-[side=bottom]:animate-slideDownAndFadeReverse",
          //
          "data-[state=open]:data-[side=left]:animate-slideLeftAndFade",
          "data-[state=closed]:data-[side=left]:animate-slideLeftAndFadeReverse",
          "data-[state=open]:data-[side=right]:animate-slideRightAndFade",
          "data-[state=closed]:data-[side=right]:animate-slideRightAndFadeReverse",
          //
          "data-[state=open]:data-[side=top]:animate-slideUpAndFade",
          "data-[state=closed]:data-[side=top]:animate-slideUpAndFadeReverse",
          className && className,
        )}
      >
        {children}
      </Dropdown.Content>
    </Dropdown.Portal>
  );
}
