import { useEnumParam, useNumericParam } from "@conformal/plugin";
import { useCallback } from "react";
import type { Ju60Mode } from "../mode";
import { JU_60_PRESETS, type Ju60ButtonMode } from "./constants";

export type Props = {
  mode: Ju60Mode;
  setMode: (mode: Ju60Mode) => void;
};

export const useJu60State = ({ mode, setMode }: Props) => {
  const { set: setRateParam } = useNumericParam("rate");
  const { set: setDepthParam } = useNumericParam("depth");
  const { set: setRoutingParam } = useEnumParam("routing");

  const setButtonMode = useCallback(
    (buttonMode: Ju60ButtonMode) => {
      if (buttonMode === mode.buttonMode) {
        return;
      }

      const preset = JU_60_PRESETS[buttonMode];
      setMode({ ...mode, buttonMode });
      setRoutingParam(preset.routing);
      setDepthParam(preset.depth);
      setRateParam(preset.rate);
    },
    [mode, setDepthParam, setMode, setRateParam, setRoutingParam],
  );

  return {
    buttonMode: mode.buttonMode,
    setButtonMode,
  };
};
