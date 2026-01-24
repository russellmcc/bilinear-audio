import { randomLcg } from "d3-random";

export type Props = {
  /**
   * The value of the shape.
   */
  value: string;
};

const GLYPH_HEIGHT = 10;
const GLYPH_Y_MARGIN = 1;

const Saw = () => (
  <svg width="22" height={GLYPH_HEIGHT} viewBox={`0 0 22 ${GLYPH_HEIGHT}`}>
    <path
      d={`M2 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN} L20 ${GLYPH_Y_MARGIN} L20 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN}`}
      stroke="var(--fg-color)"
      strokeWidth="1"
      fill="none"
    />
  </svg>
);

const Pulse = () => (
  <svg width="22" height={GLYPH_HEIGHT} viewBox={`0 0 22 ${GLYPH_HEIGHT}`}>
    <path
      d={`M2 ${GLYPH_Y_MARGIN} L2 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN} L15.5 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN} L15.5 ${GLYPH_Y_MARGIN} L20 ${GLYPH_Y_MARGIN} L20 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN}`}
      stroke="var(--fg-color)"
      strokeWidth="1"
      fill="none"
    />
  </svg>
);

const PwmSaw = () => {
  const steps = 4;
  const startX = 2;
  const endX = 20;
  const startY = GLYPH_HEIGHT - GLYPH_Y_MARGIN;
  const endY = GLYPH_Y_MARGIN;

  if (steps % 2 !== 0) {
    throw new Error("Steps must be even");
  }

  let d = `M${startX} ${startY}`;
  for (let i = 1; i <= steps; i += 2) {
    const t0 = i / steps;
    const x0 = startX + (endX - startX) * t0;
    const y0 = startY + (endY - startY) * t0;
    const t1 = (i + 1) / steps;
    const x1 = startX + (endX - startX) * t1;
    const y1 = startY + (endY - startY) * t1;
    d += ` L${x0} ${startY} L${x0} ${y0} L${x1} ${y1} L${x1} ${startY}`;
  }
  d += ` L${endX} ${startY + 0.5}`;

  return (
    <svg width="22" height={GLYPH_HEIGHT} viewBox={`0 0 22 ${GLYPH_HEIGHT}`}>
      <path d={d} stroke="var(--fg-color)" strokeWidth="1" fill="none" />
    </svg>
  );
};

const CombSaw = () => {
  const steps = 8;
  const startX = 2;
  const endX = 20;
  const startY = GLYPH_HEIGHT - GLYPH_Y_MARGIN;
  const endY = GLYPH_Y_MARGIN;

  let d = `M${startX} ${startY} L${endX + 0.5} ${startY}`;

  for (let i = 1; i <= steps; i++) {
    const t = i / steps;
    const x = startX + (endX - startX) * t;
    const y = startY + (endY - startY) * t;
    d += ` M${x} ${startY} L${x} ${y}`;
  }

  return (
    <svg width="22" height={GLYPH_HEIGHT} viewBox={`0 0 22 ${GLYPH_HEIGHT}`}>
      <path d={d} stroke="var(--fg-color)" strokeWidth="1" fill="none" />
    </svg>
  );
};

const Noise = () => {
  const steps = 18;
  const startX = 2;
  const endX = 20;
  const minY = GLYPH_Y_MARGIN;
  const maxY = GLYPH_HEIGHT - GLYPH_Y_MARGIN;
  const centerY = (maxY + minY) / 2;
  const range = maxY - minY;

  const rng = randomLcg(0.1337);
  let d = `M${startX} ${centerY}`;
  for (let i = 1; i <= steps; i++) {
    const t = i / steps;
    const scale = Math.pow(Math.sin((t * Math.PI * 2) / 2), 0.1);
    const x = startX + (endX - startX) * t;
    const y = centerY + (rng() * range - range / 2) * scale;
    d += ` L${x} ${y}`;
  }

  return (
    <svg width="22" height={GLYPH_HEIGHT} viewBox={`0 0 22 ${GLYPH_HEIGHT}`}>
      <path d={d} stroke="var(--fg-color)" strokeWidth="1" fill="none" />
    </svg>
  );
};

export const Shape = ({ value }: Props) => {
  switch (value) {
    case "SAW":
      return <Saw />;
    case "PULSE":
      return <Pulse />;
    case "PWMSAW":
      return <PwmSaw />;
    case "COMBSAW":
      return <CombSaw />;
    case "NOISE":
      return <Noise />;
    default:
      return null;
  }
};

export default Shape;
