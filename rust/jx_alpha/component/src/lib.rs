use conformal_component::parameters::{self, Flags, InfoRef, TypeSpecificInfoRef};
use conformal_component::{Component as ComponentTrait, ProcessingEnvironment};

const PARAMETERS: [InfoRef<'static, &'static str>; 47] = [
    InfoRef {
        title: "Level",
        short_title: "Level",
        unique_id: "level",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 100.,
            valid_range: 0f32..=100.,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "DCO1 Shape",
        short_title: "DCO1 Shape",
        unique_id: "dco1_shape",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 0,
            values: &["Saw", "Pulse", "PwmSaw", "CombSaw", "Noise"],
        },
    },
    InfoRef {
        title: "DCO1 PWM Depth",
        short_title: "DCO1 PWM Depth",
        unique_id: "dco1_pwm_depth",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "DCO1 PWM Rate",
        short_title: "DCO1 PWM Rate",
        unique_id: "dco1_pwm_rate",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.01..=10.0,
            units: Some("Hz"),
        },
    },
    InfoRef {
        title: "DCO1 Tune",
        short_title: "DCO1 Tune",
        unique_id: "dco1_tune",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: -36.0..=24.0,
            units: Some("Semitones"),
        },
    },
    InfoRef {
        title: "DCO1 Envelope",
        short_title: "DCO1 Env",
        unique_id: "dco1_env",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "DCO1 LFO",
        short_title: "DCO1 LFO",
        unique_id: "dco1_lfo",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "DCO2 Shape",
        short_title: "DCO2 Shape",
        unique_id: "dco2_shape",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 0,
            values: &["Saw", "Pulse", "PwmSaw", "CombSaw", "Noise"],
        },
    },
    InfoRef {
        title: "DCO2 PWM Depth",
        short_title: "DCO2 PWM Depth",
        unique_id: "dco2_pwm_depth",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "DCO2 PWM Rate",
        short_title: "DCO2 PWM Rate",
        unique_id: "dco2_pwm_rate",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.01..=10.0,
            units: Some("Hz"),
        },
    },
    InfoRef {
        title: "DCO2 Tune",
        short_title: "DCO2 Tune",
        unique_id: "dco2_tune",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: -36.0..=24.0,
            units: Some("Semitones"),
        },
    },
    InfoRef {
        title: "DCO2 Cross Modulation",
        short_title: "DCO2 X-Mod",
        unique_id: "x_mod",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 0,
            values: &["Off", "Ring", "Bit", "Sync", "Sync+Ring"],
        },
    },
    InfoRef {
        title: "DCO2 Envelope",
        short_title: "DCO2 Env",
        unique_id: "dco2_env",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "DCO2 LFO",
        short_title: "DCO2 LFO",
        unique_id: "dco2_lfo",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "DCO Bend Range",
        short_title: "DCO Bend Range",
        unique_id: "dco_bend_range",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 2.0,
            valid_range: 0.0..=12.0,
            units: Some("Semitones"),
        },
    },
    InfoRef {
        title: "DCO Env Source",
        short_title: "DCO2 Env",
        unique_id: "dco2_env_source",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 0,
            values: &[
                "Env1",
                "Env1-Inverse",
                "Env1-Dynamic",
                "Env1-Dynamic-Inverse",
                "Env2",
                "Env2-Inverse",
                "Env2-Dynamic",
                "Env2-Dynamic-Inverse",
            ],
        },
    },
    InfoRef {
        title: "Mix DCO1",
        short_title: "Mix DCO1",
        unique_id: "mix_dco1",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 100.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Mix DCO2",
        short_title: "Mix DCO2",
        unique_id: "mix_dco2",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Mix Envelope",
        short_title: "Mix Env",
        unique_id: "mix_env",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Mix Env Source",
        short_title: "Mix Env Source",
        unique_id: "mix_env_source",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 0,
            values: &[
                "Env1",
                "Env1-Inverse",
                "Env1-Dynamic",
                "Env1-Dynamic-Inverse",
                "Env2",
                "Env2-Inverse",
                "Env2-Dynamic",
                "Env2-Dynamic-Inverse",
            ],
        },
    },
    InfoRef {
        title: "HPF Mode",
        short_title: "HPF Mode",
        unique_id: "hpf_mode",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 1,
            values: &["LowBoost", "Flat", "LowCut1", "LowCut2"],
        },
    },
    InfoRef {
        title: "VCF Resonance",
        short_title: "Resonance",
        unique_id: "resonance",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "VCF Cutoff",
        short_title: "VCF Cutoff",
        unique_id: "vcf_cutoff",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 64.0,
            valid_range: 0.0..=128.0,
            units: None,
        },
    },
    InfoRef {
        title: "VCF Key Follow",
        short_title: "VCF Key Follow",
        unique_id: "vcf_key",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "VCF Envelope",
        short_title: "VCF Env",
        unique_id: "vcf_env",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "VCF LFO",
        short_title: "VCF LFO",
        unique_id: "vcf_lfo",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "VCF Env Source",
        short_title: "VCF Env Source",
        unique_id: "vcf_env_source",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 0,
            values: &[
                "Env1",
                "Env1-Inverse",
                "Env1-Dynamic",
                "Env1-Dynamic-Inverse",
                "Env2",
                "Env2-Inverse",
                "Env2-Dynamic",
                "Env2-Dynamic-Inverse",
            ],
        },
    },
    InfoRef {
        title: "VCA Env Source",
        short_title: "VCA Env Source",
        unique_id: "vca_env_source",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 0,
            values: &[
                "Gate",
                "Gate-Dynamic",
                "Env1",
                "Env1-Dynamic",
                "Env2",
                "Env2-Dynamic",
            ],
        },
    },
    InfoRef {
        title: "Env1 T1",
        short_title: "Env1 T1",
        unique_id: "env1_t1",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Env1 L1",
        short_title: "Env1 L1",
        unique_id: "env1_l1",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 100.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Env1 T2",
        short_title: "Env1 T2",
        unique_id: "env1_t2",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Env1 L2",
        short_title: "Env1 L2",
        unique_id: "env1_l2",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 100.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Env1 T3",
        short_title: "Env1 T3",
        unique_id: "env1_t3",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Env1 L3",
        short_title: "Env1 L3",
        unique_id: "env1_l3",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 100.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Env1 T4",
        short_title: "Env1 T4",
        unique_id: "env1_t4",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Env1 Key Follow",
        short_title: "Env1 Key",
        unique_id: "env1_key",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Env2 T1",
        short_title: "Env2 T1",
        unique_id: "env2_t1",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Env2 L1",
        short_title: "Env2 L1",
        unique_id: "env2_l1",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 100.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Env2 T2",
        short_title: "Env2 T2",
        unique_id: "env2_t2",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Env2 L2",
        short_title: "Env2 L2",
        unique_id: "env2_l2",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 100.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Env2 T3",
        short_title: "Env2 T3",
        unique_id: "env2_t3",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Env2 L3",
        short_title: "Env2 L3",
        unique_id: "env2_l3",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 100.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Env2 T4",
        short_title: "Env2 T4",
        unique_id: "env2_t4",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Env2 Key Follow",
        short_title: "Env2 Key",
        unique_id: "env2_key",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "LFO Rate",
        short_title: "LFO Rate",
        unique_id: "lfo_rate",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.01..=10.0,
            units: Some("Hz"),
        },
    },
    InfoRef {
        title: "LFO Delay",
        short_title: "LFO Delay",
        unique_id: "lfo_delay",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "LFO Shape",
        short_title: "LFO Shape",
        unique_id: "lfo_shape",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 0,
            values: &["Sine", "Square", "Rand"],
        },
    },
];

mod synth;

#[derive(Clone, Debug, Default)]
pub struct Component {}

impl ComponentTrait for Component {
    type Processor = synth::Synth;

    fn parameter_infos(&self) -> Vec<parameters::Info> {
        parameters::to_infos(&PARAMETERS)
    }

    fn create_processor(&self, env: &ProcessingEnvironment) -> Self::Processor {
        synth::Synth::new(env)
    }
}
