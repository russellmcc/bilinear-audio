import { Info } from "@conformal/plugin";

export const typedInfos = {
  bypass: {
    title: "Bypass",
    type_specific: {
      t: "switch" as const,
      default: false,
    } as const,
  } as const,
  rate: {
    title: "Rate",
    type_specific: {
      t: "numeric" as const,
      default: 0.35,
      valid_range: [0.08, 10.1] as [number, number],
      units: "hz",
    } as const,
  } as const,
  depth: {
    title: "Depth",
    type_specific: {
      t: "numeric" as const,
      default: 100,
      valid_range: [0, 100] as [number, number],
      units: "%",
    } as const,
  } as const,
  mix: {
    title: "Mix",
    type_specific: {
      t: "numeric" as const,
      default: 100,
      valid_range: [0, 100] as [number, number],
      units: "%",
    } as const,
  } as const,
  highpass_cutoff: {
    title: "Highpass Cutoff",
    type_specific: {
      t: "enum" as const,
      default: "Low",
      values: ["Low", "High"],
    } as const,
  } as const,
  routing: {
    title: "Routing",
    type_specific: {
      t: "enum" as const,
      default: "Synth",
      values: ["Synth", "Dimension"],
    } as const,
  } as const,
} as const;

const infos = new Map<string, Info>(
  Object.entries(typedInfos as unknown as Record<string, Info>),
);

export default infos;
