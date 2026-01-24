import { useEnumParam } from "@conformal/plugin";
import EnumSlider, { Props as EnumSliderProps } from "./EnumSlider";
import { useMemo } from "react";

export type Props = {
  /**
   * The label of the control
   */
  label: string;

  /**
   * Label for accessibility (can contain more information than `label`)
   */
  accessibilityLabel?: string;

  /**
   * The order of the values.
   */
  order?: "normal" | "reversed";

  /**
   * The values to display.
   */
  displayFormatter?: (value: string) => string;

  /**
   * Overrides the label for certain values with a custom element.
   */
  CustomGlyph?: EnumSliderProps["CustomGlyph"];

  /**
   * The unique id of the parameter to control
   */
  param: string;
};

export const ParamEnumSlider = ({
  label,
  accessibilityLabel,
  order,
  displayFormatter,
  CustomGlyph,
  param,
}: Props) => {
  const { value, set, grab, release, info } = useEnumParam(param);
  const orderedValues = useMemo(
    () => (order === "reversed" ? info.values.slice().reverse() : info.values),
    [info.values, order],
  );
  return (
    <EnumSlider
      label={label}
      accessibilityLabel={accessibilityLabel}
      value={value}
      grab={grab}
      release={release}
      values={orderedValues}
      onValue={set}
      defaultValue={info.default}
      CustomGlyph={CustomGlyph}
      displayFormatter={displayFormatter}
    />
  );
};

export default ParamEnumSlider;
