import { colors } from "@crate/tailwind.config";
import { Link } from "@inertiajs/react";
import clsx from "clsx";
import { ReactNode } from "react";

import Dropdown from "@/components/dropdown";
import { DropdownContentProps } from "@/components/dropdown/content";
import { PublicDroppableArrow } from "@/components/droppable-arrow";
import { colorWithOpacity } from "@/lib/tailwind";

type NavDropdownContentProps = Omit<DropdownContentProps, "className">;
type NavDropdownItemProps = { children: ReactNode; href: string };

export function NavDropdownContent({ children, ...rest }: NavDropdownContentProps) {
  return (
    <Dropdown.Content
      sideOffset={-3}
      style={{
        boxShadow: `inset 0px 2px 0 0 ${colorWithOpacity(colors.white, 15)},
                    0 2px 0 0 ${colorWithOpacity(colors.black, 20)}`,
      }}
      className="group z-20 bg-gray-800 border-2 border-black rounded-lg text-white font-bold"
      {...rest}
    >
      <div className="max-w-full max-h-full overflow-hidden flex flex-col min-w-52 select-none">
        {children}
      </div>

      <PublicDroppableArrow component="dropdown" />
    </Dropdown.Content>
  );
};

export function NavDropdownItem({ children, href }: NavDropdownItemProps) {
  return (
    <Link
      href={href}
      className={clsx(
        "px-4 py-2 shadow-[inset_0_2px_0_0] shadow-white/15 first:shadow-none",
        "border-b-2 last-of-type:border-b-0 border-black transition-all duration-100",
        "hover:pl-6 hover:bg-purple-500/10 hover:text-purple-100 active:bg-purple-500/20",
      )}
    >
      {children}
    </Link>
  );
}
