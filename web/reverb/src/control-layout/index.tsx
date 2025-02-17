import Knob from "../components/Knob";

export const ControlLayout = () => (
  <div style={{ width: "100%" }}>
    <div
      style={{
        display: "flex",
        justifyContent: "center",
        marginBottom: "1rem",
      }}
    >
      <Knob style="big" param="mix" label="mix" />
    </div>
    <div
      style={{
        display: "flex",
        justifyContent: "space-between",
        paddingLeft: "13px",
        paddingRight: "13px",
        width: "calc(100% - 26px)",
      }}
    >
      <Knob style="small" param="time" label="time" />
      <Knob style="small" param="tone" label="tone" />
      <Knob style="small" param="brightness" label="bright" />
      <Knob style="small" param="density" label="dense" />
      <Knob style="small" param="early_reflections" label="early" />
    </div>
  </div>
);

export default ControlLayout;
