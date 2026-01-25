import { useEnumParam } from "@conformal/plugin";
import ParamEnumSlider from "../components/ParamEnumSlider";
import { useCallback } from "react";
import ParamSlider from "../components/ParamSlider";
import EnvModeSlider from "../components/EnvModeSlider";

const HPFSlider = () => {
  const param = "hpf_mode";
  const { info } = useEnumParam(param);
  const valueIndexFormatter = useCallback(
    (value: string) => info.values.indexOf(value).toString(),
    [info.values],
  );
  return (
    <ParamEnumSlider
      param={param}
      label="HPF"
      order="reversed"
      displayFormatter={valueIndexFormatter}
    />
  );
};

export const VCF = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "column",
      height: "100%",
    }}
  >
    <h1>VCF</h1>
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
        <HPFSlider />
        <div
          style={{
            display: "flex",
            flexDirection: "row",
          }}
        >
          <ParamSlider param="vcf_cutoff" label="FREQ" scale="labeled" />
          <ParamSlider param="resonance" label="RES" scale="continuation" />
          <ParamSlider param="vcf_env" label="ENV" scale="continuation" />
          <ParamSlider param="vcf_lfo" label="LFO" scale="continuation" />
          <ParamSlider
            param="vcf_key_follow"
            label="KEY"
            scale="continuation"
          />
          <EnvModeSlider param="vcf_env_source"></EnvModeSlider>
        </div>
      </div>
      <div style={{ flexGrow: 1 }}></div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-around",
        }}
      ></div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-evenly",
        }}
      ></div>
    </div>
  </div>
);

export default VCF;
