import ParamEnumKnob from "../components/ParamEnumKnob";
import ParamEnumSlider from "../components/ParamEnumSlider";
import ParamSlider from "../components/ParamSlider";
import Shape from "../glyphs/Shape";

export const DCO1 = () => (
  <div
    style={{
      borderRight: "2px solid var(--darkest-color)",
      backgroundOrigin: "border-box",
      paddingRight: "11px",
      paddingTop: "11px",
    }}
  >
    <h1>DCO-1</h1>
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
          justifyContent: "space-between",
          gap: "11px",
        }}
      >
        <ParamEnumSlider param="dco1_range" label="RANGE" order="reversed" />
        <ParamEnumSlider param="dco1_shape" label="SHAPE" CustomGlyph={Shape} />
      </div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-evenly",
        }}
      >
        <ParamEnumKnob
          param="dco1_tune"
          label="TUNE"
          minLabel="-12"
          maxLabel="+12"
        />
      </div>
      <div style={{ display: "flex", flexDirection: "row" }}>
        <ParamSlider param="dco1_env" label="ENV" scale="labeled" />
        <ParamSlider param="dco1_lfo" label="LFO" scale="continuation" />
        <ParamSlider param="dco1_pwm_depth" label="PWM" scale="continuation" />
        <ParamSlider param="dco1_pwm_rate" label="RATE" scale="continuation" />
      </div>
    </div>
  </div>
);

export default DCO1;
