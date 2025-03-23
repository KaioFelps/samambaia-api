import { CaretLeft } from "@phosphor-icons/react/dist/ssr/CaretLeft";
import { Item as PItem, ItemIndicator, ItemText } from "@radix-ui/react-select";
import clsx from "clsx";
import { memo } from "react";

type ItemProps = {
  label: string;
  value: string;
};

export const SelectItem = memo(({ label, value }: ItemProps) => {
  return (
    <PItem
      value={value}
      className={clsx(
        "data-[state=checked]:bg-purple-500/20",
        "flex items-center justify-between px-3 py-0.5 transition-all duration-75",
        "hover:bg-purple-500/10 active:bg-purple-500/20 rounded-md",
        "outline-hidden ring-purple-500/40 ring-inset ring-0 focus-visible:ring-2",
      )}
    >
      <ItemText className="outline-hidden">{label}</ItemText>
      <ItemIndicator>
        <CaretLeft
          size={12}
          weight="bold"
          className="text-purple-700"
        />
      </ItemIndicator>
    </PItem>
  );
});
