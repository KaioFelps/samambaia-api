import clsx from "clsx";
import { memo, type ReactNode } from "react";

type AdminDialogContainerProps = { children: ReactNode; className?: string };

export const AdminDialogContainer = memo(({ children, className }: AdminDialogContainerProps) => {
  return (
    <div
      className={clsx(
        "p-3 rounded-es-lg rounded-ee-lg bg-white shadow-sm shadow-black/5",
        className && className,
      )}>
      {children}
    </div>
  );
});
