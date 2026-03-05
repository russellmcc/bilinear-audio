import { LinkButton } from "./LinkButton";
import styles from "./Nav.module.css";

const base = import.meta.env.BASE_URL;
export const Nav = () => (
  <nav className={styles.nav}>
    <a className={styles.logo} href={base}>
      Bilinear Audio
    </a>
    <div className={styles.menu}>
      <div className={styles.productsMenu}>
        <a href={`${base}#products`}>products</a>
        <div className={styles.dropdown}>
          <div className={styles.dropdownColumn}>
            <span className={styles.categoryLabel}>synths</span>
            <a href={`${base}docs/alpha-jx/`}>Alpha JX</a>
            <a href={`${base}docs/poly-81/`}>Poly 81</a>
          </div>
          <div className={styles.dropdownColumn}>
            <span className={styles.categoryLabel}>effects</span>
            <a href={`${base}docs/fluffyverb/`}>Fluffyverb</a>
            <a href={`${base}docs/chorus-r/`}>Chorus-R</a>
          </div>
        </div>
      </div>
      <a href={`${base}about/`}>about us</a>
    </div>
    <LinkButton href={`${base}#products`} text="explore" mode="primary" />
  </nav>
);
export default Nav;
