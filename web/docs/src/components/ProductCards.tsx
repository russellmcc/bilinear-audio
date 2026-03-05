import React from "react";
import styles from "./ProductCards.module.css";

export type ProductCardsProps = {
  children: React.ReactNode;
};

export const ProductCards = ({ children }: ProductCardsProps) => (
  <div className={styles.productCards}>{children}</div>
);

export default ProductCards;
