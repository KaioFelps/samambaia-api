import { CustomThemeConfig } from "tailwindcss/types/config";

function toHexaDecimal(percentage: number) {
  if (percentage < 0 || percentage > 100) {
    throw new Error("Percentage must be between 0 and 100.");
  }
  const decimal = Math.round((percentage / 100) * 255);
  return decimal.toString(16).padStart(2, "0").toUpperCase();
}

export function colorWithOpacity(
  color: string,
  opacity?: number,
) {
  if (opacity) {
    return color + toHexaDecimal(opacity);
  }
  return color;
}

export function generateReverseableKeyFrame(
  name: string,
  keyframe: { from: object, to: object },
): CustomThemeConfig["extend"]["keyframes"] {
  return {
    [name]: keyframe,
    [name + "Reverse"]: { from: keyframe.to, to: keyframe.from },
  };
}

export function generateReverseableAnimation(
  keyframe: string,
  animation: string,
): Record<string, string> {
  return {
    [keyframe]: `${keyframe} ${animation}`,
    [keyframe + "Reverse"]: `${keyframe + "Reverse"} ${animation}`,
  };
}
