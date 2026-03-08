import styles from "./ProductCard.module.css";
import CardHero, { type CardHeroProps } from "./CardHero";
import { TypeTag } from "./TypeTag";
import { LinkButton } from "./LinkButton";
import { DownloadButton, type DownloadButtonProps } from "./DownloadButton";

export type ProductCardProps = {
  hero: CardHeroProps;
  title: string;
  description: string;
  type: string;
  wide?: boolean;
  downloads: DownloadButtonProps;
  learnMoreUrl: string;
};

export const ProductCard = ({
  hero,
  title,
  description,
  type,
  wide,
  downloads,
  learnMoreUrl,
}: ProductCardProps) => (
  <div
    className={
      `${styles.productCard}${wide ? ` ${styles.wide}` : ""}` + " card-hover"
    }
  >
    <CardHero {...hero} />
    <div className={styles.titleRow}>
      <h2>{title}</h2>
      <TypeTag type={type} />
    </div>
    <p>{description}</p>
    <div className={styles.buttonGroup}>
      <DownloadButton
        macUrl={downloads.macUrl}
        windowsUrl={downloads.windowsUrl}
      />
      <LinkButton href={learnMoreUrl} text="Learn more" mode="secondary" />
    </div>
  </div>
);

export default ProductCard;
