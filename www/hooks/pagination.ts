import { useEffect, useState } from "react";

import { Paginator, type PaginatorConstructorArgs } from "@/utils/paginator";

export type GetPaginatorArgs = Omit<PaginatorConstructorArgs, "url">;

export function getPaginator(args: GetPaginatorArgs) {
  const url = window.location.origin + window.location.pathname;
  const paginator = new Paginator({ url, ...args });
  return paginator;
}

export function useMemoizedPaginatorParameters({
  lastPage,
  align,
  currentPage,
  pageQuery,
  visibleButtons,
}: GetPaginatorArgs) {
  const [articlePaginator, setArticlePaginator] = useState<GetPaginatorArgs | null>();

  // biome-ignore lint/correctness/useExhaustiveDependencies: it must run only once
  useEffect(() => {
    if (articlePaginator?.currentPage === currentPage && articlePaginator?.lastPage === lastPage) {
      return;
    }

    setArticlePaginator({
      lastPage,
      visibleButtons,
      currentPage,
      align,
      pageQuery,
    } satisfies GetPaginatorArgs);
  }, [currentPage, lastPage, align, pageQuery, visibleButtons]);

  return articlePaginator!;
}

export function useMemoizedPaginationFilter<T = string>(key: T, value: string | number) {
  const [filter, setFilter] = useState<{ key: T; value: string | number } | null>(null);

  // biome-ignore lint/correctness/useExhaustiveDependencies: it must not rerender due to filter key or value changes
  useEffect(() => {
    if (key === filter?.key && value === filter?.value) return;
    setFilter({ key, value });
  }, [key, value]);

  return filter!;
}
