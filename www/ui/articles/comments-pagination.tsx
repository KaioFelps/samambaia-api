import Pagination from "@/components/pagination";
import { useMemoizedPaginatorParameters } from "@/hooks/pagination";
import type { Pagination as TPagination } from "@/types/pagination";

export function CommentsPagination({ totalPages, currentPage }: TPagination) {
  return (
    <div>
      <Pagination.Root
        paginator={
          useMemoizedPaginatorParameters({
            lastPage: totalPages,
            visibleButtons: 7,
            currentPage: currentPage,
            align: "left",
            pageQuery: "commentsPage",
          })!
        }
        className="mt-3 justify-end">
        <Pagination.ArrowButton preserveScroll direction="backward" />
        <Pagination.Buttons preserveScroll />
        <Pagination.ArrowButton preserveScroll direction="forward" />
      </Pagination.Root>
    </div>
  );
}
