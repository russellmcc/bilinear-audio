import React from "react";
import Link from "next/link";
import styles from "./index.module.css";

type PlugCardProps = {
  title: string;
  logLine: string;
  href: string;
  gradientColors?: string; // New prop for gradient colors
};

const PlugCard: React.FC<PlugCardProps> = ({
  title,
  logLine,
  href,
  gradientColors,
}) => {
  const gradientStyle = {
    backgroundImage:
      gradientColors ?? "linear-gradient(45deg, #ff6b6b, #4ecdc4, #45b7d1)",
  };

  return (
    <Link href={href} className={styles.card}>
      <div className={styles.content}>
        <h3 className={styles.title}>{title}</h3>
        <p className={styles.logLine}>{logLine}</p>
      </div>
      <div className={styles.gradient} style={gradientStyle}></div>
    </Link>
  );
};

export default PlugCard;
