import { useNumericParam } from "@conformal/plugin";
import Slider, { ScaleType } from "./Slider";
import { useCallback } from "react";
import { rescale } from "music-ui/util";

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
   * The visual scale to display to the left of the slider.
   */
  scale?: ScaleType;

  /**
   * Label for accessibility (can contain more information than `label`)
   */
  accessibilityLabel?: string;
};

export const ParamSlider = ({
  label,
  param,
  scale,
  accessibilityLabel,
}: Props) => {
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
    <Slider
      label={label}
      value={rescaleToPercentage(value)}
      onValue={onValue}
      grab={grab}
      release={release}
      scale={scale}
      defaultValue={rescaleToPercentage(info.default)}
      accessibilityLabel={accessibilityLabel}
    />
  );
};

export default ParamSlider;
