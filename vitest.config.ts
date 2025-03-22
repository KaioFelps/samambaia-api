import swc from "unplugin-swc";
import { defineConfig } from "vitest/config";

import { tsconfigPathAliases } from "./vite.config";

export default defineConfig({
  test: {
    globals: true,
    root: "./",
  },
  plugins: [
    swc.vite({
      module: { type: "es6" },
    }),
  ],
  resolve: {
    alias: tsconfigPathAliases,
  },
  root: "./",
});
