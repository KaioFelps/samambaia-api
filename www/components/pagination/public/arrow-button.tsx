import { memo, useMemo } from "react";
import PublicButton from "@/components/button/public-button";
import { Sprite } from "@/components/sprite";
import { CorePaginationArrowButton, type CorePaginationArrowButtonProps } from "../arrow-button";

export const PublicPaginationArrowButton = memo(
  ({ direction, ...props }: CorePaginationArrowButtonProps) => {
    const icon = useMemo(
      () =>
        direction === "backward" ? (
          <Sprite x={-184} y={-62} height={20} width={14} />
        ) : (
          <Sprite x={-184} y={-62} height={20} width={14} className="rotate-180" />
        ),
      [direction],
    );

    return (
      <PublicButton.Default
        asChild
        variant="black"
        className="text-sm font-rowdies px-2.5! py-0! self-stretch button-like-disabled:opacity-50">
        <CorePaginationArrowButton direction={direction} icon={icon} {...props} />
      </PublicButton.Default>
    );
  },
);
