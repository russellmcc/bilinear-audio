import { useKnob } from "plugin-ui";
import Knob, { Props } from ".";
import { Scale } from "music-ui/util";

export const ParamKnob = ({
  param,
  style,
  scale,
  ...props
}: Partial<Props> & {
  style?: Props["style"];
  param: string;
  scale?: Scale;
}) => {
  const knobProps = useKnob({
    param,
    scale,
  });
  return <Knob style={style} {...knobProps} {...props}></Knob>;
};

export default ParamKnob;
