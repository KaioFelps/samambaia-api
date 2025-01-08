import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import { ReactNode } from "react";

type DropdownTriggerProps = DropdownMenu.DropdownMenuTriggerProps & {
  children: ReactNode
};

export function DropdownTrigger({ children, ...rest }: DropdownTriggerProps) {
  return (
    <DropdownMenu.Trigger {...rest}>
      {children}
    </DropdownMenu.Trigger>
  );
}
