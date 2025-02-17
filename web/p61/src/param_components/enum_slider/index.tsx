import { useEnumSlider } from "plugin-ui";
import EnumSlider, { Props } from "../../components/enum_slider";

export type ParamEnumSliderProps = {
  param: string;

  label?: string;

  accessibilityLabel?: string;

  width?: Props["width"];
  textAlign?: Props["textAlign"];

  displayFormatter?: Props["displayFormatter"];
};

export const ParamEnumSlider = ({
  param,
  label,
  accessibilityLabel,
  width,
  textAlign,
  displayFormatter,
}: ParamEnumSliderProps) => {
  const { label: defaultLabel, ...props } = useEnumSlider({ param });

  return (
    <EnumSlider
      {...props}
      label={label ?? defaultLabel}
      accessibilityLabel={accessibilityLabel}
      displayFormatter={displayFormatter ?? ((value) => value.toLowerCase())}
      width={width}
      textAlign={textAlign}
    />
  );
};

export default ParamEnumSlider;
