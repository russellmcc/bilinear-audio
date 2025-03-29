use conformal_component::parameters::{self, Flags, InfoRef, TypeSpecificInfoRef};
use conformal_component::{Component as ComponentTrait, ProcessingEnvironment};

const PARAMETERS: [InfoRef<'static, &'static str>; 1] = [InfoRef {
    title: "Gain",
    short_title: "Gain",
    unique_id: "gain",
    flags: Flags { automatable: true },
    type_specific: TypeSpecificInfoRef::Numeric {
        default: 100.,
        valid_range: 0f32..=100.,
        units: Some("%"),
    },
}];

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
