use std::array;

use crate::compander::{PeakLevelDetector, compress, expand};
use crate::nonlinearity::nonlinearity;
use crate::{anti_aliasing_filter::AntiAliasingFilter, lfo, modulated_delay};
use conformal_component::audio::channels_mut;
use conformal_component::effect::{HandleParametersContext, ProcessContext};
use conformal_component::{
    ProcessingEnvironment, Processor,
    audio::{Buffer, BufferMut, ChannelLayout},
    effect::Effect as EffectT,
    pgrab, pzip,
};
use dsp::iir::dc_blocker::DcBlocker;
use itertools::izip;
use num_derive::FromPrimitive;
use num_traits::{FromPrimitive, cast};
use rtsan_standalone::nonblocking;

struct DelayChannel {
    delay: modulated_delay::ModulatedDelay,

    pre_filter: AntiAliasingFilter,
    post_filter: AntiAliasingFilter,
    dc_blocker: DcBlocker,
    dc_blocker_high: DcBlocker,
    detector: PeakLevelDetector,
}

#[cfg(test)]
const DIMENSION_SAME_SIDE_PAD_DB: f32 = -7.5;
const DIMENSION_CUTOFF: f32 = 80.0;

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive)]
enum HighpassCutoffSetting {
    // DC blocking only
    Low,

    // ~80 hz, emulating some chorus designs
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive)]
enum RoutingSetting {
    Synth,
    Dimension,
    Pedal,
    Jazz,
    Ens,
    String,
    MonoEns,
    Vocoder,
    Vocoder2,
}

impl DelayChannel {
    fn new(
        lookaround: u16,
        max_delay: usize,
        sampling_rate: f32,
        max_samples_per_process_call: usize,
    ) -> Self {
        Self {
            delay: modulated_delay::ModulatedDelay::new(modulated_delay::Options {
                lookaround,
                max_delay,
                max_samples_per_process_call,
            }),
            pre_filter: AntiAliasingFilter::new(sampling_rate),
            post_filter: AntiAliasingFilter::new(sampling_rate),
            dc_blocker: DcBlocker::new(sampling_rate),
            dc_blocker_high: DcBlocker::new_with_custom_cutoff(sampling_rate, DIMENSION_CUTOFF),
            detector: PeakLevelDetector::new(sampling_rate),
        }
    }

    pub fn reset(&mut self) {
        self.delay.reset();
        self.pre_filter.reset();
        self.post_filter.reset();
        self.dc_blocker.reset();
        self.dc_blocker_high.reset();
        self.detector.reset();
    }

    pub fn process<'a>(
        &'a mut self,
        input: impl Iterator<Item = f32> + 'a,
        highpass_cutoff: HighpassCutoffSetting,
    ) -> modulated_delay::Buffer<'a, impl dsp::look_behind::SliceLike> {
        match highpass_cutoff {
            HighpassCutoffSetting::Low => self.dc_blocker_high.reset(),
            HighpassCutoffSetting::High => self.dc_blocker.reset(),
        }
        let dc_blocker = match highpass_cutoff {
            HighpassCutoffSetting::Low => &mut self.dc_blocker,
            HighpassCutoffSetting::High => &mut self.dc_blocker_high,
        };
        self.delay.process(
            self.post_filter
                .process(self.pre_filter.process(input).map(|x| {
                    let detected_level = self.detector.detect_level(x);
                    dc_blocker.process(expand(
                        nonlinearity(compress(x, detected_level)),
                        detected_level,
                    ))
                })),
        )
    }
}

const NUM_LFOS: usize = 4;
const NUM_DELAY_CHANNELS: usize = 5;
const SUM_2_SCALE: f32 = 0.707_106_77;
const SUM_3_SCALE: f32 = 0.577_350_26;
const SUM_4_SCALE: f32 = 0.5;

fn dimension_same_side_gain(pad_db: f32) -> f32 {
    SUM_2_SCALE * 10.0f32.powf(pad_db / 20.0)
}

pub struct Effect {
    lfo: [lfo::Lfo; NUM_LFOS],
    rate_to_incr_scale: f32,
    delay_floor: f32,
    delay_ceiling: f32,
    channels: [DelayChannel; NUM_DELAY_CHANNELS],
    lfo_forward: [Vec<f32>; NUM_LFOS],
    lfo_reverse: [Vec<f32>; NUM_LFOS],
    mixed: Vec<f32>,
}

impl Processor for Effect {
    #[nonblocking]
    fn set_processing(&mut self, processing: bool) {
        if !processing {
            for lfo in &mut self.lfo {
                lfo.reset();
            }
            for channel in &mut self.channels {
                channel.reset();
            }
        }
    }
}

const PERCENT_SCALE: f32 = 1. / 100.;

impl Effect {
    pub fn new(env: &ProcessingEnvironment) -> Self {
        const LOOKAROUND: u8 = 8;

        let mut min_delay = 0.00166 * env.sampling_rate;
        if min_delay < f32::from(LOOKAROUND) {
            min_delay = f32::from(LOOKAROUND);
        }
        let mut max_delay = 0.00535 * env.sampling_rate;
        if max_delay < min_delay {
            max_delay = min_delay + 1.0;
        }
        let max_delay_for_buffer = max_delay + (max_delay - min_delay) * 0.5;
        let max_delay_for_buffer_samples = cast::<f32, usize>(max_delay_for_buffer.ceil()).unwrap();
        Effect {
            lfo: array::from_fn(|_| {
                lfo::Lfo::new(lfo::Options {
                    min: min_delay,
                    max: max_delay,
                })
            }),
            rate_to_incr_scale: 1. / env.sampling_rate,
            delay_floor: f32::from(LOOKAROUND),
            delay_ceiling: cast::<usize, f32>(max_delay_for_buffer_samples).unwrap(),
            channels: array::from_fn(|_| {
                DelayChannel::new(
                    u16::from(LOOKAROUND),
                    max_delay_for_buffer_samples,
                    env.sampling_rate,
                    env.max_samples_per_process_call,
                )
            }),
            lfo_forward: array::from_fn(|_| vec![0.; env.max_samples_per_process_call]),
            lfo_reverse: array::from_fn(|_| vec![0.; env.max_samples_per_process_call]),
            mixed: vec![0.; env.max_samples_per_process_call],
        }
    }

    fn reset_unused_channels(&mut self, used_channels: usize) {
        for channel in &mut self.channels[used_channels..] {
            channel.reset();
        }
    }

    fn fill_mono_from_stereo(input: &impl Buffer, mixed: &mut [f32]) {
        for (mixed, l, r) in izip!(mixed, input.channel(0), input.channel(1)) {
            *mixed = (l + r) * 0.5;
        }
    }

    fn write_side_wet(
        input: &impl Buffer,
        output: &mut impl BufferMut,
        wet: impl Iterator<Item = f32>,
    ) {
        let mut outputs = channels_mut(output);
        let output_l = outputs.next().unwrap();
        let output_r = outputs.next().unwrap();

        for (il, ir, wet, ol, or) in
            izip!(input.channel(0), input.channel(1), wet, output_l, output_r)
        {
            *ol = il + wet;
            *or = ir - wet;
        }
    }

    fn run_lfo(&mut self, index: usize, num_frames: usize, rate: f32, depth: f32) {
        self.lfo[index].run(
            lfo::Parameters {
                incr: rate * self.rate_to_incr_scale,
                depth,
            },
            &mut self.lfo_forward[index][..num_frames],
            &mut self.lfo_reverse[index][..num_frames],
        );
    }

    fn ensemble_lfo_depths(
        routing: RoutingSetting,
        depth: f32,
        extra_depth_scale: f32,
    ) -> [f32; NUM_LFOS] {
        if routing == RoutingSetting::MonoEns {
            [
                depth,
                depth * (1.0 + (extra_depth_scale - 1.0) / 3.0),
                depth * (1.0 + (extra_depth_scale - 1.0) * 2.0 / 3.0),
                depth * extra_depth_scale,
            ]
        } else {
            [depth; NUM_LFOS]
        }
    }

    fn run_lfos_for_routing(
        &mut self,
        num_frames: usize,
        routing: RoutingSetting,
        rates: [f32; NUM_LFOS],
        depth: f32,
        extra_depth_scale: f32,
    ) {
        if routing == RoutingSetting::String {
            self.run_string_lfos(num_frames, rates[0], rates[1], depth, extra_depth_scale);
        } else if routing == RoutingSetting::Vocoder {
            self.run_lfo(0, num_frames, rates[0], depth);
            self.run_lfo(1, num_frames, rates[1], depth);
            self.run_lfo(2, num_frames, rates[2], depth * extra_depth_scale);
        } else if routing == RoutingSetting::Vocoder2 {
            self.run_lfo(0, num_frames, rates[0], depth);
            self.run_lfo(1, num_frames, rates[1], depth * extra_depth_scale);
        } else {
            self.run_lfo(0, num_frames, rates[0], depth);
            if matches!(routing, RoutingSetting::Ens | RoutingSetting::MonoEns) {
                let depths = Self::ensemble_lfo_depths(routing, depth, extra_depth_scale);
                self.run_lfo(1, num_frames, rates[1], depths[1]);
                self.run_lfo(2, num_frames, rates[2], depths[2]);
                self.run_lfo(3, num_frames, rates[3], depths[3]);
            }
        }
    }

    fn process_mono_dual(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        self.reset_unused_channels(1);
        let delay_buffer =
            self.channels[0].process(input.channel(0).iter().copied(), highpass_cutoff);
        dsp::iter::move_into(
            izip!(
                input.channel(0),
                delay_buffer.process(self.lfo_forward[0].iter().copied()),
                delay_buffer.process(self.lfo_reverse[0].iter().copied()),
                mix
            )
            .map(|(i, l, r, m)| i + (l + r) * SUM_2_SCALE * m * PERCENT_SCALE),
            output.channel_mut(0),
        );
    }

    fn process_mono_pedal(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        self.reset_unused_channels(1);
        let delay_buffer =
            self.channels[0].process(input.channel(0).iter().copied(), highpass_cutoff);
        dsp::iter::move_into(
            izip!(
                input.channel(0),
                delay_buffer.process(self.lfo_forward[0].iter().copied()),
                mix
            )
            .map(|(i, delayed, m)| i + delayed * m * PERCENT_SCALE),
            output.channel_mut(0),
        );
    }

    fn process_pedal(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        self.reset_unused_channels(2);
        let [cl, cr, ..] = &mut self.channels;
        let processed_l = cl.process(input.channel(0).iter().copied(), highpass_cutoff);
        let processed_r = cr.process(input.channel(1).iter().copied(), highpass_cutoff);
        let mut outputs = channels_mut(output);
        let output_l = outputs.next().unwrap();
        let output_r = outputs.next().unwrap();

        for (il, ir, dl, dr, ol, or, m) in izip!(
            input.channel(0),
            input.channel(1),
            processed_l.process(self.lfo_forward[0].iter().copied()),
            processed_r.process(self.lfo_forward[0].iter().copied()),
            output_l,
            output_r,
            mix
        ) {
            *ol = il + dl * m * PERCENT_SCALE;
            *or = ir + dr * m * PERCENT_SCALE;
        }
    }

    fn process_jazz(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        let mixed = izip!(input.channel(0), input.channel(1)).map(|(l, r)| (l + r) * 0.5);
        self.reset_unused_channels(1);

        let delay_buffer = self.channels[0].process(mixed, highpass_cutoff);

        Self::write_side_wet(
            input,
            output,
            izip!(
                delay_buffer.process(self.lfo_forward[0].iter().copied()),
                mix
            )
            .map(|(delayed, m)| delayed * m * PERCENT_SCALE),
        );
    }

    fn process_synth(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        let mixed = izip!(input.channel(0), input.channel(1)).map(|(l, r)| (l + r) * 0.5);
        self.reset_unused_channels(1);

        let delay_buffer = self.channels[0].process(mixed, highpass_cutoff);

        dsp::iter::move_into(
            izip!(
                input.channel(0),
                delay_buffer.process(self.lfo_forward[0].iter().copied()),
                mix.clone()
            )
            .map(|(i, l, m)| i + l * m * PERCENT_SCALE),
            output.channel_mut(0),
        );
        dsp::iter::move_into(
            izip!(
                input.channel(1),
                delay_buffer.process(self.lfo_reverse[0].iter().copied()),
                mix
            )
            .map(|(i, r, m)| i + r * m * PERCENT_SCALE),
            output.channel_mut(1),
        );
    }

    // True-stereo mode based on famous dimension effects
    fn process_dimension(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        dimension_same_side_gain: f32,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        self.reset_unused_channels(2);
        let [cl, cr, ..] = &mut self.channels;
        let processed_l = cl.process(input.channel(0).iter().copied(), highpass_cutoff);
        let processed_r = cr.process(input.channel(1).iter().copied(), highpass_cutoff);
        let mut outputs = channels_mut(output);
        let output_l = outputs.next().unwrap();
        let output_r = outputs.next().unwrap();

        for (il, ir, dl, dr, ol, or, m) in izip!(
            input.channel(0),
            input.channel(1),
            processed_l.process(self.lfo_forward[0].iter().copied()),
            processed_r.process(self.lfo_reverse[0].iter().copied()),
            output_l,
            output_r,
            mix
        ) {
            let wet_scale = m * PERCENT_SCALE;
            *ol = (dl * dimension_same_side_gain + dr * SUM_2_SCALE) * wet_scale + il;
            *or = (dr * dimension_same_side_gain + dl * SUM_2_SCALE) * wet_scale + ir;
        }
    }

    fn process_mono_ens(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
        extra_depth_scale: f32,
    ) {
        let [c0, c1, c2, c3, ..] = &mut self.channels;
        let processed_0 = c0.process(input.channel(0).iter().copied(), highpass_cutoff);
        let processed_1 = c1.process(input.channel(0).iter().copied(), highpass_cutoff);
        let processed_2 = c2.process(input.channel(0).iter().copied(), highpass_cutoff);
        let processed_3 = c3.process(input.channel(0).iter().copied(), highpass_cutoff);
        let delay_floor = self.delay_floor;
        let delay_ceiling = self.delay_ceiling;

        dsp::iter::move_into(
            izip!(
                input.channel(0),
                processed_0.process(self.lfo_forward[0].iter().copied()),
                processed_1.process(
                    izip!(
                        self.lfo_reverse[0].iter().copied(),
                        self.lfo_forward[2].iter().copied(),
                        self.lfo_reverse[2].iter().copied()
                    )
                    .map(|(delay, extra_forward, extra_reverse)| {
                        (delay + (extra_forward - extra_reverse) * 0.5 * extra_depth_scale)
                            .clamp(delay_floor, delay_ceiling)
                    })
                ),
                processed_2.process(self.lfo_forward[1].iter().copied()),
                processed_3.process(
                    izip!(
                        self.lfo_reverse[1].iter().copied(),
                        self.lfo_forward[3].iter().copied(),
                        self.lfo_reverse[3].iter().copied()
                    )
                    .map(|(delay, extra_forward, extra_reverse)| {
                        (delay + (extra_forward - extra_reverse) * 0.5 * extra_depth_scale)
                            .clamp(delay_floor, delay_ceiling)
                    })
                ),
                mix
            )
            .map(|(i, d0, d1, d2, d3, m)| {
                i + (d0 + d1 + d2 + d3) * SUM_4_SCALE * m * PERCENT_SCALE
            }),
            output.channel_mut(0),
        );
    }

    fn process_mono_forward_ens(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        let [c0, c1, c2, c3, ..] = &mut self.channels;
        let processed_0 = c0.process(input.channel(0).iter().copied(), highpass_cutoff);
        let processed_1 = c1.process(input.channel(0).iter().copied(), highpass_cutoff);
        let processed_2 = c2.process(input.channel(0).iter().copied(), highpass_cutoff);
        let processed_3 = c3.process(input.channel(0).iter().copied(), highpass_cutoff);

        dsp::iter::move_into(
            izip!(
                input.channel(0),
                processed_0.process(self.lfo_forward[0].iter().copied()),
                processed_1.process(self.lfo_forward[1].iter().copied()),
                processed_2.process(self.lfo_forward[2].iter().copied()),
                processed_3.process(self.lfo_forward[3].iter().copied()),
                mix
            )
            .map(|(i, d0, d1, d2, d3, m)| {
                i + (d0 + d1 + d2 + d3) * SUM_4_SCALE * m * PERCENT_SCALE
            }),
            output.channel_mut(0),
        );
    }

    fn process_ens(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
        extra_depth_scale: f32,
    ) {
        let [c0, c1, c2, c3, ..] = &mut self.channels;
        let processed_0 = c0.process(input.channel(0).iter().copied(), highpass_cutoff);
        let processed_1 = c1.process(input.channel(1).iter().copied(), highpass_cutoff);
        let processed_2 = c2.process(input.channel(1).iter().copied(), highpass_cutoff);
        let processed_3 = c3.process(input.channel(0).iter().copied(), highpass_cutoff);
        let delay_floor = self.delay_floor;
        let delay_ceiling = self.delay_ceiling;
        let mut outputs = channels_mut(output);
        let output_l = outputs.next().unwrap();
        let output_r = outputs.next().unwrap();

        for (il, ir, dl0, dl1, dr0, dr1, ol, or, m) in izip!(
            input.channel(0),
            input.channel(1),
            processed_0.process(self.lfo_forward[0].iter().copied()),
            processed_1.process(
                izip!(
                    self.lfo_reverse[0].iter().copied(),
                    self.lfo_forward[2].iter().copied(),
                    self.lfo_reverse[2].iter().copied()
                )
                .map(|(delay, extra_forward, extra_reverse)| {
                    (delay + (extra_forward - extra_reverse) * 0.5 * extra_depth_scale)
                        .clamp(delay_floor, delay_ceiling)
                })
            ),
            processed_2.process(self.lfo_forward[1].iter().copied()),
            processed_3.process(
                izip!(
                    self.lfo_reverse[1].iter().copied(),
                    self.lfo_forward[3].iter().copied(),
                    self.lfo_reverse[3].iter().copied()
                )
                .map(|(delay, extra_forward, extra_reverse)| {
                    (delay + (extra_forward - extra_reverse) * 0.5 * extra_depth_scale)
                        .clamp(delay_floor, delay_ceiling)
                })
            ),
            output_l,
            output_r,
            mix
        ) {
            let wet_scale = m * PERCENT_SCALE;
            *ol = il + (dl0 + dl1) * SUM_2_SCALE * wet_scale;
            *or = ir + (dr0 + dr1) * SUM_2_SCALE * wet_scale;
        }
    }

    fn process_stereo_mono_ens(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        Self::fill_mono_from_stereo(input, &mut self.mixed[..input.num_frames()]);
        let mixed = &self.mixed[..input.num_frames()];

        let [c0, c1, c2, c3, ..] = &mut self.channels;
        let processed_0 = c0.process(mixed.iter().copied(), highpass_cutoff);
        let processed_1 = c1.process(mixed.iter().copied(), highpass_cutoff);
        let processed_2 = c2.process(mixed.iter().copied(), highpass_cutoff);
        let processed_3 = c3.process(mixed.iter().copied(), highpass_cutoff);

        Self::write_side_wet(
            input,
            output,
            izip!(
                processed_0.process(self.lfo_forward[0].iter().copied()),
                processed_1.process(self.lfo_forward[1].iter().copied()),
                processed_2.process(self.lfo_forward[2].iter().copied()),
                processed_3.process(self.lfo_forward[3].iter().copied()),
                mix
            )
            .map(|(d0, d1, d2, d3, m)| (d0 + d1 + d2 + d3) * SUM_4_SCALE * m * PERCENT_SCALE),
        );
    }

    fn process_mono_vocoder(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        let num_frames = input.num_frames();
        let v_mid = &mut self.mixed[..num_frames];
        let [c0, c1, c2, c3, c4] = &mut self.channels;

        {
            let vibrato_buffer = c4.process(input.channel(0).iter().copied(), highpass_cutoff);
            dsp::iter::move_into(
                vibrato_buffer.process(self.lfo_forward[2].iter().copied()),
                &mut v_mid[..],
            );
        }

        let processed_0 = c0.process(input.channel(0).iter().copied(), highpass_cutoff);
        let processed_1 = c1.process(v_mid.iter().copied(), highpass_cutoff);
        let processed_2 = c2.process(v_mid.iter().copied(), highpass_cutoff);
        let processed_3 = c3.process(input.channel(0).iter().copied(), highpass_cutoff);

        dsp::iter::move_into(
            izip!(
                input.channel(0),
                processed_0.process(self.lfo_forward[0].iter().copied()),
                processed_1.process(self.lfo_reverse[0].iter().copied()),
                processed_2.process(self.lfo_forward[1].iter().copied()),
                processed_3.process(self.lfo_reverse[1].iter().copied()),
                mix
            )
            .map(|(i, d0, d1, d2, d3, m)| {
                let l1 = (d0 + d1) * SUM_2_SCALE;
                let r1 = (d2 + d3) * SUM_2_SCALE;
                i + (l1 + r1) * SUM_2_SCALE * m * PERCENT_SCALE
            }),
            output.channel_mut(0),
        );
    }

    fn process_vocoder(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        let num_frames = input.num_frames();
        let v_mid = &mut self.mixed[..num_frames];
        let [c0, c1, c2, c3, c4] = &mut self.channels;

        {
            let mono = izip!(input.channel(0), input.channel(1)).map(|(l, r)| (l + r) * 0.5);
            let vibrato_buffer = c4.process(mono, highpass_cutoff);
            dsp::iter::move_into(
                vibrato_buffer.process(self.lfo_forward[2].iter().copied()),
                &mut v_mid[..],
            );
        }

        let processed_0 = c0.process(
            izip!(input.channel(0), input.channel(1)).map(|(l, r)| (l + r) * 0.5),
            highpass_cutoff,
        );
        let processed_1 = c1.process(v_mid.iter().copied(), highpass_cutoff);
        let processed_2 = c2.process(v_mid.iter().copied(), highpass_cutoff);
        let processed_3 = c3.process(
            izip!(input.channel(0), input.channel(1)).map(|(l, r)| (l + r) * 0.5),
            highpass_cutoff,
        );
        let mut outputs = channels_mut(output);
        let output_l = outputs.next().unwrap();
        let output_r = outputs.next().unwrap();

        for (il, ir, l_mid, l_v_mid, r_v_mid, r_mid, ol, or, m) in izip!(
            input.channel(0),
            input.channel(1),
            processed_0.process(self.lfo_forward[0].iter().copied()),
            processed_1.process(self.lfo_reverse[0].iter().copied()),
            processed_2.process(self.lfo_forward[1].iter().copied()),
            processed_3.process(self.lfo_reverse[1].iter().copied()),
            output_l,
            output_r,
            mix
        ) {
            let wet_scale = m * PERCENT_SCALE;
            let l1 = (l_mid + l_v_mid) * SUM_2_SCALE;
            let r1 = (r_v_mid + r_mid) * SUM_2_SCALE;
            *ol = il + l1 * wet_scale;
            *or = ir + r1 * wet_scale;
        }
    }

    fn vocoder2_full_delay(
        lfo_1: f32,
        lfo_2: f32,
        center_delay: f32,
        delay_floor: f32,
        delay_ceiling: f32,
    ) -> f32 {
        (lfo_1 + lfo_2 - center_delay).clamp(delay_floor, delay_ceiling)
    }

    fn vocoder2_half_delay(
        lfo_1: f32,
        lfo_2: f32,
        center_delay: f32,
        delay_floor: f32,
        delay_ceiling: f32,
    ) -> f32 {
        (Self::vocoder2_full_delay(lfo_1, lfo_2, center_delay, delay_floor, delay_ceiling) * 0.5)
            .clamp(delay_floor, delay_ceiling)
    }

    fn process_mono_vocoder2(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        self.reset_unused_channels(2);
        let [c_forward, c_reverse, ..] = &mut self.channels;
        let processed_forward =
            c_forward.process(input.channel(0).iter().copied(), highpass_cutoff);
        let processed_reverse =
            c_reverse.process(input.channel(0).iter().copied(), highpass_cutoff);
        let center_delay = self.lfo[0].center_delay();
        let delay_floor = self.delay_floor;
        let delay_ceiling = self.delay_ceiling;

        dsp::iter::move_into(
            izip!(
                input.channel(0),
                processed_forward.process(
                    izip!(
                        self.lfo_forward[0].iter().copied(),
                        self.lfo_forward[1].iter().copied()
                    )
                    .map(move |(lfo_1, lfo_2)| Self::vocoder2_half_delay(
                        lfo_1,
                        lfo_2,
                        center_delay,
                        delay_floor,
                        delay_ceiling
                    ))
                ),
                processed_forward.process(
                    izip!(
                        self.lfo_forward[0].iter().copied(),
                        self.lfo_forward[1].iter().copied()
                    )
                    .map(move |(lfo_1, lfo_2)| Self::vocoder2_full_delay(
                        lfo_1,
                        lfo_2,
                        center_delay,
                        delay_floor,
                        delay_ceiling
                    ))
                ),
                processed_reverse.process(
                    izip!(
                        self.lfo_reverse[0].iter().copied(),
                        self.lfo_reverse[1].iter().copied()
                    )
                    .map(move |(lfo_1, lfo_2)| Self::vocoder2_half_delay(
                        lfo_1,
                        lfo_2,
                        center_delay,
                        delay_floor,
                        delay_ceiling
                    ))
                ),
                processed_reverse.process(
                    izip!(
                        self.lfo_reverse[0].iter().copied(),
                        self.lfo_reverse[1].iter().copied()
                    )
                    .map(move |(lfo_1, lfo_2)| Self::vocoder2_full_delay(
                        lfo_1,
                        lfo_2,
                        center_delay,
                        delay_floor,
                        delay_ceiling
                    ))
                ),
                mix
            )
            .map(|(i, forward_50, forward_100, reverse_50, reverse_100, m)| {
                let wet_l = (forward_50 + reverse_100) * SUM_2_SCALE;
                let wet_r = (forward_100 + reverse_50) * SUM_2_SCALE;
                let wet_mid = (wet_l + wet_r) * SUM_2_SCALE;
                i + wet_mid * m * PERCENT_SCALE
            }),
            output.channel_mut(0),
        );
    }

    fn process_vocoder2(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        Self::fill_mono_from_stereo(input, &mut self.mixed[..input.num_frames()]);
        self.reset_unused_channels(2);
        let mixed = &self.mixed[..input.num_frames()];

        let [c_forward, c_reverse, ..] = &mut self.channels;
        let processed_forward = c_forward.process(mixed.iter().copied(), highpass_cutoff);
        let processed_reverse = c_reverse.process(mixed.iter().copied(), highpass_cutoff);
        let center_delay = self.lfo[0].center_delay();
        let delay_floor = self.delay_floor;
        let delay_ceiling = self.delay_ceiling;
        let mut outputs = channels_mut(output);
        let output_l = outputs.next().unwrap();
        let output_r = outputs.next().unwrap();

        for (il, ir, forward_50, forward_100, reverse_50, reverse_100, ol, or, m) in izip!(
            input.channel(0),
            input.channel(1),
            processed_forward.process(
                izip!(
                    self.lfo_forward[0].iter().copied(),
                    self.lfo_forward[1].iter().copied()
                )
                .map(move |(lfo_1, lfo_2)| Self::vocoder2_half_delay(
                    lfo_1,
                    lfo_2,
                    center_delay,
                    delay_floor,
                    delay_ceiling
                ))
            ),
            processed_forward.process(
                izip!(
                    self.lfo_forward[0].iter().copied(),
                    self.lfo_forward[1].iter().copied()
                )
                .map(move |(lfo_1, lfo_2)| Self::vocoder2_full_delay(
                    lfo_1,
                    lfo_2,
                    center_delay,
                    delay_floor,
                    delay_ceiling
                ))
            ),
            processed_reverse.process(
                izip!(
                    self.lfo_reverse[0].iter().copied(),
                    self.lfo_reverse[1].iter().copied()
                )
                .map(move |(lfo_1, lfo_2)| Self::vocoder2_half_delay(
                    lfo_1,
                    lfo_2,
                    center_delay,
                    delay_floor,
                    delay_ceiling
                ))
            ),
            processed_reverse.process(
                izip!(
                    self.lfo_reverse[0].iter().copied(),
                    self.lfo_reverse[1].iter().copied()
                )
                .map(move |(lfo_1, lfo_2)| Self::vocoder2_full_delay(
                    lfo_1,
                    lfo_2,
                    center_delay,
                    delay_floor,
                    delay_ceiling
                ))
            ),
            output_l,
            output_r,
            mix
        ) {
            let wet_scale = m * PERCENT_SCALE;
            let wet_l = (forward_50 + reverse_100) * SUM_2_SCALE;
            let wet_r = (forward_100 + reverse_50) * SUM_2_SCALE;
            *ol = il + wet_l * wet_scale;
            *or = ir + wet_r * wet_scale;
        }
    }

    fn run_string_lfos(
        &mut self,
        num_frames: usize,
        rate: f32,
        rate_2: f32,
        depth: f32,
        extra_depth_scale: f32,
    ) {
        let center_delay = self.lfo[0].center_delay();
        let delay_floor = self.delay_floor;
        let delay_ceiling = self.delay_ceiling;
        let rate_to_incr_scale = self.rate_to_incr_scale;

        let [lfo_0, lfo_1, ..] = &mut self.lfo;
        let [forward_0, forward_1, forward_2, ..] = &mut self.lfo_forward;
        let [reverse_0, reverse_1, reverse_2, ..] = &mut self.lfo_reverse;

        let delay_0 = &mut forward_0[..num_frames];
        let delay_120 = &mut reverse_0[..num_frames];
        let delay_240 = &mut forward_1[..num_frames];
        let extra_0 = &mut reverse_1[..num_frames];
        let extra_120 = &mut forward_2[..num_frames];
        let extra_240 = &mut reverse_2[..num_frames];

        lfo_0.run_three_phase_modulation(
            lfo::Parameters {
                incr: rate * rate_to_incr_scale,
                depth,
            },
            [&mut delay_0[..], &mut delay_120[..], &mut delay_240[..]],
        );
        lfo_1.run_three_phase_modulation(
            lfo::Parameters {
                incr: rate_2 * rate_to_incr_scale,
                depth: depth * extra_depth_scale,
            },
            [&mut extra_0[..], &mut extra_120[..], &mut extra_240[..]],
        );

        for (delay, extra) in delay_0.iter_mut().zip(extra_0.iter().copied()) {
            *delay = (center_delay + *delay + extra).clamp(delay_floor, delay_ceiling);
        }
        for (delay, extra) in delay_120.iter_mut().zip(extra_120.iter().copied()) {
            *delay = (center_delay + *delay + extra).clamp(delay_floor, delay_ceiling);
        }
        for (delay, extra) in delay_240.iter_mut().zip(extra_240.iter().copied()) {
            *delay = (center_delay + *delay + extra).clamp(delay_floor, delay_ceiling);
        }
    }

    fn process_mono_string(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        self.reset_unused_channels(3);
        let [c0, c1, c2, ..] = &mut self.channels;
        let processed_0 = c0.process(input.channel(0).iter().copied(), highpass_cutoff);
        let processed_1 = c1.process(input.channel(0).iter().copied(), highpass_cutoff);
        let processed_2 = c2.process(input.channel(0).iter().copied(), highpass_cutoff);

        dsp::iter::move_into(
            izip!(
                input.channel(0),
                processed_0.process(self.lfo_forward[0].iter().copied()),
                processed_1.process(self.lfo_reverse[0].iter().copied()),
                processed_2.process(self.lfo_forward[1].iter().copied()),
                mix
            )
            .map(|(i, d0, d1, d2, m)| i + (d0 + d1 + d2) * SUM_3_SCALE * m * PERCENT_SCALE),
            output.channel_mut(0),
        );
    }

    fn process_string(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        Self::fill_mono_from_stereo(input, &mut self.mixed[..input.num_frames()]);
        self.reset_unused_channels(3);
        let mixed = &self.mixed[..input.num_frames()];

        let [c0, c1, c2, ..] = &mut self.channels;
        let processed_0 = c0.process(mixed.iter().copied(), highpass_cutoff);
        let processed_1 = c1.process(mixed.iter().copied(), highpass_cutoff);
        let processed_2 = c2.process(mixed.iter().copied(), highpass_cutoff);

        Self::write_side_wet(
            input,
            output,
            izip!(
                processed_0.process(self.lfo_forward[0].iter().copied()),
                processed_1.process(self.lfo_reverse[0].iter().copied()),
                processed_2.process(self.lfo_forward[1].iter().copied()),
                mix
            )
            .map(|(d0, d1, d2, m)| (d0 + d1 + d2) * SUM_3_SCALE * m * PERCENT_SCALE),
        );
    }
}

impl EffectT for Effect {
    #[nonblocking]
    fn handle_parameters(&mut self, _: &impl HandleParametersContext) {}

    #[nonblocking]
    fn process(
        &mut self,
        context: &impl ProcessContext,
        input: &impl Buffer,
        output: &mut impl BufferMut,
    ) {
        debug_assert_eq!(input.channel_layout(), output.channel_layout());
        debug_assert_eq!(input.num_frames(), output.num_frames());
        let parameters = context.parameters();
        let (
            rate,
            rate_2,
            rate_3,
            rate_4,
            depth,
            ens_depth,
            bypass,
            highpass_cutoff,
            routing,
            dimension_same_side_pad,
        ) = pgrab!(parameters[numeric "rate", numeric "rate_2", numeric "rate_3", numeric "rate_4", numeric "depth", numeric "ens_depth", switch "bypass", enum "highpass_cutoff", enum "routing", numeric "dimension_same_side_pad"]);
        let routing = FromPrimitive::from_u32(routing).unwrap_or(RoutingSetting::Synth);
        let mix = pzip!(parameters[numeric "mix"]).map(move |mix| if bypass { 0.0 } else { mix });
        let extra_depth_scale = ens_depth * PERCENT_SCALE;
        self.run_lfos_for_routing(
            input.num_frames(),
            routing,
            [rate, rate_2, rate_3, rate_4],
            depth,
            extra_depth_scale,
        );

        let highpass_cutoff =
            FromPrimitive::from_u32(highpass_cutoff).unwrap_or(HighpassCutoffSetting::Low);
        match input.channel_layout() {
            ChannelLayout::Mono => match routing {
                RoutingSetting::Pedal | RoutingSetting::Jazz => {
                    self.process_mono_pedal(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Synth | RoutingSetting::Dimension => {
                    self.process_mono_dual(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Ens => {
                    self.process_mono_ens(input, output, mix, highpass_cutoff, extra_depth_scale);
                }
                RoutingSetting::MonoEns => {
                    self.process_mono_forward_ens(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::String => {
                    self.process_mono_string(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Vocoder => {
                    self.process_mono_vocoder(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Vocoder2 => {
                    self.process_mono_vocoder2(input, output, mix, highpass_cutoff);
                }
            },
            ChannelLayout::Stereo => match routing {
                RoutingSetting::Synth => {
                    self.process_synth(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Dimension => {
                    let dimension_same_side_gain =
                        dimension_same_side_gain(dimension_same_side_pad);
                    self.process_dimension(
                        input,
                        output,
                        mix,
                        dimension_same_side_gain,
                        highpass_cutoff,
                    );
                }
                RoutingSetting::Pedal => {
                    self.process_pedal(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Jazz => {
                    self.process_jazz(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Ens => {
                    self.process_ens(input, output, mix, highpass_cutoff, extra_depth_scale);
                }
                RoutingSetting::MonoEns => {
                    self.process_stereo_mono_ens(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::String => {
                    self.process_string(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Vocoder => {
                    self.process_vocoder(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Vocoder2 => {
                    self.process_vocoder2(input, output, mix, highpass_cutoff);
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use conformal_component::{
        audio::BufferData,
        parameters::{
            BufferStates, ConstantBufferStates, InternalValue, StatesMap, override_defaults,
        },
    };

    struct TestProcessContext<P> {
        parameters: P,
    }

    impl<P: BufferStates> ProcessContext for TestProcessContext<&P> {
        fn parameters(&self) -> &impl BufferStates {
            self.parameters
        }
    }

    fn params_for_overrides<const N: usize>(
        overrides: [(&'static str, InternalValue); N],
    ) -> ConstantBufferStates<StatesMap> {
        let overrides = HashMap::from(overrides);
        let component = crate::Component {};
        let infos = conformal_component::Component::parameter_infos(&component);
        ConstantBufferStates::new(StatesMap::from(override_defaults(
            infos.iter().map(Into::into),
            &overrides,
        )))
    }

    fn params_for_routing(routing: RoutingSetting) -> ConstantBufferStates<StatesMap> {
        params_for_overrides([("routing", InternalValue::Enum(routing as u32))])
    }

    fn process_stereo(params: &ConstantBufferStates<StatesMap>) -> (BufferData, BufferData) {
        let num_frames = 4096;
        let sampling_rate = 48000.0;
        let left = dsp::test_utils::sine(num_frames, 440.0 / sampling_rate);
        let right = dsp::test_utils::sine(num_frames, 660.0 / sampling_rate);
        let mut input = BufferData::new(ChannelLayout::Stereo, num_frames);
        dsp::iter::move_into(left.iter().copied(), input.channel_mut(0));
        dsp::iter::move_into(right.iter().map(|x| x * 0.5), input.channel_mut(1));

        let mut output = BufferData::new(ChannelLayout::Stereo, num_frames);
        let mut effect = Effect::new(&ProcessingEnvironment {
            sampling_rate,
            max_samples_per_process_call: num_frames,
            channel_layout: ChannelLayout::Stereo,
            processing_mode: conformal_component::ProcessingMode::Realtime,
        });
        effect.set_processing(true);
        effect.process(
            &TestProcessContext { parameters: params },
            &input,
            &mut output,
        );
        (input, output)
    }

    #[test]
    fn dimension_same_side_pad_db_maps_to_gain() {
        assert!((dimension_same_side_gain(-20.0) - 0.070_710_674).abs() < 1e-6);
        assert!((dimension_same_side_gain(DIMENSION_SAME_SIDE_PAD_DB) - 0.298_184_04).abs() < 1e-6);
        assert!((dimension_same_side_gain(0.0) - SUM_2_SCALE).abs() < 1e-6);
    }

    #[test]
    fn jazz_routing_puts_wet_signal_in_side_channel() {
        let num_frames = 1024;
        let sampling_rate = 48000.0;
        let left = dsp::test_utils::sine(num_frames, 440.0 / sampling_rate);
        let right = dsp::test_utils::sine(num_frames, 660.0 / sampling_rate);
        let mut input = BufferData::new(ChannelLayout::Stereo, num_frames);
        dsp::iter::move_into(left.iter().copied(), input.channel_mut(0));
        dsp::iter::move_into(right.iter().map(|x| x * 0.5), input.channel_mut(1));

        let mut output = BufferData::new(ChannelLayout::Stereo, num_frames);
        let mut effect = Effect::new(&ProcessingEnvironment {
            sampling_rate,
            max_samples_per_process_call: num_frames,
            channel_layout: ChannelLayout::Stereo,
            processing_mode: conformal_component::ProcessingMode::Realtime,
        });
        effect.set_processing(true);
        let params = params_for_routing(RoutingSetting::Jazz);
        effect.process(
            &TestProcessContext {
                parameters: &params,
            },
            &input,
            &mut output,
        );

        let mut max_side_delta = 0.0f32;
        for (il, ir, ol, or) in izip!(
            input.channel(0),
            input.channel(1),
            output.channel(0),
            output.channel(1)
        ) {
            assert!(((ol + or) - (il + ir)).abs() < 1e-5);
            max_side_delta = max_side_delta.max(((ol - or) - (il - ir)).abs());
        }
        assert!(max_side_delta > 1e-3);
    }

    #[test]
    fn string_routing_puts_wet_signal_in_side_channel() {
        let num_frames = 1024;
        let sampling_rate = 48000.0;
        let left = dsp::test_utils::sine(num_frames, 440.0 / sampling_rate);
        let right = dsp::test_utils::sine(num_frames, 660.0 / sampling_rate);
        let mut input = BufferData::new(ChannelLayout::Stereo, num_frames);
        dsp::iter::move_into(left.iter().copied(), input.channel_mut(0));
        dsp::iter::move_into(right.iter().map(|x| x * 0.5), input.channel_mut(1));

        let mut output = BufferData::new(ChannelLayout::Stereo, num_frames);
        let mut effect = Effect::new(&ProcessingEnvironment {
            sampling_rate,
            max_samples_per_process_call: num_frames,
            channel_layout: ChannelLayout::Stereo,
            processing_mode: conformal_component::ProcessingMode::Realtime,
        });
        effect.set_processing(true);
        let params = params_for_routing(RoutingSetting::String);
        effect.process(
            &TestProcessContext {
                parameters: &params,
            },
            &input,
            &mut output,
        );

        let mut max_side_delta = 0.0f32;
        for (il, ir, ol, or) in izip!(
            input.channel(0),
            input.channel(1),
            output.channel(0),
            output.channel(1)
        ) {
            assert!(((ol + or) - (il + ir)).abs() < 1e-5);
            max_side_delta = max_side_delta.max(((ol - or) - (il - ir)).abs());
        }
        assert!(max_side_delta > 1e-3);
    }

    #[test]
    fn mono_ens_routing_puts_wet_signal_in_side_channel() {
        let params = params_for_routing(RoutingSetting::MonoEns);
        let (input, output) = process_stereo(&params);

        let mut max_side_delta = 0.0f32;
        for (il, ir, ol, or) in izip!(
            input.channel(0),
            input.channel(1),
            output.channel(0),
            output.channel(1)
        ) {
            assert!(((ol + or) - (il + ir)).abs() < 1e-5);
            max_side_delta = max_side_delta.max(((ol - or) - (il - ir)).abs());
        }
        assert!(max_side_delta > 1e-3);
    }

    #[test]
    fn ens_stereo_routing_does_not_sum_inputs_before_delay_lines() {
        let num_frames = 4096;
        let sampling_rate = 48000.0;
        let left = dsp::test_utils::sine(num_frames, 440.0 / sampling_rate);
        let mut input = BufferData::new(ChannelLayout::Stereo, num_frames);
        dsp::iter::move_into(left.iter().copied(), input.channel_mut(0));
        dsp::iter::move_into(left.iter().map(|x| -x), input.channel_mut(1));

        let mut output = BufferData::new(ChannelLayout::Stereo, num_frames);
        let mut effect = Effect::new(&ProcessingEnvironment {
            sampling_rate,
            max_samples_per_process_call: num_frames,
            channel_layout: ChannelLayout::Stereo,
            processing_mode: conformal_component::ProcessingMode::Realtime,
        });
        effect.set_processing(true);
        let params = params_for_routing(RoutingSetting::Ens);
        effect.process(
            &TestProcessContext {
                parameters: &params,
            },
            &input,
            &mut output,
        );

        let mut max_wet_delta = 0.0f32;
        for (il, ir, ol, or) in izip!(
            input.channel(0),
            input.channel(1),
            output.channel(0),
            output.channel(1)
        ) {
            max_wet_delta = max_wet_delta.max((ol - il).abs());
            max_wet_delta = max_wet_delta.max((or - ir).abs());
        }
        assert!(max_wet_delta > 1e-3);
    }

    #[test]
    fn vocoder_routing_sums_inputs_before_delay_lines() {
        let num_frames = 4096;
        let sampling_rate = 48000.0;
        let left = dsp::test_utils::sine(num_frames, 440.0 / sampling_rate);
        let mut input = BufferData::new(ChannelLayout::Stereo, num_frames);
        dsp::iter::move_into(left.iter().copied(), input.channel_mut(0));
        dsp::iter::move_into(left.iter().map(|x| -x), input.channel_mut(1));

        let mut output = BufferData::new(ChannelLayout::Stereo, num_frames);
        let mut effect = Effect::new(&ProcessingEnvironment {
            sampling_rate,
            max_samples_per_process_call: num_frames,
            channel_layout: ChannelLayout::Stereo,
            processing_mode: conformal_component::ProcessingMode::Realtime,
        });
        effect.set_processing(true);
        let params = params_for_routing(RoutingSetting::Vocoder);
        effect.process(
            &TestProcessContext {
                parameters: &params,
            },
            &input,
            &mut output,
        );

        let mut max_wet_delta = 0.0f32;
        for (il, ir, ol, or) in izip!(
            input.channel(0),
            input.channel(1),
            output.channel(0),
            output.channel(1)
        ) {
            max_wet_delta = max_wet_delta.max((ol - il).abs());
            max_wet_delta = max_wet_delta.max((or - ir).abs());
        }
        assert!(max_wet_delta < 1e-5, "{max_wet_delta}");
    }

    #[test]
    fn vocoder_routing_preserves_original_channels_at_full_mix() {
        let num_frames = 1024;
        let sampling_rate = 48000.0;
        let mut input = BufferData::new(ChannelLayout::Stereo, num_frames);
        input.channel_mut(0)[0] = 1.0;
        input.channel_mut(1)[0] = 0.25;

        let mut output = BufferData::new(ChannelLayout::Stereo, num_frames);
        let mut effect = Effect::new(&ProcessingEnvironment {
            sampling_rate,
            max_samples_per_process_call: num_frames,
            channel_layout: ChannelLayout::Stereo,
            processing_mode: conformal_component::ProcessingMode::Realtime,
        });
        effect.set_processing(true);
        let params = params_for_routing(RoutingSetting::Vocoder);
        effect.process(
            &TestProcessContext {
                parameters: &params,
            },
            &input,
            &mut output,
        );

        assert!((output.channel(0)[0] - 1.0).abs() < 1e-6);
        assert!((output.channel(1)[0] - 0.25).abs() < 1e-6);
    }

    #[test]
    fn vocoder2_routing_sums_inputs_before_delay_lines() {
        let num_frames = 4096;
        let sampling_rate = 48000.0;
        let left = dsp::test_utils::sine(num_frames, 440.0 / sampling_rate);
        let mut input = BufferData::new(ChannelLayout::Stereo, num_frames);
        dsp::iter::move_into(left.iter().copied(), input.channel_mut(0));
        dsp::iter::move_into(left.iter().map(|x| -x), input.channel_mut(1));

        let mut output = BufferData::new(ChannelLayout::Stereo, num_frames);
        let mut effect = Effect::new(&ProcessingEnvironment {
            sampling_rate,
            max_samples_per_process_call: num_frames,
            channel_layout: ChannelLayout::Stereo,
            processing_mode: conformal_component::ProcessingMode::Realtime,
        });
        effect.set_processing(true);
        let params = params_for_routing(RoutingSetting::Vocoder2);
        effect.process(
            &TestProcessContext {
                parameters: &params,
            },
            &input,
            &mut output,
        );

        let mut max_wet_delta = 0.0f32;
        for (il, ir, ol, or) in izip!(
            input.channel(0),
            input.channel(1),
            output.channel(0),
            output.channel(1)
        ) {
            max_wet_delta = max_wet_delta.max((ol - il).abs());
            max_wet_delta = max_wet_delta.max((or - ir).abs());
        }
        assert!(max_wet_delta < 1e-5, "{max_wet_delta}");
    }

    #[test]
    fn vocoder2_routing_preserves_original_channels_at_full_mix() {
        let num_frames = 1024;
        let sampling_rate = 48000.0;
        let mut input = BufferData::new(ChannelLayout::Stereo, num_frames);
        input.channel_mut(0)[0] = 1.0;
        input.channel_mut(1)[0] = 0.25;

        let mut output = BufferData::new(ChannelLayout::Stereo, num_frames);
        let mut effect = Effect::new(&ProcessingEnvironment {
            sampling_rate,
            max_samples_per_process_call: num_frames,
            channel_layout: ChannelLayout::Stereo,
            processing_mode: conformal_component::ProcessingMode::Realtime,
        });
        effect.set_processing(true);
        let params = params_for_routing(RoutingSetting::Vocoder2);
        effect.process(
            &TestProcessContext {
                parameters: &params,
            },
            &input,
            &mut output,
        );

        assert!((output.channel(0)[0] - 1.0).abs() < 1e-6);
        assert!((output.channel(1)[0] - 0.25).abs() < 1e-6);
    }

    #[test]
    fn vocoder_vibrato_depth_is_scaled_by_ens_depth() {
        let depth_0_slow_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::Vocoder as u32),
            ),
            ("ens_depth", InternalValue::Numeric(0.0)),
            ("rate_3", InternalValue::Numeric(0.35)),
        ]);
        let depth_0_fast_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::Vocoder as u32),
            ),
            ("ens_depth", InternalValue::Numeric(0.0)),
            ("rate_3", InternalValue::Numeric(2.1)),
        ]);
        let depth_100_slow_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::Vocoder as u32),
            ),
            ("ens_depth", InternalValue::Numeric(100.0)),
            ("rate_3", InternalValue::Numeric(0.35)),
        ]);
        let depth_100_fast_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::Vocoder as u32),
            ),
            ("ens_depth", InternalValue::Numeric(100.0)),
            ("rate_3", InternalValue::Numeric(2.1)),
        ]);

        let (_, depth_0_slow) = process_stereo(&depth_0_slow_params);
        let (_, depth_0_fast) = process_stereo(&depth_0_fast_params);
        let (_, depth_100_slow) = process_stereo(&depth_100_slow_params);
        let (_, depth_100_fast) = process_stereo(&depth_100_fast_params);

        let mut max_depth_0_delta = 0.0f32;
        let mut max_depth_100_delta = 0.0f32;
        for (depth_0_slow_l, depth_0_fast_l, depth_100_slow_l, depth_100_fast_l) in izip!(
            depth_0_slow.channel(0),
            depth_0_fast.channel(0),
            depth_100_slow.channel(0),
            depth_100_fast.channel(0)
        ) {
            max_depth_0_delta = max_depth_0_delta.max((depth_0_slow_l - depth_0_fast_l).abs());
            max_depth_100_delta =
                max_depth_100_delta.max((depth_100_slow_l - depth_100_fast_l).abs());
        }

        assert!(max_depth_0_delta < 1e-6);
        assert!(max_depth_100_delta > 1e-3);
    }

    #[test]
    fn vocoder2_second_lfo_depth_is_scaled_by_ens_depth() {
        let depth_0_slow_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::Vocoder2 as u32),
            ),
            ("ens_depth", InternalValue::Numeric(0.0)),
            ("rate_2", InternalValue::Numeric(0.35)),
        ]);
        let depth_0_fast_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::Vocoder2 as u32),
            ),
            ("ens_depth", InternalValue::Numeric(0.0)),
            ("rate_2", InternalValue::Numeric(2.1)),
        ]);
        let depth_100_slow_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::Vocoder2 as u32),
            ),
            ("ens_depth", InternalValue::Numeric(100.0)),
            ("rate_2", InternalValue::Numeric(0.35)),
        ]);
        let depth_100_fast_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::Vocoder2 as u32),
            ),
            ("ens_depth", InternalValue::Numeric(100.0)),
            ("rate_2", InternalValue::Numeric(2.1)),
        ]);

        let (_, depth_0_slow) = process_stereo(&depth_0_slow_params);
        let (_, depth_0_fast) = process_stereo(&depth_0_fast_params);
        let (_, depth_100_slow) = process_stereo(&depth_100_slow_params);
        let (_, depth_100_fast) = process_stereo(&depth_100_fast_params);

        let mut max_depth_0_delta = 0.0f32;
        let mut max_depth_100_delta = 0.0f32;
        for (depth_0_slow_l, depth_0_fast_l, depth_100_slow_l, depth_100_fast_l) in izip!(
            depth_0_slow.channel(0),
            depth_0_fast.channel(0),
            depth_100_slow.channel(0),
            depth_100_fast.channel(0)
        ) {
            max_depth_0_delta = max_depth_0_delta.max((depth_0_slow_l - depth_0_fast_l).abs());
            max_depth_100_delta =
                max_depth_100_delta.max((depth_100_slow_l - depth_100_fast_l).abs());
        }

        assert!(max_depth_0_delta < 1e-6);
        assert!(max_depth_100_delta > 1e-3);
    }

    #[test]
    fn ens_zero_extra_depth_second_rate_controls_right_channel_lfo() {
        let slow_params = params_for_overrides([
            ("routing", InternalValue::Enum(RoutingSetting::Ens as u32)),
            ("ens_depth", InternalValue::Numeric(0.0)),
            ("rate_2", InternalValue::Numeric(0.35)),
        ]);
        let fast_params = params_for_overrides([
            ("routing", InternalValue::Enum(RoutingSetting::Ens as u32)),
            ("ens_depth", InternalValue::Numeric(0.0)),
            ("rate_2", InternalValue::Numeric(2.1)),
        ]);

        let (_, slow) = process_stereo(&slow_params);
        let (_, fast) = process_stereo(&fast_params);

        let mut max_left_delta = 0.0f32;
        let mut max_right_delta = 0.0f32;
        for (slow_l, fast_l, slow_r, fast_r) in izip!(
            slow.channel(0),
            fast.channel(0),
            slow.channel(1),
            fast.channel(1)
        ) {
            max_left_delta = max_left_delta.max((slow_l - fast_l).abs());
            max_right_delta = max_right_delta.max((slow_r - fast_r).abs());
        }
        assert!(max_left_delta < 1e-6);
        assert!(max_right_delta > 1e-3);
    }

    #[test]
    fn ens_extra_rates_are_scaled_by_extra_depth() {
        let depth_0_slow_params = params_for_overrides([
            ("routing", InternalValue::Enum(RoutingSetting::Ens as u32)),
            ("ens_depth", InternalValue::Numeric(0.0)),
            ("rate_3", InternalValue::Numeric(0.35)),
            ("rate_4", InternalValue::Numeric(0.35)),
        ]);
        let depth_0_fast_params = params_for_overrides([
            ("routing", InternalValue::Enum(RoutingSetting::Ens as u32)),
            ("ens_depth", InternalValue::Numeric(0.0)),
            ("rate_3", InternalValue::Numeric(2.1)),
            ("rate_4", InternalValue::Numeric(2.1)),
        ]);
        let depth_100_fast_params = params_for_overrides([
            ("routing", InternalValue::Enum(RoutingSetting::Ens as u32)),
            ("ens_depth", InternalValue::Numeric(100.0)),
            ("rate_3", InternalValue::Numeric(2.1)),
            ("rate_4", InternalValue::Numeric(2.1)),
        ]);

        let (_, depth_0_slow) = process_stereo(&depth_0_slow_params);
        let (_, depth_0_fast) = process_stereo(&depth_0_fast_params);
        let (_, depth_100_fast) = process_stereo(&depth_100_fast_params);

        let mut max_depth_0_delta = 0.0f32;
        let mut max_depth_100_delta = 0.0f32;
        for (depth_0_slow_l, depth_0_fast_l, depth_100_fast_l) in izip!(
            depth_0_slow.channel(0),
            depth_0_fast.channel(0),
            depth_100_fast.channel(0)
        ) {
            max_depth_0_delta = max_depth_0_delta.max((depth_0_slow_l - depth_0_fast_l).abs());
            max_depth_100_delta =
                max_depth_100_delta.max((depth_0_fast_l - depth_100_fast_l).abs());
        }
        assert!(max_depth_0_delta < 1e-6);
        assert!(max_depth_100_delta > 1e-3);
    }

    #[test]
    fn mono_ens_interpolates_lfo_depths_at_zero_ens_depth() {
        let base_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::MonoEns as u32),
            ),
            ("ens_depth", InternalValue::Numeric(0.0)),
            ("rate_2", InternalValue::Numeric(0.35)),
            ("rate_3", InternalValue::Numeric(0.35)),
            ("rate_4", InternalValue::Numeric(0.35)),
        ]);
        let rate_2_fast_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::MonoEns as u32),
            ),
            ("ens_depth", InternalValue::Numeric(0.0)),
            ("rate_2", InternalValue::Numeric(2.1)),
            ("rate_3", InternalValue::Numeric(0.35)),
            ("rate_4", InternalValue::Numeric(0.35)),
        ]);
        let rate_3_fast_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::MonoEns as u32),
            ),
            ("ens_depth", InternalValue::Numeric(0.0)),
            ("rate_2", InternalValue::Numeric(0.35)),
            ("rate_3", InternalValue::Numeric(2.1)),
            ("rate_4", InternalValue::Numeric(0.35)),
        ]);
        let rate_4_fast_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::MonoEns as u32),
            ),
            ("ens_depth", InternalValue::Numeric(0.0)),
            ("rate_2", InternalValue::Numeric(0.35)),
            ("rate_3", InternalValue::Numeric(0.35)),
            ("rate_4", InternalValue::Numeric(2.1)),
        ]);

        let (_, base) = process_stereo(&base_params);
        let (_, rate_2_fast) = process_stereo(&rate_2_fast_params);
        let (_, rate_3_fast) = process_stereo(&rate_3_fast_params);
        let (_, rate_4_fast) = process_stereo(&rate_4_fast_params);

        let mut max_rate_2_delta = 0.0f32;
        let mut max_rate_3_delta = 0.0f32;
        let mut max_rate_4_delta = 0.0f32;
        for (base_l, rate_2_fast_l, rate_3_fast_l, rate_4_fast_l) in izip!(
            base.channel(0),
            rate_2_fast.channel(0),
            rate_3_fast.channel(0),
            rate_4_fast.channel(0)
        ) {
            max_rate_2_delta = max_rate_2_delta.max((base_l - rate_2_fast_l).abs());
            max_rate_3_delta = max_rate_3_delta.max((base_l - rate_3_fast_l).abs());
            max_rate_4_delta = max_rate_4_delta.max((base_l - rate_4_fast_l).abs());
        }

        assert!(max_rate_2_delta > 1e-3);
        assert!(max_rate_3_delta > 1e-3);
        assert!(max_rate_4_delta < 1e-6);
    }

    #[test]
    fn mono_ens_ens_depth_controls_fourth_lfo_depth() {
        let depth_0_slow_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::MonoEns as u32),
            ),
            ("ens_depth", InternalValue::Numeric(0.0)),
            ("rate_4", InternalValue::Numeric(0.35)),
        ]);
        let depth_0_fast_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::MonoEns as u32),
            ),
            ("ens_depth", InternalValue::Numeric(0.0)),
            ("rate_4", InternalValue::Numeric(2.1)),
        ]);
        let depth_100_slow_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::MonoEns as u32),
            ),
            ("ens_depth", InternalValue::Numeric(100.0)),
            ("rate_4", InternalValue::Numeric(0.35)),
        ]);
        let depth_100_fast_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::MonoEns as u32),
            ),
            ("ens_depth", InternalValue::Numeric(100.0)),
            ("rate_4", InternalValue::Numeric(2.1)),
        ]);

        let (_, depth_0_slow) = process_stereo(&depth_0_slow_params);
        let (_, depth_0_fast) = process_stereo(&depth_0_fast_params);
        let (_, depth_100_slow) = process_stereo(&depth_100_slow_params);
        let (_, depth_100_fast) = process_stereo(&depth_100_fast_params);

        let mut max_depth_0_delta = 0.0f32;
        let mut max_depth_100_delta = 0.0f32;
        for (depth_0_slow_l, depth_0_fast_l, depth_100_slow_l, depth_100_fast_l) in izip!(
            depth_0_slow.channel(0),
            depth_0_fast.channel(0),
            depth_100_slow.channel(0),
            depth_100_fast.channel(0)
        ) {
            max_depth_0_delta = max_depth_0_delta.max((depth_0_slow_l - depth_0_fast_l).abs());
            max_depth_100_delta =
                max_depth_100_delta.max((depth_100_slow_l - depth_100_fast_l).abs());
        }

        assert!(max_depth_0_delta < 1e-6);
        assert!(max_depth_100_delta > 1e-3);
    }

    #[test]
    fn string_second_rate_is_scaled_by_extra_depth() {
        let depth_0_slow_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::String as u32),
            ),
            ("ens_depth", InternalValue::Numeric(0.0)),
            ("rate_2", InternalValue::Numeric(0.35)),
        ]);
        let depth_0_fast_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::String as u32),
            ),
            ("ens_depth", InternalValue::Numeric(0.0)),
            ("rate_2", InternalValue::Numeric(2.1)),
        ]);
        let depth_100_fast_params = params_for_overrides([
            (
                "routing",
                InternalValue::Enum(RoutingSetting::String as u32),
            ),
            ("ens_depth", InternalValue::Numeric(100.0)),
            ("rate_2", InternalValue::Numeric(2.1)),
        ]);

        let (_, depth_0_slow) = process_stereo(&depth_0_slow_params);
        let (_, depth_0_fast) = process_stereo(&depth_0_fast_params);
        let (_, depth_100_fast) = process_stereo(&depth_100_fast_params);

        let mut max_depth_0_delta = 0.0f32;
        let mut max_depth_100_delta = 0.0f32;
        for (depth_0_slow_l, depth_0_fast_l, depth_100_fast_l) in izip!(
            depth_0_slow.channel(0),
            depth_0_fast.channel(0),
            depth_100_fast.channel(0)
        ) {
            max_depth_0_delta = max_depth_0_delta.max((depth_0_slow_l - depth_0_fast_l).abs());
            max_depth_100_delta =
                max_depth_100_delta.max((depth_0_fast_l - depth_100_fast_l).abs());
        }
        assert!(max_depth_0_delta < 1e-6);
        assert!(max_depth_100_delta > 1e-3);
    }
}
