import type { Preset } from "../preset";

export const RS_79_ENSEMBLE_MODES = ["I", "II"] as const;
export type Rs79EnsembleMode = (typeof RS_79_ENSEMBLE_MODES)[number];

export const RS_79_DEFAULT_ENSEMBLE_MODE: Rs79EnsembleMode = "I";
export const RS_79_HIGHLIGHT_COLOR = "#fe5f55";
export const RS_79_SWITCH_COLOR = "#d6dbd2";
export const RS_79_BALL_HIGHLIGHT_COLOR = "#2e282a";
export const RS_79_BACKGROUND =
  "linear-gradient(-45deg, #2e282a 0%, #373032 100%)";

const RS_79_FIXED_PARAMS = {
  rate: 0.15,
  rate_2: 0.18,
  rate_3: 4.7,
  rate_4: 5.9,
  depth: 80,
  ens_2_depth: 20,
} satisfies Pick<
  Preset,
  "rate" | "rate_2" | "rate_3" | "rate_4" | "depth" | "ens_2_depth"
>;

export const RS_79_PRESETS = {
  I: {
    ...RS_79_FIXED_PARAMS,
    routing: "Ens 1",
  },
  II: {
    ...RS_79_FIXED_PARAMS,
    routing: "Ens 2",
  },
} satisfies Record<
  Rs79EnsembleMode,
  Pick<
    Preset,
    | "routing"
    | "rate"
    | "rate_2"
    | "rate_3"
    | "rate_4"
    | "depth"
    | "ens_2_depth"
  >
>;

export const RS_79_DEFAULT_PRESET: Preset = {
  ...RS_79_PRESETS[RS_79_DEFAULT_ENSEMBLE_MODE],
  mix: 100,
  highpass_cutoff: "Low",
};
