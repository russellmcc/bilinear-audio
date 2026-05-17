import { EnumSlider, useEnumSlider } from "music-ui/kit";
import { useCallback } from "react";
import {
  RS_79_BALL_HIGHLIGHT_COLOR,
  RS_79_ENSEMBLE_MODES,
  RS_79_SWITCH_COLOR,
  type Rs79EnsembleMode,
} from "./constants";

const SWITCH_WIDTH = 20;
const SWITCH_HEIGHT = 67;
const SWITCH_BORDER_WIDTH = 1;
const SWITCH_INNER_HEIGHT = SWITCH_HEIGHT - SWITCH_BORDER_WIDTH * 2;
const SWITCH_BALL_SIZE = 14;
const SWITCH_BALL_MARGIN = 2;
const SWITCH_LINE_SPACING =
  SWITCH_INNER_HEIGHT - SWITCH_BALL_SIZE - SWITCH_BALL_MARGIN * 2;
const SWITCH_CENTER_BALL_TOP = SWITCH_BALL_MARGIN + SWITCH_LINE_SPACING / 2;
const SWITCH_HIGHLIGHT_WIDTH = SWITCH_WIDTH - 2;

export type EnsembleSwitchProps = {
  value: Rs79EnsembleMode;
  onValue: (value: Rs79EnsembleMode) => void;
};

const EnsembleSwitchSlider = ({
  index,
  count,
  selectIndex,
  onGrabOrRelease,
}: EnumSlider.SliderProps) => {
  const { containerRef, ballRef, ball, ...divProps } = useEnumSlider<
    HTMLDivElement,
    HTMLDivElement
  >({
    ballMargin: SWITCH_BALL_MARGIN,
    lineSpacing: SWITCH_LINE_SPACING,
    ballSize: SWITCH_BALL_SIZE,
    index,
    count,
    selectIndex,
    onGrabOrRelease,
  });
  const ballTop =
    ball === undefined
      ? SWITCH_BALL_MARGIN
      : SWITCH_INNER_HEIGHT - ball.bottom - SWITCH_BALL_SIZE;
  const ballBottom = ballTop + SWITCH_BALL_SIZE;
  const centerBallBottom = SWITCH_CENTER_BALL_TOP + SWITCH_BALL_SIZE;
  const ballAboveCenter = ballTop < SWITCH_CENTER_BALL_TOP;
  const highlightTop =
    (ballAboveCenter ? ballTop : SWITCH_CENTER_BALL_TOP) - SWITCH_BALL_MARGIN;
  const highlightBottom =
    (ballAboveCenter ? centerBallBottom : ballBottom) + SWITCH_BALL_MARGIN;

  return (
    <div
      style={{
        position: "absolute",
        left: "50px",
        top: "72px",
        width: `${SWITCH_WIDTH}px`,
        height: `${SWITCH_HEIGHT}px`,
      }}
    >
      <svg
        aria-hidden="true"
        width={SWITCH_WIDTH}
        height={SWITCH_HEIGHT}
        viewBox={`0 0 ${SWITCH_WIDTH} ${SWITCH_HEIGHT}`}
        style={{
          position: "absolute",
          left: "0px",
          top: "0px",
          width: `${SWITCH_WIDTH}px`,
          height: `${SWITCH_HEIGHT}px`,
          pointerEvents: "none",
        }}
      >
        <rect
          x="0.5"
          y="0.5"
          width={SWITCH_WIDTH - 1}
          height={SWITCH_HEIGHT - 1}
          rx="9.5"
          fill="#181516"
          stroke={RS_79_SWITCH_COLOR}
          strokeWidth="1"
        />
      </svg>
      <div
        {...divProps}
        ref={containerRef}
        style={{
          position: "absolute",
          left: `${SWITCH_BORDER_WIDTH}px`,
          top: `${SWITCH_BORDER_WIDTH}px`,
          width: `${SWITCH_HIGHLIGHT_WIDTH}px`,
          height: `${SWITCH_INNER_HEIGHT}px`,
        }}
      >
        <div
          style={{
            position: "absolute",
            left: "0px",
            top: `${highlightTop}px`,
            width: `${SWITCH_HIGHLIGHT_WIDTH}px`,
            height: `${highlightBottom - highlightTop}px`,
            borderRadius: `${SWITCH_HIGHLIGHT_WIDTH / 2}px`,
            background: RS_79_BALL_HIGHLIGHT_COLOR,
          }}
        />
        {ball !== undefined && (
          <div
            ref={ballRef}
            style={{
              position: "absolute",
              left: `${(SWITCH_HIGHLIGHT_WIDTH - SWITCH_BALL_SIZE) / 2}px`,
              bottom: `${ball.bottom}px`,
              width: `${SWITCH_BALL_SIZE}px`,
              height: `${SWITCH_BALL_SIZE}px`,
              borderRadius: "7px",
              background: RS_79_SWITCH_COLOR,
            }}
          />
        )}
      </div>
    </div>
  );
};

const EnsembleSwitchValueLabel = ({
  label,
  ...props
}: EnumSlider.ValueLabelProps) => {
  const isModeI = label === "I";
  return (
    <div
      {...props}
      style={{
        position: "absolute",
        left: "50%",
        top: isModeI ? "47px" : "143px",
        transform: "translateX(-50%)",
        width: "32px",
        color: RS_79_SWITCH_COLOR,
        fontSize: "18px",
        lineHeight: "21px",
        textAlign: "center",
      }}
    >
      {label}
    </div>
  );
};

const EnsembleSwitch = ({ value, onValue }: EnsembleSwitchProps) => {
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
        left: "140px",
        top: "115px",
        width: "120px",
        height: "180px",
        color: RS_79_SWITCH_COLOR,
      }}
    >
      <div
        style={{
          position: "absolute",
          left: "0px",
          top: "0px",
          width: "1px",
          height: "180px",
          background: RS_79_SWITCH_COLOR,
        }}
      />
      <div
        style={{
          position: "absolute",
          right: "0px",
          top: "0px",
          width: "1px",
          height: "180px",
          background: RS_79_SWITCH_COLOR,
        }}
      />
      <div
        style={{
          position: "absolute",
          left: "3px",
          top: "31px",
          width: "114px",
          height: "1px",
          background: RS_79_SWITCH_COLOR,
        }}
      />
      <div
        style={{
          position: "absolute",
          left: "3px",
          bottom: "0px",
          width: "114px",
          height: "1px",
          background: RS_79_SWITCH_COLOR,
        }}
      />
      <div
        style={{
          position: "absolute",
          left: "0px",
          top: "0px",
          width: "120px",
          fontSize: "12px",
          lineHeight: "12px",
          textAlign: "center",
        }}
      >
        {"ENSEMBLE\nMODE"}
      </div>
      <EnumSlider.EnumSlider
        values={RS_79_ENSEMBLE_MODES}
        value={value}
        onValue={setMode}
        accessibilityLabel="Ensemble mode"
        ValueLabel={EnsembleSwitchValueLabel}
        Slider={EnsembleSwitchSlider}
      />
    </div>
  );
};

export default EnsembleSwitch;
