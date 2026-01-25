import EnvModeSlider from "../components/EnvModeSlider";
import ParamSlider from "../components/ParamSlider";

export const Mixer = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "column",
      height: "100%",
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
      </div>
      <div style={{ flexGrow: 1 }}></div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-evenly",
        }}
      >
        <ParamSlider param="mix_env" label="ENV" scale="labeled" />
      </div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-evenly",
        }}
      >
        <EnvModeSlider param="mix_env_source" />
      </div>
    </div>
  </div>
);

export default Mixer;
