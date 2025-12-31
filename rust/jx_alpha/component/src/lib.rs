use conformal_component::parameters::{self, Flags, InfoRef, TypeSpecificInfoRef};
use conformal_component::{Component as ComponentTrait, ProcessingEnvironment};

const PARAMETERS: [InfoRef<'static, &'static str>; 7] = [
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
        title: "Resonance",
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
        title: "Cross Modulation",
        short_title: "X-Mod",
        unique_id: "x_mod",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 0,
            values: &["Off", "Ring", "Bit"],
        },
    },
    InfoRef {
        title: "Sync",
        short_title: "Sync",
        unique_id: "sync",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 0,
            values: &["Off", "Hard"],
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
