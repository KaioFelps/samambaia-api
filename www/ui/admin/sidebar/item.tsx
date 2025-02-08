import { Link, usePage } from "@inertiajs/react";
import { ArrowRight } from "@phosphor-icons/react/dist/ssr/ArrowRight";
import clsx from "clsx";
import { useMemo } from "react";

import { useCanSee } from "@/hooks/useCanSee";

import { SidebarMenuLinkProps } from "./link";

export const SidebarMenuItem = ({
  href,
  label,
  requires,
}: Omit<SidebarMenuLinkProps, "icon">) => {
  const pageSegment = usePage().url;
  const isActive = useMemo(() => href === pageSegment, [href, pageSegment]);
  const isVisible = useCanSee(requires);

  if (!isVisible) return null;

  return (
    <Link
      data-active={isActive}
      href={href}
      className={clsx(
        "group block w-full ml-1 px-2 py-1 bg-transparent rounded-lg mt-1 relative cursor-default",
        "data-[active=false]:active:bg-purple-700/10 data-[active=false]:hover:bg-purple-700/5",
        "data-[active=true]:bg-purple-700/5 data-[active=false]:active:cursor-pointer",
      )}
    >
      <span className="px-7">
        {label}
      </span>

      <ArrowRight className={clsx(
        "absolute right-2 top-1/2 -translate-y-1/2 w-0 transition-[width] duration-300",
        "group-data-[active=false]:group-hover:w-5",
      )}
      />
    </Link>
  );
};
