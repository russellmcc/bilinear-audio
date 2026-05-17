import type { Preset } from "../preset";

export const JU_60_BUTTON_MODES = ["I", "II", "III"] as const;
export type Ju60ButtonMode = (typeof JU_60_BUTTON_MODES)[number];

export const JU_60_DEFAULT_BUTTON_MODE: Ju60ButtonMode = "I";
export const JU_60_ACCENT_COLOR = "#669bbc";
export const JU_60_BACKGROUND =
  "linear-gradient(-45.38510521241395deg, #312f2f 7.0881%, #3c3b3b 90.779%)";

const JU_60_DELAY_MODULATION_SPAN_MS = 5.35 - 1.66;
const depthFromMilliseconds = (milliseconds: number) =>
  Math.min((milliseconds / JU_60_DELAY_MODULATION_SPAN_MS) * 100, 100);

export const JU_60_PRESETS = {
  I: {
    routing: "Synth",
    depth: depthFromMilliseconds(4),
    rate: 0.5,
  },
  II: {
    routing: "Synth",
    depth: depthFromMilliseconds(4),
    rate: 0.8,
  },
  III: {
    routing: "Pedal",
    depth: depthFromMilliseconds(0.5),
    rate: 9,
  },
} satisfies Record<Ju60ButtonMode, Pick<Preset, "routing" | "depth" | "rate">>;

export const JU_60_DEFAULT_PRESET: Preset = {
  ...JU_60_PRESETS[JU_60_DEFAULT_BUTTON_MODE],
  mix: 100,
  highpass_cutoff: "Low",
};
