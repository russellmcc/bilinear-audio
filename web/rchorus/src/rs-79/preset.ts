import type { Preset } from "../preset";
import type { Rs79EnsembleMode } from "./state";

const RS_79_DEFAULT_ENSEMBLE_MODE: Rs79EnsembleMode = "I";

const RS_79_FIXED_PARAMS = {
  rate: 0.15,
  rate_2: 0.18,
  rate_3: 4.7,
  rate_4: 5.9,
  depth: 80,
  delay_scale: 1,
} satisfies Pick<
  Preset,
  "rate" | "rate_2" | "rate_3" | "rate_4" | "depth" | "delay_scale"
>;

export const RS_79_PRESETS = {
  I: {
    ...RS_79_FIXED_PARAMS,
    ens_depth: 0,
    routing: "Ens",
  },
  II: {
    ...RS_79_FIXED_PARAMS,
    ens_depth: 20,
    routing: "Ens",
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
    | "delay_scale"
    | "ens_depth"
  >
>;

export const preset: Preset = {
  ...RS_79_PRESETS[RS_79_DEFAULT_ENSEMBLE_MODE],
  mix: 100,
  highpass_cutoff: "Low",
};

export default preset;
