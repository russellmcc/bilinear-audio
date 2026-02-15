import ParamEnumSlider from "../components/ParamEnumSlider";
import ParamSlider from "../components/ParamSlider";
import formatLfoTrig from "../glyphs/lfo_trig";
import LFOShape from "../glyphs/LFOShape";

export const LFO = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "column",
      position: "relative",
      paddingLeft: "11px",
      paddingRight: "11px",
      paddingTop: "13px",
    }}
  >
    <div
      style={{
        position: "absolute",
        top: "0px",
        left: "0px",
        width: "100%",
        paddingTop: "13px",
        paddingLeft: "11px",
        paddingBottom: "11px",
        paddingRight: "11px",
        height: "308px",
        display: "flex",
        borderLeft: "2px solid var(--darkest-color)",
        borderTop: "2px solid var(--darkest-color)",
        flexDirection: "column",
      }}
    >
      <h1>LFO</h1>
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          alignItems: "stretch",
          justifyContent: "space-between",
          flexGrow: 1,
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
        </div>
        <div
          style={{
            display: "flex",
            flexDirection: "row",
            gap: "5px",
          }}
        >
          {" "}
          <ParamSlider param="lfo_delay" label="DELAY" scale="labeled" />
          <ParamEnumSlider
            param="lfo_trig"
            label="TRIG"
            order="reversed"
            displayFormatter={formatLfoTrig}
          />
        </div>
      </div>
    </div>
  </div>
);

export default LFO;
