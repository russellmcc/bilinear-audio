module.exports = {
  root: true,
  extends: ["eslint-config-custom", "plugin:tailwindcss/recommended"],
  parserOptions: {
    project: ["./tsconfig.json"],
    tsconfigRootDir: __dirname,
  },
  rules: {
    "tailwindcss/classnames-order": "off",
  },
};
