import type React from "react";
import { memo, useContext } from "react";
import type { CorePaginationButtonProps } from "./button";
import { PaginationContext } from "./context";

export type CorePaginationButtonsProps = {
  preserveScroll?: boolean;
};

type Props = CorePaginationButtonsProps & {
  paginationButton: React.JSXElementConstructor<CorePaginationButtonProps>;
};

export const CorePaginationButtons = memo(
  ({ preserveScroll, paginationButton: PaginationButton }: Props) => {
    const paginationContext = useContext(PaginationContext)!;

    if (!paginationContext) return null;

    const { paginator, ...getPaginationArgs } = paginationContext;

    getPaginationArgs.queryString ??= window.location.search;

    return (
      <div className="flex items-center gap-1">
        {paginator.getPagination(getPaginationArgs).map(({ link, page }) => (
          <PaginationButton
            key={`pagination-btn-${link}`}
            link={link}
            page={page}
            preserveScroll={preserveScroll}
          />
        ))}
      </div>
    );
  },
);
