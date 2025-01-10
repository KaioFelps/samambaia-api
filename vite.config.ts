import react from "@vitejs/plugin-react";
import laravel from "laravel-vite-plugin";
import path from "path";
import type { UserConfig } from "vite";

import tsconfig from "./tsconfig.json";

const tsconfigPathAliases = Object.fromEntries(
  Object.entries(tsconfig.compilerOptions.paths).map(([key, values]) => {
    let value = values[0];
    if (key.endsWith("/*")) {
      key = key.slice(0, -2);
      value = value.slice(0, -2);
    }

    const nodeModulesPrefix = "node_modules/";
    if (value.startsWith(nodeModulesPrefix)) {
      value = value.replace(nodeModulesPrefix, "");
    } else {
      value = path.join(__dirname, value);
    }

    return [key, value];
  }),
);

export default {
  plugins: [
    react(),
    laravel({
      input: ["www/app.tsx"],
      buildDirectory: "bundle",
      refresh: "www/**",
      ssrOutputDirectory: "dist/ssr",
      ssr: "www/ssr.tsx",
    }),
  ],
  publicDir: "/public",
  resolve: {
    alias: tsconfigPathAliases,
  },
  server: {
    watch: {
      ignored: ["*"],
    },
  },
} satisfies UserConfig;
