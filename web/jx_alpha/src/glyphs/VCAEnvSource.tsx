import Adsr from "./Adsr";
import { GLYPH_HEIGHT, GLYPH_Y_MARGIN } from "./constants";
import { Dynamic } from "./EnvSource";

export type Props = {
  value: string;
};

const Gate = () => (
  <svg width="22" height={GLYPH_HEIGHT} viewBox={`0 0 22 ${GLYPH_HEIGHT}`}>
    <path
      d={`M2 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN} L4 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN} L4 ${GLYPH_Y_MARGIN} L16 ${GLYPH_Y_MARGIN} L16 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN} L20 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN}`}
      stroke="var(--fg-color)"
      strokeWidth="1"
      fill="none"
    />
  </svg>
);

export const VCAEnvSource = ({ value }: Props) => {
  const isDynamic = value.toLowerCase().includes("dynamic");
  const isEnv2 = value.toLowerCase().includes("env2");

  return (
    <div
      style={{
        display: "inline-flex",
        alignItems: "center",
        justifyContent: "flex-end",
      }}
    >
      {isDynamic && <Dynamic />}
      {isEnv2 ? <Adsr /> : <Gate />}
    </div>
  );
};

export default VCAEnvSource;
