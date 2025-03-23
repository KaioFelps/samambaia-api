import { InertiaLinkProps, Link } from "@inertiajs/react";
import { ArrowLeft } from "@phosphor-icons/react/dist/ssr/ArrowLeft";
import { ArrowRight } from "@phosphor-icons/react/dist/ssr/ArrowRight";
import clsx from "clsx";
import { JSXElementConstructor, memo, useContext, useEffect, useMemo, useState } from "react";

import { Paginator, SearchParameters } from "@/utils/paginator";

import { PaginationArrowButtonProps } from "..";
import { PaginationContext } from "../context";

function getDirectionButtonProps(
  direction: PaginationArrowButtonProps["direction"],
  paginator: Paginator,
  params: SearchParameters = {},
) {
  let disabled;

  switch (direction) {
    case "backward": disabled = !paginator.hasPreviousPage(); break;
    case "forward": disabled = !paginator.hasNextPage(); break;
  }

  let page = null;

  if (!disabled) {
    switch (direction) {
      case "backward": page = paginator.getCurrentPage() - 1; break;
      case "forward": page = paginator.getCurrentPage() + 1; break;
    }
  }

  return {
    disabled,
    pagination: page
      ? paginator.getPaginationLinkForPage(page, params)
      : null,
  };
}

export const AdminPaginationArrowButton = memo(({
  direction,
  extraArgs,
}: PaginationArrowButtonProps) => {
  const { paginator } = useContext(PaginationContext)!;

  const [Button, setButton] = useState<JSXElementConstructor<InertiaLinkProps> | "div">("div");
  const [disabled, setDisabled] = useState(true);
  const [href, setHref] = useState<string | null>(null);

  useEffect(() => {
    const { disabled, pagination } = getDirectionButtonProps(direction, paginator, extraArgs);

    setDisabled(disabled);
    setHref(pagination && pagination.link);
    setButton(disabled
      ? "div"
      : Link);
  }, [direction, extraArgs, paginator]);

  const Icon = useMemo(() => direction === "backward"
    ? ArrowLeft
    : ArrowRight, [direction]);

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
      )}
    >
      <Icon
        size={16}
        weight="bold"
      />
    </Button>
  );
});
