import type { Preview } from "@storybook/react-vite";
import { themes } from "storybook/theming";
import "./example.css";

const preview: Preview = {
  parameters: {
    docs: {
      theme: themes.dark,
    },
    backgrounds: {
      options: {
        black: { name: "black", value: "#000000" },
      },
    },
    actions: { argTypesRegex: "^on[A-Z].*" },
    options: {
      urlArgs: false,
    },
  },

  initialGlobals: {
    backgrounds: {
      value: "black",
    },
  },
};

export default preview;
