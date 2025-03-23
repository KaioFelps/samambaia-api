import { memo, useContext } from "react";

import { PaginationContext } from "../context";
import { AdminPaginationButton } from "./button";

export const AdminPaginationButtons = memo(() => {
  const { paginator } = useContext(PaginationContext)!;

  return (
    <div className="flex items-center gap-1">
      {paginator.getPagination().map(({ link, page }) =>
        <AdminPaginationButton
          key={"pagination-btn-" + link}
          link={link}
          page={page}
        />)}

    </div>
  );
});
