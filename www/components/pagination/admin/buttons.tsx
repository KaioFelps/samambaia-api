import { memo, useContext } from "react";

import { PaginationContext } from "../context";
import { AdminPaginationButton } from "./button";

export const AdminPaginationButtons = memo(() => {
  const paginationContext = useContext(PaginationContext)!;

  if (!paginationContext) return null;

  const { paginator, ...getPaginationArgs } = paginationContext;

  getPaginationArgs.queryString ??= window.location.search;

  return (
    <div className="flex items-center gap-1">
      {paginator.getPagination(getPaginationArgs).map(({ link, page }) =>
        <AdminPaginationButton
          key={"pagination-btn-" + link}
          link={link}
          page={page}
        />)}

    </div>
  );
});
