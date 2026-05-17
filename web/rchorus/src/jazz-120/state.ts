import { useNumericParam } from "@conformal/plugin";
import { useCallback } from "react";
import type { Jazz120Mode } from "../mode";
import { JAZZ_CHORUS_DEPTH, JAZZ_CHORUS_RATE } from "./constants";

export type JazzChorusMode = Jazz120Mode["chorusMode"];

export type Props = {
  mode: Jazz120Mode;
  setMode: (mode: Jazz120Mode) => void;
};

export const useJazzChorusState = ({ mode, setMode }: Props) => {
  const {
    value: rate,
    set: setRateParam,
    grab: grabRate,
    release: releaseRate,
    info: rateInfo,
  } = useNumericParam("rate");
  const {
    value: depth,
    set: setDepthParam,
    grab: grabDepth,
    release: releaseDepth,
    info: depthInfo,
  } = useNumericParam("depth");

  const setChorusMode = useCallback(
    (chorusMode: JazzChorusMode) => {
      if (chorusMode === mode.chorusMode) {
        return;
      }
      setMode({ ...mode, chorusMode });
      if (chorusMode === "chorus") {
        setRateParam(JAZZ_CHORUS_RATE);
        setDepthParam(JAZZ_CHORUS_DEPTH);
      } else {
        setRateParam(mode.lastRate);
        setDepthParam(mode.lastDepth);
      }
    },
    [mode, setDepthParam, setMode, setRateParam],
  );

  const setRate = useCallback(
    (value: number) => {
      if (mode.chorusMode !== "vibrato") {
        return;
      }
      setMode({ ...mode, lastRate: value });
      setRateParam(value);
    },
    [mode, setMode, setRateParam],
  );

  const setDepth = useCallback(
    (value: number) => {
      if (mode.chorusMode !== "vibrato") {
        return;
      }
      setMode({ ...mode, lastDepth: value });
      setDepthParam(value);
    },
    [mode, setDepthParam, setMode],
  );

  return {
    chorusMode: mode.chorusMode,
    controlsActive: mode.chorusMode === "vibrato",
    setChorusMode,
    rate: {
      value: mode.chorusMode === "vibrato" ? rate : mode.lastRate,
      set: setRate,
      grab: grabRate,
      release: releaseRate,
      info: rateInfo,
    },
    depth: {
      value: mode.chorusMode === "vibrato" ? depth : mode.lastDepth,
      set: setDepth,
      grab: grabDepth,
      release: releaseDepth,
      info: depthInfo,
    },
  };
};
