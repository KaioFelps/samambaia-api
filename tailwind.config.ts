import type { Config } from "tailwindcss";

export default {
  content: ["./www/**/*.{html,ts,tsx,js}"],
  theme: {
    colors: {
      black: "#000000",
      white: "#ffffff",

      "gray-100": "#F6F6F6",
      "gray-200": "#EFEFEF",
      "gray-250": "#E0E0E0",
      "gray-300": "#DDDDDD",
      "gray-400": "#C0C0C0",
      "gray-700": "#555555",
      "gray-800": "#303037",

      "blue-200": "#E6F8FF",
      "blue-500": "#0997E8",
      "blue-700": "#0075FF",

      "green-300": "#14FF48",
      "green-500": "#27C13F",
      "green-600": "#23AE39",

      "purple-100": "#F5EAF9",
      "purple-300": "#E942E9",
      "purple-500": "#BC36D8",
      "purple-700": "#8E2EB8",

      "red-700": "#CA5757",
      "red-800": "#942D46",

      "yellow-200": "#FCFF7E",
      "yellow-400": "#FFD600",
      "yellow-500": "#EFBA00",
      "yellow-700": "#FFB800",
      "yellow-900": "#9D4200",
    },
    extend: {},
  },
  plugins: [],
} satisfies Config;
