import clsx from "clsx";
import type { HTMLAttributes } from "react";

type MainProps = HTMLAttributes<HTMLElement> & {
  admin?: boolean;
};

export const Main = ({ admin = false, className, ...props }: MainProps) => {
  return (
    <main
      className={clsx(
        admin
          ? "admin-main-container"
          : "flex-1 max-w-main-center-content basis-main-center-content",
        className && className,
      )}
      {...props}
    />
  );
};
