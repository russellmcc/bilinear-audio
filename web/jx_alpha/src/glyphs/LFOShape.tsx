import { GLYPH_HEIGHT, GLYPH_Y_MARGIN } from "./constants";

export type Props = {
  /**
   * The value of the shape.
   */
  value: string;
};

const Square = () => (
  <svg width="22" height={GLYPH_HEIGHT} viewBox={`0 0 22 ${GLYPH_HEIGHT}`}>
    <path
      d={`M2 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN} L2 ${GLYPH_Y_MARGIN} L11 ${GLYPH_Y_MARGIN} L11 ${
        GLYPH_HEIGHT - GLYPH_Y_MARGIN
      } L20 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN} L20 ${GLYPH_Y_MARGIN}`}
      stroke="var(--fg-color)"
      strokeWidth="1"
      fill="none"
    />
  </svg>
);

const Sine = () => {
  const steps = 20;
  const startX = 2;
  const endX = 20;
  const minY = GLYPH_Y_MARGIN;
  const maxY = GLYPH_HEIGHT - GLYPH_Y_MARGIN;
  const centerY = (maxY + minY) / 2;
  const amplitude = (maxY - minY) / 2;

  let d = "";
  for (let i = 0; i <= steps; i++) {
    const t = i / steps;
    const x = startX + (endX - startX) * t;
    const y = centerY - amplitude * Math.sin(t * 2 * Math.PI);
    d += `${i === 0 ? "M" : "L"}${x} ${y}`;
  }

  return (
    <svg width="22" height={GLYPH_HEIGHT} viewBox={`0 0 22 ${GLYPH_HEIGHT}`}>
      <path d={d} stroke="var(--fg-color)" strokeWidth="1" fill="none" />
    </svg>
  );
};

export const LFOShape = ({ value }: Props) => {
  switch (value.toLowerCase()) {
    case "rand":
      return <span>RND</span>;
    case "square":
      return <Square />;
    case "sine":
      return <Sine />;
    default:
      return null;
  }
};

export default LFOShape;
