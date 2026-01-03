use conformal_component::{events::NoteData, parameters, pzip};
use conformal_poly::{EventData, Voice as VoiceTrait};
use dsp::f32::{rescale, rescale_clamped, rescale_points};
use itertools::izip;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use oscillators::OscillatorSettings;

mod env;
mod oscillators;
mod vcf;

const KEY_FOLLOW_NOMINAL_PITCH: f32 = 69.0;

/// Converts a MIDI pitch to a phase increment
fn increment(midi_pitch: f32, sampling_rate: f32) -> f32 {
    (440f32 * 2.0f32.powf((midi_pitch - 69f32) / 12f32) / sampling_rate).clamp(0.0, 0.45)
}

#[derive(Debug, Clone)]
pub struct SharedData<'a> {
    pub lfo: &'a [f32],
}

#[derive(FromPrimitive, Copy, Clone, Debug, PartialEq, Default)]
pub enum Dco2XMod {
    #[default]
    Off,
    Ring,
    Bit,
    Sync,
    SyncPlusRing,
}

#[derive(FromPrimitive, Copy, Clone, Debug, PartialEq, Default)]
pub enum EnvSource {
    #[default]
    Env1,
    Env1Inverse,
    Env1Dynamic,
    Env1DynamicInverse,
    Env2,
    Env2Inverse,
    Env2Dynamic,
    Env2DynamicInverse,
}

#[derive(FromPrimitive, Copy, Clone, Debug, PartialEq, Default)]
pub enum VcaEnvSource {
    #[default]
    Gate,
    GateDynamic,
    Env1,
    Env1Dynamic,
    Env2,
    Env2Dynamic,
}

#[derive(Debug)]
pub struct Voice {
    pitch: f32,
    velocity: f32,
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

/// Rescales a 0->1 "volume" to a 0->1 gain.
///
/// We got this function by measuring the main VCA response at various env levels.
/// We have not yet measured the mix VCA response but we assume it's the same for now.
fn volume_to_gain(volume: f32) -> f32 {
    const BREAK_VOLUME: f32 = 0.1;
    const BREAK_GAIN_DB: f32 = -25.0;
    let break_gain = 10.0f32.powf(BREAK_GAIN_DB / 20.0);
    let break_log_gain = break_gain.ln();

    if volume < BREAK_VOLUME {
        rescale_clamped(volume, 0.0..=BREAK_VOLUME, 0.0..=break_gain)
    } else {
        rescale_clamped(volume, BREAK_VOLUME..=0.1, break_log_gain..=0.0).exp()
    }
}

fn env_param_to_time(param: f32) -> f32 {
    // Measured time at param 0
    const MIN_TIME: f32 = 0.004f32;
    // Measured time at param 100
    const MAX_TIME: f32 = 33.141f32;

    // Note that the params are in log2 space
    let min_time_log2 = MIN_TIME.log2();
    let max_time_log2 = MAX_TIME.log2();

    let time_log2 = rescale_clamped(param, 0.0..=100.0, min_time_log2..=max_time_log2);
    time_log2.exp2()
}

fn get_env_from_source(source: EnvSource, env1: f32, env2: f32, velocity: f32) -> f32 {
    // Linear scale with velocity for now - this hasn't been measured yet.
    match source {
        EnvSource::Env1 => env1,
        EnvSource::Env1Inverse => -env1,
        EnvSource::Env1Dynamic => env1 * velocity,
        EnvSource::Env1DynamicInverse => -env1 * velocity,
        EnvSource::Env2 => env2,
        EnvSource::Env2Inverse => -env2,
        EnvSource::Env2Dynamic => env2 * velocity,
        EnvSource::Env2DynamicInverse => -env2 * velocity,
    }
}

fn get_vca_env_from_source(
    source: VcaEnvSource,
    gate: f32,
    env1: f32,
    env2: f32,
    velocity: f32,
) -> f32 {
    match source {
        VcaEnvSource::Env1 => env1,
        VcaEnvSource::Env1Dynamic => env1 * velocity,
        VcaEnvSource::Env2 => env2,
        VcaEnvSource::Env2Dynamic => env2 * velocity,
        VcaEnvSource::Gate => gate,
        VcaEnvSource::GateDynamic => gate * velocity,
    }
}

struct RawEnvParams {
    t1: f32,
    l1: f32,
    t2: f32,
    l2: f32,
    t3: f32,
    l3: f32,
    t4: f32,
    pitch: f32,
    key_follow: f32,
}

fn env_params(params: &RawEnvParams) -> env::Params {
    // We've measured the param -> time calculation, but we have not measured the key follow scaling,
    // we're inspired from "note 2" in the jx-8p manual and assuming it is correct.
    // basically at 100% key follow, we half the time every octave.
    let key_follow_shift = (params.pitch - KEY_FOLLOW_NOMINAL_PITCH) / 12.0
        * rescale(params.key_follow, 0.0..=100.0, 0.0..=1.0);
    env::Params {
        attack_time: env_param_to_time(params.t1 + key_follow_shift),
        attack_target: rescale(params.l1, 0.0..=100.0, 0.0..=1.0),
        decay_time: env_param_to_time(params.t2 + key_follow_shift),
        decay_target: rescale(params.l2, 0.0..=100.0, 0.0..=1.0),
        to_sustain_time: env_param_to_time(params.t3 + key_follow_shift),
        sustain: rescale(params.l3, 0.0..=100.0, 0.0..=1.0),
        release_time: env_param_to_time(params.t4 + key_follow_shift),
    }
}

impl VoiceTrait for Voice {
    type SharedData<'a> = SharedData<'a>;

    fn new(_max_samples_per_process_call: usize, sampling_rate: f32) -> Self {
        Self {
            pitch: 20.0,
            velocity: 0.0,
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
                data: NoteData {
                    pitch, velocity, ..
                },
            } => {
                self.pitch = f32::from(*pitch);
                self.velocity = *velocity;
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

    #[allow(clippy::too_many_lines)]
    fn process(
        &mut self,
        events: impl IntoIterator<Item = conformal_poly::Event>,
        params: &impl parameters::BufferStates,
        note_expressions: conformal_poly::NoteExpressionCurve<
            impl Iterator<Item = conformal_poly::NoteExpressionPoint> + Clone,
        >,
        shared_data: Self::SharedData<'_>,
        output: &mut [f32],
    ) {
        let mut events = events.into_iter().peekable();
        for (
            (index, sample),
            (
                level,
                dco1_shape_int,
                dco1_pwm_depth,
                dco1_pwm_rate,
                dco1_tune,
                dco1_env,
                dco2_shape_int,
                dco2_pwm_depth,
                dco2_pwm_rate,
                dco2_tune,
                dco2_env,
                x_mod_int,
                dco_bend_range,
                dco_env_source_int,
                mix_dco1,
                mix_dco2,
                mix_env,
                mix_env_source_int,
                global_pitch_bend,
                vcf_cutoff,
                resonance,
                vcf_key,
                vcf_env,
                vcf_env_source_int,
                vca_env_source_int,
                env1_t1,
                env1_l1,
                env1_t2,
                env1_l2,
                env1_t3,
                env1_l3,
                env1_t4,
                env1_key,
                env2_t1,
                env2_l1,
                env2_t2,
                env2_l2,
                env2_t3,
                env2_l3,
                env2_t4,
                env2_key,
            ),
            expression,
            _lfo,
        ) in izip!(
            output.iter_mut().enumerate(),
            pzip!(params[
                numeric "level",
                enum "dco1_shape",
                numeric "dco1_pwm_depth",
                numeric "dco1_pwm_rate",
                numeric "dco1_tune",
                numeric "dco1_env",
                enum "dco2_shape",
                numeric "dco2_pwm_depth",
                numeric "dco2_pwm_rate",
                numeric "dco2_tune",
                numeric "dco2_env",
                enum "x_mod",
                numeric "dco_bend_range",
                enum "dco_env_source",
                numeric "mix_dco1",
                numeric "mix_dco2",
                numeric "mix_env",
                enum "mix_env_source",
                numeric "pitch_bend",
                numeric "vcf_cutoff",
                numeric "resonance",
                numeric "vcf_key",
                numeric "vcf_env",
                enum "vcf_env_source",
                enum "vca_env_source",
                numeric "env1_t1",
                numeric "env1_l1",
                numeric "env1_t2",
                numeric "env1_l2",
                numeric "env1_t3",
                numeric "env1_l3",
                numeric "env1_t4",
                numeric "env1_key",
                numeric "env2_t1",
                numeric "env2_l1",
                numeric "env2_t2",
                numeric "env2_l2",
                numeric "env2_t3",
                numeric "env2_l3",
                numeric "env2_t4",
                numeric "env2_key"
            ]),
            note_expressions.iter_by_sample(),
            shared_data.lfo,
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

            let total_pitch_bend = global_pitch_bend * dco_bend_range + expression.pitch_bend;
            let adjusted_pitch = self.pitch + total_pitch_bend;

            let env1_coeffs = env::calc_coeffs(
                &env_params(&RawEnvParams {
                    t1: env1_t1,
                    l1: env1_l1,
                    t2: env1_t2,
                    l2: env1_l2,
                    t3: env1_t3,
                    l3: env1_l3,
                    t4: env1_t4,
                    pitch: adjusted_pitch,
                    key_follow: env1_key,
                }),
                self.sampling_rate,
            );
            let env1 = self.env1.process(&env1_coeffs);
            let env2_coeffs = env::calc_coeffs(
                &env_params(&RawEnvParams {
                    t1: env2_t1,
                    l1: env2_l1,
                    t2: env2_t2,
                    l2: env2_l2,
                    t3: env2_t3,
                    l3: env2_l3,
                    t4: env2_t4,
                    pitch: adjusted_pitch,
                    key_follow: env2_key,
                }),
                self.sampling_rate,
            );
            let env2 = self.env2.process(&env2_coeffs);
            let gate = self.gate.process(&self.gate_coeffs);
            let dco_env = get_env_from_source(
                FromPrimitive::from_u32(dco_env_source_int).unwrap(),
                env1,
                env2,
                self.velocity,
            );
            // Note the nonlinearity on the control scales - this is measured.
            let osc0_env_scale = rescale(dco1_env, 0.0..=100.0, 0.0..=1.0);
            let osc0_env = dco_env * osc0_env_scale * osc0_env_scale * 32.0;
            let osc1_env_scale = rescale(dco2_env, 0.0..=100.0, 0.0..=1.0);
            let osc1_env = dco_env * osc1_env_scale * osc1_env_scale * 32.0;
            let osc0_incr = increment(adjusted_pitch + dco1_tune + osc0_env, self.sampling_rate);
            let osc1_incr = increment(adjusted_pitch + dco2_tune + osc1_env, self.sampling_rate);
            let x_mod: Dco2XMod = FromPrimitive::from_u32(x_mod_int).unwrap();
            let osc0_gain = volume_to_gain(rescale(mix_dco1, 0.0..=100.0, 0.0..=1.0));
            let osc1_gain = volume_to_gain(
                rescale(mix_dco2, 0.0..=100.0, 0.0..=1.0)
                    + rescale(mix_env, 0.0..=100.0, 0.0..=1.0)
                        * get_env_from_source(
                            FromPrimitive::from_u32(mix_env_source_int).unwrap(),
                            env1,
                            env2,
                            self.velocity,
                        ),
            );
            let oscillators_output = self.oscillators.generate(&oscillators::Settings {
                oscillators: [
                    OscillatorSettings {
                        increment: osc0_incr,
                        shape: FromPrimitive::from_u32(dco1_shape_int).unwrap(),
                        gain: osc0_gain,
                        pwm_depth: dco1_pwm_depth,
                        pwm_incr: dco1_pwm_rate / self.sampling_rate,
                    },
                    OscillatorSettings {
                        increment: osc1_incr,
                        shape: FromPrimitive::from_u32(dco2_shape_int).unwrap(),
                        gain: osc1_gain,
                        pwm_depth: dco2_pwm_depth,
                        pwm_incr: dco2_pwm_rate / self.sampling_rate,
                    },
                ],
                x_mod: match x_mod {
                    Dco2XMod::Bit => oscillators::CrossModulation::And,
                    Dco2XMod::Off | Dco2XMod::Sync => oscillators::CrossModulation::Off,
                    Dco2XMod::Ring | Dco2XMod::SyncPlusRing => oscillators::CrossModulation::Ring,
                },
                sync: match x_mod {
                    Dco2XMod::Sync | Dco2XMod::SyncPlusRing => oscillators::Sync::Hard,
                    _ => oscillators::Sync::Off,
                },
            });

            // Note that we have not measured the vcf key following, this is a guess
            let adjusted_vcf_cutoff = vcf_cutoff
                + rescale_points(
                    adjusted_pitch - KEY_FOLLOW_NOMINAL_PITCH,
                    0.0,
                    12.0,
                    0.0,
                    12.0 * rescale(vcf_key, 0.0..=100.0, 0.0..=1.0),
                )
                + 120.0
                    * rescale(vcf_env, 0.0..=100.0, 0.0..=1.0)
                    * get_env_from_source(
                        FromPrimitive::from_u32(vcf_env_source_int).unwrap(),
                        env1,
                        env2,
                        self.velocity,
                    );
            let cutoff_incr = increment(adjusted_vcf_cutoff, self.sampling_rate);
            let resonance = rescale(resonance, 0.0..=100.0, 0.0..=1.0);
            let vcf_settings = vcf::Settings {
                cutoff_incr,
                resonance,
            };
            let vcf_output = self.vcf.process(oscillators_output, &vcf_settings);
            let vca_volume = rescale(level, 0.0..=100.0, 0.0..=1.0)
                * get_vca_env_from_source(
                    FromPrimitive::from_u32(vca_env_source_int).unwrap(),
                    gate,
                    env1,
                    env2,
                    self.velocity,
                );

            *sample = volume_to_gain(vca_volume) * vcf_output;
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
