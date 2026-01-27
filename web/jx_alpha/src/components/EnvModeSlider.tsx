import ParamEnumSlider from "../components/ParamEnumSlider";
import EnvSource from "../glyphs/EnvSource";
import EnvSourceLabels from "./EnvSourceLabels";

export type Props = {
  param: string;
};

export const EnvModeSlider = ({ param }: Props) => (
  <div style={{ display: "flex", flexDirection: "row" }}>
    <ParamEnumSlider param={param} label="ENV MODE" CustomGlyph={EnvSource} />
    <EnvSourceLabels />
  </div>
);

export default EnvModeSlider;
