import styles from "./DownloadCard.module.css";

export type DownloadItem = {
  label: string;
  href: string;
};

export type DownloadCardProps = {
  title: string;
  downloads: readonly DownloadItem[];
};

export const DownloadCard = ({ title, downloads }: DownloadCardProps) => (
  <div className={styles.card}>
    <h3 className={styles.title}>{title}</h3>
    <ul className={styles.list}>
      {downloads.map((download) => (
        <li key={download.href}>
          <a href={download.href}>{download.label}</a>
        </li>
      ))}
      <li>
        Are we missing your favorite platform?{" "}
        <a href="https://github.com/russellmcc/bilinear-audio/discussions">
          Let us know
        </a>
      </li>
    </ul>
  </div>
);

export default DownloadCard;
