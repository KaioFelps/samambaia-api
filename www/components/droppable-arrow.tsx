import clsx from "clsx";

export const adminDroppableArrowProps = {
  className: clsx(
    "fill-purple-100 shadow-gray-400",
    "[filter:_drop-shadow(0_1px_0_var(--tw-shadow-color))]",
    "admin-droppable-arrow-bottom-drop-shadow",
  ),
  width: 20,
  height: 10,
};

export const publicDroppableArrowProps = {
  className: clsx(
    "fill-gray-800 group-data-[side=bottom]:fill-[#4f4f55]",
    "drop-shadow-[0_2px_0_black] public-droppable-arrow-bottom-drop-shadow",
  ),
  width: 20,
  height: 10,
};
