import ParamEnumKnob from "../components/ParamEnumKnob";
import ParamEnumSlider from "../components/ParamEnumSlider";
import ParamKnob from "../components/ParamKnob";
import ParamSlider from "../components/ParamSlider";
import Shape from "../glyphs/Shape";
import { formatXmod } from "../glyphs/xmod";

export const DCO2 = () => (
  <div
    style={{
      paddingLeft: "5px",
    }}
  >
    <h1>DCO-2</h1>
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "stretch",
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-between",
          gap: "5px",
        }}
      >
        <ParamEnumSlider param="dco2_range" label="RANGE" order="reversed" />
        <ParamEnumSlider param="dco2_shape" label="SHAPE" CustomGlyph={Shape} />
        <ParamEnumSlider
          param="x_mod"
          label="X-MOD"
          order="reversed"
          displayFormatter={formatXmod}
        />
      </div>
      <div style={{ flexGrow: 1, minHeight: "11px" }}></div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-evenly",
        }}
      >
        <ParamEnumKnob
          param="dco2_tune"
          label="TUNE"
          minLabel="-12"
          maxLabel="+12"
        />
        <ParamKnob
          param="dco2_fine_tune"
          label="FINE"
          minLabel="-"
          maxLabel="+"
        />
      </div>
      <div style={{ flexGrow: 1, minHeight: "11px" }}></div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "center",
        }}
      >
        <ParamSlider param="dco2_env" label="ENV" scale="labeled" />
        <ParamSlider param="dco2_lfo" label="LFO" scale="continuation" />
        <ParamSlider param="dco2_pwm_depth" label="PWM" scale="continuation" />
        <ParamSlider param="dco2_pwm_rate" label="RATE" scale="continuation" />
      </div>
    </div>
  </div>
);

export default DCO2;
