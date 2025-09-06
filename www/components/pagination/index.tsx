import { memo } from "react";

import { AdminPaginationArrowButton } from "./admin/arrow-button";
import { AdminPaginationButtons } from "./admin/buttons";
import { PaginationRoot } from "./root";

export type PaginationButtonProps = {
  page: number;
  link: string;
};

export type PaginationArrowButtonProps = {
  direction: "backward" | "forward";
};

const PaginationButtons = memo(({ admin = false }: { admin?: boolean }) => {
  if (admin) return <AdminPaginationButtons />;
  throw new Error("PaginationButtons has no non-admin variant implemented.");
});

const PaginationArrowButton = memo(
  ({ admin = false, ...props }: PaginationArrowButtonProps & { admin?: boolean }) => {
    if (admin) return <AdminPaginationArrowButton {...props} />;
    throw new Error("PaginationArrowButton has no non-admin variant implemented.");
  },
);

export default {
  Root: PaginationRoot,
  Buttons: PaginationButtons,
  ArrowButton: PaginationArrowButton,
};
