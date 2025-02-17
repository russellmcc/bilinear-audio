import { Knob as KnobKit } from "music-ui/kit";
import { useCallback, useState } from "react";
import { useNumericParam } from "@conformal/plugin";
import { Scale } from "music-ui/util";

export type Props = {
  /**
   * The label of the knob. To hide the label, set `showLabel` to false.
   */
  label?: string;

  /**
   * Whether we should show the label
   */
  showLabel?: boolean;

  /**
   * Label for accessibility (can contain more information than `label`)
   */
  accessibilityLabel?: string;

  /** The param id to connect this knob to */
  param: string;

  /**
   * The scale to apply to the knob
   */
  scale?: Scale;

  /**
   * A component for the knob display
   */
  Display?: KnobKit.DisplayComponent;

  /**
   * A component for the knob label
   */
  Label?: KnobKit.LabelComponent;
};

export const Knob = ({
  label,
  showLabel,
  accessibilityLabel,
  Display,
  Label,
  scale,
  param,
}: Props) => {
  const [grabbed, setGrabbed] = useState(false);
  const {
    info: {
      title,
      valid_range: [min_value, max_value],
      default: defaultValue,
      units,
    },
    value,
    set,
    grab,
    release,
  } = useNumericParam(param);
  let scaled = ((value - min_value) / (max_value - min_value)) * 100;
  if (scale) {
    scaled = scale.from(scaled / 100) * 100;
  }
  const unscale = useCallback(
    (scaled: number) => {
      let unscaledValue = Math.min(Math.max(scaled / 100, 0.0), 1.0);
      if (scale) {
        unscaledValue = scale.to(unscaledValue);
      }

      return unscaledValue * (max_value - min_value) + min_value;
    },
    [max_value, min_value, scale],
  );

  const onGrabOrRelease = useCallback(
    (grabbed: boolean) => {
      setGrabbed(grabbed);
      if (grabbed) {
        grab();
      } else {
        release();
      }
    },
    [grab, release, setGrabbed],
  );
  return (
    <KnobKit.Knob
      label={label ?? title}
      value={scaled}
      onValue={(scaled) => {
        set(unscale(scaled));
      }}
      grabbed={grabbed}
      onGrabOrRelease={onGrabOrRelease}
      valueFormatter={
        units ? (value) => `${unscale(value).toFixed(0)}${units}` : undefined
      }
      accessibilityLabel={accessibilityLabel}
      defaultValue={defaultValue}
      Display={Display}
      Label={Label}
      showLabel={showLabel}
    />
  );
};
