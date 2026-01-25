import { LABEL_MARGIN } from "./constants";

const LINE_SPACING = 18;
const BRACKET_WIDTH = 6;
const BRACKET_STROKE = 1;
const LABEL_GAP_HEIGHT = 18;

const BrokenBracket = ({ height }: { height: number }) => {
  const midY = height / 2;
  const gapHalf = LABEL_GAP_HEIGHT / 2;

  // Draw top part
  // M 0 0 -> L width 0 -> L width (midY - gapHalf)
  const dTop = `M 0 0 L ${BRACKET_WIDTH} 0 L ${BRACKET_WIDTH} ${midY - gapHalf}`;

  // Draw bottom part
  // M width (midY + gapHalf) -> L width height -> L 0 height
  const dBottom = `M ${BRACKET_WIDTH} ${midY + gapHalf} L ${BRACKET_WIDTH} ${height} L 0 ${height}`;

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
  const bracketHeight = height - LINE_SPACING; // Center-to-center span

  return (
    <div
      style={{
        height: `${height}px`,
        position: "relative",
        width: `${BRACKET_WIDTH}px`, // Ensure enough width for the label
      }}
    >
      <div
        style={{
          position: "absolute",
          top: `${LINE_SPACING / 2}px`,
          left: 0,
          height: bracketHeight,
          width: BRACKET_WIDTH,
        }}
      >
        <BrokenBracket height={bracketHeight} />
      </div>

      {/* Label positioned at the second row */}
      <div
        style={{
          position: "absolute",
          top: `${Math.floor((count - 1) / 2) * LINE_SPACING}px`,
          left: 0,
          width: "100%",
          height: `${LINE_SPACING}px`,
          display: "flex",
          alignItems: "center",
          justifyContent: "flex-start",
          paddingLeft: BRACKET_WIDTH - 3, // Align roughly with the spine
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
        visibility: "hidden",
        width: `${BRACKET_WIDTH}px`,
      }}
    >
      ENV MODE
    </div>

    {/* Group 1: First 3 items */}
    <GroupLabel label="1" count={3} />

    {/* Group 2: Next 3 items */}
    <GroupLabel label="2" count={3} />

    {/* Last item (Dynamic) has no label */}
    <div style={{ height: `${LINE_SPACING}px` }} />
  </div>
);

export default EnvSourceLabels;
