import { Info } from "@conformal/plugin";

const percentage = (defaultValue = 100): Info["type_specific"] => ({
  t: "numeric",
  default: defaultValue,
  valid_range: [0, 100],
  units: "%",
});

const dcoShape: Info["type_specific"] = {
  t: "enum",
  default: "Saw",
  values: ["Saw", "Pulse", "PwmSaw", "CombSaw", "Noise"],
};

const dcoRange: Info["type_specific"] = {
  t: "enum",
  default: "8'",
  values: ["16'", "8'", "4'", "2'"],
};

const dcoTune: Info["type_specific"] = {
  t: "enum",
  default: "0",
  values: Array.from({ length: 25 }, (_, i) => String(i - 12)),
};

const envSource: Info["type_specific"] = {
  t: "enum",
  default: "Env1",
  values: [
    "Env1",
    "Env1-Inverse",
    "Env1-Dynamic",
    "Env2",
    "Env2-Inverse",
    "Env2-Dynamic",
    "Dynamic",
  ],
};

const envParams = (envName: string, prefix: string): Record<string, Info> =>
  Object.fromEntries(
    (
      [
        ...Array.from(
          { length: 3 },
          (_, i) =>
            [
              `${prefix}_t${i + 1}`,
              `${envName} T${i + 1}`,
              percentage(0),
            ] as const,
        ),
        ...Array.from(
          { length: 3 },
          (_, i) =>
            [
              `${prefix}_l${i + 1}`,
              `${envName} L${i + 1}`,
              percentage(100),
            ] as const,
        ),
        [`${prefix}_t4`, `${envName} T4`, percentage(0)],
        [`${prefix}_key_follow`, `${envName} Key Follow`, percentage(0)],
      ] as const
    ).map(([key, title, type_specific]): [string, Info] => [
      key,
      {
        title,
        type_specific,
      },
    ]),
  );

const infos = new Map<string, Info>(
  Object.entries({
    level: {
      title: "Gain",
      type_specific: percentage(100),
    },
    dco1_shape: {
      title: "DCO1 Shape",
      type_specific: dcoShape,
    },
    dco1_pwm_depth: {
      title: "DCO1 PWM Depth",
      type_specific: percentage(0),
    },
    dco1_pwm_rate: {
      title: "DCO1 PWM Rate",
      type_specific: percentage(0),
    },
    dco1_range: {
      title: "DCO1 Range",
      type_specific: dcoRange,
    },
    dco1_tune: {
      title: "DCO1 Tune",
      type_specific: dcoTune,
    },
    dco1_env: {
      title: "DCO1 Envelope",
      type_specific: percentage(0),
    },
    dco1_lfo: {
      title: "DCO1 LFO",
      type_specific: percentage(0),
    },
    dco2_shape: {
      title: "DCO2 Shape",
      type_specific: dcoShape,
    },
    dco2_pwm_depth: {
      title: "DCO2 PWM Depth",
      type_specific: percentage(0),
    },
    dco2_pwm_rate: {
      title: "DCO2 PWM Rate",
      type_specific: percentage(0),
    },
    dco2_range: {
      title: "DCO2 Range",
      type_specific: dcoRange,
    },
    dco2_tune: {
      title: "DCO2 Tune",
      type_specific: dcoTune,
    },
    dco2_fine_tune: {
      title: "DCO2 Fine Tune",
      type_specific: {
        t: "numeric",
        default: 0,
        valid_range: [-50, 50],
        units: "Cents",
      },
    },
    x_mod: {
      title: "DCO2 Cross Modulation",
      type_specific: {
        t: "enum",
        default: "Off",
        values: ["Off", "Ring", "Bit", "Sync", "Sync+Ring"],
      },
    },
    dco2_env: {
      title: "DCO2 Envelope",
      type_specific: percentage(0),
    },
    dco2_lfo: {
      title: "DCO2 LFO",
      type_specific: percentage(0),
    },
    dco_bend_range: {
      title: "DCO Bend Range",
      type_specific: {
        t: "enum",
        default: "1",
        values: Array.from({ length: 12 }, (_, i) => String(i + 1)),
      },
    },
    dco_env_source: {
      title: "DCO Env Source",
      type_specific: envSource,
    },
    mix_dco1: {
      title: "Mix DCO1",
      type_specific: percentage(100),
    },
    mix_dco2: {
      title: "Mix DCO2",
      type_specific: percentage(0),
    },
    mix_env: {
      title: "Mix Envelope",
      type_specific: percentage(0),
    },
    mix_env_source: {
      title: "Mix Env Source",
      type_specific: envSource,
    },
    hpf_mode: {
      title: "HPF Mode",
      type_specific: {
        t: "enum",
        default: "LowBoost",
        values: ["LowBoost", "Flat", "LowCut1", "LowCut2"],
      },
    },
    resonance: {
      title: "VCF Resonance",
      type_specific: percentage(0),
    },
    vcf_cutoff: {
      title: "VCF Cutoff",
      type_specific: percentage(100),
    },
    vcf_key_follow: {
      title: "VCF Key Follow",
      type_specific: percentage(0),
    },
    vcf_env: {
      title: "VCF Envelope",
      type_specific: percentage(0),
    },
    vcf_lfo: {
      title: "VCF LFO",
      type_specific: percentage(0),
    },
    vcf_env_source: {
      title: "VCF Env Source",
      type_specific: envSource,
    },
    vca_env_source: {
      title: "VCA Env Source",
      type_specific: {
        t: "enum",
        default: "Gate",
        values: ["Gate", "Gate-Dynamic", "Env2", "Env2-Dynamic"],
      },
    },
    ...envParams("Env1", "env1"),
    ...envParams("Env2", "env2"),
    lfo_rate: {
      title: "LFO Rate",
      type_specific: percentage(50),
    },
    lfo_delay: {
      title: "LFO Delay",
      type_specific: percentage(0),
    },
    lfo_shape: {
      title: "LFO Shape",
      type_specific: {
        t: "enum",
        default: "Sine",
        values: ["Sine", "Square", "Rand"],
      },
    },
  }),
);

export default infos;
