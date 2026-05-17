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

const DIMENSION_PAD: f32 = 0.3;
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
    Ens1,
    Ens2,
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
const NUM_DELAY_CHANNELS: usize = 4;

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
            .map(|(i, l, r, m)| i + (l + r) * m * PERCENT_SCALE),
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
        let mut outputs = channels_mut(output);
        let output_l = outputs.next().unwrap();
        let output_r = outputs.next().unwrap();

        for (il, ir, delayed, ol, or, m) in izip!(
            input.channel(0),
            input.channel(1),
            delay_buffer.process(self.lfo_forward[0].iter().copied()),
            output_l,
            output_r,
            mix
        ) {
            let wet = delayed * m * PERCENT_SCALE;
            *ol = il + wet;
            *or = ir - wet;
        }
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
            *ol = (dl * DIMENSION_PAD + dr * (1.0 - DIMENSION_PAD)) * m * PERCENT_SCALE + il;
            *or = (dr * DIMENSION_PAD + dl * (1.0 - DIMENSION_PAD)) * m * PERCENT_SCALE + ir;
        }
    }

    fn process_mono_ens1(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        let [c0, c1, c2, c3] = &mut self.channels;
        let processed_0 = c0.process(input.channel(0).iter().copied(), highpass_cutoff);
        let processed_1 = c1.process(input.channel(0).iter().copied(), highpass_cutoff);
        let processed_2 = c2.process(input.channel(0).iter().copied(), highpass_cutoff);
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
            .map(|(i, d0, d1, d2, d3, m)| i + (d0 + d1 + d2 + d3) * m * PERCENT_SCALE),
            output.channel_mut(0),
        );
    }

    fn process_mono_ens2(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
        extra_depth_scale: f32,
    ) {
        let [c0, c1, c2, c3] = &mut self.channels;
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
            .map(|(i, d0, d1, d2, d3, m)| i + (d0 + d1 + d2 + d3) * m * PERCENT_SCALE),
            output.channel_mut(0),
        );
    }

    fn process_ens1(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        for (mixed, l, r) in izip!(
            &mut self.mixed[..input.num_frames()],
            input.channel(0),
            input.channel(1)
        ) {
            *mixed = (l + r) * 0.5;
        }
        let mixed = &self.mixed[..input.num_frames()];

        let [c0, c1, c2, c3] = &mut self.channels;
        let processed_0 = c0.process(mixed.iter().copied(), highpass_cutoff);
        let processed_1 = c1.process(mixed.iter().copied(), highpass_cutoff);
        let processed_2 = c2.process(mixed.iter().copied(), highpass_cutoff);
        let processed_3 = c3.process(mixed.iter().copied(), highpass_cutoff);
        let mut outputs = channels_mut(output);
        let output_l = outputs.next().unwrap();
        let output_r = outputs.next().unwrap();

        for (il, ir, dl0, dl1, dr0, dr1, ol, or, m) in izip!(
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
            *ol = il + (dl0 + dl1) * wet_scale;
            *or = ir + (dr0 + dr1) * wet_scale;
        }
    }

    fn process_ens2(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
        extra_depth_scale: f32,
    ) {
        for (mixed, l, r) in izip!(
            &mut self.mixed[..input.num_frames()],
            input.channel(0),
            input.channel(1)
        ) {
            *mixed = (l + r) * 0.5;
        }
        let mixed = &self.mixed[..input.num_frames()];

        let [c0, c1, c2, c3] = &mut self.channels;
        let processed_0 = c0.process(mixed.iter().copied(), highpass_cutoff);
        let processed_1 = c1.process(mixed.iter().copied(), highpass_cutoff);
        let processed_2 = c2.process(mixed.iter().copied(), highpass_cutoff);
        let processed_3 = c3.process(mixed.iter().copied(), highpass_cutoff);
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
            *ol = il + (dl0 + dl1) * wet_scale;
            *or = ir + (dr0 + dr1) * wet_scale;
        }
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
        let rate_to_incr_scale = self.rate_to_incr_scale;
        let parameters = context.parameters();
        let (rate, rate_2, rate_3, rate_4, depth, ens_2_depth, bypass, highpass_cutoff, routing) = pgrab!(parameters[numeric "rate", numeric "rate_2", numeric "rate_3", numeric "rate_4", numeric "depth", numeric "ens_2_depth", switch "bypass", enum "highpass_cutoff", enum "routing"]);
        self.lfo[0].run(
            lfo::Parameters {
                incr: rate * rate_to_incr_scale,
                depth,
            },
            &mut self.lfo_forward[0][..input.num_frames()],
            &mut self.lfo_reverse[0][..input.num_frames()],
        );
        self.lfo[1].run(
            lfo::Parameters {
                incr: rate_2 * rate_to_incr_scale,
                depth,
            },
            &mut self.lfo_forward[1][..input.num_frames()],
            &mut self.lfo_reverse[1][..input.num_frames()],
        );
        self.lfo[2].run(
            lfo::Parameters {
                incr: rate_3 * rate_to_incr_scale,
                depth,
            },
            &mut self.lfo_forward[2][..input.num_frames()],
            &mut self.lfo_reverse[2][..input.num_frames()],
        );
        self.lfo[3].run(
            lfo::Parameters {
                incr: rate_4 * rate_to_incr_scale,
                depth,
            },
            &mut self.lfo_forward[3][..input.num_frames()],
            &mut self.lfo_reverse[3][..input.num_frames()],
        );
        let mix = pzip!(parameters[numeric "mix"]).map(move |mix| if bypass { 0.0 } else { mix });
        let extra_depth_scale = ens_2_depth * PERCENT_SCALE;

        let highpass_cutoff =
            FromPrimitive::from_u32(highpass_cutoff).unwrap_or(HighpassCutoffSetting::Low);
        let routing = FromPrimitive::from_u32(routing).unwrap_or(RoutingSetting::Synth);
        match input.channel_layout() {
            ChannelLayout::Mono => match routing {
                RoutingSetting::Pedal | RoutingSetting::Jazz => {
                    self.process_mono_pedal(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Synth | RoutingSetting::Dimension => {
                    self.process_mono_dual(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Ens1 => {
                    self.process_mono_ens1(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Ens2 => {
                    self.process_mono_ens2(input, output, mix, highpass_cutoff, extra_depth_scale);
                }
            },
            ChannelLayout::Stereo => match routing {
                RoutingSetting::Synth => {
                    self.process_synth(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Dimension => {
                    self.process_dimension(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Pedal => {
                    self.process_pedal(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Jazz => {
                    self.process_jazz(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Ens1 => {
                    self.process_ens1(input, output, mix, highpass_cutoff);
                }
                RoutingSetting::Ens2 => {
                    self.process_ens2(input, output, mix, highpass_cutoff, extra_depth_scale);
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
    fn ens1_second_rate_controls_right_channel_lfo() {
        let slow_params = params_for_overrides([
            ("routing", InternalValue::Enum(RoutingSetting::Ens1 as u32)),
            ("rate_2", InternalValue::Numeric(0.35)),
        ]);
        let fast_params = params_for_overrides([
            ("routing", InternalValue::Enum(RoutingSetting::Ens1 as u32)),
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
    fn ens2_extra_rates_are_scaled_by_extra_depth() {
        let depth_0_slow_params = params_for_overrides([
            ("routing", InternalValue::Enum(RoutingSetting::Ens2 as u32)),
            ("ens_2_depth", InternalValue::Numeric(0.0)),
            ("rate_3", InternalValue::Numeric(0.35)),
            ("rate_4", InternalValue::Numeric(0.35)),
        ]);
        let depth_0_fast_params = params_for_overrides([
            ("routing", InternalValue::Enum(RoutingSetting::Ens2 as u32)),
            ("ens_2_depth", InternalValue::Numeric(0.0)),
            ("rate_3", InternalValue::Numeric(2.1)),
            ("rate_4", InternalValue::Numeric(2.1)),
        ]);
        let depth_100_fast_params = params_for_overrides([
            ("routing", InternalValue::Enum(RoutingSetting::Ens2 as u32)),
            ("ens_2_depth", InternalValue::Numeric(100.0)),
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
}
