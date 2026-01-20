import {
  EnumKnob as InternalEnumKnob,
  DisplayProps,
  LabelProps,
} from "../kit/enum-knob";
import KnobDisplay from "./KnobDisplay";

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

const Display = ({ valueCount, value, grabbed, hover }: DisplayProps) => (
  <KnobDisplay
    value={value ? (value / (valueCount - 1)) * 100 : 0}
    grabbed={grabbed}
    hover={hover}
  />
);
const Label = ({ label, grabbed, hover, value }: LabelProps) => (
  <div className="knob-label">{grabbed || hover ? value : label}</div>
);

export const EnumKnob = (props: Props) => (
  <InternalEnumKnob {...props} Display={Display} Label={Label} />
);

export default EnumKnob;
