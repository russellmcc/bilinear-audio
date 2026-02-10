import ParamEnumKnob from "../components/ParamEnumKnob";
import EnvModeSlider from "../components/EnvModeSlider";

export const DCOMod = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "row",
      flexGrow: 1,
      paddingRight: "11px",
      paddingBottom: "11px",
      justifyContent: "space-around",
    }}
  >
    <EnvModeSlider param="dco_env_source" />
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "space-evenly",
        paddingRight: "11px",
      }}
    >
      <ParamEnumKnob
        param="dco_bend_range"
        label="BEND"
        minLabel="1"
        maxLabel="12"
        showCenterTick={false}
      />
    </div>
  </div>
);
export default DCOMod;
