import { LABEL_MARGIN } from "../components/constants";
import DynModeSlider from "../components/DynModeSlider";
import { LINE_SPACING } from "../components/EnumSlider";
import { BRACKET_WIDTH, GroupLabel } from "../components/EnvSourceLabels";
import ParamEnumSlider from "../components/ParamEnumSlider";
import ParamSlider from "../components/ParamSlider";
import VCAEnvSource from "../glyphs/VCAEnvSource";

export const VCA = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "column",
    }}
  >
    <h1>VCA</h1>
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
          gap: "5px",
        }}
      >
        <ParamSlider param="level" label="LEVEL" scale="labeled" />
        <div style={{ display: "flex", flexDirection: "row" }}>
          <ParamEnumSlider
            param="vca_env_source"
            label="ENV MODE"
            CustomGlyph={VCAEnvSource}
          />
          <div style={{ display: "flex", flexDirection: "column" }}>
            {/* Spacer to align with the slider label "ENV MODE" */}
            <div
              style={{
                textAlign: "right",
                marginBottom: `${LABEL_MARGIN}px`,
                width: `${BRACKET_WIDTH}px`,
                whiteSpace: "nowrap",
                visibility: "hidden",
                flexGrow: 0,
              }}
            >
              ENV MODE
            </div>
            <div style={{ height: `${LINE_SPACING * 3}px` }}></div>

            <GroupLabel label="2" count={2} />
          </div>
          <div style={{ marginLeft: "11px" }}>
            <DynModeSlider param="vca_dyn_mode" />
          </div>
        </div>
      </div>
    </div>
  </div>
);

export default VCA;
