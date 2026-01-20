import type { Preview } from "@storybook/react-vite";
import { themes } from "storybook/theming";
import "../src/index.css";

const preview: Preview = {
  parameters: {
    docs: {
      theme: themes.dark,
    },
    backgrounds: {
      options: {
        background: { name: "background", value: "#0F1A20" },
        zone: { name: "zone", value: "#25283D" }
      }
    },
    actions: { argTypesRegex: "^on[A-Z].*" },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
  },

  initialGlobals: {
    backgrounds: {
      value: "zone"
    }
  }
};

export default preview;
