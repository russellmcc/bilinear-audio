import { useEnumParam } from "@conformal/plugin";
import { useGrabbed } from "./useGrabbed";
import { useCallback } from "react";

export type Props = {
  param: string;

  valueFormatter?: (value: string) => string;
};

export const useEnumSlider = ({ param }: Props) => {
  const {
    info: { title, values, default: defaultValue },
    value,
    set,
    grab,
    release,
  } = useEnumParam(param);
  const { grabbed, onGrabOrRelease } = useGrabbed({ grab, release });

  const onValue = useCallback(
    (value: string) => {
      set(value);
    },
    [set],
  );
  return {
    label: title,
    grabbed,
    onGrabOrRelease,
    onValue,
    value,
    values,
    defaultValue,
  };
};

export default useEnumSlider;
