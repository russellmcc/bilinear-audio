import { Knob as KnobKit } from "music-ui/kit";
import { clamp, rescale } from "music-ui/util";
import { useCallback } from "react";
import knobArc from "./assets/knob-arc.svg";
import knobBody from "./assets/knob-body.svg";
import knobInner from "./assets/knob-inner.svg";

const WIDTH = 139;
const HEIGHT = 150;
const DISPLAY_TOP = 20;
const CENTER_X = 69.3;
const CENTER_Y = 68.3;
const POINTER_WIDTH = 6;
const POINTER_HEIGHT = 18;
const THROW_START = -135;
const THROW_END = 135;

type Range = readonly [number, number];

type Props = {
  label: string;
  value: number;
  set: (value: number) => void;
  grab: () => void;
  release: () => void;
  range: Range;
  active: boolean;
  accessibilityLabel: string;
  defaultValue: number;
};

const Display = ({ value }: KnobKit.DisplayProps) => {
  const angle = rescale(value, 0, 100, THROW_START, THROW_END);

  return (
    <div
      style={{
        position: "relative",
        width: `${WIDTH}px`,
        height: "130px",
      }}
    >
      <img
        src={knobArc}
        alt=""
        draggable={false}
        style={{
          position: "absolute",
          left: "19.3px",
          top: "18.3px",
          width: "100px",
          height: "100px",
          transform: "rotate(135deg)",
        }}
      />
      <img
        src={knobBody}
        alt=""
        draggable={false}
        style={{
          position: "absolute",
          left: "25.3px",
          top: "24.3px",
          width: "88px",
          height: "88px",
        }}
      />
      <img
        src={knobInner}
        alt=""
        draggable={false}
        style={{
          position: "absolute",
          left: "41.3px",
          top: "40.3px",
          width: "56px",
          height: "56px",
        }}
      />
      <div
        style={{
          position: "absolute",
          left: `${CENTER_X - POINTER_WIDTH / 2}px`,
          top: "40.3px",
          width: `${POINTER_WIDTH}px`,
          height: `${POINTER_HEIGHT}px`,
          borderRadius: "0px 0px 3px 3px",
          background: "var(--bg-color-jazz-120)",
          transform: `rotate(${angle}deg)`,
          transformOrigin: `3px ${CENTER_Y - 40.3}px`,
        }}
      />
      <div
        style={{
          position: "absolute",
          left: "63.8px",
          top: "9.3px",
          width: "11px",
          height: "15px",
          background: "var(--panel-color-jazz-120)",
        }}
      />
      <div
        style={{
          position: "absolute",
          left: "42.3px",
          top: "103.3px",
          transform: "translateX(-100%)",
          fontSize: "14px",
          lineHeight: "normal",
        }}
      >
        0
      </div>
      <div
        style={{
          position: "absolute",
          left: "73.3px",
          top: "8.3px",
          transform: "translateX(-100%)",
          fontSize: "14px",
          lineHeight: "normal",
        }}
      >
        5
      </div>
      <div
        style={{
          position: "absolute",
          left: "106.3px",
          top: "103.3px",
          transform: "translateX(-100%)",
          fontSize: "14px",
          lineHeight: "normal",
        }}
      >
        10
      </div>
    </div>
  );
};

const Knob = ({
  label,
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
        width: `${WIDTH}px`,
        height: `${HEIGHT}px`,
        color: "var(--text-color)",
        position: "relative",
        textAlign: "center",
      }}
    >
      <div
        style={{
          fontSize: "18px",
          lineHeight: "22px",
          height: `${DISPLAY_TOP}px`,
        }}
      >
        {label}
      </div>
      <div
        style={{ position: "absolute", left: "0px", top: `${DISPLAY_TOP}px` }}
      >
        {active ? (
          <KnobKit.Knob
            value={displayValue}
            onValue={onValue}
            onGrabOrRelease={onGrabOrRelease}
            label={label}
            accessibilityLabel={accessibilityLabel}
            defaultValue={scaleToPercent(defaultValue)}
            showLabel="hidden"
            Display={Display}
          />
        ) : (
          <div
            aria-disabled="true"
            aria-label={accessibilityLabel}
            style={{
              display: "inline-block",
              opacity: 0.58,
            }}
          >
            <Display value={displayValue} grabbed={false} hover={false} />
          </div>
        )}
      </div>
    </div>
  );
};

export default Knob;
