import { type InertiaLinkProps, Link } from "@inertiajs/react";
import clsx from "clsx";
import { type JSXElementConstructor, memo, useContext, useEffect, useState } from "react";

import type { PaginationButtonProps } from "..";
import { PaginationContext } from "../context";

export const AdminPaginationButton = memo(({ link, page }: PaginationButtonProps) => {
  const [Button, setButton] = useState<JSXElementConstructor<InertiaLinkProps> | "div">("div");
  const [isActive, setIsActive] = useState(false);
  const { paginator } = useContext(PaginationContext)!;

  useEffect(() => {
    const isActive = paginator.getCurrentPage() === page;
    setIsActive(isActive);
    setButton(isActive ? "div" : Link);
  }, [paginator, page]);

  return (
    <Button
      role={!isActive ? "button" : undefined}
      href={link}
      className={clsx(
        "text-sm transition-all self-stretch h-8 px-2 aspect-square grid place-items-center",
        "font-rowdies rounded-sm text-purple-700 select-none",
        "outline-hidden ring-0 ring-purple-500/40 focus-visible:ring-4",
        !isActive && "bg-purple-700/20 hover:bg-purple-700/30 active:bg-purple-700/40",
        !isActive && "underline decoration-dotted decoration-2",
        isActive && "bg-purple-700/5",
      )}>
      {page}
    </Button>
  );
});
