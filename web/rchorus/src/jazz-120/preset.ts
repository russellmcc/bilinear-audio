import { Preset } from "../preset";
import { JAZZ_VIBRATO_DEPTH, JAZZ_VIBRATO_RATE } from "./constants";

export const preset: Preset = {
  rate: JAZZ_VIBRATO_RATE,
  depth: JAZZ_VIBRATO_DEPTH,
  delay_scale: 1,
  mix: 100,
  highpass_cutoff: "Low",
  routing: "Jazz",
};

export default preset;
