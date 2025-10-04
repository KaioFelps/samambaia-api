import { memo, type ReactNode, useEffect, useState } from "react";

import { type GetPaginatorArgs, getPaginator } from "@/hooks/pagination";
import type { Paginator } from "@/utils/paginator";
import { PaginationContext, type PaginationContextProps } from "./context";

type PaginationRootProps = {
  children?: ReactNode;
  paginatorArgs?: GetPaginatorArgs;
  paginator?: Paginator;
  filter?: { key: string; value: string | number };
};

export const PaginationRoot = memo(
  ({ children, paginatorArgs, paginator, filter }: PaginationRootProps) => {
    const [contextValue, setContextValue] = useState<PaginationContextProps | null>(null);

    useEffect(() => {
      console.log(paginator, paginatorArgs);
      if (!paginator && !paginatorArgs)
        throw new Error(
          "Tried to instantiate a `PaginationRoot` without neither " +
            "an instance of paginators or args for instantiating it.",
        );

      setContextValue({
        paginator: paginator ?? getPaginator(paginatorArgs!),
        extraArgs: filter,
      });
    }, [paginator, paginatorArgs, filter]);

    return <PaginationContext.Provider value={contextValue}>{children}</PaginationContext.Provider>;
  },
);
