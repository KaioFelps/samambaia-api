import react from "@vitejs/plugin-react";
import laravel from "laravel-vite-plugin";
import { builtinModules } from "module";
import type { UserConfig } from "vite";
import tsConfigPaths from "vite-tsconfig-paths";

const allExternal = [
  ...builtinModules,
  ...builtinModules.map((m) => `node:${m}`),
];
export default {
  plugins: [
    laravel({
      input: ["www/app.tsx"],
      buildDirectory: "bundle",
      refresh: "www/**",
      ssrOutputDirectory: "dist/ssr",
      ssr: "www/ssr.tsx",
    }),
    tsConfigPaths(),
    react(),
  ],
  publicDir: "/public",
  build: {
    rollupOptions: {
      external: ["fsevents", ...allExternal],
    },
  },
} satisfies UserConfig;
