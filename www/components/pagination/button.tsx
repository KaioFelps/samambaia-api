import { type InertiaLinkProps, Link } from "@inertiajs/react";
import { type JSXElementConstructor, memo, useContext, useEffect, useState } from "react";
import { PaginationContext } from "./context";

type InertiaLinkConstructorProps = JSXElementConstructor<InertiaLinkProps>;

export type CorePaginationButtonProps = {
  page: number;
  link: string;
  preserveScroll?: boolean;
} & Partial<InertiaLinkConstructorProps>;

export const CorePaginationButton = memo(
  ({ link, page, preserveScroll, ...props }: CorePaginationButtonProps) => {
    const [isActive, setIsActive] = useState(false);
    const { paginator } = useContext(PaginationContext)!;

    useEffect(() => {
      const isActive = paginator.getCurrentPage() === page;
      setIsActive(isActive);
    }, [paginator, page]);

    if (isActive)
      return (
        <button {...props} disabled data-disabled="disabled">
          {page}
        </button>
      );

    return (
      <Link
        {...props}
        preserveScroll={preserveScroll}
        data-disabled="enabled"
        role="button"
        href={link}>
        {page}
      </Link>
    );
  },
);
