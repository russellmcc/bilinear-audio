export const JAZZ_CHORUS_RATE = 1.5;
export const JAZZ_CHORUS_DEPTH = 100;
export const JAZZ_RATE_RANGE = [3, 10] as const;
const JAZZ_DELAY_MODULATION_SPAN_MS = 5.35 - 1.66;
export const JAZZ_DEPTH_RANGE = [
  (1 / JAZZ_DELAY_MODULATION_SPAN_MS) * 100,
  100,
] as const;
export const JAZZ_VIBRATO_RATE = (JAZZ_RATE_RANGE[0] + JAZZ_RATE_RANGE[1]) / 2;
export const JAZZ_VIBRATO_DEPTH =
  (JAZZ_DEPTH_RANGE[0] + JAZZ_DEPTH_RANGE[1]) / 2;
