import EnvModeSlider from "../components/EnvModeSlider";
import ParamSlider from "../components/ParamSlider";

export const Mixer = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "column",
      height: "100%",
      paddingBottom: "11px",
    }}
  >
    <h1>MIXER</h1>
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
        }}
      >
        <ParamSlider param="mix_dco1" label="DCO1" scale="labeled" />
        <ParamSlider param="mix_dco2" label="DCO2" scale="continuation" />
        <ParamSlider param="mix_env" label="ENV" scale="continuation" />
      </div>
      {/* Somewhat gross alignment hack to get this lined up visually with DCO modules*/}
      <div style={{ height: "43px" }}> </div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-around",
        }}
      >
        <EnvModeSlider param="mix_env_source" />
      </div>
    </div>
  </div>
);

export default Mixer;
