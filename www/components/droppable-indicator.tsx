import { CaretDown } from "@phosphor-icons/react/dist/ssr/CaretDown";
import clsx from "clsx";

import { Sprite } from "@/components/sprite";

export function DroppableIndicator({ className }: { className?: string }) {
  return (
    <Sprite
      x={-184}
      y={-62}
      height={20}
      width={14}
      className={clsx(
        "rotate-90 group-data-[state=open]:-rotate-90 transition-[transform] duration-150",
        className && className,
      )}
    />
  );
}

export function AdminDroppableIndicator({ className }: { className?: string }) {
  return (
    <CaretDown
      size={14}
      weight="bold"
      className={clsx(
        "group-data-[state=open]:-rotate-180 transition-[transform] duration-150",
        "text-gray-700",
        className && className,
      )}
    />
  );
}
