import ParamEnumSlider from "../components/ParamEnumSlider";
import ParamSlider from "../components/ParamSlider";
import LFOShape from "../glyphs/LFOShape";

export const LFO = () => (
  <div>
    <h1>LFO</h1>
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
        <ParamEnumSlider
          param="lfo_shape"
          label="SHAPE"
          CustomGlyph={LFOShape}
        />

        <ParamSlider param="lfo_rate" label="RATE" scale="labeled" />
        <ParamSlider param="lfo_delay" label="DELAY" scale="continuation" />
      </div>
    </div>
  </div>
);

export default LFO;
