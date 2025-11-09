import { defineConfig } from "eslint/config";
import config from "eslint-config-custom";
import tailwind from "eslint-plugin-tailwindcss";

export default defineConfig([
  config,
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
