export const indexOf = <T>(v: T, vs: T[]): number | undefined => {
  const ret = vs.indexOf(v);
  if (ret === -1) {
    return undefined;
  }
  return ret;
};

export const clamp = (v: number, min: number, max: number): number =>
  Math.min(max, Math.max(min, v));

export const lerp = (v: number, value_0: number, value_1: number): number =>
  value_0 + (value_1 - value_0) * v;

export const rescale = (
  v: number,
  from_low: number,
  from_high: number,
  to_low: number,
  to_high: number,
): number => lerp((v - from_low) / (from_high - from_low), to_low, to_high);

export { exponentialScale } from "./scale";
export type { Scale } from "./scale";
