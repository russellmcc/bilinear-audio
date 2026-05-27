import type { Preset } from "../preset";

export const SVC_350_BACKGROUND =
  "linear-gradient(135deg, #230611 0%, #120309 100%)";
export const SVC_350_ACCENT_COLOR = "#f7717d";

export const SVC_350_PRESET: Preset = {
  routing: "Vocoder2",
  rate: 0.22,
  rate_2: 5,
  depth: 80,
  ens_depth: 20,
  mix: 100,
  highpass_cutoff: "Low",
};
