import { Paginator, PaginatorConstructorArgs } from "@/utils/paginator";

export function usePaginator(args: Omit<PaginatorConstructorArgs, "url">) {
  const url = window.location.origin + window.location.pathname;
  const paginator = new Paginator({ url, ...args });
  return paginator;
}
