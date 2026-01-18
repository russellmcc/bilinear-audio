import { useSlider } from "../kit";
import {
  Slider as InternalSlider,
  SliderProps,
  LabelProps,
} from "../kit/slider";

export type Props = {
  /**
   * The current value of the knob (scaled to 0-100)
   */
  value: number;

  /**
   * True if the knob is grabbed
   */
  grabbed?: boolean;

  /**
   * Callback for when the knob is grabbed or release through a pointer event.
   */
  onGrabOrRelease?: (grabbed: boolean) => void;

  /**
   * Callback for when the value of the knob changes.
   */
  onValue?: (value: number) => void;

  /**
   * The label of the knob. Note this is required for accessibility. To hide the label, set `showLabel` to "hidden".
   */
  label: string;

  /**
   * Whether we should show the label
   */
  showLabel?: "before" | "after" | "hidden";

  /**
   * Value formatter to convert values into strings
   */
  valueFormatter?: (value: number) => string;

  /**
   * Label for accessibility (can contain more information than `label`)
   */
  accessibilityLabel?: string;

  /**
   * Value to reset the knob to on reset-to-default gesture (double click)
   */
  defaultValue?: number;
};

const BALL_SIZE = 12;
const HEIGHT = 150;
const BALL_MARGIN = 2;
const BORDER_WIDTH = 1;

const DisplaySlider = ({
  value,
  grabbed,
  onGrabOrRelease,
  onValue,
}: SliderProps) => {
  const { containerProps, ballBottom } = useSlider({
    ballMargin: BALL_MARGIN,
    ballSize: BALL_SIZE,
    onGrabOrRelease,
    value,
    onValue,
  });
  return (
    <div
      className="slider-track"
      style={{
        height: `${HEIGHT}px`,
        width: `${BALL_SIZE + BALL_MARGIN * 2}px`,
        borderWidth: `${BORDER_WIDTH}px`,
        position: "relative",
        borderStyle: "solid",
        borderRadius: `${BALL_SIZE / 2}px`,
        marginRight: "6px",
        marginTop: `2px`,
        cursor: "pointer",
      }}
      {...containerProps}
    >
      <div>
        <div
          className={`slider-ball ${grabbed ? "slider-ball-grabbed" : ""}`}
          data-testid="slider-ball"
          style={{
            width: `${BALL_SIZE - BORDER_WIDTH * 2}px`,
            height: `${BALL_SIZE - BORDER_WIDTH * 2}px`,
            bottom: `${ballBottom}px`,
            left: `${BALL_MARGIN}px`,
            position: "absolute",
            borderRadius: `1000px`,
            borderWidth: `${BORDER_WIDTH}px`,
            borderStyle: "solid",
          }}
        ></div>
      </div>
    </div>
  );
};

const label = ({ label, grabbed, hover, valueLabel }: LabelProps) => (
  <div className="knob-label">{grabbed || hover ? valueLabel : label}</div>
);

export const Slider = ({ ...props }: Props) => (
  <InternalSlider
    {...props}
    Slider={DisplaySlider}
    Label={label}
    valueFormatter={(v) => v.toFixed(0)}
  />
);

export default Slider;
