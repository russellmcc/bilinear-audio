import ParamEnumSlider from "../components/ParamEnumSlider";
import { formatDynMode } from "../glyphs/dyn_mode";

export type Props = {
  param: string;
};

export const EnvModeSlider = ({ param }: Props) => (
  <ParamEnumSlider
    param={param}
    label="DYN MODE"
    displayFormatter={formatDynMode}
    order="reversed"
  />
);

export default EnvModeSlider;
