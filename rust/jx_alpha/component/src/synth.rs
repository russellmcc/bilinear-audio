use conformal_component::{
    ProcessingEnvironment, Processor,
    audio::{BufferMut, channels_mut},
    events::{self, Event, Events},
    parameters::{self, BufferStates},
    pzip,
    synth::Synth as SynthTrait,
};
use conformal_poly::Poly;
use hpf::{Hpf, Mode};
use num_traits::FromPrimitive;

mod hpf;
mod lfo;
mod voice;

#[derive(Debug)]
pub struct Synth {
    poly: Poly<voice::Voice>,
    hpfs: [Hpf; 2],
}

impl Synth {
    pub fn new(env: &ProcessingEnvironment) -> Self {
        Self {
            poly: Poly::new(env, 8),
            hpfs: core::array::from_fn(|_| Hpf::new(env.sampling_rate)),
        }
    }
}

impl Processor for Synth {
    fn set_processing(&mut self, processing: bool) {
        if !processing {
            self.poly.reset();
            self.hpfs.iter_mut().for_each(hpf::Hpf::reset);
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

        // we don't support sample-accurate switching of hpf mode, we just grab from parameters at start of buffer
        let mode = Mode::from_u32(pzip!(parameters[enum "hpf_mode"]).next().unwrap()).unwrap();
        for (channel, hpf) in channels_mut(output).zip(self.hpfs.iter_mut()) {
            hpf.process(mode, channel);
        }
    }
}
