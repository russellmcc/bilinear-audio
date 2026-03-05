import styles from "./TypeTag.module.css";
export type TypeTagProps = {
  type: string;
};

export const TypeTag = ({ type }: TypeTagProps) => (
  <div className={styles.typeTag}>{type}</div>
);
