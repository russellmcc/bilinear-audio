import { DownloadButton, type DownloadButtonProps } from "./DownloadButton";
import styles from "./ProductHero.module.css";

export type ProductHeroProps = {
  imageSrc: string;
  title: string;
  tagline: string;
  downloads: DownloadButtonProps;
};

export const ProductHero = ({
  imageSrc,
  title,
  tagline,
  downloads,
}: ProductHeroProps) => (
  <section className={styles.hero}>
    <img className={styles.bgImage} src={imageSrc} alt="" />
    <div className={styles.overlay} />
    <div className={styles.content}>
      <h1>{title}</h1>
      <p>{tagline}</p>
      <DownloadButton {...downloads} />
    </div>
  </section>
);

export default ProductHero;
