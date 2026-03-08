import styles from "./ProductShowcase.module.css";

export type ProductShowcaseProps = {
  imageSrc: string;
  imageAlt: string;
  gradientFrom: string;
  gradientTo: string;
  gradientAngle?: number;
};

export const ProductShowcase = ({
  imageSrc,
  imageAlt,
  gradientFrom,
  gradientTo,
  gradientAngle = 122,
}: ProductShowcaseProps) => (
  <section
    className={styles.showcase}
    style={{
      backgroundImage: `linear-gradient(${String(gradientAngle)}deg, ${gradientFrom}, ${gradientTo})`,
    }}
  >
    <img className={styles.image} src={imageSrc} alt={imageAlt} />
  </section>
);

export default ProductShowcase;
