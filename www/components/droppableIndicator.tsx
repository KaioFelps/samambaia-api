import clsx from "clsx";

import { Sprite } from "@/components/sprite";

export function DroppableIndicator({ className }: { className?: string }) {
  return (
    <Sprite
      x={-185}
      y={-62}
      height={20}
      width={13}
      className={clsx(
        "rotate-90 group-data-[state=open]:-rotate-90 transition-[transform] duration-150",
        className && className,
      )}
    />
  );
}
