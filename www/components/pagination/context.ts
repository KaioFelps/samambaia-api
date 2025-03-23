import { createContext } from "react";

import { GetPaginationParams, Paginator } from "@/utils/paginator";

export type PaginationContextProps = {
  paginator: Paginator;
} & GetPaginationParams;

export const PaginationContext = createContext<PaginationContextProps | null>(null);
