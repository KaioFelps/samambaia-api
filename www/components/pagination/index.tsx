import { type JSXElementConstructor, memo } from "react";

import { AdminPaginationArrowButton } from "./admin/arrow-button";
import { AdminPaginationButton } from "./admin/button";
import type { CorePaginationArrowButtonProps } from "./arrow-button";
import type { CorePaginationButtonProps } from "./button";
import { CorePaginationButtons, type CorePaginationButtonsProps } from "./buttons";
import { PaginationContainer } from "./container";
import { PublicPaginationArrowButton } from "./public/arrow-button";
import { PublicPaginationButton } from "./public/button";
import { PaginationRoot } from "./root";

const PaginationButtons = memo(
  ({ admin = false, preserveScroll }: CorePaginationButtonsProps & { admin?: boolean }) => {
    let paginationButton: JSXElementConstructor<CorePaginationButtonProps>;
    if (admin) paginationButton = AdminPaginationButton;
    else paginationButton = PublicPaginationButton;

    return (
      <CorePaginationButtons paginationButton={paginationButton} preserveScroll={preserveScroll} />
    );
  },
);

const PaginationArrowButton = memo(
  ({ admin = false, ...props }: CorePaginationArrowButtonProps & { admin?: boolean }) => {
    if (admin) return <AdminPaginationArrowButton {...props} />;
    return <PublicPaginationArrowButton {...props} />;
  },
);

export default {
  Root: PaginationRoot,
  Container: PaginationContainer,
  Buttons: PaginationButtons,
  ArrowButton: PaginationArrowButton,
};
