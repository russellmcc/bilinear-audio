import ParamKnob from "./components/ParamKnob";
import ParamEnumSlider from "./components/ParamEnumSlider";
import ParamSlider from "./components/ParamSlider";

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
  </div>
);

export default Layout;
