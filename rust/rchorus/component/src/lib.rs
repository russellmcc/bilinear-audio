use conformal_component::parameters::{self, InfoRef};
use conformal_component::parameters::{Flags, TypeSpecificInfoRef};
use conformal_component::{Component as ComponentT, ProcessingEnvironment};

const PARAMETERS: [InfoRef<'static, &'static str>; 6] = [
    InfoRef {
        title: "Rate",
        short_title: "Rate",
        unique_id: "rate",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 0.35,
            valid_range: 0.08..=10.1,
            units: Some("hz"),
        },
    },
    InfoRef {
        title: "Depth",
        short_title: "Depth",
        unique_id: "depth",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 100.,
            valid_range: 0f32..=100.,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Mix",
        short_title: "Mix",
        unique_id: "mix",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 100.,
            valid_range: 0f32..=100.,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Highpass Cutoff",
        short_title: "Highpass",
        unique_id: "highpass_cutoff",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 0,
            values: &["Low", "High"],
        },
    },
    InfoRef {
        title: "Bypass",
        short_title: "Bypass",
        unique_id: "bypass",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Switch { default: false },
    },
    InfoRef {
        title: "Routing",
        short_title: "Routing",
        unique_id: "routing",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Enum {
            default: 0,
            values: &["Synth", "Dimension"],
        },
    },
];

mod anti_aliasing_filter;
mod compander;
mod effect;
mod kernel;
mod lfo;
mod modulated_delay;
mod nonlinearity;
mod polyphase_kernel;

#[derive(Clone, Debug, Default)]
pub struct Component {}

impl ComponentT for Component {
    type Processor = effect::Effect;

    fn parameter_infos(&self) -> Vec<parameters::Info> {
        parameters::to_infos(&PARAMETERS)
    }

    fn create_processor(&self, env: &ProcessingEnvironment) -> Self::Processor {
        effect::Effect::new(env)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use component_snapshots::{
        ProcessingParams,
        effect::{generate_basic_snapshot, generate_snapshot_with_reset},
    };
    use conformal_component::{ProcessingMode, audio::all_approx_eq};
    use snapshots::assert_snapshot;

    #[test]
    fn reset() {
        let test_sig = dsp::test_utils::sine(25, 440. / 48000.);
        let (before, after) = generate_snapshot_with_reset(
            &Component {},
            &test_sig,
            &ProcessingParams {
                max_buffer_size: 100,
                sampling_rate: 48000.0,
                processing_mode: ProcessingMode::Realtime,
            },
            &HashMap::new(),
        );
        assert!(all_approx_eq(before, after, 1e-6));
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn snapshot_sine() {
        let test_sig: Vec<_> = dsp::test_utils::sine(48000, 440. / 48000.)
            .iter()
            .map(|x| x * 1. / 3.)
            .collect();
        assert_snapshot!(
            "sine",
            48000,
            generate_basic_snapshot(&Component {}, &test_sig, &HashMap::new())
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn snapshot_sweep() {
        let test_sig: Vec<_> = dsp::test_utils::linear_sine_sweep(48000, 48000., 10., 20000.)
            .iter()
            .map(|x| x * 1. / 4.)
            .collect();
        assert_snapshot!(
            "sweep",
            48000,
            generate_basic_snapshot(&Component {}, &test_sig, &HashMap::new())
        );
    }
}
