import type { Preset } from "../preset";
import type { StringEnsembleMode } from "./state";

const STRING_DEFAULT_ENSEMBLE_MODE: StringEnsembleMode = "II";

const STRING_FIXED_PARAMS = {
  routing: "String",
  rate: 0.66,
  rate_2: 6.25,
  depth: 80,
  delay_scale: 1,
} satisfies Pick<
  Preset,
  "routing" | "rate" | "rate_2" | "depth" | "delay_scale"
>;

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
  Pick<
    Preset,
    "routing" | "rate" | "rate_2" | "depth" | "delay_scale" | "ens_depth"
  >
>;

export const preset: Preset = {
  ...STRING_PRESETS[STRING_DEFAULT_ENSEMBLE_MODE],
  mix: 100,
  highpass_cutoff: "Low",
  dry_highpass_cutoff: "Low",
};

export default preset;
