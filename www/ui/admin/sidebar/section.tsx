import type { Icon } from "@phosphor-icons/react";
import * as Accordion from "@radix-ui/react-accordion";
import clsx from "clsx";
import type { ReactNode } from "react";

import { useCanSee } from "@/hooks/useCanSee";
import type { TPermission } from "@/types/auth";
import type { CanMode } from "@/utils/can";

import { SidebarMenuSectionTrigger } from "./trigger";

type SidebarMenuProps = {
  children: ReactNode;
  label: string;
  icon: Icon;
  requires?: { permissions: TPermission[]; mode?: CanMode };
};

export const SidebarMenuSection = ({ label, icon, requires, children }: SidebarMenuProps) => {
  const isVisible = useCanSee(requires?.permissions, requires?.mode);

  if (!isVisible) return null;

  return (
    <Accordion.Item value={label}>
      <SidebarMenuSectionTrigger title={label} icon={icon} />
      <Accordion.Content
        className={clsx(
          "overflow-hidden",
          "data-[state=open]:animate-radix-accordion-slide",
          "data-[state=closed]:animate-radix-accordion-slide-reverse",
        )}>
        {children}
      </Accordion.Content>
    </Accordion.Item>
  );
};
