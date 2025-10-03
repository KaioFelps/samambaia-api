import { memo } from "react";
import PublicButton from "@/components/button/public-button";
import { CorePaginationButton, type CorePaginationButtonProps } from "../button";

export const PublicPaginationButton = memo((props: CorePaginationButtonProps) => {
  return (
    <PublicButton.Default
      asChild
      className="text-sm font-rowdies px-3! py-1! button-like-disabled:opacity-50">
      <CorePaginationButton {...props} />
    </PublicButton.Default>
  );
});
