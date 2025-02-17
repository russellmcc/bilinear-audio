import { useNumericParam } from "@conformal/plugin";
import { Scale } from "music-ui/util";
import { useCallback, useMemo } from "react";
import { useGrabbed } from "./useGrabbed";

export type Props = {
  param: string;

  scale?: Scale;
};

export const useKnob = ({ param, scale }: Props) => {
  const {
    info: {
      title,
      valid_range: [min_value, max_value],
      default: unscaledDefault,
      units,
    },
    value,
    set,
    grab,
    release,
  } = useNumericParam(param);

  const { grabbed, onGrabOrRelease } = useGrabbed({ grab, release });

  const doScale = useCallback(
    (value: number) => {
      let scaled = ((value - min_value) / (max_value - min_value)) * 100;
      if (scale) {
        scaled = scale.from(scaled / 100) * 100;
      }
      return scaled;
    },
    [min_value, max_value, scale],
  );

  const scaled = doScale(value);
  const defaultValue = doScale(unscaledDefault);

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

  const onValue = useCallback(
    (scaled: number) => {
      set(unscale(scaled));
    },
    [unscale, set],
  );

  const valueFormatter = useMemo(
    () => (value: number) => `${unscale(value).toFixed(0)}${units}`,
    [unscale, units],
  );

  return {
    grabbed,
    value: scaled,
    onGrabOrRelease,
    onValue,
    defaultValue,
    valueFormatter,
    label: title,
  };
};

export default useKnob;
