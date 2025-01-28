import clsx from "clsx";
import { memo } from "react";

import iconsSprite from "../assets/icons-sprite.png";

export type SpriteProps = {
  spriteUrl?: string;
  x: number;
  y: number;
  width: number;
  height: number;
  className?: string;
};

export const Sprite = memo(({
  height,
  width,
  x,
  y,
  spriteUrl = iconsSprite,
  className,
}: SpriteProps) => {
  return (
    <span
      style={{
        background: `url("${spriteUrl}") no-repeat ${x}px ${y}px`,
        width,
        height,
        minWidth: width,
        minHeight: height,
      }}
      className={clsx("block pixelated", className && className)}
    />
  );
});
