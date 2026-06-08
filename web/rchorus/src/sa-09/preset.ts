import type { Preset } from "../preset";

export const SA_09_VIBRATO_RATE_RANGE = [4, 8] as const;
export const SA_09_VIBRATO_RATE =
  (SA_09_VIBRATO_RATE_RANGE[0] + SA_09_VIBRATO_RATE_RANGE[1]) / 2;
export const SA_09_VIBRATO_DEPTH = 20;
export const SA_09_CHORUS_RATE = 0.23;
export const SA_09_CHORUS_DEPTH = 80;

export const preset: Preset = {
  rate: SA_09_CHORUS_RATE,
  depth: SA_09_CHORUS_DEPTH,
  delay_scale: 1,
  mix: 100,
  highpass_cutoff: "Low",
  dry_highpass_cutoff: "Low",
  routing: "Pedal",
};

export default preset;
