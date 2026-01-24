import { Knob as KnobModule } from "music-ui/kit";
import KnobDisplay from "./KnobDisplay";
import { useCallback } from "react";
import { LABEL_MARGIN } from "./constants";
import useOnGrabOrRelease from "./useGrabOrRelease";

export type Props = {
  /**
   * The current value of the knob (scaled to 0-100)
   */
  value: number;

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
  onValue?: (value: number) => void;

  /**
   * The default value of the knob.
   */
  defaultValue?: number;

  /**
   * Callback for when the knob is grabbed.
   */
  grab?: () => void;

  /**
   * Callback for when the knob is released.
   */
  release?: () => void;
};

const Label = ({ label }: KnobModule.LabelProps) => (
  <div style={{ textAlign: "center", marginBottom: `${LABEL_MARGIN}px` }}>
    {label}
  </div>
);

export const Knob = (props: Props) => {
  const { grab, release, minLabel, maxLabel } = props;
  const onGrabOrRelease = useOnGrabOrRelease({ grab, release });
  const display = useCallback(
    (props: KnobModule.DisplayProps) => (
      <KnobDisplay {...props} minLabel={minLabel} maxLabel={maxLabel} />
    ),
    [minLabel, maxLabel],
  );
  return (
    <KnobModule.Knob
      {...props}
      onGrabOrRelease={onGrabOrRelease}
      showLabel="before"
      Display={display}
      Label={Label}
    />
  );
};

export default Knob;
