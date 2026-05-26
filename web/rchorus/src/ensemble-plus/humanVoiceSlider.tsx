import { Slider as MusicUISlider, useSlider } from "music-ui/kit";
import { useCallback } from "react";
import { ENSEMBLE_PLUS_TEXT_COLOR } from "./constants";

const SLIDER_THROW = 120;
const SLIDER_RAIL_HEIGHT = 121;
const SLIDER_WIDTH = 42;
const TRACK_WIDTH = 8;
const HANDLE_WIDTH = 42;
const HANDLE_HEIGHT = 25;
const SLIDER_TOUCH_HEIGHT = SLIDER_THROW + HANDLE_HEIGHT;
const SLIDER_TOUCH_TOP = -11;
const SLIDER_RAIL_TOP = 11;
const HANDLE_LINE_CENTER_Y = 10;

export type HumanVoiceSliderProps = {
  label: string;
  value: number;
  set: (value: number) => void;
  grab: () => void;
  release: () => void;
  range: readonly [number, number];
  active: boolean;
  accessibilityLabel: string;
  defaultValue: number;
  trackLeft: number;
};

const clamp = (value: number, min: number, max: number) =>
  Math.min(Math.max(value, min), max);

const rescale = (
  value: number,
  inMin: number,
  inMax: number,
  outMin: number,
  outMax: number,
) => outMin + ((value - inMin) / (inMax - inMin)) * (outMax - outMin);

type SliderDisplayProps = MusicUISlider.SliderProps &
  Pick<HumanVoiceSliderProps, "active" | "trackLeft">;

const SliderDisplay = ({
  value,
  onValue,
  onGrabOrRelease,
  active,
  trackLeft,
}: SliderDisplayProps) => {
  const { containerProps, ballBottom } = useSlider({
    value,
    ballMargin: 0,
    ballSize: HANDLE_HEIGHT,
    onValue,
    onGrabOrRelease,
  });

  return (
    <div
      {...containerProps}
      style={{
        position: "absolute",
        left: "0px",
        top: `${SLIDER_TOUCH_TOP}px`,
        width: `${SLIDER_WIDTH}px`,
        height: `${SLIDER_TOUCH_HEIGHT}px`,
        pointerEvents: active ? "auto" : "none",
        touchAction: "none",
      }}
    >
      <div
        style={{
          position: "absolute",
          left: `${trackLeft}px`,
          top: `${SLIDER_RAIL_TOP}px`,
          width: `${TRACK_WIDTH}px`,
          height: `${SLIDER_RAIL_HEIGHT}px`,
          borderRadius: "4px",
          background: "#0d070a",
        }}
      />
      <div
        style={{
          position: "absolute",
          left: "0px",
          bottom: `${ballBottom}px`,
          width: `${HANDLE_WIDTH}px`,
          height: `${HANDLE_HEIGHT}px`,
          borderRadius: "8px",
          background: "#141414",
          boxShadow: "2px 2px 4px rgba(0, 0, 0, 0.4)",
        }}
      >
        <div
          style={{
            position: "absolute",
            left: "0px",
            top: "0px",
            width: `${HANDLE_WIDTH - 4}px`,
            height: `${HANDLE_HEIGHT - 4}px`,
            borderRadius: "8px",
            background: "#3d3d3d",
          }}
        />
        <div
          style={{
            position: "absolute",
            left: "4px",
            top: `${HANDLE_LINE_CENTER_Y - 1}px`,
            width: "30px",
            height: "2px",
            background: ENSEMBLE_PLUS_TEXT_COLOR,
          }}
        />
      </div>
    </div>
  );
};

const HumanVoiceSlider = ({
  label,
  value,
  set,
  grab,
  release,
  range,
  active,
  accessibilityLabel,
  defaultValue,
  trackLeft,
}: HumanVoiceSliderProps) => {
  const [min, max] = range;
  const scaledValue = rescale(clamp(value, min, max), min, max, 0, 100);
  const setScaledValue = useCallback(
    (value: number) => {
      if (!active) {
        return;
      }
      set(rescale(value, 0, 100, min, max));
    },
    [active, max, min, set],
  );
  const onGrabOrRelease = useCallback(
    (grabbed: boolean) => {
      if (!active) {
        return;
      }
      if (grabbed) {
        grab();
      } else {
        release();
      }
    },
    [active, grab, release],
  );
  const sliderDisplay = useCallback(
    (props: MusicUISlider.SliderProps) => (
      <SliderDisplay {...props} active={active} trackLeft={trackLeft} />
    ),
    [active, trackLeft],
  );

  return (
    <div
      style={{
        position: "relative",
        width: `${SLIDER_WIDTH}px`,
        height: "143px",
        overflow: "visible",
      }}
    >
      <MusicUISlider.Slider
        Slider={sliderDisplay}
        value={scaledValue}
        onValue={setScaledValue}
        onGrabOrRelease={onGrabOrRelease}
        label={label}
        accessibilityLabel={accessibilityLabel}
        defaultValue={rescale(defaultValue, min, max, 0, 100)}
        showLabel="hidden"
      />
      <div
        style={{
          position: "absolute",
          left: "50%",
          top: "133px",
          transform: "translateX(-50%)",
          fontSize: "14px",
          lineHeight: "17px",
          color: ENSEMBLE_PLUS_TEXT_COLOR,
        }}
      >
        {label}
      </div>
    </div>
  );
};

export default HumanVoiceSlider;
