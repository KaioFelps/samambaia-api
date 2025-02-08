import { Link, usePage } from "@inertiajs/react";
import { type Icon } from "@phosphor-icons/react";
import clsx from "clsx";
import { memo, useMemo } from "react";

import { useCanSee } from "@/hooks/useCanSee";
import { TPermission } from "@/types/auth";

export type SidebarMenuLinkProps = {
  label: string;
  href: string;
  icon: Icon;
  requires?: TPermission[] | TPermission;
};

export const SidebarMenuLink = memo(({ icon: I, label, href, requires }: SidebarMenuLinkProps) => {
  const path = usePage().url;
  const isActive = useMemo(() => {
    return path === href;
  }, [href, path]);

  const isVisible = useCanSee(requires);

  if (!isVisible) return null;

  return (
    <Link
      href={href}
      data-active={isActive}
      className={clsx(
        "transition-all duration-150 flex items-center gap-2 px-2 py-1 rounded-lg cursor-default",
        "data-[active=false]:hover:bg-purple-500/10 data-[active=false]:active:bg-purple-500/20",
        "data-[active=false]:active:cursor-pointer data-[active=true]:bg-purple-500/10",
        "relative ml-1 before:absolute before:inset-y-full before:left-0 before:rounded-full",
        "before:w-0.5 before:bg-purple-500 before:-ml-1 data-[active=true]:before:inset-y-0",
        "before:transition-all before:will-change-[inset-y,_inset] before:duration-75",
      )}
    >
      <I
        size={20}
        weight="bold"
        className="text-purple-500"
      />
      {label}
    </Link>
  );
});
