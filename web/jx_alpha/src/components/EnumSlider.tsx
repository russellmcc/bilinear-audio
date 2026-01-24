import { EnumSlider as EnumSliderModule, useEnumSlider } from "music-ui/kit";
import { BALL_SIZE, DOT_OFFSET, DOT_SIZE, LABEL_MARGIN } from "./constants";
import SliderBall from "./SliderBall";
import SliderTrack from "./SliderTrack";
import useOnGrabOrRelease from "./useGrabOrRelease";
import { useCallback } from "react";

const LINE_SPACING = 18;

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
   * The default value of the enum.
   */
  defaultValue?: string;

  /**
   * Overrides the label for certain values with a custom element.
   */
  CustomGlyph?: React.FC<{ value: string }>;

  /**
   * Formatter for the values.
   */
  displayFormatter?: (value: string) => string;

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
  defaultValue,
  onGrabOrRelease,
}: EnumSliderModule.SliderProps) => {
  const {
    onPointerDown,
    onPointerMove,
    onPointerUp,
    onPointerCancel,
    onDoubleClick,
    containerRef,
    ballRef,
    ball,
  } = useEnumSlider<HTMLDivElement, HTMLDivElement>({
    ballMargin: 0,
    lineSpacing: LINE_SPACING,
    ballSize: BALL_SIZE,
    index,
    count,
    selectIndex,
    onGrabOrRelease,
    defaultValue,
  });

  const trackHeight = LINE_SPACING * count;
  return (
    <div
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onPointerCancel={onPointerCancel}
      onDoubleClick={onDoubleClick}
      ref={containerRef}
      style={{
        height: `${trackHeight}px`,
        width: `${BALL_SIZE}px`,
        position: "relative",
        cursor: "pointer",
        flexGrow: 0,
      }}
    >
      <SliderTrack height={trackHeight - BALL_SIZE / 2} />
      {ball && <SliderBall bottom={ball.bottom} ref={ballRef} />}
    </div>
  );
};

const ValueLabel = ({
  label,
  checked,
  CustomGlyph,
  ...props
}: EnumSliderModule.ValueLabelProps & {
  CustomGlyph?: Props["CustomGlyph"];
}) => (
  <div
    {...props}
    style={{
      height: `${LINE_SPACING}px`,
      fontWeight: checked ? "400" : "200",
      textAlign: "right",
      cursor: "pointer",
    }}
  >
    {CustomGlyph ? <CustomGlyph value={label} /> : <span>{label}</span>}
    <div
      style={{
        verticalAlign: "middle",
        display: "inline-block",
        marginTop: "-0.5px",
        height: `${DOT_SIZE}px`,
        width: `${DOT_SIZE}px`,
        marginLeft: `${DOT_OFFSET}px`,
        marginRight: `${DOT_OFFSET}px`,
        backgroundColor: "var(--fg-color)",
        borderRadius: `${DOT_SIZE}px`,
      }}
    ></div>
  </div>
);

const Label = ({ label }: { label: string }) => (
  <div
    style={{
      textAlign: "right",
      marginBottom: `${LABEL_MARGIN}px`,
    }}
  >
    {label}
  </div>
);

export const EnumSlider = (props: Props) => {
  const { grab, release, CustomGlyph, displayFormatter } = props;
  const onGrabOrRelease = useOnGrabOrRelease({ grab, release });
  const valueLabel = useCallback(
    (props: EnumSliderModule.ValueLabelProps) => (
      <ValueLabel {...props} CustomGlyph={CustomGlyph} />
    ),
    [CustomGlyph],
  );
  const valueFormatter = useCallback(
    (value: string) => displayFormatter?.(value) ?? value.toUpperCase(),
    [displayFormatter],
  );
  return (
    <div style={{ display: "flex", flexDirection: "column" }}>
      <Label label={props.label} />
      <div style={{ flexGrow: 1 }}></div>
      <EnumSliderModule.EnumSlider
        {...props}
        accessibilityLabel={props.accessibilityLabel ?? props.label}
        Slider={Slider}
        ValueLabel={valueLabel}
        layout="labels-first"
        onGrabOrRelease={onGrabOrRelease}
        displayFormatter={valueFormatter}
      />
    </div>
  );
};

export default EnumSlider;
