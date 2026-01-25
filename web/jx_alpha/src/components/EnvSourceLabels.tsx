import { BALL_SIZE, LABEL_MARGIN } from "./constants";
import { LINE_SPACING, VALUE_LABEL_TOP_PADDING } from "./EnumSlider";

const BRACKET_WIDTH = 6;
const BRACKET_STROKE = 1;
const LABEL_OFFSET = 1;

const BrokenBracket = ({ height }: { height: number }) => {
  const midY = height / 2;
  const gapHalf = LINE_SPACING / 2;

  const dTop = `M 0 ${BRACKET_STROKE / 2} L ${BRACKET_WIDTH - BRACKET_STROKE / 2} ${BRACKET_STROKE / 2} L ${BRACKET_WIDTH - BRACKET_STROKE / 2} ${midY - gapHalf + VALUE_LABEL_TOP_PADDING}`;

  const dBottom = `M ${BRACKET_WIDTH - BRACKET_STROKE / 2} ${midY + gapHalf + VALUE_LABEL_TOP_PADDING} L ${BRACKET_WIDTH - BRACKET_STROKE / 2} ${height - BRACKET_STROKE / 2} L ${BRACKET_STROKE / 2} ${height - BRACKET_STROKE / 2}`;

  return (
    <svg
      width={BRACKET_WIDTH}
      height={height}
      viewBox={`0 0 ${BRACKET_WIDTH} ${height}`}
      style={{ overflow: "visible" }}
    >
      <path
        d={dTop + " " + dBottom}
        stroke="var(--fg-color)"
        strokeWidth={BRACKET_STROKE}
        fill="none"
      />
    </svg>
  );
};

const GroupLabel = ({ label, count }: { label: string; count: number }) => {
  const height = count * LINE_SPACING;
  const bracketHeight = height - LINE_SPACING;

  return (
    <div
      style={{
        height: `${height}px`,
        position: "relative",
        width: `${BRACKET_WIDTH}px`,
      }}
    >
      <div
        style={{
          position: "absolute",
          top: `${LINE_SPACING - BALL_SIZE / 2 - BRACKET_STROKE / 2}px`,
          left: 0,
          height: bracketHeight,
          width: BRACKET_WIDTH,
        }}
      >
        <BrokenBracket height={bracketHeight} />
      </div>

      <div
        style={{
          position: "absolute",
          top: `${Math.floor((count - 1) / 2) * LINE_SPACING + LABEL_OFFSET}px`,
          left: 0,
          width: "100%",
          height: `${LINE_SPACING}px`,
          display: "flex",
          alignItems: "center",
          justifyContent: "flex-start",
          paddingLeft: BRACKET_WIDTH - 3,
        }}
      >
        <span>{label}</span>
      </div>
    </div>
  );
};

export const EnvSourceLabels = () => (
  <div style={{ display: "flex", flexDirection: "column" }}>
    {/* Spacer to align with the slider label "ENV MODE" */}
    <div
      style={{
        textAlign: "right",
        marginBottom: `${LABEL_MARGIN}px`,
        width: `${BRACKET_WIDTH}px`,
        whiteSpace: "nowrap",
        visibility: "hidden",
        flexGrow: 0,
      }}
    >
      ENV MODE
    </div>

    <GroupLabel label="1" count={3} />

    <GroupLabel label="2" count={3} />
  </div>
);

export default EnvSourceLabels;
