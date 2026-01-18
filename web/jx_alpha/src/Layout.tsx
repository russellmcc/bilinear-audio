import { ParamSlider } from "./components/ParamSlider";

const Layout = () => (
  <>
    <ParamSlider param="level" label="LEVEL" scale="labeled" />
    <ParamSlider param="dco1_pwm_depth" label="DEPTH" scale="continuation" />
  </>
);

export default Layout;
