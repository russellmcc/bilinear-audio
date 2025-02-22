module.exports = {
  root: true,
  extends: ["eslint-config-custom", "plugin:@next/next/recommended"],
  parserOptions: {
    project: ["./tsconfig.json"],
    tsconfigRootDir: __dirname,
  },
  rules: {
    "react-refresh/only-export-components": "off",
  },
};
