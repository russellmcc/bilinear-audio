use conformal_component::{
    ProcessingEnvironment, Processor,
    audio::BufferMut,
    events::{self, Event, Events},
    parameters::{self, BufferStates},
    synth::Synth as SynthTrait,
};
use conformal_poly::Poly;

mod voice;

#[derive(Debug)]
pub struct Synth {
    poly: Poly<voice::Voice>,
}

impl Synth {
    pub fn new(env: &ProcessingEnvironment) -> Self {
        Self {
            poly: Poly::new(env, 8),
        }
    }
}

impl Processor for Synth {
    fn set_processing(&mut self, processing: bool) {
        if !processing {
            self.poly.reset();
        }
    }
}

impl SynthTrait for Synth {
    fn handle_events<E: Iterator<Item = events::Data> + Clone, P: parameters::States>(
        &mut self,
        events: E,
        _parameters: P,
    ) {
        self.poly.handle_events(events);
    }

    fn process<E: Iterator<Item = Event> + Clone, P: BufferStates, O: BufferMut>(
        &mut self,
        events: Events<E>,
        parameters: P,
        output: &mut O,
    ) {
        self.poly
            .process(events.into_iter(), &parameters, &Default::default(), output);
    }
}
