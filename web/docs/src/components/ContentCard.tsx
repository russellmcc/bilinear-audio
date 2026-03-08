import type { ReactNode } from "react";
import styles from "./ContentCard.module.css";

export type ContentCardProps = {
  children: ReactNode;
};

export const ContentCard = ({ children }: ContentCardProps) => (
  <div className={styles.card}>{children}</div>
);

export default ContentCard;
