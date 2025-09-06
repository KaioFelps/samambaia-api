import clsx from "clsx";

import { Sprite, type SpriteProps } from "@/components/sprite";

export type SummaryCardProps = {
  label: string;
  count: number;
  spriteCoords: SpriteProps;
};

export function SummaryCard({ count, label, spriteCoords }: SummaryCardProps) {
  return (
    <article
      key={`summary-card-${label}`}
      className={clsx(
        "rounded-lg shadow-purple-700/30 shadow-lg flex gap-3 p-6 flex-1",
        "from-purple-300 to-purple-700 bg-linear-to-br ring-inset ring-2 ring-yellow-200/30",
      )}>
      <Sprite
        {...spriteCoords}
        className="shadow-purple-100 [filter:_drop-shadow(4px_4px_10px_var(--tw-shadow-color))]"
      />
      <div>
        <strong
          style={{
            textShadow:
              "1px 1px 10px color-mix(in oklab, var(--color-purple-700) 20%, transparent)",
          }}
          className="font-black text-4xl leading-none text-purple-100">
          {count}
        </strong>
        <br />
        <span className="uppercase text-purple-100">{label}</span>
      </div>
    </article>
  );
}
