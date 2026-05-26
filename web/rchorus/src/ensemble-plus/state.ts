import { useEnumParam, useNumericParam } from "@conformal/plugin";
import { useCallback } from "react";
import type { EnsemblePlusMode } from "../mode";
import { ENSEMBLE_PLUS_PRESETS, type EnsemblePlusVoiceMode } from "./constants";

export type Props = {
  mode: EnsemblePlusMode;
  setMode: (mode: EnsemblePlusMode) => void;
};

export const useEnsemblePlusState = ({ mode, setMode }: Props) => {
  const { set: setRateParam } = useNumericParam("rate");
  const { set: setRate2Param } = useNumericParam("rate_2");
  const {
    value: vibratoRate,
    set: setVibratoRateParam,
    grab: grabVibratoRate,
    release: releaseVibratoRate,
    info: vibratoRateInfo,
  } = useNumericParam("rate_3");
  const { set: setDepthParam } = useNumericParam("depth");
  const {
    value: ensDepth,
    set: setEnsDepthParam,
    grab: grabEnsDepth,
    release: releaseEnsDepth,
    info: ensDepthInfo,
  } = useNumericParam("ens_depth");
  const { set: setRoutingParam } = useEnumParam("routing");

  const setVoiceMode = useCallback(
    (voiceMode: EnsemblePlusVoiceMode) => {
      if (voiceMode === mode.voiceMode) {
        return;
      }

      const preset = ENSEMBLE_PLUS_PRESETS[voiceMode];
      setMode({ ...mode, voiceMode });
      setRoutingParam(preset.routing);
      setRateParam(preset.rate);
      setRate2Param(preset.rate_2);
      setDepthParam(preset.depth);
      setEnsDepthParam(
        voiceMode === "humanVoice" ? mode.lastEnsDepth : preset.ens_depth,
      );
      if (voiceMode === "humanVoice") {
        setVibratoRateParam(mode.lastRate);
      }
    },
    [
      mode,
      setDepthParam,
      setEnsDepthParam,
      setMode,
      setRate2Param,
      setRateParam,
      setRoutingParam,
      setVibratoRateParam,
    ],
  );

  const setVibratoRate = useCallback(
    (value: number) => {
      if (mode.voiceMode !== "humanVoice") {
        return;
      }
      setMode({ ...mode, lastRate: value });
      setVibratoRateParam(value);
    },
    [mode, setMode, setVibratoRateParam],
  );

  const setHumanVoiceDepth = useCallback(
    (value: number) => {
      if (mode.voiceMode !== "humanVoice") {
        return;
      }
      setMode({ ...mode, lastEnsDepth: value });
      setEnsDepthParam(value);
    },
    [mode, setEnsDepthParam, setMode],
  );

  return {
    voiceMode: mode.voiceMode,
    controlsActive: mode.voiceMode === "humanVoice",
    setVoiceMode,
    vibratoRate: {
      value: mode.voiceMode === "humanVoice" ? vibratoRate : mode.lastRate,
      set: setVibratoRate,
      grab: grabVibratoRate,
      release: releaseVibratoRate,
      info: vibratoRateInfo,
    },
    humanVoiceDepth: {
      value: mode.voiceMode === "humanVoice" ? ensDepth : mode.lastEnsDepth,
      set: setHumanVoiceDepth,
      grab: grabEnsDepth,
      release: releaseEnsDepth,
      info: ensDepthInfo,
    },
  };
};
