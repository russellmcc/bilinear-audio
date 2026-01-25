export type Props = {
  value: string;
};

const GLYPH_HEIGHT = 10;
const GLYPH_Y_MARGIN = 1;

const Adsr = () => (
  <svg width="22" height={GLYPH_HEIGHT} viewBox={`0 0 22 ${GLYPH_HEIGHT}`}>
    <path
      d={`M2 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN} L6 ${GLYPH_Y_MARGIN} L10 4 L16 4 L20 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN}`}
      stroke="var(--fg-color)"
      strokeWidth="1"
      fill="none"
    />
  </svg>
);

const AdsrInverted = () => (
  <svg width="22" height={GLYPH_HEIGHT} viewBox={`0 0 22 ${GLYPH_HEIGHT}`}>
    <path
      d={`M2 ${GLYPH_Y_MARGIN} L6 ${GLYPH_HEIGHT - GLYPH_Y_MARGIN} L10 6 L16 6 L20 ${GLYPH_Y_MARGIN}`}
      stroke="var(--fg-color)"
      strokeWidth="1"
      fill="none"
    />
  </svg>
);

export const EnvSource = ({ value }: Props) => {
  // Special case: "dynamic" should say DYN
  if (value.toLowerCase() === "dynamic") {
    return <span>DYN</span>;
  }
  const isDynamic = value.toLowerCase().includes("dynamic");
  const isInverse = value.toLowerCase().includes("inverse");
  const isEnv = value.toLowerCase().includes("env");

  return (
    <div
      style={{
        display: "inline-flex",
        alignItems: "center",
        justifyContent: "flex-end",
        height: "100%",
        verticalAlign: "middle",
      }}
    >
      {isDynamic && <span style={{ marginRight: isEnv ? 4 : 0 }}>D</span>}
      {isEnv && (isInverse ? <AdsrInverted /> : <Adsr />)}
    </div>
  );
};

export default EnvSource;
