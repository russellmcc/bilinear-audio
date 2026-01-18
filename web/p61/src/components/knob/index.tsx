import Label from "./Label.tsx";
import { Knob as KnobKit } from "music-ui/kit";
import { useDisplay } from "./useDisplay.tsx";

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
  showLabel?: "before" | "after" | "hidden";

  /**
   * Value formatter to convert values into strings
   */
  valueFormatter?: (value: number) => string;

  /**
   * The style of the knob
   */
  style?: "primary" | "secondary";

  /**
   * Label for accessibility (can contain more information than `label`)
   */
  accessibilityLabel?: string;

  /**
   * Value to reset the knob to on reset-to-default gesture (double click)
   */
  defaultValue?: number;
};

const Knob = ({ style = "primary", ...props }: Props) => (
  <KnobKit.Knob {...props} Display={useDisplay({ style })} Label={Label} />
);

export default Knob;
