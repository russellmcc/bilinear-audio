// Simple hook for smoothing a value over time

import { useEffect, useRef } from "react";
import useAnimation from "./useAnimation";

const DEFAULT_TIME = 0.5;

export type Options = {
  /**
   * Coefficient that controls the speed of the smoothing. Default is 10.
   * The exact dynamics are dependent on the frame rate of the display,
   */
  time?: number;
};

export const useSmoothedValue = (value: number, options: Options = {}) => {
  const { time = DEFAULT_TIME } = options;
  const rate = -Math.log(0.01) / time;
  const rateRef = useRef(rate);
  useEffect(() => {
    rateRef.current = rate;
  }, [rate]);
  const smoothedValue = useAnimation(
    {
      initialState: () => value,
      update: (elapsed, prev, value) => {
        const next = elapsed
          ? prev + (value - prev) * Math.min(1.0, rateRef.current * elapsed)
          : prev;
        if (Math.abs(next - value) < 0.01) {
          return value;
        }
        return next;
      },
      shouldAnimate: (state: number, value: number) => state !== value,
    },
    value,
  );
  return smoothedValue;
};
