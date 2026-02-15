import ParamEnumKnob from "../components/ParamEnumKnob";
import EnvModeSlider from "../components/EnvModeSlider";
import DynModeSlider from "../components/DynModeSlider";

export const DCOMod = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "row",
      paddingRight: "11px",
      paddingBottom: "11px",
      gap: "11px",
    }}
  >
    <EnvModeSlider param="dco_env_source" />
    <DynModeSlider param="dco_dyn_mode" />
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
