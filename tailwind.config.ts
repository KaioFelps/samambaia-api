import typography from "@tailwindcss/typography";
import type { Config } from "tailwindcss";

import { generateReverseableAnimation, generateReverseableKeyFrame } from "./www/lib/tailwind";

export const tailwindConfig = {
  content: ["./www/**/*.{html,ts,tsx,js}"],
  theme: {
    colors: {
      black: "#000000",
      white: "#ffffff",

      gray: {
        100: "#F6F6F6",
        200: "#EFEFEF",
        250: "#E0E0E0",
        300: "#DDDDDD",
        400: "#C0C0C0",
        700: "#555555",
        800: "#303037",
      },
      blue: {
        200: "#E6F8FF",
        500: "#0997E8",
        700: "#0075FF",
      },

      green: {
        300: "#14FF48",
        500: "#27C13F",
        600: "#23AE39",
      },

      purple: {
        100: "#F5EAF9",
        300: "#E942E9",
        500: "#BC36D8",
        700: "#8E2EB8",
      },

      red: {
        700: "#CA5757",
        800: "#942D46",
      },

      yellow: {
        200: "#FCFF7E",
        400: "#FFD600",
        500: "#EFBA00",
        700: "#FFB800",
        900: "#9D4200",
      },
    },
    extend: {
      screens: {
        main: "1392px",
      },
      animation: {
        "top-bg": "topBgKeyframe 50s linear infinite",
        ...generateReverseableAnimation("slideDownAndFade", "400ms cubic-bezier(0.16, 1, 0.3, 1)"),
        ...generateReverseableAnimation("slideLeftAndFade", "400ms cubic-bezier(0.16, 1, 0.3, 1)"),
        ...generateReverseableAnimation("slideUpAndFade", "400ms cubic-bezier(0.16, 1, 0.3, 1)"),
        ...generateReverseableAnimation("slideRightAndFade", "400ms cubic-bezier(0.16, 1, 0.3, 1)"),
        ...generateReverseableAnimation("slideDownAndFade", "400ms cubic-bezier(0.16, 1, 0.3, 1)"),
      },

      keyframes: {
        topBgKeyframe: {
          "0%": {
            backgroundPositionX: "0px",
          },
          "100%": {
            backgroundPositionX: "1366px",
          },
        },
        ...generateReverseableKeyFrame("slideDownAndFade", {
          from: { opacity: "0", transform: "translateY(-2px)" },
          to: { opacity: "1", transform: "translateY(0)" },
        }),

        ...generateReverseableKeyFrame("slideLeftAndFade", {
          from: { opacity: "0", transform: "translateX(2px)" },
          to: { opacity: "1", transform: "translateX(0)" },
        }),
        ...generateReverseableKeyFrame("slideUpAndFade", {
          from: { opacity: "0", transform: "translateY(2px)" },
          to: { opacity: "1", transform: "translateY(0)" },
        }),
        ...generateReverseableKeyFrame("slideRightAndFade", {
          from: { opacity: "0", transform: "translateX(-2px)" },
          to: { opacity: "1", transform: "translateX(0)" },
        }),
      },
    },
  },
  plugins: [
    typography(),
  ],
} satisfies Config;

export const colors = tailwindConfig.theme.colors;

export default tailwindConfig;
