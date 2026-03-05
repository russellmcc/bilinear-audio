import { defineConfig } from "astro/config";
import mdx from "@astrojs/mdx";
import react from "@astrojs/react";
export default defineConfig({
  site: "https://russellmcc.github.io",
  base: "/bilinear-audio",
  trailingSlash: "always",
  outDir: "out",
  integrations: [mdx(), react({ include: ["**/*.tsx"] })],
});
