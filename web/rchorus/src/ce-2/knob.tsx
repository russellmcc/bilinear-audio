import { useNumericParam } from "@conformal/plugin";
import { Knob as KnobKit } from "music-ui/kit";
import { rescale } from "music-ui/util";
import { useCallback } from "react";

const DISPLAY_WIDTH = 100;
const DISPLAY_HEIGHT = 130;
const KNOB_RADIUS = 50;
const KNOB_CENTER_X = DISPLAY_WIDTH / 2;
const KNOB_CENTER_Y = 74;
const GUIDE_DOT_RADIUS = 6;
const GUIDE_DOT_DISTANCE = KNOB_RADIUS + GUIDE_DOT_RADIUS + 12;
const INNER_RADIUS = 32;
const POINTER_WIDTH = 4;
const POINTER_OUTER_RADIUS = KNOB_RADIUS - POINTER_WIDTH / 2;
const POINTER_INNER_RADIUS = INNER_RADIUS - POINTER_WIDTH;
const THROW_START = (-5 * Math.PI) / 4;
const THROW_END = Math.PI / 4;
const THROW_CENTER = (THROW_START + THROW_END) / 2;

const pointAt = (angle: number, distance: number) => ({
  x: KNOB_CENTER_X + Math.cos(angle) * distance,
  y: KNOB_CENTER_Y + Math.sin(angle) * distance,
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
      {[THROW_START, THROW_CENTER, THROW_END].map((guideAngle) => {
        const guideDot = pointAt(guideAngle, GUIDE_DOT_DISTANCE);
        return (
          <circle
            key={guideAngle}
            cx={guideDot.x}
            cy={guideDot.y}
            r={GUIDE_DOT_RADIUS}
            fill="var(--panel-color-ce-2)"
          />
        );
      })}
      <circle
        cx={KNOB_CENTER_X}
        cy={KNOB_CENTER_Y}
        r={KNOB_RADIUS}
        fill="var(--panel-color-ce-2)"
      />
      <line
        x1={pointerOuter.x}
        y1={pointerOuter.y}
        x2={pointerInner.x}
        y2={pointerInner.y}
        stroke="var(--text-color)"
        strokeWidth={POINTER_WIDTH}
        strokeLinecap="round"
      />
      <circle
        cx={KNOB_CENTER_X}
        cy={KNOB_CENTER_Y}
        r={INNER_RADIUS}
        fill="var(--text-color)"
      />
    </svg>
  );
};

const Label = ({ label }: KnobKit.LabelProps) => (
  <div
    style={{
      color: "var(--panel-color-ce-2)",
      fontSize: "14px",
      lineHeight: "normal",
      textAlign: "center",
      marginTop: "0px",
    }}
  >
    {label}
  </div>
);

type Range = readonly [number, number];

type Props = {
  label: string;
  param: string;
  range: Range;
  defaultValue: number;
};

const clamp = (value: number, min: number, max: number) =>
  Math.min(Math.max(value, min), max);

const Knob = ({ label, param, range, defaultValue }: Props) => {
  const { value, set, grab, release, info } = useNumericParam(param);
  const [min, max] = range;

  const scaleToPercent = useCallback(
    (value: number) => rescale(clamp(value, min, max), min, max, 0, 100),
    [min, max],
  );
  const scaleFromPercent = useCallback(
    (value: number) => rescale(value, 0, 100, min, max),
    [min, max],
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

  return (
    <KnobKit.Knob
      value={scaleToPercent(value)}
      onValue={onValue}
      onGrabOrRelease={onGrabOrRelease}
      label={label}
      accessibilityLabel={info.title}
      defaultValue={defaultValue}
      Display={Display}
      Label={Label}
    />
  );
};

export default Knob;
