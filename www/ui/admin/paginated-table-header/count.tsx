import { memo, useMemo } from "react";

type PaginatedTableHeaderProps = {
  currentPage: number;
  itemsPerPage: number;
  itemsInCurrentPage: number;
  totalItems: number;
  subject: string;
};
export const PaginatedTableHeaderCount = memo(({
  currentPage,
  itemsInCurrentPage,
  itemsPerPage,
  subject,
  totalItems,
}: PaginatedTableHeaderProps) => {
  const itemsShown = useMemo(() => {
    return (currentPage - 1) * itemsPerPage + itemsInCurrentPage;
  }, [currentPage, itemsInCurrentPage, itemsPerPage]);

  return (
    <span className="font-light text-sm">
      Exibindo {itemsShown} de {totalItems} {subject}
    </span>
  );
});
