import clsx from "clsx";

export const adminDroppableArrowProps = {
  className: "fill-purple-100 shadow-gray-400",
  width: 20,
  height: 10,
  style: {
    filter: "drop-shadow(0 1px 0 var(--tw-shadow-color))",
  },
};

export const publicDroppableArrowProps = {
  className: clsx(
    "fill-gray-800 group-data-[side=bottom]:fill-[#4f4f55]",
    "group-data-[side=bottom]:drop-shadow-[0_2px_0_black]",
    "public-droppable-arrow-bottom-drop-shadow",
    "group-data-[side=top]:drop-shadow-[0_2px_0_black]",
    "group-data-[side=right]:drop-shadow-[0_2px_0_black]",
    "group-data-[side=left]:drop-shadow-[0_2px_0_black]",
  ),
  width: 20,
  height: 10,
};
