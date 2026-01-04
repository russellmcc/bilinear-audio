use std::ops::RangeInclusive;

use conformal_component::{
    ProcessingEnvironment, Processor,
    audio::{BufferMut, channels_mut},
    events::{self, Data, Event, Events},
    parameters::{self, BufferStates},
    pzip,
    synth::Synth as SynthTrait,
};
use conformal_poly::Poly;
use dsp::f32::rescale;
use hpf::{Hpf, Mode};
use num_traits::FromPrimitive;

mod hpf;
mod lfo;
mod voice;

const LFO_NOTE_RANGE: RangeInclusive<f32> = -105.0f32..=15.0f32;

/// Converts a MIDI pitch to a phase increment
fn increment(midi_pitch: f32, sampling_rate: f32) -> f32 {
    (440f32 * 2.0f32.powf((midi_pitch - 69f32) / 12f32) / sampling_rate).clamp(0.0, 0.45)
}

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
        let lfo_scratch = &mut self.lfo_scratch[..output.num_frames()];
        let mut lfo_events = events.clone().into_iter().peekable();

        for ((index, sample), (rate, delay, shape_int)) in lfo_scratch
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
            let incr = increment(
                rescale(rate, 0.0..=100.0, LFO_NOTE_RANGE),
                self.sampling_rate,
            );
            // LFO delay follows the somewhat bizarre trimming measured from hardware.
            let lfo_delay_time_seconds = if delay > 0.0 {
                rescale(delay, 0.0..=100.0, -5.5..=4.5).exp2()
            } else {
                0.0
            };
            let lfo_delay_attack = (lfo_delay_time_seconds / 2.0).min(1.0);
            let coeffs = dsp::env::duck::calc_hold_coeffs(
                &dsp::env::duck::HoldParams {
                    attack_time: lfo_delay_attack,
                    hold_time: lfo_delay_time_seconds - lfo_delay_attack,
                    release_time: if lfo_delay_time_seconds > 0.0 {
                        0.005
                    } else {
                        0.0
                    },
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
            &voice::SharedData { lfo: lfo_scratch },
            output,
        );

        // we don't support sample-accurate switching of hpf mode, we just grab from parameters at start of buffer
        let mode = Mode::from_u32(pzip!(parameters[enum "hpf_mode"]).next().unwrap()).unwrap();
        for (channel, hpf) in channels_mut(output).zip(self.hpfs.iter_mut()) {
            hpf.process(mode, channel);
        }
    }
}
