// For more info, see https://github.com/storybookjs/eslint-plugin-storybook#configuration-flat-config-format
import storybook from "eslint-plugin-storybook";

import { defineConfig } from "eslint/config";
import config from "eslint-config-custom";
import tailwind from "eslint-plugin-tailwindcss";

export default defineConfig([
  config,
  ...storybook.configs["flat/recommended"],
  tailwind.configs["flat/recommended"],
  {
    files: ["**/*.ts", "**/*.tsx"],
    rules: {
      "tailwindcss/classnames-order": "off",
    },
    languageOptions: {
      parserOptions: {
        project: ["./tsconfig.json"],
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
]);
