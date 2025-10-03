import { type InertiaLinkProps, Link } from "@inertiajs/react";
import { type JSXElementConstructor, memo, useContext, useEffect, useState } from "react";
import { PaginationContext } from "./context";

type InertiaLinkConstructorProps = JSXElementConstructor<InertiaLinkProps>;

export type CorePaginationButtonProps = {
  page: number;
  link: string;
  preserveScroll?: boolean;
} & Partial<InertiaLinkConstructorProps>;

export const CorePaginationButton = memo(
  ({ link, page, preserveScroll, ...props }: CorePaginationButtonProps) => {
    const [Button, setButton] = useState<InertiaLinkConstructorProps | "div">("div");
    const [isActive, setIsActive] = useState(false);
    const { paginator } = useContext(PaginationContext)!;

    useEffect(() => {
      const isActive = paginator.getCurrentPage() === page;
      setIsActive(isActive);
      setButton(isActive ? "div" : Link);
    }, [paginator, page]);

    return (
      <Button
        {...props}
        disabled={!isActive}
        data-state={isActive ? "active" : "deactivated"}
        role={!isActive ? "button" : undefined}
        href={link}
        preserveScroll={preserveScroll}>
        {page}
      </Button>
    );
  },
);
