import { useEnumParam, useNumericParam } from "@conformal/plugin";
import { useCallback } from "react";
import type { Rs79Mode } from "../mode";
import { RS_79_PRESETS, type Rs79EnsembleMode } from "./constants";

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
  const { set: setEns2DepthParam } = useNumericParam("ens_2_depth");
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
      setEns2DepthParam(preset.ens_2_depth);
    },
    [
      mode,
      setDepthParam,
      setEns2DepthParam,
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
