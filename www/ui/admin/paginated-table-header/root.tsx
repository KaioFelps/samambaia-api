import clsx from "clsx";
import { memo, ReactNode } from "react";

type PaginatedTableHeaderRootProps = {
  children?: ReactNode;
  className?: string;
};

export const PaginatedTableHeaderRoot = memo(({
  children,
  className,
}: PaginatedTableHeaderRootProps) => {
  return (
    <div className={clsx(
      "bg-white/50 px-4 py-3 rounded-md flex gap-2 items-center justify-between",
      "border border-gray-250 shadow-md shadow-black/5",
      className && className,
    )}
    >
      {children}
    </div>
  );
});
