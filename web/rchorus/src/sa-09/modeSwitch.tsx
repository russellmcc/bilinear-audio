import { EnumSlider, useEnumSlider } from "music-ui/kit";
import { useCallback } from "react";

const SWITCH_BALL_WIDTH = 12;
const SWITCH_SVG_HEIGHT = 37;
const SWITCH_LABEL_SPACING = 57;
const SWITCH_COLOR = "var(--text-color)";
const ARROW_TIP_TOP = 0.853493;
const ARROW_BASE_TOP = 8.46219;
const ARROW_TIP_BOTTOM = 35.8535;
const ARROW_HEAD_HEIGHT = ARROW_BASE_TOP - ARROW_TIP_TOP;
const SWITCH_THROW = ARROW_TIP_BOTTOM - ARROW_TIP_TOP;
const SWITCH_HEIGHT = ARROW_TIP_BOTTOM;

type ArrowEnumSwitchProps = {
  values: readonly string[];
  value: string;
  onValue: (value: string) => void;
  accessibilityLabel: string;
  displayFormatter: (value: string) => string;
  topLabel: string;
  left: number;
  labelWidth: number;
};

const clampProgress = (value: number) => Math.min(Math.max(value, 0), 1);

const arrowHeadForProgress = (progress: number) => {
  const direction = progress < 0.5 ? -1 : 1;
  const strength = Math.abs(progress * 2 - 1);
  const tipY = ARROW_TIP_TOP + progress * SWITCH_THROW;
  const baseY =
    tipY + (direction < 0 ? ARROW_HEAD_HEIGHT : -ARROW_HEAD_HEIGHT) * strength;
  const top = Math.min(tipY, baseY);
  const height = Math.max(Math.abs(baseY - tipY), 1);

  return { baseY, height, tipY, top };
};

const ArrowHandle = ({ progress }: { progress: number }) => {
  const { baseY, tipY } = arrowHeadForProgress(progress);

  return (
    <svg
      width={SWITCH_BALL_WIDTH}
      height={SWITCH_SVG_HEIGHT}
      viewBox={`0 0 ${SWITCH_BALL_WIDTH} ${SWITCH_SVG_HEIGHT}`}
      style={{ display: "block", overflow: "visible" }}
    >
      <line
        x1="6"
        y1={ARROW_TIP_TOP}
        x2="6"
        y2={ARROW_TIP_BOTTOM}
        stroke={SWITCH_COLOR}
        strokeLinecap="round"
      />
      <path
        d={`M11.5 ${baseY}L6 ${tipY}L0.5 ${baseY}`}
        stroke={SWITCH_COLOR}
        strokeLinecap="round"
        fill="none"
      />
    </svg>
  );
};

const ArrowSwitchSlider = ({
  index,
  count,
  selectIndex,
  onGrabOrRelease,
  left,
}: EnumSlider.SliderProps & { left: number }) => {
  const {
    containerRef,
    ballRef,
    ball,
    onDoubleClick,
    onPointerDown,
    onPointerMove,
    onPointerUp,
    onPointerCancel,
  } = useEnumSlider<HTMLDivElement, HTMLDivElement>({
    ballMargin: 0,
    lineSpacing: SWITCH_THROW,
    ballSize: 0,
    index,
    count,
    selectIndex,
    onGrabOrRelease,
    rubberBandExponent: 1.5,
  });
  const progress =
    ball === undefined
      ? 0
      : clampProgress((SWITCH_THROW - ball.bottom) / SWITCH_THROW);
  const head = arrowHeadForProgress(progress);

  return (
    <div
      ref={containerRef}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onPointerCancel={onPointerCancel}
      style={{
        position: "absolute",
        left: `${left}px`,
        top: "20px",
        width: `${SWITCH_BALL_WIDTH}px`,
        height: `${SWITCH_HEIGHT}px`,
      }}
    >
      <div
        style={{
          position: "absolute",
          left: "0px",
          top: "0px",
          width: `${SWITCH_BALL_WIDTH}px`,
          height: `${SWITCH_SVG_HEIGHT}px`,
          pointerEvents: "none",
        }}
      >
        <ArrowHandle progress={progress} />
      </div>
      {ball !== undefined && (
        <div
          ref={ballRef}
          onDoubleClick={onDoubleClick}
          onPointerDown={onPointerDown}
          style={{
            position: "absolute",
            left: "0px",
            top: `${head.top}px`,
            width: `${SWITCH_BALL_WIDTH}px`,
            height: `${head.height}px`,
            opacity: 0,
          }}
        />
      )}
    </div>
  );
};

const ArrowEnumSwitch = ({
  values,
  value,
  onValue,
  accessibilityLabel,
  displayFormatter,
  topLabel,
  left,
  labelWidth,
}: ArrowEnumSwitchProps) => {
  const sliderLeft = (labelWidth - SWITCH_BALL_WIDTH) / 2;
  const slider = useCallback(
    (props: EnumSlider.SliderProps) => (
      <ArrowSwitchSlider {...props} left={sliderLeft} />
    ),
    [sliderLeft],
  );
  const valueLabel = useCallback(
    ({ label, ...props }: EnumSlider.ValueLabelProps) => (
      <div
        {...props}
        style={{
          position: "absolute",
          left: "0px",
          top: label === topLabel ? "0px" : `${SWITCH_LABEL_SPACING}px`,
          width: `${labelWidth}px`,
          height: "17px",
          color: SWITCH_COLOR,
          fontSize: "14px",
          lineHeight: "17px",
          textAlign: "center",
        }}
      >
        {label}
      </div>
    ),
    [labelWidth, topLabel],
  );

  return (
    <div
      style={{
        position: "absolute",
        left: `${left}px`,
        top: "163px",
        width: `${labelWidth}px`,
        height: `${SWITCH_LABEL_SPACING + 17}px`,
      }}
    >
      <EnumSlider.EnumSlider
        values={values}
        value={value}
        onValue={onValue}
        accessibilityLabel={accessibilityLabel}
        displayFormatter={displayFormatter}
        ValueLabel={valueLabel}
        Slider={slider}
      />
    </div>
  );
};

export default ArrowEnumSwitch;
