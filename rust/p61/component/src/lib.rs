use conformal_component::parameters::{self, InfoRef};
use conformal_component::parameters::{Flags, TypeSpecificInfoRef};
use conformal_component::{Component as ComponentT, ProcessingEnvironment};

mod synth;

#[cfg(test)]
mod tests;
#[derive(Clone, Debug, Default)]
pub struct Component {}

const fn percentage(default: f32) -> TypeSpecificInfoRef<'static, &'static str> {
    TypeSpecificInfoRef::Numeric {
        default,
        valid_range: 0.0..=100.0,
        units: Some("%"),
    }
}

static PARAMETERS: [InfoRef<'static, &'static str>; 27] = [
    InfoRef {
        title: "DCO1 Shape",
        short_title: "DCO1Shape",
        unique_id: "dco1_shape",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 0,
            values: &["Saw", "Pulse", "PWM"],
        },
    },
    InfoRef {
        title: "DCO1 Width",
        short_title: "DCO1Width",
        unique_id: "dco1_width",
        flags: Flags { automatable: true },
        type_specific: percentage(50.0),
    },
    InfoRef {
        title: "DCO1 Octave",
        short_title: "DCO1Octave",
        unique_id: "dco1_octave",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 1,
            values: &["Low", "Med", "High"],
        },
    },
    InfoRef {
        title: "DCO2 Shape",
        short_title: "DCO2Shape",
        unique_id: "dco2_shape",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 0,
            values: &["Off", "Saw", "Square"],
        },
    },
    InfoRef {
        title: "DCO2 Octave",
        short_title: "DCO2Octave",
        unique_id: "dco2_octave",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 1,
            values: &["Low", "Med", "High"],
        },
    },
    InfoRef {
        title: "DCO2 Detune",
        short_title: "DCO2Detune",
        unique_id: "dco2_detune",
        flags: Flags { automatable: true },
        type_specific: percentage(0.0),
    },
    InfoRef {
        title: "DCO2 Interval",
        short_title: "DCO2Interval",
        unique_id: "dco2_interval",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 1,
            values: &["-3", "1", "3", "4", "5"],
        },
    },
    InfoRef {
        title: "VCF Cutoff",
        short_title: "VCFCutoff",
        unique_id: "vcf_cutoff",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 64.0,
            valid_range: 0.0..=128.0,
            units: None,
        },
    },
    InfoRef {
        title: "VCF Resonance",
        short_title: "VCFResonance",
        unique_id: "vcf_resonance",
        flags: Flags { automatable: true },
        type_specific: percentage(0.0),
    },
    InfoRef {
        title: "VCF Key Tracking",
        short_title: "VCFKey",
        unique_id: "vcf_tracking",
        flags: Flags { automatable: true },
        type_specific: percentage(66.0),
    },
    InfoRef {
        title: "VCF Envelope",
        short_title: "VCFEG",
        unique_id: "vcf_env",
        flags: Flags { automatable: true },
        type_specific: percentage(0.0),
    },
    InfoRef {
        title: "VCF Velocity",
        short_title: "VCFVelocity",
        unique_id: "vcf_velocity",
        flags: Flags { automatable: true },
        type_specific: percentage(0.0),
    },
    InfoRef {
        title: "VCF Timbre Control",
        short_title: "VCF Timbre",
        unique_id: "timbre_vcf",
        flags: Flags { automatable: true },
        type_specific: percentage(0.0),
    },
    InfoRef {
        title: "Attack Time",
        short_title: "Attack",
        unique_id: "attack",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.001,
            valid_range: 0.001..=10.0,
            units: Some("s"),
        },
    },
    InfoRef {
        title: "Decay Time",
        short_title: "Decay",
        unique_id: "decay",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.001,
            valid_range: 0.001..=10.0,
            units: Some("s"),
        },
    },
    InfoRef {
        title: "Sustain",
        short_title: "Sustain",
        unique_id: "sustain",
        flags: Flags { automatable: true },
        type_specific: percentage(100.0),
    },
    InfoRef {
        title: "Release Time",
        short_title: "Release",
        unique_id: "release",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.001,
            valid_range: 0.001..=10.0,
            units: Some("s"),
        },
    },
    InfoRef {
        title: "VCA Mode",
        short_title: "VCAMode",
        unique_id: "vca_mode",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 1,
            values: &["Gate", "Envelope"],
        },
    },
    InfoRef {
        title: "VCA Velocity",
        short_title: "VCAVelocity",
        unique_id: "vca_velocity",
        flags: Flags { automatable: true },
        type_specific: percentage(0.0),
    },
    InfoRef {
        title: "VCA Level",
        short_title: "VCALevel",
        unique_id: "vca_level",
        flags: Flags { automatable: true },
        type_specific: percentage(80.0),
    },
    InfoRef {
        title: "MG Rate",
        short_title: "MGRate",
        unique_id: "mg_rate",
        flags: Flags { automatable: true },
        type_specific: percentage(60.0),
    },
    InfoRef {
        title: "MG Delay",
        short_title: "MGDelay",
        unique_id: "mg_delay",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.0,
            valid_range: 0.0..=10.0,
            units: Some("s"),
        },
    },
    InfoRef {
        title: "MG Pitch",
        short_title: "MGPitch",
        unique_id: "mg_pitch",
        flags: Flags { automatable: true },
        type_specific: percentage(0.0),
    },
    InfoRef {
        title: "MG VCF",
        short_title: "MGVCF",
        unique_id: "mg_vcf",
        flags: Flags { automatable: true },
        type_specific: percentage(0.0),
    },
    InfoRef {
        title: "Mod Wheel Rate",
        short_title: "WheelRate",
        unique_id: "wheel_rate",
        flags: Flags { automatable: true },
        type_specific: percentage(60.0),
    },
    InfoRef {
        title: "Mod Wheel DCO Depth",
        short_title: "WheelDCODepth",
        unique_id: "wheel_dco",
        flags: Flags { automatable: true },
        type_specific: percentage(10.0),
    },
    InfoRef {
        title: "Mod Wheel DCO Depth",
        short_title: "WheelDCODepth",
        unique_id: "wheel_vcf",
        flags: Flags { automatable: true },
        type_specific: percentage(0.0),
    },
];

impl ComponentT for Component {
    type Processor = synth::Synth;

    fn parameter_infos(&self) -> Vec<parameters::Info> {
        parameters::to_infos(&PARAMETERS)
    }

    fn create_processor(&self, env: &ProcessingEnvironment) -> Self::Processor {
        synth::Synth::new(env)
    }
}
