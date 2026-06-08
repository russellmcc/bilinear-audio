import type { Preset } from "../preset";
import type { EnsemblePlusVoiceMode } from "./state";

const ENSEMBLE_PLUS_DEFAULT_MODE: EnsemblePlusVoiceMode = "humanVoice";
const ENSEMBLE_PLUS_LOCKED_DEPTH = 80;
export const HUMAN_VOICE_DEFAULT_RATE = 3.85;
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

export const preset: Preset = {
  ...ENSEMBLE_PLUS_PRESETS[ENSEMBLE_PLUS_DEFAULT_MODE],
  mix: 100,
  highpass_cutoff: "Low",
  dry_highpass_cutoff: "Low",
};

export default preset;
