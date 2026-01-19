import { ParamEnumSlider } from "./components/ParamEnumSlider";
import { ParamSlider } from "./components/ParamSlider";

const Layout = () => (
  <div style={{ display: "flex" }}>
    <ParamSlider param="level" label="LEVEL" scale="labeled" />
    <ParamSlider param="dco1_pwm_depth" label="DEPTH" scale="continuation" />
    <ParamEnumSlider label="MOD" param="x_mod" />
  </div>
);

export default Layout;
