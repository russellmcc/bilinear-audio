use conformal_poly::{Event, EventData, NoteExpressionCurve, NoteExpressionPoint, Voice as VoiceT};
use dsp::{
    f32::{lerp, rescale},
    osc_utils::increment,
};

use conformal_component::{parameters, pzip};

mod dco1;
mod dco2;
mod vca;
mod vcf;

use itertools::izip;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use super::env::adsr;

#[derive(Debug, Default)]
struct Note {
    midi_number: f32,
    velocity: f32,
}

#[derive(Debug)]
pub struct Voice {
    sampling_rate: f32,

    note: Note,
    dco1: dco1::Dco1,
    dco2: dco2::Dco2,
    adsr: adsr::Adsr,

    gate: adsr::Adsr,
    gate_coeffs: adsr::Coeffs,

    vca: vca::Vca,
    vcf: vcf::Vcf,
}

#[derive(FromPrimitive, Copy, Clone, Debug, PartialEq)]
enum Octave {
    Low,
    Medium,
    High,
}

#[derive(FromPrimitive, Copy, Clone, Debug, PartialEq)]
pub(crate) enum Dco1Shape {
    Saw,
    Pulse,
    Pwm,
}

#[derive(FromPrimitive, Copy, Clone, Debug, PartialEq)]
pub(crate) enum Dco2Shape {
    Off,
    Saw,
    Square,
}

#[derive(FromPrimitive, Copy, Clone, Debug, PartialEq)]
pub(crate) enum Dco2Interval {
    MinorThird,
    Unison,
    ThirdAbove,
    FourthAbove,
    FifthAbove,
}

#[derive(FromPrimitive, Copy, Clone, Debug, PartialEq)]
pub(crate) enum VcaMode {
    Gate,
    Envelope,
}

struct OscSectionParams {
    dco1_shape: Dco1Shape,
    dco1_width: f32,
    dco1_octave: Octave,
    dco2_shape: Dco2Shape,
    dco2_octave: dco2::Octave,
    dco2_detune: f32,
    dco2_interval: Dco2Interval,
}

struct Params {
    osc: OscSectionParams,

    vcf_cutoff: f32,
    vcf_resonance: f32,
    vcf_tracking: f32,
    vcf_env: f32,
    vcf_velocity: f32,

    attack_time: f32,
    decay_time: f32,
    sustain: f32,
    release_time: f32,

    vca_mode: VcaMode,
    vca_velocity: f32,
    vca_level: f32,

    mg_pitch: f32,
    mg_vcf: f32,

    pitch_bend: f32,

    wheel: f32,
    wheel_dco: f32,
    wheel_vcf: f32,

    timbre: f32,
    timbre_vcf: f32,
}

fn per_sample_params(params: &impl parameters::BufferStates) -> impl Iterator<Item = Params> {
    pzip!(params[enum "dco1_shape",
                 numeric "dco1_width",
                 enum "dco1_octave",
                 enum "dco2_shape",
                 enum "dco2_octave",
                 numeric "dco2_detune",
                 enum "dco2_interval",
                 numeric "vcf_cutoff",
                 numeric "vcf_resonance",
                 numeric "vcf_tracking",
                 numeric "vcf_env",
                 numeric "vcf_velocity",
                 numeric "attack",
                 numeric "decay",
                 numeric "sustain",
                 numeric "release",
                 enum "vca_mode",
                 numeric "vca_velocity",
                 numeric "vca_level",
                 numeric "mg_pitch",
                 numeric "mg_vcf",
                 numeric "pitch_bend",
                 numeric "mod_wheel",
                 numeric "wheel_dco",
                 numeric "wheel_vcf",
                 numeric "timbre",
                 numeric "timbre_vcf"
    ])
    .map(
        |(
            dco1_shape,
            dco1_width,
            dco1_octave,
            dco2_shape,
            dco2_octave,
            dco2_detune,
            dco2_interval,
            vcf_cutoff,
            vcf_resonance,
            vcf_tracking,
            vcf_env,
            vcf_velocity,
            attack_time,
            decay_time,
            sustain,
            release_time,
            vca_mode,
            vca_velocity,
            vca_level,
            mg_pitch,
            mg_vcf,
            pitch_bend,
            wheel,
            wheel_dco,
            wheel_vcf,
            timbre,
            timbre_vcf,
        )| Params {
            osc: OscSectionParams {
                dco1_shape: FromPrimitive::from_u32(dco1_shape).unwrap(),
                dco1_width,
                dco1_octave: FromPrimitive::from_u32(dco1_octave).unwrap(),
                dco2_shape: FromPrimitive::from_u32(dco2_shape).unwrap(),
                dco2_octave: FromPrimitive::from_u32(dco2_octave).unwrap(),
                dco2_detune,
                dco2_interval: FromPrimitive::from_u32(dco2_interval).unwrap(),
            },
            vcf_cutoff,
            vcf_resonance,
            vcf_tracking,
            vcf_env,
            vcf_velocity,

            attack_time,
            decay_time,
            sustain,
            release_time,

            vca_mode: FromPrimitive::from_u32(vca_mode).unwrap(),
            vca_velocity,
            vca_level,
            mg_pitch,
            mg_vcf,

            pitch_bend,

            wheel,
            wheel_dco,
            wheel_vcf,

            timbre,
            timbre_vcf,
        },
    )
}

#[derive(Debug, Clone)]
pub struct SharedData<'a> {
    pub mg_data: &'a [f32],

    // Mod-wheel modulation data
    pub wheel_data: &'a [f32],
}

impl Voice {
    fn osc_section_sample(
        &mut self,
        OscSectionParams {
            dco1_shape,
            dco1_width,
            dco1_octave,
            dco2_shape,
            dco2_octave,
            dco2_detune,
            dco2_interval,
        }: &OscSectionParams,
        midi_number: f32,
        mg: f32,
    ) -> f32 {
        let dco1_incr = increment(
            match dco1_octave {
                Octave::Low => -12.0,
                Octave::Medium => 0.0,
                Octave::High => 12.0,
            } + midi_number,
            self.sampling_rate,
        );
        let dco1 = match dco1_shape {
            Dco1Shape::Saw => self.dco1.generate(dco1_incr, midi_number, dco1::Shape::Saw),
            Dco1Shape::Pulse => self.dco1.generate(
                dco1_incr,
                midi_number,
                dco1::Shape::Pulse {
                    width: (*dco1_width * 0.0090) + 0.05,
                },
            ),
            Dco1Shape::Pwm => self.dco1.generate(
                dco1_incr,
                midi_number,
                dco1::Shape::Pulse {
                    width: (*dco1_width * 0.0045) * mg + 0.5,
                },
            ),
        };
        let dco2_incr = || {
            let dco2_detune_cents = *dco2_detune * 0.5 + 5.0;
            let dco2_octave_offset = match dco2_octave {
                dco2::Octave::Low => -12.0,
                dco2::Octave::Medium => 0.0,
                dco2::Octave::High => 12.0,
            };
            let dco2_interval_offset = match dco2_interval {
                Dco2Interval::MinorThird => 3.0,
                Dco2Interval::Unison => 0.0,
                Dco2Interval::ThirdAbove => 4.0,
                Dco2Interval::FourthAbove => 5.0,
                Dco2Interval::FifthAbove => 7.0,
            };
            // Optimization opportunity - it might be possible to
            // use a rational approximation here.
            increment(
                dco2_octave_offset
                    + dco2_interval_offset
                    + midi_number
                    + (dco2_detune_cents) / 100.0,
                self.sampling_rate,
            )
        };
        // Sound quality opportunity - currently we instantly turn
        // DCO2 on and off, causing a click!
        let dco2 = match dco2_shape {
            Dco2Shape::Off => 0.0,
            Dco2Shape::Saw => self
                .dco2
                .generate(dco2_incr(), dco2::Shape::Saw, *dco2_octave),
            Dco2Shape::Square => self
                .dco2
                .generate(dco2_incr(), dco2::Shape::Square, *dco2_octave),
        };
        match dco2_shape {
            Dco2Shape::Off => dco1,
            _ => 0.707 * (dco1 + dco2),
        }
    }
}

const PITCH_BEND_WIDTH: f32 = 2.0;
const MAX_WHEEL_DEPTH: f32 = 12.0;
const MAX_TIMBRE_DEPTH: f32 = 60.0;

struct VcfIncrParams {
    midi_number: f32,
    velocity: f32,

    env: f32,

    mg: f32,
    mg_vcf: f32,

    vcf_cutoff: f32,
    vcf_tracking: f32,
    vcf_velocity: f32,
    vcf_env: f32,

    pitch_bend: f32,

    wheel_mg: f32,
    wheel: f32,
    wheel_vcf: f32,

    timbre: f32,
    timbre_vcf: f32,

    sampling_rate: f32,
}
fn vcf_incr(
    VcfIncrParams {
        midi_number,
        velocity,
        env,
        mg,
        mg_vcf,
        vcf_cutoff,
        vcf_tracking,
        vcf_velocity,
        vcf_env,
        pitch_bend,
        wheel_mg,
        wheel,
        wheel_vcf,
        timbre,
        timbre_vcf,
        sampling_rate,
    }: VcfIncrParams,
) -> f32 {
    let vcf_mg = lerp(0.0, 12.0, mg_vcf * 0.01) * mg;
    let vcf_wheel = wheel_mg * wheel * lerp(0.0, MAX_WHEEL_DEPTH, wheel_vcf * 0.01);
    let vcf_timbre = lerp(0.0, MAX_TIMBRE_DEPTH, timbre_vcf * 0.01) * timbre;
    let vcf_env = lerp(vcf_env, vcf_env * velocity, vcf_velocity * 0.01);
    let vcf_midi_number = pitch_bend + midi_number;
    {
        const MIDI_TRACKING_BASE: f32 = 60.0;
        increment(
            vcf_mg
                + vcf_wheel
                + vcf_cutoff
                + vcf_timbre
                + 0.01
                    * (vcf_tracking * (vcf_midi_number - MIDI_TRACKING_BASE)
                        + vcf_env * env * 128.0),
            sampling_rate,
        )
    }
}

impl VoiceT for Voice {
    type SharedData<'a> = SharedData<'a>;

    fn new(_max_samples_per_process_call: usize, sampling_rate: f32) -> Self {
        Self {
            sampling_rate,

            note: Default::default(),
            dco1: Default::default(),
            dco2: Default::default(),
            adsr: Default::default(),
            gate: Default::default(),
            gate_coeffs: adsr::calc_coeffs(
                &adsr::Params {
                    attack_time: 0.005,
                    decay_time: 0.000,
                    sustain: 1.0,
                    release_time: 0.005,
                },
                sampling_rate,
            ),
            vca: vca::Vca::new(sampling_rate),
            vcf: vcf::Vcf::new(),
        }
    }

    fn process(
        &mut self,
        events: impl IntoIterator<Item = Event>,
        params: &impl parameters::BufferStates,
        note_expression: NoteExpressionCurve<impl Iterator<Item = NoteExpressionPoint> + Clone>,
        shared_data: Self::SharedData<'_>,
        output: &mut [f32],
    ) {
        let mut events = events.into_iter().peekable();
        for ((index, sample), params, mg, wheel_mg, expression) in izip!(
            output.iter_mut().enumerate(),
            per_sample_params(params),
            shared_data.mg_data,
            shared_data.wheel_data,
            note_expression.iter_by_sample(),
        ) {
            while let Some(Event {
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
            let Note {
                midi_number,
                velocity,
            } = self.note;

            let coeffs = adsr::calc_coeffs(
                &adsr::Params {
                    attack_time: params.attack_time,
                    decay_time: params.decay_time,
                    sustain: params.sustain * 0.01,
                    release_time: params.release_time,
                },
                self.sampling_rate,
            );

            let osc_wheel =
                wheel_mg * params.wheel * lerp(0.0, MAX_WHEEL_DEPTH, params.wheel_dco * 0.01);
            let pitch_bend = params.pitch_bend * PITCH_BEND_WIDTH + expression.pitch_bend;
            let timbre = params.timbre + expression.timbre;
            let osc_midi_number =
                lerp(0.0, 12.0, params.mg_pitch * 0.01) * mg + pitch_bend + midi_number + osc_wheel;

            let osc = self.osc_section_sample(&params.osc, osc_midi_number, *mg);

            let env = self.adsr.process(&coeffs);
            let gate = self.gate.process(&self.gate_coeffs);

            *sample = self.vca.process(
                self.vcf.process(
                    osc,
                    vcf_incr(VcfIncrParams {
                        midi_number,
                        velocity,
                        env,
                        mg: *mg,
                        mg_vcf: params.mg_vcf,
                        vcf_cutoff: params.vcf_cutoff,
                        vcf_tracking: params.vcf_tracking,
                        vcf_velocity: params.vcf_velocity,
                        vcf_env: params.vcf_env,
                        pitch_bend,
                        wheel_mg: *wheel_mg,
                        wheel: params.wheel,
                        wheel_vcf: params.wheel_vcf,
                        timbre,
                        timbre_vcf: params.timbre_vcf,
                        sampling_rate: self.sampling_rate,
                    })
                    .clamp(0.0, 0.4),
                    rescale(params.vcf_resonance, 0.0..=100.0, -0.5f32..=3f32).exp2(),
                ),
                params.vca_level
                    * 0.01
                    * lerp(1.0, velocity, params.vca_velocity * 0.01)
                    * match params.vca_mode {
                        VcaMode::Gate => gate,
                        VcaMode::Envelope => env,
                    },
            );
        }
    }

    fn reset(&mut self) {
        self.dco1.reset();
        self.dco2.reset();
        self.adsr.reset();
        self.vca.reset();
        self.vcf.reset();
    }

    fn handle_event(&mut self, event: &EventData) {
        match event {
            EventData::NoteOn { data } => {
                let midi_pitch = f32::from(data.pitch);
                self.note = Note {
                    midi_number: midi_pitch,
                    velocity: data.velocity,
                };
                self.adsr.on();
                self.gate.on();
            }
            EventData::NoteOff { .. } => {
                self.adsr.off();
                self.gate.off();
            }
        }
    }

    fn quiescent(&self) -> bool {
        self.adsr.quiescent() && self.gate.quiescent()
    }
}

#[cfg(test)]
mod tests {
    use crate::PARAMETERS;
    use assert_approx_eq::assert_approx_eq;
    use conformal_component::{
        events::{NoteData, NoteID},
        parameters::{ConstantBufferStates, InternalValue, StatesMap, override_synth_defaults},
    };
    use conformal_poly::{
        Event, EventData, NoteExpressionCurve, NoteExpressionPoint, NoteExpressionState,
        Voice as VoiceT, default_note_expression_curve,
    };
    use snapshots::assert_snapshot;
    use std::collections::HashMap;

    use super::{Dco2Shape, SharedData, Voice};

    fn get_silent_mg(len: usize) -> Vec<f32> {
        vec![0f32; len]
    }

    fn get_sine_mg(incr: f32, len: usize) -> Vec<f32> {
        (0..len)
            .map(|x| (x as f32 * incr * std::f32::consts::TAU).sin())
            .collect()
    }

    fn get_shared_data_from_mg<'a, 'b: 'a>(
        mg: &'a Vec<f32>,
        wheel_mg: &'b Vec<f32>,
    ) -> SharedData<'a> {
        SharedData {
            mg_data: &mg,
            wheel_data: &wheel_mg,
        }
    }

    fn dummy_params() -> ConstantBufferStates<StatesMap> {
        dummy_params_with(&[])
    }

    fn dummy_params_with(
        extra_params: &[(&str, InternalValue)],
    ) -> ConstantBufferStates<StatesMap> {
        ConstantBufferStates::new(StatesMap::from(override_synth_defaults(
            PARAMETERS.iter().cloned(),
            &HashMap::<_, _>::from_iter(
                [
                    ("dco1_width", InternalValue::Numeric(25.0)),
                    ("dco2_shape", InternalValue::Enum(Dco2Shape::Saw as u32)),
                    ("vcf_cutoff", InternalValue::Numeric(0.0)),
                    ("vcf_resonance", InternalValue::Numeric(14.2)),
                    ("vcf_tracking", InternalValue::Numeric(0.0)),
                    ("vcf_env", InternalValue::Numeric(100.0)),
                    ("attack", InternalValue::Numeric(0.01)),
                    ("decay", InternalValue::Numeric(0.1)),
                    ("sustain", InternalValue::Numeric(80.0)),
                    ("release", InternalValue::Numeric(0.2)),
                    ("vca_level", InternalValue::Numeric(100.0)),
                    ("mg_pitch", InternalValue::Numeric(100.0)),
                ]
                .into_iter()
                .chain(extra_params.iter().cloned()),
            ),
        )))
    }

    #[test]
    fn reset_basics() {
        let mut voice = Voice::new(100, 48000.0);
        let mut output = vec![0f32; 100];
        let events = vec![Event {
            sample_offset: 0,
            data: EventData::NoteOn {
                data: NoteData {
                    id: NoteID::from_pitch(60),
                    pitch: 60,
                    velocity: 1.0,
                    tuning: 0.0,
                },
            },
        }];

        let params = dummy_params();
        voice.process(
            events.iter().cloned(),
            &params,
            default_note_expression_curve(),
            get_shared_data_from_mg(&get_silent_mg(output.len()), &get_silent_mg(output.len())),
            &mut output,
        );
        voice.reset();
        let mut reset = vec![0f32; 100];
        voice.process(
            events.iter().cloned(),
            &params,
            default_note_expression_curve(),
            get_shared_data_from_mg(&get_silent_mg(output.len()), &get_silent_mg(output.len())),
            &mut reset,
        );
        for (a, b) in output.iter().zip(reset.iter()) {
            assert_approx_eq!(a, b);
        }
    }

    fn snapshot_for_data_and_params(
        data: SharedData<'_>,
        params: ConstantBufferStates<StatesMap>,
        expression: NoteExpressionCurve<impl Iterator<Item = NoteExpressionPoint> + Clone>,
    ) -> Vec<f32> {
        let num_samples = data.mg_data.len();
        let mut voice = Voice::new(num_samples, 48000.0);
        let mut output = vec![0f32; num_samples];
        let events = vec![
            Event {
                sample_offset: 0,
                data: EventData::NoteOn {
                    data: NoteData {
                        id: NoteID::from_pitch(60),
                        pitch: 60,
                        velocity: 1.0,
                        tuning: 0.0,
                    },
                },
            },
            Event {
                sample_offset: 40000,
                data: EventData::NoteOff {
                    data: NoteData {
                        id: NoteID::from_pitch(60),
                        pitch: 60,
                        velocity: 1.0,
                        tuning: 0.0,
                    },
                },
            },
        ];

        voice.process(
            events.iter().cloned(),
            &params,
            expression,
            data,
            &mut output,
        );
        output
    }

    fn snapshot_for_data(data: SharedData<'_>) -> Vec<f32> {
        snapshot_for_data_and_params(data, dummy_params(), default_note_expression_curve())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn basic_snapshot() {
        assert_snapshot!(
            "voice/basic",
            48000,
            snapshot_for_data(get_shared_data_from_mg(
                &get_silent_mg(48000),
                &get_silent_mg(48000)
            ))
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn modulated_snapshot() {
        assert_snapshot!(
            "voice/modulated",
            48000,
            snapshot_for_data(get_shared_data_from_mg(
                &get_sine_mg(4.0 / 48000.0, 48000),
                &get_silent_mg(48000)
            ))
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn wheel_snapshot() {
        assert_snapshot!(
            "voice/wheel",
            48000,
            snapshot_for_data_and_params(
                get_shared_data_from_mg(&get_silent_mg(48000), &get_sine_mg(4.0 / 48000.0, 48000)),
                dummy_params_with(&[("mod_wheel", InternalValue::Numeric(1.0))]),
                default_note_expression_curve()
            )
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn pitch_bend_snapshot() {
        assert_snapshot!(
            "voice/pitch_bend_expression",
            48000,
            snapshot_for_data_and_params(
                get_shared_data_from_mg(&get_silent_mg(48000), &get_silent_mg(48000)),
                dummy_params(),
                NoteExpressionCurve::new(
                    [
                        NoteExpressionPoint {
                            sample_offset: 0,
                            state: NoteExpressionState {
                                pitch_bend: 0f32,
                                timbre: 0f32,
                                aftertouch: 0f32,
                            },
                        },
                        NoteExpressionPoint {
                            sample_offset: 20000,
                            state: NoteExpressionState {
                                pitch_bend: 12f32,
                                timbre: 0f32,
                                aftertouch: 0f32,
                            },
                        },
                    ]
                    .into_iter()
                )
                .unwrap()
            )
        );
    }
}
