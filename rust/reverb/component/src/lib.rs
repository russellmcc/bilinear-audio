use conformal_component::audio::{Buffer, BufferMut};
use conformal_component::effect::Effect as EffectTrait;
use conformal_component::parameters::{self, BufferStates, Flags, InfoRef, TypeSpecificInfoRef};
use conformal_component::pzip;
use conformal_component::{Component as ComponentTrait, ProcessingEnvironment, Processor};
use dsp::f32::lerp;
use rtsan_standalone::nonblocking;

mod diffuser;
mod multi_channel_feedback_loop;
mod multi_channel_per_sample_delay;
mod per_sample_delay;
mod per_sample_modulated_delay;
mod reverb;
mod shuffler;

const TIME_MIN: f32 = 0.7;
const COEFF_MIN: f32 = 0.3;
const TIME_MID: f32 = 1.2;
const COEFF_MID: f32 = 0.55;
const TIME_MAX: f32 = 45.0;
const COEFF_MAX: f32 = 0.97;

const PARAMETERS: [InfoRef<'static, &'static str>; 7] = [
    InfoRef {
        title: "Bypass",
        short_title: "Bypass",
        unique_id: "bypass",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Switch { default: false },
    },
    InfoRef {
        title: "Mix",
        short_title: "Mix",
        unique_id: "mix",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 50.,
            valid_range: 0f32..=100.,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Brightness",
        short_title: "Brightness",
        unique_id: "brightness",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 100.,
            valid_range: 0f32..=100.,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Tone",
        short_title: "Tone",
        unique_id: "tone",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 100.,
            valid_range: 0f32..=100.,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Time",
        short_title: "Time",
        unique_id: "time",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: TIME_MID,
            valid_range: TIME_MIN..=TIME_MAX,
            units: Some("s"),
        },
    },
    InfoRef {
        title: "Early Reflection Character",
        short_title: "Early Reflections",
        unique_id: "early_reflections",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 50.0,
            valid_range: 0f32..=100.0,
            units: Some("%"),
        },
    },
    InfoRef {
        title: "Density",
        short_title: "Density",
        unique_id: "density",
        flags: Flags { automatable: true },
        type_specific: TypeSpecificInfoRef::Numeric {
            default: 100.0,
            valid_range: 0f32..=100.0,
            units: Some("%"),
        },
    },
];

const INTERNAL_MIX: [f32; 2] = [0.0, 1.0];
const INTERNAL_BRIGHTNESS: [f32; 2] = [0.125, 1.0];
const INTERNAL_DAMPING: [f32; 2] = [0.125, 1.0];
const INTERNAL_EARLY_REFLECTIONS: [f32; 2] = [0.0, 1.0];
const INTERNAL_DENSITY: [f32; 2] = [0.0, 1.0];

fn to_internal(value: f32, internal_range: [f32; 2]) -> f32 {
    let ratio = value / 100.0;
    lerp(internal_range[0], internal_range[1], ratio)
}

fn time_to_coeff(time: f32) -> f32 {
    if time <= TIME_MIN {
        lerp(
            COEFF_MIN,
            COEFF_MID,
            (time - TIME_MIN) / (TIME_MID - TIME_MIN),
        )
    } else {
        lerp(
            COEFF_MID,
            COEFF_MAX,
            (time - TIME_MID) / (TIME_MAX - TIME_MID),
        )
    }
}

#[derive(Clone, Debug, Default)]
pub struct Component {}

impl Component {
    #[must_use]
    pub fn new() -> Self {
        Component {}
    }
}

pub struct Effect {
    reverb: reverb::Reverb,
}

impl Effect {
    #[must_use]
    pub fn new(env: &ProcessingEnvironment) -> Self {
        Effect {
            reverb: reverb::Reverb::new(env),
        }
    }
}

impl Processor for Effect {
    #[nonblocking]
    fn set_processing(&mut self, processing: bool) {
        if !processing {
            self.reverb.reset();
        }
    }
}

impl EffectTrait for Effect {
    #[nonblocking]
    fn handle_parameters<P: parameters::States>(&mut self, _: P) {}

    #[nonblocking]
    fn process<P: BufferStates, I: Buffer, O: BufferMut>(
        &mut self,
        parameters: P,
        input: &I,
        output: &mut O,
    ) {
        // Snapshot the parameters at the start of the buffer - we don't support per-sample automation.
        if let Some(params) = pzip!(
            parameters[switch "bypass", numeric "mix", numeric "brightness", numeric "tone", numeric "time", numeric "early_reflections", numeric "density"]
        )
        .map(|(bypass, mix, brightness, tone, time, early_reflections, density)| reverb::Params {
            mix: if bypass { 0.0 } else { to_internal(mix, INTERNAL_MIX) },
            brightness: to_internal(brightness, INTERNAL_BRIGHTNESS),
            damping: to_internal(tone, INTERNAL_DAMPING),
            feedback: time_to_coeff(time),
            early_reflections: to_internal(early_reflections, INTERNAL_EARLY_REFLECTIONS),
            density: to_internal(density, INTERNAL_DENSITY),
        })
        .next()
        {
            self.reverb.process(&params, input, output);
        }
    }
}

impl ComponentTrait for Component {
    type Processor = Effect;

    fn parameter_infos(&self) -> Vec<parameters::Info> {
        parameters::to_infos(&PARAMETERS)
    }

    fn create_processor(&self, env: &ProcessingEnvironment) -> Self::Processor {
        Effect::new(env)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use component_snapshots::{
        ProcessingParams,
        effect::{generate_basic_snapshot, generate_snapshot, generate_snapshot_with_reset},
    };
    use conformal_component::{ProcessingMode, audio::all_approx_eq, parameters::InternalValue};
    use snapshots::assert_snapshot;

    use super::*;

    #[test]
    fn reset() {
        let test_sig = dsp::test_utils::sine(25, 440. / 48000.);
        let (before, after) = generate_snapshot_with_reset(
            &Component::new(),
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
    fn buffer_size_agnostic() {
        let test_sig: Vec<f32> = dsp::test_utils::sine(3000, 440. / 48000.);
        let buff_512 = generate_snapshot(
            &Component::new(),
            &test_sig,
            &ProcessingParams {
                max_buffer_size: 512,
                ..Default::default()
            },
            &HashMap::new(),
        );
        let buff_1024 = generate_snapshot(
            &Component::new(),
            &test_sig,
            &ProcessingParams {
                max_buffer_size: 1024,
                ..Default::default()
            },
            &HashMap::new(),
        );
        assert!(all_approx_eq(buff_512, buff_1024, 1e-6));
    }

    fn impulse_response_for_params(params: HashMap<&'_ str, InternalValue>) -> Vec<f32> {
        const SNAPSHOT_LENGTH: usize = 48_000 * 2;
        let mut impulse_vec = vec![0.0; SNAPSHOT_LENGTH];
        impulse_vec[0] = 1.0;
        generate_basic_snapshot(&Component::new(), &impulse_vec, &params)
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn impulse_default() {
        assert_snapshot!(
            "impulse_default",
            48000,
            impulse_response_for_params(HashMap::new())
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn impulse_short() {
        assert_snapshot!(
            "impulse_short",
            48000,
            impulse_response_for_params(HashMap::from([("time", InternalValue::Numeric(0.7))]))
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn impulse_long() {
        assert_snapshot!(
            "impulse_long",
            48000,
            impulse_response_for_params(HashMap::from([("time", InternalValue::Numeric(3.1))]))
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn impulse_dark() {
        assert_snapshot!(
            "impulse_dark",
            48000,
            impulse_response_for_params(HashMap::from([
                ("brightness", InternalValue::Numeric(0.0)),
                ("tone", InternalValue::Numeric(0.0)),
            ]))
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn impulse_mid_dark() {
        assert_snapshot!(
            "impulse_mid_dark",
            48000,
            impulse_response_for_params(HashMap::from([
                ("brightness", InternalValue::Numeric(50.0)),
                ("tone", InternalValue::Numeric(50.0)),
            ]))
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn impulse_early_reflections_late() {
        assert_snapshot!(
            "impulse_early_reflections_late",
            48000,
            impulse_response_for_params(HashMap::from([(
                "early_reflections",
                InternalValue::Numeric(100.0)
            )]))
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn impulse_early_reflections_early() {
        assert_snapshot!(
            "impulse_early_reflections_early",
            48000,
            impulse_response_for_params(HashMap::from([(
                "early_reflections",
                InternalValue::Numeric(0.0)
            )]))
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn impulse_mid_density() {
        assert_snapshot!(
            "impulse_mid_density",
            48000,
            impulse_response_for_params(HashMap::from([("density", InternalValue::Numeric(50.0))]))
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn impulse_low_density() {
        assert_snapshot!(
            "impulse_low_density",
            48000,
            impulse_response_for_params(HashMap::from([("density", InternalValue::Numeric(0.0))]))
        );
    }
}
