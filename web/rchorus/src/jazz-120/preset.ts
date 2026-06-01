import { Preset } from "../preset";

const JAZZ_DELAY_MODULATION_SPAN_MS = 5.35 - 1.66;
export const JAZZ_RATE_RANGE = [3, 10] as const;
export const JAZZ_DEPTH_RANGE = [
  (1 / JAZZ_DELAY_MODULATION_SPAN_MS) * 100,
  100,
] as const;
export const JAZZ_VIBRATO_RATE =
  (JAZZ_RATE_RANGE[0] + JAZZ_RATE_RANGE[1]) / 2;
export const JAZZ_VIBRATO_DEPTH =
  (JAZZ_DEPTH_RANGE[0] + JAZZ_DEPTH_RANGE[1]) / 2;

export const preset: Preset = {
  rate: JAZZ_VIBRATO_RATE,
  depth: JAZZ_VIBRATO_DEPTH,
  delay_scale: 1,
  mix: 100,
  highpass_cutoff: "Low",
  routing: "Jazz",
};

export default preset;
