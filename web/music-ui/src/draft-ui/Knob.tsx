import { Knob as InternalKnob, LabelProps } from "../kit/knob";
import KnobDisplay from "./KnobDisplay";

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
   * The label of the knob. Note this is required for accessibility. To hide the label, set `showLabel` to false.
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

const label = ({ label, grabbed, hover, valueLabel }: LabelProps) => (
  <div className="knob-label">{grabbed || hover ? valueLabel : label}</div>
);

export const Knob = ({ ...props }: Props) => (
  <InternalKnob
    {...props}
    Display={KnobDisplay}
    Label={label}
    valueFormatter={(v) => v.toFixed(0)}
  />
);

export default Knob;
