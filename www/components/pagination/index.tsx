import { type JSXElementConstructor, memo } from "react";

import { AdminPaginationArrowButton } from "./admin/arrow-button";
import { AdminPaginationButton } from "./admin/button";
import type { CorePaginationArrowButtonProps } from "./arrow-button";
import type { CorePaginationButtonProps } from "./button";
import { CorePaginationButtons, type CorePaginationButtonsProps } from "./buttons";
import { PaginationRoot } from "./root";

const PaginationButtons = memo(
  ({ admin = false, preserveScroll }: CorePaginationButtonsProps & { admin?: boolean }) => {
    let paginationButton: JSXElementConstructor<CorePaginationButtonProps>;
    if (admin) paginationButton = AdminPaginationButton;
    else throw new Error("PaginationButtons has no non-admin variant implemented.");

    return (
      <CorePaginationButtons paginationButton={paginationButton} preserveScroll={preserveScroll} />
    );
  },
);

const PaginationArrowButton = memo(
  ({ admin = false, ...props }: CorePaginationArrowButtonProps & { admin?: boolean }) => {
    if (admin) return <AdminPaginationArrowButton {...props} />;
    throw new Error("PaginationArrowButton has no non-admin variant implemented.");
  },
);

export default {
  Root: PaginationRoot,
  Buttons: PaginationButtons,
  ArrowButton: PaginationArrowButton,
};
