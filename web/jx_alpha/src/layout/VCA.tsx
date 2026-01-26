import ParamEnumSlider from "../components/ParamEnumSlider";
import ParamSlider from "../components/ParamSlider";
import VCAEnvSource from "../glyphs/VCAEnvSource";

export const VCA = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "column",
    }}
  >
    <h1>VCA</h1>
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "stretch",
        gap: "11px",
        height: "100%",
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          gap: "5px",
        }}
      >
        <ParamSlider param="level" label="LEVEL" scale="labeled" />
        {/* TODO: add right label for env 2 */}
        <ParamEnumSlider
          param="vca_env_source"
          label="ENV MODE"
          CustomGlyph={VCAEnvSource}
        />
      </div>
    </div>
  </div>
);

export default VCA;
