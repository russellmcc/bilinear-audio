import ParamKnob from "./components/ParamKnob";
import ParamEnumSlider from "./components/ParamEnumSlider";
import ParamSlider from "./components/ParamSlider";
import EnumKnob from "./components/EnumKnob";

const Layout = () => (
  <div style={{ display: "flex" }}>
    <ParamSlider param="level" label="LEVEL" scale="labeled" />
    <ParamSlider param="dco1_pwm_depth" label="DEPTH" scale="continuation" />
    <ParamEnumSlider label="MOD" param="x_mod" />
    <ParamKnob
      label="TUNE"
      param="dco2_fine_tune"
      minLabel="-12"
      maxLabel="+12"
    />
    <EnumKnob
      label="TUNE"
      value="0"
      minLabel="-12"
      maxLabel="+12"
      values={["-12", "-6", "0", "+6", "+12"]}
    />
  </div>
);

export default Layout;
