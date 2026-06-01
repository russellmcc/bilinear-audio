import { describe, expect, test } from "bun:test";
import { act, renderHook } from "@testing-library/react";
import { useEnumParam, useNumericParam } from "@conformal/plugin";
import { RootProviders } from "../Root";
import type { Mode } from "../mode";
import { defaultEnsemblePlusMode, useMode, useNextMode } from "../mode";
import {
  ENSEMBLE_PLUS_PRESETS,
  HUMAN_VOICE_CHORUS_RATE,
  HUMAN_VOICE_CHORUS_RATE_2,
} from "./preset";
import { useEnsemblePlusState } from "./state";

const useEnsemblePlusHarness = () => {
  const mode = useMode();
  const rate = useNumericParam("rate");
  const rate2 = useNumericParam("rate_2");
  const rate3 = useNumericParam("rate_3");
  const depth = useNumericParam("depth");
  const ensDepth = useNumericParam("ens_depth");
  const routing = useEnumParam("routing");

  const ensemblePlus = useEnsemblePlusState({
    mode: mode.ensemblePlusMode,
    setMode: mode.setEnsemblePlusMode,
  });
  return { mode, rate, rate2, rate3, depth, ensDepth, routing, ensemblePlus };
};

const useEnsemblePlusCycleHarness = () => {
  const mode = useMode();
  const nextMode = useNextMode();
  const rate = useNumericParam("rate");
  const rate2 = useNumericParam("rate_2");
  const rate3 = useNumericParam("rate_3");
  const depth = useNumericParam("depth");
  const ensDepth = useNumericParam("ens_depth");
  const routing = useEnumParam("routing");

  return {
    mode,
    nextMode,
    rate,
    rate2,
    rate3,
    depth,
    ensDepth,
    routing,
  };
};

const getEnsemblePlusMode = (mode: Mode | undefined) => {
  if (mode?.id !== "ensemble-plus") {
    throw new Error("Expected Ensemble Plus mode");
  }
  return mode;
};

describe("useEnsemblePlusState", () => {
  test("human voice controls update vibrato params and remembered values", () => {
    const { result } = renderHook(useEnsemblePlusHarness, {
      wrapper: RootProviders,
    });

    act(() => {
      result.current.mode.setMode({
        id: "ensemble-plus",
        ...defaultEnsemblePlusMode,
      });
    });
    act(() => {
      result.current.ensemblePlus.vibratoRate.set(4.2);
    });
    act(() => {
      result.current.ensemblePlus.humanVoiceDepth.set(75);
    });

    const mode = getEnsemblePlusMode(result.current.mode.mode);
    expect(result.current.rate3.value).toBe(4.2);
    expect(result.current.ensDepth.value).toBe(75);
    expect(mode.lastRate).toBe(4.2);
    expect(mode.lastEnsDepth).toBe(75);
  });

  test("strings mode fixes dsp params and restores human voice values", () => {
    const { result } = renderHook(useEnsemblePlusHarness, {
      wrapper: RootProviders,
    });

    act(() => {
      result.current.mode.setMode({
        id: "ensemble-plus",
        ...defaultEnsemblePlusMode,
        lastRate: 4.2,
        lastEnsDepth: 75,
      });
      result.current.rate3.set(4.2);
      result.current.ensDepth.set(75);
    });
    act(() => {
      result.current.ensemblePlus.setVoiceMode("strings");
    });

    const stringsMode = getEnsemblePlusMode(result.current.mode.mode);
    const stringsPreset = ENSEMBLE_PLUS_PRESETS.strings;
    expect(stringsMode.voiceMode).toBe("strings");
    expect(stringsMode.lastRate).toBe(4.2);
    expect(stringsMode.lastEnsDepth).toBe(75);
    expect(result.current.routing.value).toBe(stringsPreset.routing);
    expect(result.current.rate.value).toBe(stringsPreset.rate);
    expect(result.current.rate2.value).toBe(stringsPreset.rate_2);
    expect(result.current.depth.value).toBe(stringsPreset.depth);
    expect(result.current.ensDepth.value).toBe(stringsPreset.ens_depth);

    act(() => {
      result.current.ensemblePlus.vibratoRate.set(6);
      result.current.ensemblePlus.humanVoiceDepth.set(20);
    });

    expect(result.current.rate3.value).toBe(4.2);
    expect(result.current.ensDepth.value).toBe(stringsPreset.ens_depth);

    act(() => {
      result.current.ensemblePlus.setVoiceMode("humanVoice");
    });

    const humanVoiceMode = getEnsemblePlusMode(result.current.mode.mode);
    expect(humanVoiceMode.voiceMode).toBe("humanVoice");
    expect(result.current.routing.value).toBe(
      ENSEMBLE_PLUS_PRESETS.humanVoice.routing,
    );
    expect(result.current.rate.value).toBe(HUMAN_VOICE_CHORUS_RATE);
    expect(result.current.rate2.value).toBe(HUMAN_VOICE_CHORUS_RATE_2);
    expect(result.current.depth.value).toBe(
      ENSEMBLE_PLUS_PRESETS.humanVoice.depth,
    );
    expect(result.current.rate3.value).toBe(4.2);
    expect(result.current.ensDepth.value).toBe(75);
  });

  test("mode carousel enters Ensemble Plus in human voice mode", () => {
    const { result } = renderHook(useEnsemblePlusCycleHarness, {
      wrapper: RootProviders,
    });

    act(() => {
      result.current.nextMode();
    });
    act(() => {
      result.current.nextMode();
    });
    act(() => {
      result.current.nextMode();
    });
    act(() => {
      result.current.nextMode();
    });
    act(() => {
      result.current.nextMode();
    });
    act(() => {
      result.current.nextMode();
    });
    act(() => {
      result.current.nextMode();
    });
    act(() => {
      result.current.nextMode();
    });

    const mode = getEnsemblePlusMode(result.current.mode.mode);
    const preset = ENSEMBLE_PLUS_PRESETS.humanVoice;
    expect(mode.voiceMode).toBe("humanVoice");
    expect(result.current.routing.value).toBe(preset.routing);
    expect(result.current.rate.value).toBe(preset.rate);
    expect(result.current.rate2.value).toBe(preset.rate_2);
    expect(result.current.rate3.value).toBe(preset.rate_3);
    expect(result.current.depth.value).toBe(preset.depth);
    expect(result.current.ensDepth.value).toBe(preset.ens_depth);
  });
});
