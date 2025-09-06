import { type InertiaLinkProps, Link } from "@inertiajs/react";
import { ArrowLeft } from "@phosphor-icons/react/dist/ssr/ArrowLeft";
import { ArrowRight } from "@phosphor-icons/react/dist/ssr/ArrowRight";
import clsx from "clsx";
import { type JSXElementConstructor, memo, useContext, useEffect, useMemo, useState } from "react";

import type { GetPaginationParams, Paginator } from "@/utils/paginator";

import type { PaginationArrowButtonProps } from "..";
import { PaginationContext } from "../context";

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

export const AdminPaginationArrowButton = memo(({ direction }: PaginationArrowButtonProps) => {
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

  const Icon = useMemo(() => (direction === "backward" ? ArrowLeft : ArrowRight), [direction]);

  if (!paginationContext) return null;

  return (
    <Button
      role="button"
      href={href ?? "#"}
      className={clsx(
        "h-8 aspect-square grid place-items-center rounded-sm",
        "transittion-all duration-300 text-gray-700",
        disabled
          ? "border border-gray-400"
          : "text-white bg-purple-500 hover:bg-purple-700 active:brightness-90",
      )}>
      <Icon size={16} weight="bold" />
    </Button>
  );
});
