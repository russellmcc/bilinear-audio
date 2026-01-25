import ParamEnumKnob from "../components/ParamEnumKnob";
import EnvModeSlider from "../components/EnvModeSlider";

export const DCOMod = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "row",
      justifyContent: "space-around",
      paddingRight: "5px",
      width: "100%",
    }}
  >
    <EnvModeSlider param="dco_env_source" />
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "space-evenly",
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
