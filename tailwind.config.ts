import typography from "@tailwindcss/typography";
import type { Config } from "tailwindcss";

import {
  generateReverseableAnimation,
  generateReverseableKeyFrame,
  textShadow,
} from "./www/lib/tailwind";

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
        700: "#10A530",
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
        800: "#D3A200",
        900: "#9D4200",
      },
    },
    fontFamily: {
      default: ["Kanit", "ui-sans-serif", "system-ui"],
      rowdies: ["Rowdies", "Kanit", "ui-sans-serif", "system-ui"],
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
        ...generateReverseableAnimation("scaleUpAndFade", "400ms cubic-bezier(0.16, 1, 0.3, 1)"),
        ...generateReverseableAnimation(
          "dialogScaleUpAndFade",
          "400ms cubic-bezier(0.16, 1, 0.3, 1) both",
        ),
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
        ...generateReverseableKeyFrame("scaleUpAndFade", {
          from: { opacity: "0", transform: "scale(0.9)" },
          to: { opacity: "1", transform: "scale(1)" },
        }),
        ...generateReverseableKeyFrame("dialogScaleUpAndFade", {
          from: { opacity: "0", transform: "translate(-50%, -50%) scale(0.9)" },
          to: { opacity: "1", transform: "translate(-50%, -50%) scale(1)" },
        }),
      },
    },
  },
  plugins: [
    typography(),
    textShadow({ extras: 1 }),
  ],
} satisfies Config;

export const colors = tailwindConfig.theme.colors;

export default tailwindConfig;
