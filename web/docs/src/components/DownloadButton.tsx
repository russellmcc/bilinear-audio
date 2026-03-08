import styles from "./LinkButton.module.css";

export type DownloadButtonProps = {
  macUrl: string;
  windowsUrl: string;
};

export const DownloadButton = ({ macUrl, windowsUrl }: DownloadButtonProps) => (
  <a
    className={`${styles.linkButton} ${styles.primary} download-link`}
    data-mac-url={macUrl}
    data-win-url={windowsUrl}
    href={macUrl}
    hidden
  >
    download
  </a>
);
