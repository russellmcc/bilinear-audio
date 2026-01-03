use conformal_component::{
    ProcessingEnvironment, Processor,
    audio::{BufferMut, channels_mut},
    events::{self, Data, Event, Events},
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
    lfo: lfo::Lfo,
    lfo_delay_env: dsp::env::duck::Ar,
    lfo_scratch: Vec<f32>,
    sampling_rate: f32,
}

impl Synth {
    pub fn new(env: &ProcessingEnvironment) -> Self {
        Self {
            poly: Poly::new(env, 6),
            hpfs: core::array::from_fn(|_| Hpf::new(env.sampling_rate)),
            lfo: Default::default(),
            lfo_delay_env: Default::default(),
            lfo_scratch: vec![0f32; env.max_samples_per_process_call],
            sampling_rate: env.sampling_rate,
        }
    }
}

impl Processor for Synth {
    fn set_processing(&mut self, processing: bool) {
        if !processing {
            self.poly.reset();
            self.lfo.reset();
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
        let mut lfo_events = events.clone().into_iter().peekable();

        for ((index, sample), (rate, delay, shape_int)) in self
            .lfo_scratch
            .iter_mut()
            .enumerate()
            .zip(pzip!(parameters[numeric "lfo_rate", numeric "lfo_delay", enum "lfo_shape"]))
        {
            while let Some(Event {
                sample_offset,
                data,
            }) = lfo_events.peek()
            {
                if sample_offset > &index {
                    break;
                }
                match data {
                    Data::NoteOn { .. } => {
                        self.lfo_delay_env.on();
                    }
                    Data::NoteOff { .. } => {
                        self.lfo_delay_env.off();
                    }
                    Data::NoteExpression { .. } => {}
                }
                lfo_events.next();
            }
            let incr = rate / self.sampling_rate;
            let coeffs = dsp::env::duck::calc_coeffs(
                &dsp::env::duck::Params {
                    attack_time: delay,
                    release_time: 0.010,
                },
                self.sampling_rate,
            );
            *sample = self.lfo_delay_env.process(&coeffs)
                * self
                    .lfo
                    .generate(incr, FromPrimitive::from_u32(shape_int).unwrap());
        }

        self.poly.process(
            events.into_iter(),
            &parameters,
            &voice::SharedData {
                lfo: &self.lfo_scratch[..output.num_frames()],
            },
            output,
        );

        // we don't support sample-accurate switching of hpf mode, we just grab from parameters at start of buffer
        let mode = Mode::from_u32(pzip!(parameters[enum "hpf_mode"]).next().unwrap()).unwrap();
        for (channel, hpf) in channels_mut(output).zip(self.hpfs.iter_mut()) {
            hpf.process(mode, channel);
        }
    }
}
