import { describe, expect, test } from "bun:test";
import { act, renderHook } from "@testing-library/react";
import { useNumericParam } from "@conformal/plugin";
import { RootProviders } from "./Root.tsx";
import { useApplyPreset } from "./preset";
import superDimensionPreset from "./super-dimension/preset";

const useApplyPresetAndRate = () => {
  const apply = useApplyPreset();
  const { value: rate } = useNumericParam("rate");
  return { apply, rate };
};

describe("useApplyPreset", () => {
  test("applyPreset updates params under RootProviders", () => {
    const { result } = renderHook(useApplyPresetAndRate, {
      wrapper: RootProviders,
    });

    act(() => {
      result.current.apply(superDimensionPreset);
    });

    expect(result.current.rate).toBe(superDimensionPreset.rate);
  });
});
