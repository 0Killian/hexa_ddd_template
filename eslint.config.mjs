import globals from "globals";
import pluginJs from "@eslint/js";

export default [
  {
    ignores: [
      "prettierrc.cjs",
      "eslint.config.mjs",
      "tailwind.config.cjs",
      "assets/htmx.min.js",
      "assets/htmx-response-target.min.js",
    ],
  },
  {
    files: ["assets/*.js"],
    languageOptions: {
      sourceType: "script",
    },
  },
  {
    languageOptions: {
      globals: {
        ...globals.browser,
        htmx: "readonly",
      },
    },
  },
  pluginJs.configs.recommended,
];
