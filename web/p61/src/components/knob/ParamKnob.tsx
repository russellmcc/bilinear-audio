import { Knob } from "plugin-ui";
import { Props } from ".";
import { useDisplay } from "./useDisplay";
import Label from "./Label";

export const ParamKnob = ({
  style,
  ...props
}: Omit<Knob.Props, "Display" | "Label"> & { style?: Props["style"] }) => (
  <Knob.Knob {...props} Display={useDisplay({ style })} Label={Label} />
);

export default ParamKnob;
