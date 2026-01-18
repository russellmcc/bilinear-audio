import { useNumericParam } from "@conformal/plugin";
import Slider, { ScaleType } from "./Slider";
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
   * The visual scale to display to the left of the slider.
   */
  scale?: ScaleType;
};

export const ParamSlider = ({ label, param, scale }: Props) => {
  const { value, set, grab, release, info } = useNumericParam(param);
  const [min, max] = info.valid_range;
  const rescaleToPercentage = useCallback(
    (value: number) => ((value - min) / (max - min)) * 100,
    [min, max],
  );
  const rescaleFromPercentage = useCallback(
    (value: number) => min + (max - min) * (value / 100),
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
    />
  );
};
