import react from "@vitejs/plugin-react";

/** @type {import('vite').UserConfig} */
export default {
  plugins: [
    react({
      babel: {
        plugins: ["babel-plugin-react-compiler"],
      },
    }),
  ],
};
