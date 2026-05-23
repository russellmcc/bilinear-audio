import type { Preset } from "../preset";

export const STRING_ENSEMBLE_MODES = ["I", "II"] as const;
export type StringEnsembleMode = (typeof STRING_ENSEMBLE_MODES)[number];

export const STRING_DEFAULT_ENSEMBLE_MODE: StringEnsembleMode = "II";
export const STRING_ACCENT_COLOR = "#70a5df";
export const STRING_BACKGROUND = "#100007";
export const STRING_PANEL_BACKGROUND = "#456990";

const STRING_FIXED_PARAMS = {
  routing: "String",
  rate: 0.66,
  rate_2: 6.25,
  depth: 80,
} satisfies Pick<Preset, "routing" | "rate" | "rate_2" | "depth">;

export const STRING_PRESETS = {
  I: {
    ...STRING_FIXED_PARAMS,
    ens_depth: 30,
  },
  II: {
    ...STRING_FIXED_PARAMS,
    ens_depth: 0,
  },
} satisfies Record<
  StringEnsembleMode,
  Pick<Preset, "routing" | "rate" | "rate_2" | "depth" | "ens_depth">
>;

export const STRING_DEFAULT_PRESET: Preset = {
  ...STRING_PRESETS[STRING_DEFAULT_ENSEMBLE_MODE],
  mix: 100,
  highpass_cutoff: "Low",
};
