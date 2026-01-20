import {
  BORDER_WIDTH,
  DOT_OFFSET,
  DOT_SIZE,
  DROP_SHADOW_FILTER,
  LABEL_MARGIN,
} from "./constants";
import { rescale } from "music-ui/util";
import { Knob as KnobModule } from "music-ui/kit";

const KNOB_SIZE = 42;
const TAU = Math.PI * 2;
const MIN_ANGLE = -TAU * (3 / 8);
const MAX_ANGLE = TAU * (3 / 8);
const TICK_LENGTH = 5;
const TICK_WIDTH = 1;
const TICK_MARGIN = 2;
const KNOB_MARGIN = 5;

export const KnobDisplay = ({
  value,
  minLabel,
  maxLabel,
}: KnobModule.DisplayProps & { minLabel?: string; maxLabel?: string }) => {
  const angle = rescale(value, 0, 100, MIN_ANGLE, MAX_ANGLE);
  return (
    <div
      style={{
        position: "relative",
        marginTop: `${TICK_MARGIN + TICK_LENGTH + KNOB_MARGIN}px`,
      }}
    >
      {[MIN_ANGLE, 0, MAX_ANGLE].map((a) => (
        <div
          key={a}
          style={{
            position: "absolute",
            left: "50%",
            top: "50%",
            width: `${TICK_WIDTH}px`,
            height: `${TICK_LENGTH}px`,
            backgroundColor: "var(--fg-color)",
            transform: `translate(-50%, -50%) rotate(${a}rad) translateY(-${
              KNOB_SIZE / 2 + BORDER_WIDTH + TICK_LENGTH / 2 + TICK_MARGIN
            }px)`,
          }}
        />
      ))}
      {minLabel && (
        <div
          style={{
            position: "absolute",
            left: "0",
            top: `${KNOB_SIZE + LABEL_MARGIN}px`,
            textAlign: "left",
          }}
        >
          {minLabel}
        </div>
      )}
      {maxLabel && (
        <div
          style={{
            position: "absolute",
            right: "0",
            top: `${KNOB_SIZE + LABEL_MARGIN}px`,
            textAlign: "right",
          }}
        >
          {maxLabel}
        </div>
      )}
      <div
        style={{
          width: `${KNOB_SIZE}px`,
          height: `${KNOB_SIZE}px`,
          borderRadius: `${KNOB_SIZE}px`,
          borderWidth: `${BORDER_WIDTH}px`,
          borderStyle: "solid",
          borderColor: "var(--darkest-color)",
          backgroundColor: "var(--darker-color)",
          filter: DROP_SHADOW_FILTER,
          // Hack for safari to prevent stale rendering
          transform: "translateZ(0)",
        }}
      >
        <div
          style={{
            position: "absolute",
            top: `${KNOB_SIZE / 2 - (KNOB_SIZE / 2 - DOT_OFFSET - BORDER_WIDTH) * Math.cos(angle) - DOT_SIZE / 2}px`,
            left: `${KNOB_SIZE / 2 + (KNOB_SIZE / 2 - DOT_OFFSET - BORDER_WIDTH) * Math.sin(angle) - DOT_SIZE / 2}px`,
            width: `${DOT_SIZE}px`,
            height: `${DOT_SIZE}px`,
            borderRadius: `${DOT_SIZE}px`,
            backgroundColor: "var(--highlight-red)",
          }}
        ></div>
      </div>
    </div>
  );
};

export default KnobDisplay;
