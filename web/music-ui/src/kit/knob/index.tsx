import useGesture from "./useGesture.ts";
import { useMemo } from "react";

export type DisplayProps = {
  /** Currrent value of the knob */
  value: number;

  /** True if the knob is grabbed */
  grabbed: boolean;

  /** True if the knob is hovered */
  hover: boolean;
};

export type DisplayComponent = React.ComponentType<DisplayProps>;

export type LabelProps = {
  /** The label of the knob */
  label: string;

  /** True if the knob is hovered */
  hover: boolean;

  /** True if the knob is grabbed */
  grabbed: boolean;

  /** The to display for the current value of the knob */
  valueLabel: string;
};

export type LabelComponent = React.ComponentType<LabelProps>;

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
   * Note that this may be called spruriously even if the grabbed state didn't change.
   */
  onGrabOrRelease?: (grabbed: boolean) => void;

  /**
   * Callback for when the value of the knob changes.
   * Note that this may be called spuriously even if the value didn't change.
   */
  onValue?: (value: number) => void;

  /**
   * The label of the knob. Note this is required for accessibility. To hide the label, set `showLabel` to false.
   */
  label: string;

  /**
   * Whether we should show the label
   */
  showLabel?: boolean;

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

  /**
   * A component for the knob display
   */
  Display?: DisplayComponent;

  /**
   * A component for the knob label
   */
  Label?: LabelComponent;
};

export const Knob = ({
  value,
  grabbed,
  onGrabOrRelease,
  onValue,
  label,
  valueFormatter,
  showLabel = true,
  accessibilityLabel,
  defaultValue,
  Display,
  Label,
}: Props) => {
  const { hover, props } = useGesture({
    value,
    onGrabOrRelease,
    onValue,
    defaultValue,
  });
  const valueLabel = useMemo(
    () => (valueFormatter ? valueFormatter(value) : label),
    [valueFormatter, value, label],
  );
  return (
    <div
      role="slider"
      aria-label={accessibilityLabel ?? label}
      aria-valuemin={0}
      aria-valuemax={100}
      aria-valuenow={value}
      aria-orientation="vertical"
      aria-valuetext={valueFormatter ? valueLabel : String(value)}
      tabIndex={0}
      style={{
        display: "inline-block",
        cursor: "default",
        touchAction: "none",
        userSelect: "none",
        WebkitUserSelect: "none",
      }}
      {...props}
    >
      {Display && (
        <Display value={value} grabbed={grabbed ?? false} hover={hover} />
      )}
      {showLabel && Label && (
        <Label
          label={label}
          hover={hover}
          grabbed={grabbed ?? false}
          valueLabel={valueLabel}
        />
      )}
    </div>
  );
};

export default Knob;
