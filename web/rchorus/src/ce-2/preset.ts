import { Preset } from "../preset";

export const preset: Preset = {
  rate: 1.9,
  depth: (3.3 / (5.35 - 1.66)) * 50,
  mix: 100,
  highpass_cutoff: "Low",
  routing: "Pedal",
};

export default preset;
