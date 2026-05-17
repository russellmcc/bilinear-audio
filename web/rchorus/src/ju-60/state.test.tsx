import { describe, expect, test } from "bun:test";
import { act, renderHook } from "@testing-library/react";
import { useEnumParam, useNumericParam } from "@conformal/plugin";
import { RootProviders } from "../Root";
import type { Mode } from "../mode";
import { defaultJu60Mode, useMode, useNextMode } from "../mode";
import { JU_60_PRESETS } from "./constants";
import { useJu60State } from "./state";

const useJu60Harness = () => {
  const mode = useMode();
  const rate = useNumericParam("rate");
  const depth = useNumericParam("depth");
  const routing = useEnumParam("routing");

  const ju60 = useJu60State({
    mode: mode.ju60Mode,
    setMode: mode.setJu60Mode,
  });
  return { mode, rate, depth, routing, ju60 };
};

const useJu60CycleHarness = () => {
  const mode = useMode();
  const nextMode = useNextMode();
  const rate = useNumericParam("rate");
  const depth = useNumericParam("depth");
  const routing = useEnumParam("routing");

  return { mode, nextMode, rate, depth, routing };
};

const getJu60Mode = (mode: Mode | undefined) => {
  if (mode?.id !== "ju-60") {
    throw new Error("Expected JU-60 mode");
  }
  return mode;
};

describe("useJu60State", () => {
  test("button modes update params and ui state", () => {
    const { result } = renderHook(useJu60Harness, {
      wrapper: RootProviders,
    });

    act(() => {
      result.current.mode.setMode({ id: "ju-60", ...defaultJu60Mode });
    });
    act(() => {
      result.current.ju60.setButtonMode("III");
    });

    const mode = getJu60Mode(result.current.mode.mode);
    const preset = JU_60_PRESETS.III;
    expect(mode.buttonMode).toBe("III");
    expect(result.current.routing.value).toBe(preset.routing);
    expect(result.current.depth.value).toBe(preset.depth);
    expect(result.current.rate.value).toBe(preset.rate);
  });

  test("mode carousel enters JU-60 on button I", () => {
    const { result } = renderHook(useJu60CycleHarness, {
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

    const mode = getJu60Mode(result.current.mode.mode);
    const preset = JU_60_PRESETS.I;
    expect(mode.buttonMode).toBe("I");
    expect(result.current.routing.value).toBe(preset.routing);
    expect(result.current.depth.value).toBe(preset.depth);
    expect(result.current.rate.value).toBe(preset.rate);
  });
});
