import type { Preview } from "@storybook/react";
import { themes } from "@storybook/theming";
import "./example.css";

const preview: Preview = {
  parameters: {
    docs: {
      theme: themes.dark,
    },
    backgrounds: {
      default: "black",
      values: [{ name: "black", value: "#000000" }],
    },
    actions: { argTypesRegex: "^on[A-Z].*" },
  },
};

export default preview;
