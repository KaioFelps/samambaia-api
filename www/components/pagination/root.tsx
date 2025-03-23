import clsx from "clsx";
import { memo, ReactNode, useEffect, useState } from "react";

import { getPaginator, GetPaginatorArgs } from "@/hooks/pagination";

import { PaginationContext, PaginationContextProps } from "./context";

type PaginationRootProps = {
  children?: ReactNode;
  className?: string;
  paginator: GetPaginatorArgs;
  filter?: { key: string; value: string | number };
};

export const PaginationRoot = memo(({
  children,
  className,
  paginator: paginatorParams,
  filter,
}: PaginationRootProps) => {
  const [contextValue, setContextValue] = useState<PaginationContextProps | null>(null);

  useEffect(() => {
    setContextValue({
      paginator: getPaginator(paginatorParams),
      extraArgs: filter,
    });
  }, [paginatorParams, filter]);

  return (
    <PaginationContext.Provider value={contextValue}>
      <div className={clsx("flex items-center gap-3", className && className)}>
        {children}
      </div>
    </PaginationContext.Provider>
  );
});
