import { rescale } from "music-ui/util";
import Knob from "./Knob";
import { useNumericParam } from "@conformal/plugin";
import { useCallback } from "react";

export type Props = {
  /**
   * The label of the slider
   */
  label: string;

  /**
   * The unique id of the parameter to control
   */
  param: string;

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
};

export const ParamKnob = (props: Props) => {
  const { param } = props;
  const { value, set, grab, release, info } = useNumericParam(param);
  const [min, max] = info.valid_range;
  const rescaleToPercentage = useCallback(
    (value: number) => rescale(value, min, max, 0, 100),
    [min, max],
  );
  const rescaleFromPercentage = useCallback(
    (value: number) => rescale(value, 0, 100, min, max),
    [min, max],
  );
  const onValue = useCallback(
    (value: number) => {
      set(rescaleFromPercentage(value));
    },
    [rescaleFromPercentage, set],
  );
  return (
    <Knob
      {...props}
      value={rescaleToPercentage(value)}
      onValue={onValue}
      grab={grab}
      release={release}
      defaultValue={rescaleToPercentage(info.default)}
    />
  );
};

export default ParamKnob;
