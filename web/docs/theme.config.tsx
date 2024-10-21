import { DocsThemeConfig, useConfig } from "nextra-theme-docs";
import Head from "./src/head";
const themeConfig: DocsThemeConfig = {
  project: {
    link: "https://github.com/russellmcc/bilinear-audio",
  },
  docsRepositoryBase:
    "https://github.com/russellmcc/bilinear-audio/tree/main/web/docs",
  logo: <b>Bilinear Audio</b>,
  feedback: {
    useLink: () => {
      const config = useConfig();
      const title = config.title;

      return `https://github.com/russellmcc/bilinear-audio/discussions/new?category=q-a&title=Feedback regarding ${title}`;
    },
  },
  head: Head,
  footer: {
    component: <></>,
  },
};

export default themeConfig;
