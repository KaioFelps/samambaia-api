import config from "@rocketseat/eslint-config/react.mjs";
import stylisticTs from "@stylistic/eslint-plugin-ts";
import simpleImportSort from "eslint-plugin-simple-import-sort";

export default [
  ...config,
  {
    plugins: {
      "@stylistic/ts": stylisticTs,
      ...config.plugins,
      "simple-import-sort": simpleImportSort,
    },
    rules: {
      ...config.rules,
      "simple-import-sort/imports": "error",
      "@stylistic/jsx-closing-bracket-location": "error",
      "@stylistic/jsx-closing-tag-location": "error",
      "@stylistic/jsx-first-prop-new-line": ["error", "multiprop"],
      "@stylistic/jsx-function-call-newline": ["error", "always"],
      "@stylistic/semi": ["error", "always"],
      "@stylistic/ts/semi": ["error", "always"],
      "@stylistic/jsx-quotes": ["error", "prefer-double"],
      "@stylistic/quotes": ["error", "double"],
      "@stylistic/jsx-max-props-per-line": ["error", {
        maximum: 1,
      }],
      "@stylistic/max-len": ["warn", 100],
      "no-unused-vars": ["warn", { argsIgnorePattern: "^_", varsIgnorePattern: "^_" }],
      "@typescript-eslint/no-unused-vars": ["warn", {
        argsIgnorePattern: "^_",
        varsIgnorePattern: "^_"
      }],
      "@stylistic/ts/member-delimiter-style": ["error", {
        multiline: {
          delimiter: "semi",
          requireLast: true,
        },
        singleline: {
          delimiter: "semi",
          requireLast: false,
        },
        multilineDetection: "brackets",
      }],
    },
  },
];
