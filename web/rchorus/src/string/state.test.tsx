import { describe, expect, test } from "bun:test";
import { act, renderHook } from "@testing-library/react";
import { useEnumParam, useNumericParam } from "@conformal/plugin";
import { RootProviders } from "../Root";
import type { Mode } from "../mode";
import { defaultStringMode, useMode, useNextMode } from "../mode";
import { STRING_PRESETS } from "./preset";
import { useStringState } from "./state";

const useStringHarness = () => {
  const mode = useMode();
  const rate = useNumericParam("rate");
  const rate2 = useNumericParam("rate_2");
  const depth = useNumericParam("depth");
  const ensDepth = useNumericParam("ens_depth");
  const routing = useEnumParam("routing");

  const string = useStringState({
    mode: mode.stringMode,
    setMode: mode.setStringMode,
  });
  return { mode, rate, rate2, depth, ensDepth, routing, string };
};

const useStringCycleHarness = () => {
  const mode = useMode();
  const nextMode = useNextMode();
  const rate = useNumericParam("rate");
  const rate2 = useNumericParam("rate_2");
  const depth = useNumericParam("depth");
  const ensDepth = useNumericParam("ens_depth");
  const routing = useEnumParam("routing");

  return {
    mode,
    nextMode,
    rate,
    rate2,
    depth,
    ensDepth,
    routing,
  };
};

const getStringMode = (mode: Mode | undefined) => {
  if (mode?.id !== "string") {
    throw new Error("Expected string mode");
  }
  return mode;
};

describe("useStringState", () => {
  test("ensemble modes update params and ui state", () => {
    const { result } = renderHook(useStringHarness, {
      wrapper: RootProviders,
    });

    act(() => {
      result.current.mode.setMode({ id: "string", ...defaultStringMode });
    });
    act(() => {
      result.current.string.setEnsembleMode("I");
    });

    const mode = getStringMode(result.current.mode.mode);
    const preset = STRING_PRESETS.I;
    expect(mode.ensembleMode).toBe("I");
    expect(result.current.routing.value).toBe(preset.routing);
    expect(result.current.rate.value).toBe(preset.rate);
    expect(result.current.rate2.value).toBe(preset.rate_2);
    expect(result.current.depth.value).toBe(preset.depth);
    expect(result.current.ensDepth.value).toBe(preset.ens_depth);
  });

  test("mode carousel enters string on ensemble II", () => {
    const { result } = renderHook(useStringCycleHarness, {
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

    const mode = getStringMode(result.current.mode.mode);
    const preset = STRING_PRESETS.II;
    expect(mode.ensembleMode).toBe("II");
    expect(result.current.routing.value).toBe(preset.routing);
    expect(result.current.rate.value).toBe(preset.rate);
    expect(result.current.rate2.value).toBe(preset.rate_2);
    expect(result.current.depth.value).toBe(preset.depth);
    expect(result.current.ensDepth.value).toBe(preset.ens_depth);
  });
});
