import { EnumKnob as EnumKnobModule } from "music-ui/kit";
import KnobDisplay from "./KnobDisplay";
import { LABEL_MARGIN } from "./constants";
import { useCallback } from "react";
import useOnGrabOrRelease from "./useGrabOrRelease";

export type Props = {
  /**
   * The current value of the knob
   */
  value?: string;

  /**
   * The possible values of the knob
   */
  values: string[];

  /**
   * The label of the knob.
   */
  label: string;

  /**
   * Label for beginning of range
   */
  minLabel?: string;

  /**
   * Label for end of range
   */
  maxLabel?: string;

  /**
   * Label for accessibility (can contain more information than `label`)
   */
  accessibilityLabel?: string;

  /**
   * Callback for when the value of the knob changes.
   */
  onValue?: (value: string) => void;

  /**
   * The default value of the knob.
   */
  defaultValue?: string;

  /**
   * Callback for when the knob is grabbed.
   */
  grab?: () => void;

  /**
   * Callback for when the knob is released.
   */
  release?: () => void;
};

const Label = ({ label }: EnumKnobModule.LabelProps) => (
  <div style={{ textAlign: "center", marginBottom: `${LABEL_MARGIN}px` }}>
    {label}
  </div>
);

export const EnumKnob = (props: Props) => {
  const { grab, release, minLabel, maxLabel } = props;
  const onGrabOrRelease = useOnGrabOrRelease({ grab, release });
  const display = useCallback(
    (props: EnumKnobModule.DisplayProps) => {
      const valuePercent = props.value
        ? (props.value / (props.valueCount - 1)) * 100
        : 0;
      return (
        <KnobDisplay
          value={valuePercent}
          grabbed={props.grabbed}
          hover={props.hover}
          minLabel={minLabel}
          maxLabel={maxLabel}
        />
      );
    },
    [minLabel, maxLabel],
  );
  return (
    <EnumKnobModule.EnumKnob
      {...props}
      onGrabOrRelease={onGrabOrRelease}
      Label={Label}
      Display={display}
      showLabel="before"
    />
  );
};

export default EnumKnob;
