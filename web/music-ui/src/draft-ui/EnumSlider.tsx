import {
  EnumSlider as InternalEnumSlider,
  SliderProps,
  ValueLabelProps,
} from "../kit/enum-slider";
import { useEnumSlider } from "../kit";

export type Props = {
  /**
   * The label of the control
   */
  label: string;

  /**
   * Accessibility label for the enum, defaults to `label`
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
   * A callback that is called when the user selects an item.
   */
  onValue: (value: string) => void;

  /**
   * True if the slider is grabbed
   */
  grabbed?: boolean;

  /**
   * Callback for when the slider is grabbed or release through a pointer event.
   * Note that this may be called spruriously even if the grabbed state didn't change.
   */
  onGrabOrRelease?: (grabbed: boolean) => void;

  /**
   * Display formatter, if applicable. By default just shows the value.
   */
  displayFormatter?: (value: string) => string;

  /**
   * Default value.
   *
   * if present, must be one of `values`
   */
  defaultValue?: string;
};

const BALL_SIZE = 12;
const LINE_SPACING = 24;
const BALL_MARGIN = 2;
const BORDER_WIDTH = 1;

const Slider = ({
  index,
  count,
  selectIndex: selectIndex,
  grabbed,
  onGrabOrRelease,
  defaultValue,
}: SliderProps) => {
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
    ballMargin: BALL_MARGIN,
    lineSpacing: LINE_SPACING,
    ballSize: BALL_SIZE,
    index,
    count,
    selectIndex,
    onGrabOrRelease,
    defaultValue,
  });

  return (
    <div
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onPointerCancel={onPointerCancel}
      onDoubleClick={onDoubleClick}
      ref={containerRef}
      data-testid="slider-track"
      className="slider-track"
      style={{
        height: `${LINE_SPACING * count + BALL_MARGIN * 2 - BALL_SIZE}px`,
        width: `${BALL_SIZE + BALL_MARGIN * 2}px`,
        position: "relative",
        marginRight: "6px",
        marginTop: `2px`,
        cursor: "pointer",
      }}
    >
      <div
        className="slider-track-inner"
        style={{
          height: `${LINE_SPACING * (count - 1)}px`,
          width: "1px",
          position: "absolute",
          top: `${BALL_MARGIN + BALL_SIZE / 2}px`,
          left: `${BALL_MARGIN + BALL_SIZE / 2 - 0.75}px`,
        }}
      ></div>
      <div>
        {ball && (
          <div
            className={`slider-ball ${grabbed ? "slider-ball-grabbed" : ""}`}
            data-testid="slider-ball"
            ref={ballRef}
            style={{
              width: `${BALL_SIZE - BORDER_WIDTH * 2}px`,
              height: `${BALL_SIZE - BORDER_WIDTH * 2}px`,
              bottom: `${ball.bottom}px`,
              left: `${BALL_MARGIN}px`,
              position: "absolute",
            }}
          ></div>
        )}
      </div>
    </div>
  );
};

const ValueLabel = ({ label, checked, ...props }: ValueLabelProps) => (
  <div
    {...props}
    style={{
      height: `${LINE_SPACING}px`,
      fontWeight: checked ? "400" : "200",
      fontFamily: "sans-serif",
      cursor: "pointer",
    }}
    className="slider-value-label"
  >
    {label}
  </div>
);

export const EnumSlider = ({
  label,
  accessibilityLabel,
  values,
  value,
  onValue,
  grabbed,
  onGrabOrRelease,
  defaultValue,
  displayFormatter,
}: Props) => (
  <div>
    <InternalEnumSlider
      accessibilityLabel={accessibilityLabel ?? label}
      values={values}
      value={value}
      onValue={onValue}
      grabbed={grabbed}
      onGrabOrRelease={onGrabOrRelease}
      ValueLabel={ValueLabel}
      Slider={Slider}
      displayFormatter={displayFormatter}
      defaultValue={defaultValue}
    />
    <div
      className="slider-label"
      style={{
        fontFamily: "sans-serif",
        height: `${LINE_SPACING}px`,
        cursor: "pointer",
      }}
    >
      {label}
    </div>
  </div>
);

export default EnumSlider;
