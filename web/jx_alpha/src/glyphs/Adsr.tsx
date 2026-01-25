import { GLYPH_HEIGHT, GLYPH_Y_MARGIN } from "./constants";

export const Adsr = () => (
  <svg width="22" height={GLYPH_HEIGHT} viewBox={`0 0 22 ${GLYPH_HEIGHT}`}>
    <path
      d={`M2 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN} L6 ${GLYPH_Y_MARGIN} L10 4 L16 4 L20 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN}`}
      stroke="var(--fg-color)"
      strokeWidth="1"
      fill="none"
    />
  </svg>
);

export const AdsrInverted = () => (
  <svg width="22" height={GLYPH_HEIGHT} viewBox={`0 0 22 ${GLYPH_HEIGHT}`}>
    <path
      d={`M2 ${GLYPH_Y_MARGIN} L6 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN} L10 6 L16 6 L20 ${GLYPH_Y_MARGIN}`}
      stroke="var(--fg-color)"
      strokeWidth="1"
      fill="none"
    />
  </svg>
);

export default Adsr;
