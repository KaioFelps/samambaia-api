import { Arrow as DropdownArrow } from "@radix-ui/react-dropdown-menu";
import { Arrow as PopoverArrow } from "@radix-ui/react-popover";
import { Arrow as SelectArrow } from "@radix-ui/react-select";
import clsx from "clsx";

const adminDroppableArrowProps = {
  className: clsx(
    "fill-purple-100 shadow-gray-400",
    "[filter:_drop-shadow(0_1px_0_var(--tw-shadow-color))]",
    "admin-droppable-arrow-bottom-drop-shadow",
  ),
  width: 12,
  height: 6,
};

const publicDroppableArrowProps = {
  className: clsx(
    "fill-gray-800 group-data-[side=bottom]:fill-[#4f4f55]",
    "drop-shadow-[0_2px_0_black] public-droppable-arrow-bottom-drop-shadow",
  ),
  width: 20,
  height: 10,
};

type DroppableComponent = "dropdown" | "popover" | "select";

export function AdminDroppableArrow({ component }: { component: DroppableComponent }) {
  const Arrow = {
    dropdown: DropdownArrow,
    popover: PopoverArrow,
    select: SelectArrow,
  }[component];

  return <Arrow {...adminDroppableArrowProps} />;
}

export function PublicDroppableArrow({ component }: { component: DroppableComponent }) {
  const Arrow = {
    dropdown: DropdownArrow,
    popover: PopoverArrow,
    select: SelectArrow,
  }[component];

  return <Arrow {...publicDroppableArrowProps} />;
}
