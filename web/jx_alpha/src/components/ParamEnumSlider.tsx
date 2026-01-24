import { useEnumParam } from "@conformal/plugin";
import { EnumSlider } from "./EnumSlider";

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
   * The unique id of the parameter to control
   */
  param: string;
};

export const ParamEnumSlider = ({
  label,
  accessibilityLabel,
  param,
}: Props) => {
  const { value, set, grab, release, info } = useEnumParam(param);
  return (
    <EnumSlider
      label={label}
      accessibilityLabel={accessibilityLabel}
      value={value}
      grab={grab}
      release={release}
      values={info.values}
      onValue={set}
      defaultValue={info.default}
    />
  );
};

export default ParamEnumSlider;
