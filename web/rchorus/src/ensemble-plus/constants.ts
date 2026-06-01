import type { Preset } from "../preset";

export const ENSEMBLE_PLUS_BACKGROUND =
  "linear-gradient(-45.36034602069856deg, #422433 0%, #62374d 99.379%)";
export const ENSEMBLE_PLUS_ACCENT_COLOR = "#ff8811";
export const ENSEMBLE_PLUS_TEXT_COLOR = "#d6dbd2";

export const ENSEMBLE_PLUS_MODES = ["humanVoice", "strings"] as const;
export type EnsemblePlusVoiceMode = (typeof ENSEMBLE_PLUS_MODES)[number];

export const ENSEMBLE_PLUS_DEFAULT_MODE: EnsemblePlusVoiceMode = "humanVoice";
export const ENSEMBLE_PLUS_LOCKED_DEPTH = 80;
export const HUMAN_VOICE_RATE_RANGE = [0.7, 7] as const;
export const HUMAN_VOICE_ENS_DEPTH_RANGE = [0, 100] as const;
export const HUMAN_VOICE_DEFAULT_RATE =
  (HUMAN_VOICE_RATE_RANGE[0] + HUMAN_VOICE_RATE_RANGE[1]) / 2;
export const HUMAN_VOICE_DEFAULT_ENS_DEPTH = 50;
export const HUMAN_VOICE_CHORUS_RATE = 0.12;
export const HUMAN_VOICE_CHORUS_RATE_2 = 0.19;

export const ENSEMBLE_PLUS_PRESETS = {
  humanVoice: {
    routing: "Vocoder",
    rate: HUMAN_VOICE_CHORUS_RATE,
    rate_2: HUMAN_VOICE_CHORUS_RATE_2,
    rate_3: HUMAN_VOICE_DEFAULT_RATE,
    depth: ENSEMBLE_PLUS_LOCKED_DEPTH,
    delay_scale: 1,
    ens_depth: HUMAN_VOICE_DEFAULT_ENS_DEPTH,
  },
  strings: {
    routing: "Ens",
    rate: 0.12,
    rate_2: 0.19,
    depth: ENSEMBLE_PLUS_LOCKED_DEPTH,
    delay_scale: 1,
    ens_depth: 0,
  },
} satisfies Record<
  EnsemblePlusVoiceMode,
  Pick<
    Preset,
    "routing" | "rate" | "rate_2" | "depth" | "delay_scale" | "ens_depth"
  > & {
    rate_3?: Preset["rate_3"];
  }
>;

export const ENSEMBLE_PLUS_DEFAULT_PRESET: Preset = {
  ...ENSEMBLE_PLUS_PRESETS[ENSEMBLE_PLUS_DEFAULT_MODE],
  mix: 100,
  highpass_cutoff: "Low",
};
