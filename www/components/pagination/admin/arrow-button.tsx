import { ArrowLeftIcon } from "@phosphor-icons/react/dist/ssr/ArrowLeft";
import { ArrowRightIcon } from "@phosphor-icons/react/dist/ssr/ArrowRight";
import { Slot } from "@radix-ui/react-slot";
import clsx from "clsx";
import { memo, useMemo } from "react";
import { CorePaginationArrowButton, type CorePaginationArrowButtonProps } from "../arrow-button";

export const AdminPaginationArrowButton = memo(
  ({ direction, ...props }: CorePaginationArrowButtonProps) => {
    const Icon = useMemo(
      () => (direction === "backward" ? ArrowLeftIcon : ArrowRightIcon),
      [direction],
    );

    return (
      <Slot
        {...props}
        className={clsx(
          "h-8 aspect-square grid place-items-center rounded-sm",
          "transittion-all duration-300 text-gray-700",
          "data-[state=disabled]:border data-[state=disabled]:border-gray-400",
          "data-[state=enabled]:text-white data-[state=enabled]:bg-purple-500",
          "data-[state=enabled]:hover:bg-purple-700 data-[state=enabled]:active:brightness-90",
        )}>
        <CorePaginationArrowButton direction={direction} icon={<Icon size={16} weight="bold" />} />
      </Slot>
    );
  },
);
