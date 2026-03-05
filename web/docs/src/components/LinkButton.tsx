import styles from "./LinkButton.module.css";
export type LinkButtonMode = "primary" | "secondary";

export type LinkButtonProps = {
  href: string;
  text: string;
  mode: LinkButtonMode;
};

const styleForMode = (mode: LinkButtonMode) => {
  switch (mode) {
    case "primary":
      return styles.primary;
    case "secondary":
      return styles.secondary;
  }
};

export const LinkButton = ({ href, text, mode }: LinkButtonProps) => (
  <a href={href} className={`${styles.linkButton} ${styleForMode(mode)}`}>
    {text}
  </a>
);
