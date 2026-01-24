import { useEnumParam } from "@conformal/plugin";
import EnumKnob from "./EnumKnob";

export type Props = {
  /**
   * The label of the control
   */
  label: string;

  /**
   * The unique id of the parameter to control
   */
  param: string;

  /**
   * Label for accessibility (can contain more information than `label`)
   */
  accessibilityLabel?: string;

  /**
   * The label for the beginning of the range
   */
  minLabel?: string;

  /**
   * The label for the end of the range
   */
  maxLabel?: string;
};

export const ParamEnumKnob = (props: Props) => {
  const { param, label, accessibilityLabel, minLabel, maxLabel } = props;
  const { value, set, grab, release, info } = useEnumParam(param);
  return (
    <EnumKnob
      label={label}
      accessibilityLabel={accessibilityLabel}
      minLabel={minLabel}
      maxLabel={maxLabel}
      value={value}
      onValue={set}
      grab={grab}
      release={release}
      values={info.values}
      defaultValue={info.default}
    />
  );
};

export default ParamEnumKnob;
