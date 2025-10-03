import { Link } from "@inertiajs/react";
import { memo, type ReactNode, useContext, useEffect, useState } from "react";

import type { GetPaginationParams, Paginator } from "@/utils/paginator";
import { PaginationContext } from "./context";

export type CorePaginationArrowButtonProps = {
  direction: "backward" | "forward";
  preserveScroll?: boolean;
};

type PaginationArrowButtonProps = CorePaginationArrowButtonProps & {
  icon: ReactNode;
};

export const CorePaginationArrowButton = memo(
  ({ direction, preserveScroll, icon, ...props }: PaginationArrowButtonProps) => {
    const paginationContext = useContext(PaginationContext);
    const buttonTitle = `Ir para ${direction === "backward" ? "a página anterior" : "a próxima página"}`;

    const [disabled, setDisabled] = useState(true);
    const [href, setHref] = useState<string | null>(null);

    useEffect(() => {
      if (!paginationContext) return;

      const { disabled, pagination } = getDirectionButtonProps(
        direction,
        paginationContext.paginator,
        paginationContext.extraArgs,
      );

      setDisabled(disabled);
      setHref(pagination?.link ?? null);
    }, [direction, paginationContext]);

    if (!paginationContext) return null;

    if (disabled)
      return (
        <button {...props} type="button" disabled={disabled} data-disabled="disabled">
          {icon}
          <span className="sr-only">{buttonTitle}</span>
        </button>
      );

    return (
      <Link
        {...props}
        role="button"
        data-disabled="enabled"
        href={href ?? "#"}
        preserveScroll={disabled ? undefined : preserveScroll}>
        {icon}
        <span className="sr-only">{buttonTitle}</span>
      </Link>
    );
  },
);

function getDirectionButtonProps(
  direction: PaginationArrowButtonProps["direction"],
  paginator: Paginator,
  params: GetPaginationParams = {},
) {
  let disabled: boolean;

  switch (direction) {
    case "backward":
      disabled = !paginator.hasPreviousPage();
      break;
    case "forward":
      disabled = !paginator.hasNextPage();
      break;
  }

  let page = null;

  if (!disabled) {
    switch (direction) {
      case "backward":
        page = paginator.getCurrentPage() - 1;
        break;
      case "forward":
        page = paginator.getCurrentPage() + 1;
        break;
    }
  }

  params.queryString ??= window.location.search;

  const pagination = page ? paginator.getPaginationLinkForPage(page, params) : null;

  return {
    disabled,
    pagination,
  };
}
