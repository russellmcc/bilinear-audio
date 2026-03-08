import styles from "./CardHero.module.css";
import Blob from "./Blob";

export type CardHeroProps = {
  colorA: string;
  colorB: string;
  imageSrc: string;
  imageAlt: string;
  rotateDegrees: number;
  shiftX?: number;
  shiftY?: number;
  newRelease?: boolean;
};

export const CardHero = ({
  colorA,
  colorB,
  imageSrc,
  imageAlt,
  rotateDegrees,
  shiftX,
  shiftY,
  newRelease,
}: CardHeroProps) => (
  <div className={styles.cardHeroOuter}>
    <div className={styles.cardHero}>
      <div
        style={{
          transform: `translate(${shiftX ?? 0}px, ${shiftY ?? 0}px) rotate(${rotateDegrees}deg) `,
        }}
      >
        <div className={styles.blobRight}>
          <Blob colorA={colorA} colorB={colorB} />
        </div>
        <div className={styles.blobLeft}>
          <Blob colorA={colorA} colorB={colorB} />
        </div>
      </div>
      <div className={styles.cardBlur}></div>
      <img src={imageSrc} alt={imageAlt} />
      {newRelease && <div className={styles.newRelease}>new release</div>}
      <div className={styles.hoverBlur}></div>
    </div>
  </div>
);

export default CardHero;
