import { Icon } from "@phosphor-icons/react";
import clsx from "clsx";
import { memo } from "react";

type ChipProps = {
  text: string;
  className?: string;
  icon?: Icon;
  size: "sm";
};

export const Chip = memo(({
  text,
  size,
  className,
  icon: I,
}: ChipProps) => {
  return (
    <div className={clsx(
      "flex items-center gap-2 justify-center rounded-full px-2 py-1 bg-gray-300",
      className && className,
    )}
    >
      {I && (
        <I
          size={16}
          className="text-black"
        />
      )}
      <span className={clsx(
        "font-light text-black",
        size === "sm" && "text-sm leading-none",
      )}
      >
        {text}
      </span>
    </div>
  );
});
