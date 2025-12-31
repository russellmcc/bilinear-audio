use conformal_component::{events::NoteData, parameters, pzip};
use conformal_poly::{EventData, Voice as VoiceTrait};
use dsp::f32::rescale;
use itertools::izip;
use num_traits::FromPrimitive;
use oscillators::{OscillatorSettings, Shape};

mod env;
mod oscillators;
mod vcf;

const PITCH_BEND_WIDTH: f32 = 2.;

/// Converts a MIDI pitch to a phase increment
fn increment(midi_pitch: f32, sampling_rate: f32) -> f32 {
    440f32 * 2.0f32.powf((midi_pitch - 69f32) / 12f32) / sampling_rate
}

#[derive(Default, Debug, Clone)]
pub struct SharedData {}

#[derive(Clone, Debug)]
pub struct Voice {
    pitch: f32,
    sampling_rate: f32,
    oscillators: oscillators::Oscillators,
    vcf: vcf::Vcf,
    env1: env::Env,
    env2: env::Env,

    // Optimization opportunity: we're using a full env here but don't need all the logic.
    gate_coeffs: env::Coeffs,
    gate: env::Env,
}

const GATE_ATTACK_TIME_SECONDS: f32 = 0.005;
const GATE_RELEASE_TIME_SECONDS: f32 = GATE_ATTACK_TIME_SECONDS;

impl VoiceTrait for Voice {
    type SharedData<'a> = SharedData;

    fn new(_max_samples_per_process_call: usize, sampling_rate: f32) -> Self {
        Self {
            pitch: 20.0,
            oscillators: oscillators::Oscillators::default(),
            sampling_rate,
            vcf: vcf::Vcf::default(),
            env1: env::Env::default(),
            env2: env::Env::default(),
            gate_coeffs: env::calc_coeffs(
                &env::Params {
                    attack_time: GATE_ATTACK_TIME_SECONDS,
                    attack_target: 1.0,
                    decay_time: 0.0,
                    decay_target: 1.0,
                    to_sustain_time: 0.0,
                    sustain: 1.0,
                    release_time: GATE_RELEASE_TIME_SECONDS,
                },
                sampling_rate,
            ),
            gate: env::Env::default(),
        }
    }

    fn handle_event(&mut self, event: &conformal_poly::EventData) {
        match event {
            EventData::NoteOn {
                data: NoteData { pitch, .. },
            } => {
                self.pitch = f32::from(*pitch);
                self.env1.on();
                self.env2.on();
                self.gate.on();
            }
            EventData::NoteOff { .. } => {
                self.env1.off();
                self.env2.off();
                self.gate.off();
            }
        }
    }

    fn process(
        &mut self,
        events: impl IntoIterator<Item = conformal_poly::Event>,
        params: &impl parameters::BufferStates,
        note_expressions: conformal_poly::NoteExpressionCurve<
            impl Iterator<Item = conformal_poly::NoteExpressionPoint> + Clone,
        >,
        _data: Self::SharedData<'_>,
        output: &mut [f32],
    ) {
        let mut events = events.into_iter().peekable();
        for (
            (index, sample),
            (gain, dco1_shape_int, global_pitch_bend, vcf_cutoff, resonance, x_mod, sync),
            expression,
        ) in izip!(
            output.iter_mut().enumerate(),
            pzip!(params[numeric "gain", enum "dco1_shape", numeric "pitch_bend", numeric "vcf_cutoff", numeric "resonance", enum "x_mod", enum "sync"]),
            note_expressions.iter_by_sample(),
        ) {
            while let Some(conformal_poly::Event {
                sample_offset,
                data,
            }) = events.peek()
            {
                if sample_offset > &index {
                    break;
                }
                self.handle_event(data);
                events.next();
            }
            let total_pitch_bend = global_pitch_bend * PITCH_BEND_WIDTH + expression.pitch_bend;
            let adjusted_pitch = self.pitch + total_pitch_bend;
            let osc_incr = increment(adjusted_pitch, self.sampling_rate);
            *sample = self.oscillators.generate(&oscillators::Settings {
                oscillators: [
                    OscillatorSettings {
                        increment: osc_incr,
                        shape: FromPrimitive::from_u32(dco1_shape_int).unwrap(),
                        gain: 1.0,
                        width: 0.5,
                    },
                    OscillatorSettings {
                        increment: osc_incr,
                        shape: Shape::Saw,
                        gain: 0.0,
                        width: 0.5,
                    },
                ],
                x_mod: FromPrimitive::from_u32(x_mod).unwrap(),
                sync: FromPrimitive::from_u32(sync).unwrap(),
            }) * gain
                / 100.;

            let cutoff_incr = increment(vcf_cutoff, self.sampling_rate);
            let resonance = rescale(resonance, 0.0..=100.0, 0.0..=1.0);
            let vcf_settings = vcf::Settings {
                cutoff_incr,
                resonance,
            };
            *sample =
                self.gate.process(&self.gate_coeffs) * self.vcf.process(*sample, &vcf_settings);
        }
    }

    fn quiescent(&self) -> bool {
        self.env2.quiescent()
    }

    fn reset(&mut self) {
        self.pitch = 20.0;
        self.oscillators.reset();
        self.vcf.reset();
        self.env1.reset();
        self.env2.reset();
        self.gate.reset();
    }
}
