import type { ReactNode } from "react";
import styles from "./FeatureRow.module.css";

export type FeatureRowProps = {
  heading: string;
  children: ReactNode;
  imageSrc?: string;
  imageAlt?: string;
  videoSrc?: string;
  videoPosterSrc?: string;
  videoLabel?: string;
  reversed?: boolean;
};

export const FeatureRow = ({
  heading,
  children,
  imageSrc,
  imageAlt,
  videoSrc,
  videoPosterSrc,
  videoLabel,
  reversed,
}: FeatureRowProps) => {
  const hasMedia = Boolean(imageSrc ?? videoSrc);

  return (
    <section
      className={`${styles.row}${reversed ? ` ${styles.reversed}` : ""}`}
    >
      <div
        className={`${styles.imageContainer}${hasMedia ? "" : ` ${styles.placeholder}`}`}
      >
        {videoSrc ? (
          <video
            src={videoSrc}
            poster={videoPosterSrc}
            aria-hidden={videoLabel ? undefined : true}
            aria-label={videoLabel}
            autoPlay
            className={styles.image}
            loop
            muted
            playsInline
            preload="metadata"
          />
        ) : null}
        {!videoSrc && imageSrc ? (
          <img src={imageSrc} alt={imageAlt ?? ""} className={styles.image} />
        ) : null}
      </div>
      <div className={styles.text}>
        <h2>{heading}</h2>
        {children}
      </div>
    </section>
  );
};

export default FeatureRow;
