import { EnumSlider, useEnumSlider } from "music-ui/kit";
import { useCallback } from "react";
import ensemble from "./assets/ensemble.svg";
import tickLine from "./assets/tick-line.svg";
import {
  STRING_ACCENT_COLOR,
  STRING_ENSEMBLE_MODES,
  STRING_PANEL_BACKGROUND,
  type StringEnsembleMode,
} from "./constants";

const PANEL_WIDTH = 130;
const PANEL_HEIGHT = 180;
const HANDLE_WIDTH = 68;
const HANDLE_HEIGHT = 32;
const HANDLE_INNER_WIDTH = 52;
const HANDLE_INNER_HEIGHT = 16;
const TRACK_WIDTH = 8;
const TRACK_HEIGHT = 120;
const TRACK_TOP = 30;
const TOP_TICK_CENTER = 32;
const BOTTOM_TICK_CENTER = 150;
const SLIDER_TOP = TRACK_TOP;
const SLIDER_HEIGHT = BOTTOM_TICK_CENTER + HANDLE_HEIGHT / 2 - SLIDER_TOP;
const HANDLE_TRAVEL = BOTTOM_TICK_CENTER - TOP_TICK_CENTER;
const LABEL_HEIGHT = 21;
const TOP_LABEL_TOP = TOP_TICK_CENTER - LABEL_HEIGHT / 2;
const BOTTOM_LABEL_TOP = BOTTOM_TICK_CENTER - LABEL_HEIGHT / 2;

export type EnsembleSliderProps = {
  value: StringEnsembleMode;
  onValue: (value: StringEnsembleMode) => void;
};

const EnsembleSliderControl = ({
  index,
  count,
  selectIndex,
  onGrabOrRelease,
}: EnumSlider.SliderProps) => {
  const { containerRef, ballRef, ball, ...divProps } = useEnumSlider<
    HTMLDivElement,
    HTMLDivElement
  >({
    ballMargin: 0,
    lineSpacing: HANDLE_TRAVEL,
    ballSize: HANDLE_HEIGHT,
    index,
    count,
    selectIndex,
    onGrabOrRelease,
    rubberBandExponent: 1.5,
  });

  return (
    <div
      {...divProps}
      ref={containerRef}
      style={{
        position: "absolute",
        left: `${(PANEL_WIDTH - HANDLE_WIDTH) / 2}px`,
        top: `${SLIDER_TOP}px`,
        width: `${HANDLE_WIDTH}px`,
        height: `${SLIDER_HEIGHT}px`,
        zIndex: 2,
      }}
    >
      {ball !== undefined && (
        <div
          ref={ballRef}
          style={{
            position: "absolute",
            left: "0px",
            bottom: `${ball.bottom}px`,
            width: `${HANDLE_WIDTH}px`,
            height: `${HANDLE_HEIGHT}px`,
            borderRadius: `${HANDLE_HEIGHT / 2}px`,
            background: "#100007",
            boxShadow: "0px 4px 8px rgba(0, 0, 0, 0.45)",
          }}
        >
          <div
            style={{
              position: "absolute",
              left: `${(HANDLE_WIDTH - HANDLE_INNER_WIDTH) / 2}px`,
              top: `${(HANDLE_HEIGHT - HANDLE_INNER_HEIGHT) / 2}px`,
              width: `${HANDLE_INNER_WIDTH}px`,
              height: `${HANDLE_INNER_HEIGHT}px`,
              borderRadius: `${HANDLE_INNER_HEIGHT / 2}px`,
              background: STRING_ACCENT_COLOR,
            }}
          />
        </div>
      )}
    </div>
  );
};

const EnsembleSliderValueLabel = ({
  label,
  ...props
}: EnumSlider.ValueLabelProps) => {
  const isModeI = label === "I";
  return (
    <div
      {...props}
      style={{
        position: "absolute",
        left: "0px",
        top: `${isModeI ? TOP_LABEL_TOP : BOTTOM_LABEL_TOP}px`,
        width: `${PANEL_WIDTH}px`,
        height: `${LABEL_HEIGHT}px`,
        color: "var(--text-color)",
        zIndex: 1,
      }}
    >
      <span
        style={{
          position: "absolute",
          left: "12px",
          top: "0px",
          fontSize: "14px",
          lineHeight: "21px",
          pointerEvents: "none",
        }}
      >
        {label}
      </span>
    </div>
  );
};

const EnsembleSlider = ({ value, onValue }: EnsembleSliderProps) => {
  const setMode = useCallback(
    (mode: string) => {
      if (mode === "I" || mode === "II") {
        onValue(mode);
      }
    },
    [onValue],
  );

  return (
    <div
      style={{
        position: "absolute",
        left: "135px",
        top: "93px",
        width: `${PANEL_WIDTH}px`,
        height: "197px",
      }}
    >
      <img
        src={ensemble}
        alt="Ensemble"
        draggable={false}
        style={{
          display: "block",
          position: "absolute",
          left: "8px",
          top: "0px",
          width: "114.342px",
          height: "12.978px",
        }}
      />
      <div
        style={{
          position: "absolute",
          left: "0px",
          top: "17px",
          width: `${PANEL_WIDTH}px`,
          height: `${PANEL_HEIGHT}px`,
          borderRadius: "24px",
          background: STRING_PANEL_BACKGROUND,
        }}
      >
        <div
          style={{
            position: "absolute",
            left: `${(PANEL_WIDTH - TRACK_WIDTH) / 2}px`,
            top: `${TRACK_TOP}px`,
            width: `${TRACK_WIDTH}px`,
            height: `${TRACK_HEIGHT}px`,
            borderRadius: `${TRACK_WIDTH / 2}px`,
            background: "#100007",
          }}
        />
        <img
          src={tickLine}
          alt=""
          draggable={false}
          style={{
            display: "block",
            position: "absolute",
            right: "20px",
            top: "31px",
            width: "29px",
            height: "2px",
          }}
        />
        <img
          src={tickLine}
          alt=""
          draggable={false}
          style={{
            display: "block",
            position: "absolute",
            right: "20px",
            top: "149px",
            width: "29px",
            height: "2px",
          }}
        />
        <EnumSlider.EnumSlider
          values={STRING_ENSEMBLE_MODES}
          value={value}
          onValue={setMode}
          accessibilityLabel="String ensemble mode"
          ValueLabel={EnsembleSliderValueLabel}
          Slider={EnsembleSliderControl}
        />
      </div>
    </div>
  );
};

export default EnsembleSlider;
