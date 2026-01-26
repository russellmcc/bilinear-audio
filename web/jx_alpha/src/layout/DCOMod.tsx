import ParamEnumKnob from "../components/ParamEnumKnob";
import EnvModeSlider from "../components/EnvModeSlider";
import LFO from "./LFO";

export const DCOMod = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "row",
      justifyContent: "space-between",
      width: "100%",
    }}
  >
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        gap: "22px",
        paddingRight: "11px",
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
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        paddingLeft: "11px",
        paddingRight: "11px",
        paddingBottom: "11px",
        borderLeft: "2px solid var(--darkest-color)",
        borderTop: "2px solid var(--darkest-color)",
        alignItems: "flex-end",
      }}
    >
      <LFO />
    </div>
  </div>
);
export default DCOMod;
