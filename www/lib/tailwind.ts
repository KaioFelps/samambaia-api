import plugin from "tailwindcss/plugin";
import { CSSRuleObject, CustomThemeConfig } from "tailwindcss/types/config";

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
  keyframe: { from: object; to: object },
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

function hexToRgb(hex: string, opacity: number) {
  const cleanHex = hex.replace(/^#/, "");
  const bigint = parseInt(cleanHex, 16);
  const r = (bigint >> 16) & 255;
  const g = (bigint >> 8) & 255;
  const b = bigint & 255;
  return `rgb(${r} ${g} ${b} / ${opacity / 100})`;
}

export function textShadow(options?: { extras?: number }) {
  const extras = options?.extras ?? 0;

  return plugin(function ({ addUtilities, theme, e }) {
    const colors = theme("colors"); // Access the colors from the theme
    if (!colors) return;

    const generateUtilities = (class_: string, var_: string): [CSSRuleObject, CSSRuleObject] => {
      const opacities = [5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 60, 70, 75, 80, 90];

      const newUtilities: CSSRuleObject = {};
      const newUtilitiesWithOpacity: CSSRuleObject = {};

      for (const [colorName, shades] of Object.entries(colors)) {
        if (typeof shades === "string") {
          // Handle flat colors (e.g., black, white)
          newUtilities[`.${(class_)}-${e(colorName)}`] = {
            [`--tw-${var_}`]: hexToRgb(shades, 100),
          };

          for (const opacity of opacities) {
            newUtilitiesWithOpacity[`.${(class_)}-${e(colorName + "/" + opacity)}`] = {
              [`--tw-${var_}`]: hexToRgb(shades, opacity),
            };
          }
        } else {
          // Handle shades (e.g., blue-100, green-200)
          for (const [shade, value] of Object.entries(shades)) {
            newUtilities[`.${(class_)}-${e(colorName)}-${shade}`] = {
              [`--tw-${var_}`]: hexToRgb(value as string, 100),
            };

            for (const opacity of opacities) {
              newUtilitiesWithOpacity[`.${(class_)}-${e(`${colorName}-${shade}/${opacity}`)}`] = {
                [`--tw-${var_}`]: hexToRgb(value as string, opacity),
              };
            }
          }
        }
      }

      return [newUtilities, newUtilitiesWithOpacity];
    };

    const utilities = [...generateUtilities("text-shadow", "text-shadow-color")];

    for (let i = 0; i < extras; i++) {
      utilities.push(...generateUtilities(
        `text-shadow-extra-${i + 1}`,
        `text-shadow-color-extra-${i + 1}`,
      ));
    }

    addUtilities(utilities);
  });
}
