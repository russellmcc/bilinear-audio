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

#[cfg(test)]
mod tests;

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
