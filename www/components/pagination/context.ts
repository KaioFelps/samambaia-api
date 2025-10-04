import { createContext, useContext } from "react";

import type { GetPaginationParams, Paginator } from "@/utils/paginator";

export type PaginationContextProps = {
  paginator: Paginator;
} & GetPaginationParams;

export const PaginationContext = createContext<PaginationContextProps | null>(null);

export function usePaginator() {
  return useContext(PaginationContext)?.paginator;
}
