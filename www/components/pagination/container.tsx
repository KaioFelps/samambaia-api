import clsx from "clsx";
import type { PropsWithChildren } from "react";

type Props = PropsWithChildren<{
  className?: string;
}>;

export function PaginationContainer({ children, className }: Props) {
  return <div className={clsx("flex items-center gap-3", className && className)}>{children}</div>;
}
