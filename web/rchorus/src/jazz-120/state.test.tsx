import { describe, expect, test } from "bun:test";
import { act, renderHook } from "@testing-library/react";
import { useNumericParam } from "@conformal/plugin";
import { RootProviders } from "../Root";
import { Mode, defaultJazz120Mode, useMode } from "../mode";
import { JAZZ_CHORUS_DEPTH, JAZZ_CHORUS_RATE } from "./constants";
import { useJazzChorusState } from "./state";

const useJazzHarness = () => {
  const mode = useMode();
  const rate = useNumericParam("rate");
  const depth = useNumericParam("depth");

  const jazz = useJazzChorusState({
    mode: mode.jazz120Mode,
    setMode: mode.setJazz120Mode,
  });
  return { mode, rate, depth, jazz };
};

const getJazzMode = (mode: Mode | undefined) => {
  if (mode?.id !== "jazz-120") {
    throw new Error("Expected Jazz 120 mode");
  }
  return mode;
};

describe("useJazzChorusState", () => {
  test("vibrato controls update params and remembered values", () => {
    const { result } = renderHook(useJazzHarness, {
      wrapper: RootProviders,
    });

    act(() => {
      result.current.mode.setMode({ id: "jazz-120", ...defaultJazz120Mode });
    });
    act(() => {
      result.current.jazz.rate.set(2.5);
    });
    act(() => {
      result.current.jazz.depth.set(40);
    });

    const mode = getJazzMode(result.current.mode.mode);
    expect(result.current.rate.value).toBe(2.5);
    expect(result.current.depth.value).toBe(40);
    expect(mode.lastRate).toBe(2.5);
    expect(mode.lastDepth).toBe(40);
  });

  test("chorus mode fixes dsp params and restores vibrato values", () => {
    const { result } = renderHook(useJazzHarness, {
      wrapper: RootProviders,
    });

    act(() => {
      result.current.mode.setMode({
        id: "jazz-120",
        ...defaultJazz120Mode,
        lastRate: 2.5,
        lastDepth: 40,
      });
      result.current.rate.set(2.5);
      result.current.depth.set(40);
    });
    act(() => {
      result.current.jazz.setChorusMode("chorus");
    });

    const chorusMode = getJazzMode(result.current.mode.mode);
    expect(chorusMode.chorusMode).toBe("chorus");
    expect(chorusMode.lastRate).toBe(2.5);
    expect(chorusMode.lastDepth).toBe(40);
    expect(result.current.rate.value).toBe(JAZZ_CHORUS_RATE);
    expect(result.current.depth.value).toBe(JAZZ_CHORUS_DEPTH);

    act(() => {
      result.current.jazz.setChorusMode("vibrato");
    });

    const vibratoMode = getJazzMode(result.current.mode.mode);
    expect(vibratoMode.chorusMode).toBe("vibrato");
    expect(result.current.rate.value).toBe(2.5);
    expect(result.current.depth.value).toBe(40);
  });
});
