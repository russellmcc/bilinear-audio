import { useEnumParam, useNumericParam } from "@conformal/plugin";
import { useCallback } from "react";
import type { Rs79Mode } from "../mode";
import { RS_79_PRESETS } from "./preset";

export const RS_79_ENSEMBLE_MODES = ["I", "II"] as const;
export type Rs79EnsembleMode = (typeof RS_79_ENSEMBLE_MODES)[number];

export type Props = {
  mode: Rs79Mode;
  setMode: (mode: Rs79Mode) => void;
};

export const useRs79State = ({ mode, setMode }: Props) => {
  const { set: setRateParam } = useNumericParam("rate");
  const { set: setRate2Param } = useNumericParam("rate_2");
  const { set: setRate3Param } = useNumericParam("rate_3");
  const { set: setRate4Param } = useNumericParam("rate_4");
  const { set: setDepthParam } = useNumericParam("depth");
  const { set: setDelayScaleParam } = useNumericParam("delay_scale");
  const { set: setEnsDepthParam } = useNumericParam("ens_depth");
  const { set: setRoutingParam } = useEnumParam("routing");

  const setEnsembleMode = useCallback(
    (ensembleMode: Rs79EnsembleMode) => {
      if (ensembleMode === mode.ensembleMode) {
        return;
      }

      const preset = RS_79_PRESETS[ensembleMode];
      setMode({ ...mode, ensembleMode });
      setRoutingParam(preset.routing);
      setRateParam(preset.rate);
      setRate2Param(preset.rate_2);
      setRate3Param(preset.rate_3);
      setRate4Param(preset.rate_4);
      setDepthParam(preset.depth);
      setDelayScaleParam(preset.delay_scale);
      setEnsDepthParam(preset.ens_depth);
    },
    [
      mode,
      setDelayScaleParam,
      setDepthParam,
      setEnsDepthParam,
      setMode,
      setRate2Param,
      setRate3Param,
      setRate4Param,
      setRateParam,
      setRoutingParam,
    ],
  );

  return {
    ensembleMode: mode.ensembleMode,
    setEnsembleMode,
  };
};
