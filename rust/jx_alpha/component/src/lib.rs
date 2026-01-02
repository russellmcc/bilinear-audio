use conformal_component::parameters::{self, Flags, InfoRef, TypeSpecificInfoRef};
use conformal_component::{Component as ComponentTrait, ProcessingEnvironment};

const PARAMETERS: [InfoRef<'static, &'static str>; 14] = [
    InfoRef {
        title: "Gain",
        short_title: "Gain",
        unique_id: "gain",
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
            default: 50.0,
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
            default: 1.0,
            valid_range: 0.0..=10.0,
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
            default: 50.0,
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
            default: 1.0,
            valid_range: 0.0..=10.0,
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
        title: "DCO2 Cross Modulation",
        short_title: "DCO2 X-Mod",
        unique_id: "x_mod",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 0,
            values: &["Off", "Ring", "Bit", "Sync", "Sync+Ring"],
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
