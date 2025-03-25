import clsx from "clsx";
import { HTMLAttributes } from "react";

type MainProps = HTMLAttributes<HTMLElement> & {
  admin?: boolean;
};

export const Main = ({ admin = false, className, ...props }: MainProps) => {
  return (
    <main
      className={clsx(
        admin
          ? "admin-main-container"
          : "",
        className && className,
      )}
      {...props}
    />
  );
};
