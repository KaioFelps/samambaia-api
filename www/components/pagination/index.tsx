import clsx from "clsx";
import { memo, ReactNode } from "react";

import { usePaginator } from "@/hooks/usePaginator";
import { PaginatorConstructorArgs, SearchParameters } from "@/utils/paginator";

import { AdminPaginationArrowButton } from "./admin/arrow-button";
import { AdminPaginationButtons } from "./admin/buttons";
import { PaginationContext } from "./context";

export type PaginationButtonProps = {
  page: number;
  link: string;
};

export type PaginationArrowButtonProps = {
  direction: "backward" | "forward";
  extraArgs?: SearchParameters;
};

type PaginationRootProps = {
  children: ReactNode;
  paginator: Omit<PaginatorConstructorArgs, "url">;
  className?: string;
};

const PaginationButtons = memo(({ admin = false }: { admin?: boolean }) => {
  if (admin) return <AdminPaginationButtons />;
  throw new Error("PaginationButtons has no non-admin variant implemented.");
});

const PaginationArrowButton = memo(({
  admin = false,
  ...props
}: PaginationArrowButtonProps & { admin?: boolean }) => {
  if (admin) return <AdminPaginationArrowButton {...props} />;
  throw new Error("PaginationArrowButton has no non-admin variant implemented.");
});

const PaginationRoot = memo(({
  children,
  className,
  paginator: paginatorParams,

}: PaginationRootProps) => {
  const paginator = usePaginator(paginatorParams);

  return (
    <PaginationContext.Provider value={{ paginator }}>
      <div className={clsx("flex items-center gap-3", className && className)}>
        {children}
      </div>
    </PaginationContext.Provider>
  );
});

export default {
  Root: PaginationRoot,
  Buttons: PaginationButtons,
  ArrowButton: PaginationArrowButton,
};
