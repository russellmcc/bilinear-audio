import type { Preset } from "../preset";
import type { Ju60ButtonMode } from "./state";

const JU_60_DEFAULT_BUTTON_MODE: Ju60ButtonMode = "I";
const JU_60_DELAY_MODULATION_SPAN_MS = 5.35 - 1.66;
const depthFromMilliseconds = (milliseconds: number) =>
  Math.min((milliseconds / JU_60_DELAY_MODULATION_SPAN_MS) * 100, 100);

export const JU_60_PRESETS = {
  I: {
    routing: "Synth",
    depth: depthFromMilliseconds(4),
    delay_scale: 1,
    rate: 0.5,
  },
  II: {
    routing: "Synth",
    depth: depthFromMilliseconds(4),
    delay_scale: 1,
    rate: 0.8,
  },
  III: {
    routing: "Pedal",
    depth: depthFromMilliseconds(0.5),
    delay_scale: 1,
    rate: 9,
  },
} satisfies Record<
  Ju60ButtonMode,
  Pick<Preset, "routing" | "depth" | "delay_scale" | "rate">
>;

export const preset: Preset = {
  ...JU_60_PRESETS[JU_60_DEFAULT_BUTTON_MODE],
  mix: 100,
  highpass_cutoff: "Low",
  dry_highpass_cutoff: "Low",
};

export default preset;
