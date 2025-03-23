import { createContext } from "react";

import { Paginator } from "@/utils/paginator";

type PaginationContextProps = {
  paginator: Paginator;
};

export const PaginationContext = createContext<PaginationContextProps | null>(null);
