import { useEnumParam, useNumericParam } from "@conformal/plugin";
import { useCallback } from "react";
import type { StringMode } from "../mode";
import { STRING_PRESETS } from "./preset";

export const STRING_ENSEMBLE_MODES = ["I", "II"] as const;
export type StringEnsembleMode = (typeof STRING_ENSEMBLE_MODES)[number];

export type Props = {
  mode: StringMode;
  setMode: (mode: StringMode) => void;
};

export const useStringState = ({ mode, setMode }: Props) => {
  const { set: setRateParam } = useNumericParam("rate");
  const { set: setRate2Param } = useNumericParam("rate_2");
  const { set: setDepthParam } = useNumericParam("depth");
  const { set: setDelayScaleParam } = useNumericParam("delay_scale");
  const { set: setEnsDepthParam } = useNumericParam("ens_depth");
  const { set: setRoutingParam } = useEnumParam("routing");

  const setEnsembleMode = useCallback(
    (ensembleMode: StringEnsembleMode) => {
      if (ensembleMode === mode.ensembleMode) {
        return;
      }

      const preset = STRING_PRESETS[ensembleMode];
      setMode({ ...mode, ensembleMode });
      setRoutingParam(preset.routing);
      setRateParam(preset.rate);
      setRate2Param(preset.rate_2);
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
      setRateParam,
      setRoutingParam,
    ],
  );

  return {
    ensembleMode: mode.ensembleMode,
    setEnsembleMode,
  };
};
