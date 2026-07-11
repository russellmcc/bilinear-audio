import { useEnumParam, useNumericParam } from "@conformal/plugin";
import { useCallback } from "react";
import type { Sa09Mode } from "../mode";
import {
  SA_09_CHORUS_DEPTH,
  SA_09_CHORUS_RATE,
  SA_09_VIBRATO_DEPTH,
} from "./preset";

export const SA_09_CHORUS_MODES = ["vibrato", "chorus"] as const;
export type Sa09ChorusMode = (typeof SA_09_CHORUS_MODES)[number];

export const SA_09_ROUTING_MODES = ["I", "II"] as const;
export type Sa09RoutingMode = (typeof SA_09_ROUTING_MODES)[number];

export type Props = {
  mode: Sa09Mode;
  setMode: (mode: Sa09Mode) => void;
};

export const useSa09State = ({ mode, setMode }: Props) => {
  const {
    value: vibratoRate,
    set: setRateParam,
    grab: grabRate,
    release: releaseRate,
    info: rateInfo,
  } = useNumericParam("rate");
  const { set: setDepthParam } = useNumericParam("depth");
  const { set: setDryHighpassCutoffParam } = useEnumParam(
    "dry_highpass_cutoff",
  );
  const { set: setRoutingParam } = useEnumParam("routing");

  const setChorusMode = useCallback(
    (chorusMode: Sa09ChorusMode) => {
      if (chorusMode === mode.chorusMode) {
        return;
      }

      setMode({ ...mode, chorusMode });
      if (chorusMode === "chorus") {
        setRateParam(SA_09_CHORUS_RATE);
        setDepthParam(SA_09_CHORUS_DEPTH);
      } else {
        setRateParam(mode.lastRate);
        setDepthParam(SA_09_VIBRATO_DEPTH);
      }
    },
    [mode, setDepthParam, setMode, setRateParam],
  );

  const setRoutingMode = useCallback(
    (routingMode: Sa09RoutingMode) => {
      if (routingMode === mode.routingMode) {
        return;
      }

      setMode({ ...mode, routingMode });
      if (routingMode === "I") {
        setRoutingParam("Pedal");
        setDryHighpassCutoffParam("Low");
      } else {
        setRoutingParam("Synth");
        setDryHighpassCutoffParam("High");
      }
    },
    [mode, setDryHighpassCutoffParam, setMode, setRoutingParam],
  );

  const setVibratoRate = useCallback(
    (value: number) => {
      if (mode.chorusMode !== "vibrato") {
        return;
      }
      setMode({ ...mode, lastRate: value });
      setRateParam(value);
      setDepthParam(SA_09_VIBRATO_DEPTH);
    },
    [mode, setDepthParam, setMode, setRateParam],
  );

  return {
    chorusMode: mode.chorusMode,
    routingMode: mode.routingMode,
    controlsActive: mode.chorusMode === "vibrato",
    setChorusMode,
    setRoutingMode,
    vibratoRate: {
      value: mode.chorusMode === "vibrato" ? vibratoRate : mode.lastRate,
      set: setVibratoRate,
      grab: grabRate,
      release: releaseRate,
      info: rateInfo,
    },
  };
};
