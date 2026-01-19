import { EnumSlider as EnumSliderModule, useEnumSlider } from "music-ui/kit";
import { ballSize } from "./sliderConstants";
import SliderBall, { dotOffset, dotSize } from "./SliderBall";
import SliderTrack from "./SliderTrack";
import { useCallback } from "react";

const lineSpacing = 18;

export type Props = {
  /**
   * The label of the control
   */
  label: string;

  /**
   * Label for accessibility (can contain more information than `label`)
   */
  accessibilityLabel?: string;

  /**
   * The possible values of the enum
   */
  values: string[];

  /**
   * The currently selected value.
   *
   * `undefined` represents a state where no item is selected, otherwise must be one of `values`
   */
  value?: string;

  /**
   * Callback for when the slider is grabbed.
   */
  grab: () => void;

  /**
   * Callback for when the slider is released.
   */
  release: () => void;

  /**
   * Callback for when the value of the enum changes.
   */
  onValue: (value: string) => void;
};

const Slider = ({
  index,
  count,
  selectIndex: selectIndex,
  onGrabOrRelease,
}: EnumSliderModule.SliderProps) => {
  const {
    onPointerDown,
    onPointerMove,
    onPointerUp,
    onPointerCancel,
    containerRef,
    ballRef,
    ball,
  } = useEnumSlider<HTMLDivElement, HTMLDivElement>({
    ballMargin: 0,
    lineSpacing,
    ballSize,
    index,
    count,
    selectIndex,
    onGrabOrRelease,
  });

  const trackHeight = lineSpacing * count;
  return (
    <div
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onPointerCancel={onPointerCancel}
      ref={containerRef}
      style={{
        height: `${trackHeight}px`,
        width: `${ballSize}px`,
        position: "relative",
        cursor: "pointer",
        flexGrow: 0,
      }}
    >
      <SliderTrack height={trackHeight - ballSize / 2} />
      {ball && <SliderBall bottom={ball.bottom} ref={ballRef} />}
    </div>
  );
};

const ValueLabel = ({
  label,
  checked,
  ...props
}: EnumSliderModule.ValueLabelProps) => (
  <div
    {...props}
    style={{
      height: `${lineSpacing}px`,
      fontWeight: checked ? "400" : "200",
      textAlign: "right",
      cursor: "pointer",
    }}
  >
    <span>{label}</span>
    <div
      style={{
        verticalAlign: "middle",
        display: "inline-block",
        marginTop: "-0.5px",
        height: `${dotSize}px`,
        width: `${dotSize}px`,
        marginLeft: `${dotOffset}px`,
        marginRight: `${dotOffset}px`,
        backgroundColor: "var(--fg-color)",
        borderRadius: `${dotSize}px`,
      }}
    ></div>
  </div>
);

const Label = ({ label }: { label: string }) => (
  <div
    style={{
      textAlign: "right",
    }}
  >
    {label}
  </div>
);

export const EnumSlider = (props: Props) => {
  const { grab, release } = props;
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
    <div style={{ display: "flex", flexDirection: "column" }}>
      <Label label={props.label} />
      <div style={{ flexGrow: 1 }}></div>
      <EnumSliderModule.EnumSlider
        {...props}
        accessibilityLabel={props.accessibilityLabel ?? props.label}
        Slider={Slider}
        ValueLabel={ValueLabel}
        layout="labels-first"
        onGrabOrRelease={onGrabOrRelease}
      />
    </div>
  );
};
