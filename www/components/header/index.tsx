import clsx from "clsx";
import { memo, type ReactNode } from "react";

type HeaderRootProps = {
  children: ReactNode;
  className?: string;
};

const HeaderRoot = memo(({ children, className }: HeaderRootProps) => (
  <header className={clsx("flex items-center gap-4", className && className)}>{children}</header>
));

const HeaderDivisor = memo(() => <hr className="border-none bg-gray-300 h-[1px] flex-1 shrink" />);

type HeaderTitleProps = {
  heading?: "h1" | "h2" | "h3" | "h4" | "h5" | "h6";
  children: ReactNode;
  className?: string;
};

const HeaderTitle = memo(({ heading: H = "h1", children, className }: HeaderTitleProps) => {
  return (
    <H className={clsx("font-medium text-black text-xl", className && className)}>{children}</H>
  );
});

type HeaderActionsProps = {
  children: ReactNode;
};

const HeaderActions = memo(({ children }: HeaderActionsProps) => {
  return <div className="flex items-center gap-2">{children}</div>;
});

export default {
  Root: HeaderRoot,
  Title: HeaderTitle,
  Actions: HeaderActions,
  Divisor: HeaderDivisor,
};
