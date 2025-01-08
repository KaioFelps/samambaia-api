import iconsSprite from "../assets/icons-sprite.png";

type SpriteProps = {
  spriteUrl?: string;
  x: number;
  y: number;
  width: number;
  height: number;
};

export function Sprite({ height, width, x, y, spriteUrl = iconsSprite }: SpriteProps) {
  return (
    <span
      style={{
        background: `url("${spriteUrl}") no-repeat ${x}px ${y}px`,
        width,
        height,
        minWidth: width,
        minHeight: height,
      }}
      className="block pixelated"
    />
  );
}
