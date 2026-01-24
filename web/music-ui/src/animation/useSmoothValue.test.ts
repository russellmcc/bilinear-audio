import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { useSmoothedValue } from "./useSmoothedValue";

describe("useSmoothedValue", () => {
  test("should return the initial value", () => {
    const { result } = renderHook(() => useSmoothedValue(10));
    expect(result.current).toBe(10);
  });
  describe("with fake requestAnimationFrame", () => {
    let originalRequestAnimationFrame: typeof global.requestAnimationFrame;
    beforeEach(() => {
      originalRequestAnimationFrame = global.requestAnimationFrame;
      let lastTime = 0;
      global.requestAnimationFrame = (cb) =>
        setTimeout(() => {
          cb((lastTime += 16));
        }, 0) as unknown as number;
    });
    afterEach(() => {
      global.requestAnimationFrame = originalRequestAnimationFrame;
    });
    test("should not immediately animate to the new value", async () => {
      // Mock out requestAnimationFrame
      const { result, rerender } = renderHook((v) => useSmoothedValue(v), {
        initialProps: 10,
      });
      expect(result.current).toBe(10);
      act(() => {
        rerender(0);
      });
      await waitFor(() => {
        expect(result.current).toBe(0);
      });
    });
    test("faster times should finish faster", async () => {
      // Mock out requestAnimationFrame
      const { result: resultA, rerender: rerenderA } = renderHook(
        (v) => useSmoothedValue(v, { time: 0.1 }),
        {
          initialProps: 10,
        },
      );
      expect(resultA.current).toBe(10);
      const { result: resultB, rerender: rerenderB } = renderHook(
        (v) => useSmoothedValue(v, { time: 2.0 }),
        {
          initialProps: 10,
        },
      );
      expect(resultB.current).toBe(10);
      act(() => {
        rerenderA(0);
        rerenderB(0);
      });
      await waitFor(() => {
        expect(resultA.current).toBe(0);
      });
      expect(resultB.current).toBeGreaterThan(0);
    });
  });
});
