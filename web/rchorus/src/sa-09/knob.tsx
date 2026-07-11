import { Knob as KnobKit } from "music-ui/kit";
import { clamp, rescale } from "music-ui/util";
import { useCallback } from "react";

const DISPLAY_WIDTH = 82;
const DISPLAY_HEIGHT = 41;
const KNOB_HEIGHT = 80;
const KNOB_RADIUS = 20.5;
const KNOB_CENTER_X = DISPLAY_WIDTH / 2;
const KNOB_CENTER_Y = KNOB_RADIUS;
const POINTER_OUTER_RADIUS = KNOB_RADIUS - 0.5;
const POINTER_INNER_RADIUS = KNOB_RADIUS / 4;
const THROW_START = (-5 * Math.PI) / 4;
const THROW_END = Math.PI / 4;

type Range = readonly [number, number];

type Props = {
  value: number;
  set: (value: number) => void;
  grab: () => void;
  release: () => void;
  range: Range;
  active: boolean;
  accessibilityLabel: string;
  defaultValue: number;
};

const pointAt = (angle: number, radius: number) => ({
  x: KNOB_CENTER_X + Math.cos(angle) * radius,
  y: KNOB_CENTER_Y + Math.sin(angle) * radius,
});

const Display = ({ value }: KnobKit.DisplayProps) => {
  const angle = rescale(value, 0, 100, THROW_START, THROW_END);
  const pointerOuter = pointAt(angle, POINTER_OUTER_RADIUS);
  const pointerInner = pointAt(angle, POINTER_INNER_RADIUS);

  return (
    <svg
      width={DISPLAY_WIDTH}
      height={DISPLAY_HEIGHT}
      viewBox={`0 0 ${DISPLAY_WIDTH} ${DISPLAY_HEIGHT}`}
      style={{ display: "block", overflow: "visible" }}
    >
      <circle
        cx={KNOB_CENTER_X}
        cy={KNOB_CENTER_Y}
        r={KNOB_RADIUS - 0.5}
        fill="none"
        stroke="var(--text-color)"
      />
      <line
        x1={pointerOuter.x}
        y1={pointerOuter.y}
        x2={pointerInner.x}
        y2={pointerInner.y}
        stroke="var(--text-color)"
        strokeLinecap="round"
      />
    </svg>
  );
};

const Knob = ({
  value,
  set,
  grab,
  release,
  range,
  active,
  accessibilityLabel,
  defaultValue,
}: Props) => {
  const [min, max] = range;
  const scaleToPercent = useCallback(
    (value: number) => rescale(clamp(value, min, max), min, max, 0, 100),
    [max, min],
  );
  const scaleFromPercent = useCallback(
    (value: number) => rescale(value, 0, 100, min, max),
    [max, min],
  );
  const onValue = useCallback(
    (value: number) => {
      set(scaleFromPercent(value));
    },
    [scaleFromPercent, set],
  );
  const onGrabOrRelease = useCallback(
    (grabbed: boolean) => {
      if (grabbed) {
        grab();
      } else {
        release();
      }
    },
    [grab, release],
  );
  const displayValue = scaleToPercent(value);

  return (
    <div
      style={{
        width: `${DISPLAY_WIDTH}px`,
        height: `${KNOB_HEIGHT}px`,
        color: "var(--text-color)",
        position: "relative",
        textAlign: "center",
      }}
    >
      {active ? (
        <div style={{ position: "absolute", left: "0px", top: "0px" }}>
          <KnobKit.Knob
            value={displayValue}
            onValue={onValue}
            onGrabOrRelease={onGrabOrRelease}
            label="Vibrato Rate"
            accessibilityLabel={accessibilityLabel}
            defaultValue={scaleToPercent(defaultValue)}
            showLabel="hidden"
            Display={Display}
          />
        </div>
      ) : (
        <div
          aria-disabled="true"
          aria-label={accessibilityLabel}
          style={{
            position: "absolute",
            left: "0px",
            top: "0px",
            opacity: 0.35,
          }}
        >
          <Display value={displayValue} grabbed={false} hover={false} />
        </div>
      )}
      <div
        style={{
          position: "absolute",
          left: "0px",
          top: "46px",
          width: `${DISPLAY_WIDTH}px`,
          color: "var(--text-color)",
          fontSize: "14px",
          lineHeight: "17px",
          opacity: active ? 1 : 0.35,
        }}
      >
        {"Vibrato\nRate"}
      </div>
    </div>
  );
};

export default Knob;
