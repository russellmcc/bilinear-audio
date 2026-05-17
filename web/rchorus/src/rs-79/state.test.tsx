import { describe, expect, test } from "bun:test";
import { act, renderHook } from "@testing-library/react";
import { useEnumParam, useNumericParam } from "@conformal/plugin";
import { RootProviders } from "../Root";
import type { Mode } from "../mode";
import { defaultRs79Mode, useMode, useNextMode } from "../mode";
import { RS_79_PRESETS } from "./constants";
import { useRs79State } from "./state";

const useRs79Harness = () => {
  const mode = useMode();
  const rate = useNumericParam("rate");
  const rate2 = useNumericParam("rate_2");
  const rate3 = useNumericParam("rate_3");
  const rate4 = useNumericParam("rate_4");
  const depth = useNumericParam("depth");
  const ens2Depth = useNumericParam("ens_2_depth");
  const routing = useEnumParam("routing");

  const rs79 = useRs79State({
    mode: mode.rs79Mode,
    setMode: mode.setRs79Mode,
  });
  return { mode, rate, rate2, rate3, rate4, depth, ens2Depth, routing, rs79 };
};

const useRs79CycleHarness = () => {
  const mode = useMode();
  const nextMode = useNextMode();
  const rate = useNumericParam("rate");
  const rate2 = useNumericParam("rate_2");
  const rate3 = useNumericParam("rate_3");
  const rate4 = useNumericParam("rate_4");
  const depth = useNumericParam("depth");
  const ens2Depth = useNumericParam("ens_2_depth");
  const routing = useEnumParam("routing");

  return {
    mode,
    nextMode,
    rate,
    rate2,
    rate3,
    rate4,
    depth,
    ens2Depth,
    routing,
  };
};

const getRs79Mode = (mode: Mode | undefined) => {
  if (mode?.id !== "rs-79") {
    throw new Error("Expected RS-79 mode");
  }
  return mode;
};

describe("useRs79State", () => {
  test("ensemble modes update params and ui state", () => {
    const { result } = renderHook(useRs79Harness, {
      wrapper: RootProviders,
    });

    act(() => {
      result.current.mode.setMode({ id: "rs-79", ...defaultRs79Mode });
    });
    act(() => {
      result.current.rs79.setEnsembleMode("II");
    });

    const mode = getRs79Mode(result.current.mode.mode);
    const preset = RS_79_PRESETS.II;
    expect(mode.ensembleMode).toBe("II");
    expect(result.current.routing.value).toBe(preset.routing);
    expect(result.current.rate.value).toBe(preset.rate);
    expect(result.current.rate2.value).toBe(preset.rate_2);
    expect(result.current.rate3.value).toBe(preset.rate_3);
    expect(result.current.rate4.value).toBe(preset.rate_4);
    expect(result.current.depth.value).toBe(preset.depth);
    expect(result.current.ens2Depth.value).toBe(preset.ens_2_depth);
  });

  test("mode carousel enters RS-79 on ensemble I", () => {
    const { result } = renderHook(useRs79CycleHarness, {
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

    const mode = getRs79Mode(result.current.mode.mode);
    const preset = RS_79_PRESETS.I;
    expect(mode.ensembleMode).toBe("I");
    expect(result.current.routing.value).toBe(preset.routing);
    expect(result.current.rate.value).toBe(preset.rate);
    expect(result.current.rate2.value).toBe(preset.rate_2);
    expect(result.current.rate3.value).toBe(preset.rate_3);
    expect(result.current.rate4.value).toBe(preset.rate_4);
    expect(result.current.depth.value).toBe(preset.depth);
    expect(result.current.ens2Depth.value).toBe(preset.ens_2_depth);
  });
});
