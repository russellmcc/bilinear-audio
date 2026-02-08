use super::increment;
use conformal_component::{events::NoteData, pzip, synth::NumericPerNoteExpression};
use conformal_poly::{EventData, Voice as VoiceTrait, VoiceProcessContext};
use dsp::{
    env::adsr,
    f32::{exp2_approx, rescale, rescale_clamped, rescale_points},
    slew::{self, SlewLimiter},
};
use itertools::izip;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use oscillators::OscillatorSettings;

mod env;
mod oscillators;
mod vcf;

const KEY_FOLLOW_NOMINAL_PITCH: f32 = 60.0;

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
    Env2,
    Env2Inverse,
    Env2Dynamic,
    Dynamic,
}

#[derive(FromPrimitive, Copy, Clone, Debug, PartialEq, Default)]
pub enum VcaEnvSource {
    #[default]
    Gate,
    GateDynamic,
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

    level_slew: SlewLimiter,
    cutoff_slew: SlewLimiter,
    control_slew_rate: f32,

    // Optimization opportunity: we're using a full env here but don't need all the logic.
    gate_coeffs: adsr::Coeffs,
    gate: adsr::Adsr,
}

#[derive(FromPrimitive, Copy, Clone, Debug, PartialEq, Default)]
pub enum DcoRange {
    #[default]
    Sixteen,
    Eight,
    Four,
    Two,
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
        rescale_clamped(volume, BREAK_VOLUME..=1.0, break_log_gain..=0.0).exp()
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
    exp2_approx(time_log2)
}

#[allow(clippy::cast_precision_loss)]
fn dco_adjust(dco_range: DcoRange, dco_tune: u32, dco_fine_tune: f32) -> f32 {
    (match dco_range {
        DcoRange::Sixteen => -12.0f32,
        DcoRange::Eight => 0.0f32,
        DcoRange::Four => 12.0f32,
        DcoRange::Two => 24.0f32,
    }) + rescale(dco_fine_tune, 0.0..=100.0, 0.0..=1.0)
        + (dco_tune as f32 - 12.0f32)
}

fn get_env_from_source(source: EnvSource, env1: f32, env2: f32, velocity: f32) -> f32 {
    // Linear scale with velocity for now - this hasn't been measured yet.
    match source {
        EnvSource::Env1 => env1,
        EnvSource::Env1Inverse => -env1,
        EnvSource::Env1Dynamic => env1 * velocity,
        EnvSource::Env2 => env2,
        EnvSource::Env2Inverse => -env2,
        EnvSource::Env2Dynamic => env2 * velocity,
        EnvSource::Dynamic => velocity * 0.5,
    }
}

fn get_vca_env_from_source(source: VcaEnvSource, env2: f32, gate: f32, velocity: f32) -> f32 {
    match source {
        VcaEnvSource::Env2 => env2,
        VcaEnvSource::Env2Dynamic => env2 * velocity,
        VcaEnvSource::Gate => gate,
        VcaEnvSource::GateDynamic => gate * velocity,
    }
}

fn get_dco_pwm_incr(pwm_rate: f32, sampling_rate: f32) -> f32 {
    if pwm_rate == 0.0 {
        0.0
    } else {
        increment(
            rescale(pwm_rate, 0.0..=100.0, super::LFO_NOTE_RANGE),
            sampling_rate,
        )
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

    fn new(_voice_index: usize, _max_samples_per_process_call: usize, sampling_rate: f32) -> Self {
        Self {
            pitch: 20.0,
            velocity: 0.0,
            oscillators: oscillators::Oscillators::default(),
            sampling_rate,
            vcf: vcf::Vcf::default(),
            env1: env::Env::default(),
            env2: env::Env::default(),
            level_slew: SlewLimiter::default(),
            cutoff_slew: SlewLimiter::default(),
            control_slew_rate: slew::rate_from_time(0.1, sampling_rate) * 100.0,
            gate_coeffs: adsr::calc_coeffs(
                &adsr::Params {
                    attack_time: GATE_ATTACK_TIME_SECONDS,
                    decay_time: 0.0,
                    sustain: 1.0,
                    release_time: GATE_RELEASE_TIME_SECONDS,
                },
                sampling_rate,
            ),
            gate: adsr::Adsr::default(),
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
        context: &impl VoiceProcessContext,
        shared_data: &Self::SharedData<'_>,
        output: &mut [f32],
    ) {
        let mut events = context.events().peekable();
        let per_note_pitch_bend = context.per_note_expression(NumericPerNoteExpression::PitchBend);
        let params = context.parameters();

        for (
            (index, sample),
            (
                level,
                dco1_shape_int,
                dco1_pwm_depth,
                dco1_pwm_rate,
                dco1_range_int,
                dco1_tune,
                dco1_env,
                dco1_lfo,
                dco2_shape_int,
                dco2_pwm_depth,
                dco2_pwm_rate,
                dco2_range_int,
                dco2_tune,
                dco2_fine_tune,
                dco2_env,
                dco2_lfo,
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
                vcf_lfo,
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
                expression_pitch_bend,
            ),
            lfo,
        ) in izip!(
            output.iter_mut().enumerate(),
            pzip!(params[
                numeric "level",
                enum "dco1_shape",
                numeric "dco1_pwm_depth",
                numeric "dco1_pwm_rate",
                enum "dco1_range",
                enum "dco1_tune",
                numeric "dco1_env",
                numeric "dco1_lfo",
                enum "dco2_shape",
                numeric "dco2_pwm_depth",
                numeric "dco2_pwm_rate",
                enum "dco2_range",
                enum "dco2_tune",
                numeric "dco2_fine_tune",
                numeric "dco2_env",
                numeric "dco2_lfo",
                enum "x_mod",
                enum "dco_bend_range",
                enum "dco_env_source",
                numeric "mix_dco1",
                numeric "mix_dco2",
                numeric "mix_env",
                enum "mix_env_source",
                global_expression_numeric PitchBend,
                numeric "vcf_cutoff",
                numeric "resonance",
                numeric "vcf_key",
                numeric "vcf_env",
                numeric "vcf_lfo",
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
                numeric "env2_key",
                external_numeric (per_note_pitch_bend)
            ]),
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

            let total_pitch_bend = global_pitch_bend
                * (num_traits::cast::<u32, f32>(dco_bend_range + 1).unwrap())
                + expression_pitch_bend;
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
            let osc0_env_scale = rescale(dco1_env, 0.0..=100.0, 0.0..=1.0);
            let osc0_env = dco_env * osc0_env_scale * osc0_env_scale * 32.0;
            let osc0_lfo_adjust = rescale(dco1_lfo, 0.0..=100.0, 0.0..=5.0) * lfo;

            let osc1_env_scale = rescale(dco2_env, 0.0..=100.0, 0.0..=1.0);
            let osc1_env = dco_env * osc1_env_scale * osc1_env_scale * 32.0;
            let osc1_lfo_adjust = rescale(dco2_lfo, 0.0..=100.0, 0.0..=5.0) * lfo;

            let osc0_incr = increment(
                adjusted_pitch
                    + dco_adjust(
                        FromPrimitive::from_u32(dco1_range_int).unwrap(),
                        dco1_tune,
                        0.0,
                    )
                    + osc0_env
                    + osc0_lfo_adjust,
                self.sampling_rate,
            );
            let osc1_incr = increment(
                adjusted_pitch
                    + dco_adjust(
                        FromPrimitive::from_u32(dco2_range_int).unwrap(),
                        dco2_tune,
                        dco2_fine_tune,
                    )
                    + osc1_env
                    + osc1_lfo_adjust,
                self.sampling_rate,
            );
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
                        pwm_depth: rescale(dco1_pwm_depth, 0.0..=100.0, 0.0..=1.0),
                        pwm_incr: get_dco_pwm_incr(dco1_pwm_rate, self.sampling_rate),
                    },
                    OscillatorSettings {
                        increment: osc1_incr,
                        shape: FromPrimitive::from_u32(dco2_shape_int).unwrap(),
                        gain: osc1_gain,
                        pwm_depth: rescale(dco2_pwm_depth, 0.0..=100.0, 0.0..=1.0),
                        pwm_incr: get_dco_pwm_incr(dco2_pwm_rate, self.sampling_rate),
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

            let smoothed_cutoff = self.cutoff_slew.process(vcf_cutoff, self.control_slew_rate);
            let adjusted_vcf_cutoff = rescale(smoothed_cutoff, 0.0..=100.0, 0.0..=144.0)
                + rescale_points(
                    adjusted_pitch - KEY_FOLLOW_NOMINAL_PITCH,
                    0.0,
                    1.0,
                    0.0,
                    rescale(vcf_key, 0.0..=100.0, 0.0..=1.0),
                )
                + rescale(vcf_env, 0.0..=100.0, 0.0..=120.0)
                    * get_env_from_source(
                        FromPrimitive::from_u32(vcf_env_source_int).unwrap(),
                        env1,
                        env2,
                        self.velocity,
                    )
                + rescale(vcf_lfo, 0.0..=100.0, 0.0..=84.0) * lfo;
            let cutoff_incr = increment(adjusted_vcf_cutoff, self.sampling_rate);
            let resonance = rescale(resonance, 0.0..=100.0, 0.0..=1.0);
            let vcf_settings = vcf::Settings {
                cutoff_incr,
                resonance,
            };
            let vcf_output = self.vcf.process(oscillators_output, &vcf_settings);
            let vca_volume = rescale(
                self.level_slew.process(level, self.control_slew_rate),
                0.0..=100.0,
                0.0..=1.0,
            ) * get_vca_env_from_source(
                FromPrimitive::from_u32(vca_env_source_int).unwrap(),
                env2,
                gate,
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
        self.level_slew.reset();
        self.cutoff_slew.reset();
        self.gate.reset();
    }
}
