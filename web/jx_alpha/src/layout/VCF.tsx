import { useEnumParam } from "@conformal/plugin";
import ParamEnumSlider from "../components/ParamEnumSlider";
import { useCallback } from "react";
import ParamSlider from "../components/ParamSlider";
import EnvModeSlider from "../components/EnvModeSlider";
import VCA from "./VCA";

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
        justifyContent: "space-between",
        gap: "11px",
        height: "100%",
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          gap: "11px",
          paddingRight: "11px",
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
          <ParamSlider param="vcf_key" label="KEY" scale="continuation" />
        </div>
      </div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-between",
        }}
      >
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            justifyContent: "flex-end",
            paddingBottom: "11px",
          }}
        >
          <EnvModeSlider param="vcf_env_source"></EnvModeSlider>
        </div>
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            justifyContent: "flex-end",
            paddingLeft: "11px",
            paddingBottom: "11px",
            paddingRight: "11px",
            borderLeft: "2px solid var(--darkest-color)",
            borderTop: "2px solid var(--darkest-color)",
          }}
        >
          <VCA />
        </div>
      </div>
    </div>
  </div>
);

export default VCF;
