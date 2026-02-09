use conformal_component::{
    ProcessingEnvironment, Processor,
    audio::BufferMut,
    events::{Data, Event},
    parameters::{self},
    pzip,
    synth::{HandleEventsContext, ProcessContext, Synth as SynthT},
};
use rtsan_standalone::nonblocking;

use self::voice::SharedData;

use conformal_poly::Poly;
use dsp::{f32::rescale, osc_utils::increment};

mod osc_utils;
mod voice;

#[derive(Debug)]
pub struct Synth {
    poly: Poly<voice::Voice, 8>,
    mg: dsp::sine_lfo::SineLfo,
    mg_env: dsp::env::duck::Ar,
    mg_scratch: Vec<f32>,

    wheel_mg: dsp::sine_lfo::SineLfo,
    wheel_scratch: Vec<f32>,

    sampling_rate: f32,
}

impl Synth {
    pub fn new(env: &ProcessingEnvironment) -> Self {
        Self {
            poly: Poly::new(env),
            mg: Default::default(),
            mg_env: Default::default(),
            mg_scratch: vec![0f32; env.max_samples_per_process_call],

            wheel_mg: Default::default(),
            wheel_scratch: vec![0f32; env.max_samples_per_process_call],

            sampling_rate: env.sampling_rate,
        }
    }
}

struct MgParams {
    rate: f32,
    delay: f32,

    wheel_rate: f32,
}

fn mg_params(params: &impl parameters::BufferStates) -> impl Iterator<Item = MgParams> {
    pzip!(params[numeric "mg_rate", numeric "mg_delay", numeric "wheel_rate"]).map(
        |(rate, delay, wheel_rate)| MgParams {
            rate,
            delay,
            wheel_rate,
        },
    )
}

impl Processor for Synth {
    #[nonblocking]
    fn set_processing(&mut self, processing: bool) {
        if !processing {
            self.poly.reset();
            self.mg.reset();
            self.mg_env.reset();
            self.wheel_mg.reset();
        }
    }
}

impl SynthT for Synth {
    #[nonblocking]
    fn handle_events(&mut self, context: &impl HandleEventsContext) {
        self.poly.handle_events(context);
        for data in context.events() {
            match data {
                Data::NoteOn { .. } => {
                    self.mg_env.on();
                }
                Data::NoteOff { .. } => {
                    self.mg_env.off();
                }
            }
        }
    }

    #[nonblocking]
    fn process(&mut self, context: &impl ProcessContext, output: &mut impl BufferMut) {
        let mg_scratch = &mut self.mg_scratch[..output.num_frames()];
        let wheel_scratch = &mut self.wheel_scratch[..output.num_frames()];
        let mut mg_events = context.events().into_iter().peekable();
        for (
            ((index, sample), wheel_sample),
            MgParams {
                rate,
                delay,
                wheel_rate,
            },
        ) in mg_scratch
            .iter_mut()
            .enumerate()
            .zip(&mut wheel_scratch.iter_mut())
            .zip(mg_params(context.parameters()))
        {
            while let Some(Event {
                sample_offset,
                data,
            }) = mg_events.peek()
            {
                if sample_offset > &index {
                    break;
                }
                match data {
                    Data::NoteOn { .. } => {
                        self.mg_env.on();
                    }
                    Data::NoteOff { .. } => {
                        self.mg_env.off();
                    }
                }
                mg_events.next();
            }
            // Optimization opportunity - rational approximation
            let note = rescale(rate, 0.0..=100.0, -75.0..=15.0);
            let incr = increment(note, self.sampling_rate);
            let coeffs = dsp::env::duck::calc_coeffs(
                &dsp::env::duck::Params {
                    attack_time: delay,
                    release_time: 0.010,
                },
                self.sampling_rate,
            );
            *sample = self.mg_env.process(&coeffs) * self.mg.generate(incr);

            // Note that we have a slightly different rate for the wheel,
            // this adds a bit of detuning vs the MG.
            let wheel_note = rescale(wheel_rate, 0.0..=100.0, -76.0..=15.0);
            let wheel_incr = increment(wheel_note, self.sampling_rate);
            *wheel_sample = self.wheel_mg.generate(wheel_incr);
        }
        self.poly.process(
            context,
            &SharedData {
                mg_data: mg_scratch,
                wheel_data: wheel_scratch,
            },
            output,
        );
    }
}
