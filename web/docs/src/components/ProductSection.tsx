import type { ReactNode } from "react";
import styles from "./ProductSection.module.css";

export type ProductSectionProps = {
  children: ReactNode;
  compact?: boolean;
};

export const ProductSection = ({ children, compact }: ProductSectionProps) => (
  <section
    className={`${styles.section}${compact ? ` ${styles.compact}` : ""}`}
  >
    {children}
  </section>
);

export default ProductSection;
