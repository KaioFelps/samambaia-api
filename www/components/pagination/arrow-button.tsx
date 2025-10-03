import { type InertiaLinkProps, Link } from "@inertiajs/react";
import {
  type JSXElementConstructor,
  memo,
  type ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";

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

    const [Button, setButton] = useState<JSXElementConstructor<InertiaLinkProps> | "div">("div");
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
      setButton(disabled ? "div" : Link);
    }, [direction, paginationContext]);

    if (!paginationContext) return null;

    return (
      <Button
        {...props}
        disabled={disabled}
        data-disabled={disabled ? "disabled" : "enabled"}
        role="button"
        href={href ?? "#"}
        preserveScroll={preserveScroll}>
        {icon}
      </Button>
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
