import { defineConfig } from "oxlint";
import core from "ultracite/oxlint/core";
import next from "ultracite/oxlint/next";
import react from "ultracite/oxlint/react";

export default defineConfig({
  extends: [core, react, next],
  ignorePatterns: [
    ...(core.ignorePatterns ?? []),
    "**/src/generated/**",
    "**/src/env.ts",
  ],
  overrides: [
    {
      files: ["apps/native/*.config.js"],
      rules: { "node/global-require": "off", "unicorn/prefer-module": "off" },
    },
    {
      files: ["apps/native/**/*.tsx"],
      rules: {
        "react/no-unstable-nested-components": [
          "error",
          { allowAsProps: true },
        ],
      },
    },
  ],
  // Preserve the generators' named functions alongside arrow components.
  rules: {
    "func-style": ["error", "declaration", { allowArrowFunctions: true }],
    "react/function-component-definition": [
      "error",
      { namedComponents: ["function-declaration", "arrow-function"] },
    ],
  },
});
