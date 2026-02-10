import ParamEnumSlider from "../components/ParamEnumSlider";
import ParamSlider from "../components/ParamSlider";
import formatLfoTrig from "../glyphs/lfo_trig";
import LFOShape from "../glyphs/LFOShape";

export const LFO = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "column",
      paddingLeft: "5px",
      paddingRight: "11px",
      paddingTop: "13px",
      borderTop: "2px solid var(--darkest-color)",
    }}
  >
    <h1>LFO</h1>
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "stretch",
        gap: "11px",
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
        <ParamEnumSlider
          param="lfo_trig"
          label="TRIG"
          order="reversed"
          displayFormatter={formatLfoTrig}
        />
      </div>
    </div>
  </div>
);

export default LFO;
