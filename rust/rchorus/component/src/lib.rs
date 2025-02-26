use conformal_component::parameters::{self, InfoRef};
use conformal_component::parameters::{Flags, TypeSpecificInfoRef};
use conformal_component::{Component as ComponentT, ProcessingEnvironment};

const PARAMETERS: [InfoRef<'static, &'static str>; 5] = [
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
];

mod anti_aliasing_filter;
mod compander;
mod effect;
mod kernel;
mod lfo;
mod modulated_delay;
mod nonlinearity;
mod polyphase_kernel;

#[cfg(test)]
mod tests;

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
