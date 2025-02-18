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

export { exponentialScale } from "./scale";
export type { Scale } from "./scale";
