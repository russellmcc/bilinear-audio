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
    pzip,
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

pub struct Effect {
    lfo: lfo::Lfo,
    rate_to_incr_scale: f32,
    channels: [DelayChannel; 2],
}

impl Processor for Effect {
    #[nonblocking]
    fn set_processing(&mut self, processing: bool) {
        if !processing {
            self.lfo.reset();
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
        Effect {
            lfo: lfo::Lfo::new(lfo::Options {
                min: min_delay,
                max: max_delay,
            }),
            rate_to_incr_scale: 1. / env.sampling_rate,
            channels: array::from_fn(|_| {
                DelayChannel::new(
                    u16::from(LOOKAROUND),
                    cast::<f32, usize>(max_delay.ceil()).unwrap(),
                    env.sampling_rate,
                    env.max_samples_per_process_call,
                )
            }),
        }
    }

    fn process_mono(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        forward: impl Iterator<Item = f32>,
        reverse: impl Iterator<Item = f32>,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        let delay_buffer =
            self.channels[0].process(input.channel(0).iter().copied(), highpass_cutoff);
        dsp::iter::move_into(
            izip!(
                input.channel(0),
                delay_buffer.process(forward),
                delay_buffer.process(reverse),
                mix
            )
            .map(|(i, l, r, m)| i + (l + r) * m * PERCENT_SCALE),
            output.channel_mut(0),
        );
    }

    fn process_synth(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        forward: impl Iterator<Item = f32>,
        reverse: impl Iterator<Item = f32>,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        let mixed = izip!(input.channel(0), input.channel(1)).map(|(l, r)| (l + r) * 0.5);
        self.channels[1].reset();

        let delay_buffer = self.channels[0].process(mixed, highpass_cutoff);

        dsp::iter::move_into(
            izip!(input.channel(0), delay_buffer.process(forward), mix.clone())
                .map(|(i, l, m)| i + l * m * PERCENT_SCALE),
            output.channel_mut(0),
        );
        dsp::iter::move_into(
            izip!(input.channel(1), delay_buffer.process(reverse), mix)
                .map(|(i, r, m)| i + r * m * PERCENT_SCALE),
            output.channel_mut(1),
        );
    }

    // True-stereo mode based on famous dimension effects
    fn process_dimension(
        &mut self,
        input: &impl Buffer,
        output: &mut impl BufferMut,
        forward: impl Iterator<Item = f32>,
        reverse: impl Iterator<Item = f32>,
        mix: impl Iterator<Item = f32> + Clone,
        highpass_cutoff: HighpassCutoffSetting,
    ) {
        let [cl, cr] = &mut self.channels;
        let processed_l = cl.process(input.channel(0).iter().copied(), highpass_cutoff);
        let processed_r = cr.process(input.channel(1).iter().copied(), highpass_cutoff);
        let mut outputs = channels_mut(output);
        let output_l = outputs.next().unwrap();
        let output_r = outputs.next().unwrap();

        for (il, ir, dl, dr, ol, or, m) in izip!(
            input.channel(0),
            input.channel(1),
            processed_l.process(forward),
            processed_r.process(reverse),
            output_l,
            output_r,
            mix
        ) {
            *ol = (dl * DIMENSION_PAD + dr * (1.0 - DIMENSION_PAD)) * m * PERCENT_SCALE + il;
            *or = (dr * DIMENSION_PAD + dl * (1.0 - DIMENSION_PAD)) * m * PERCENT_SCALE + ir;
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
        let lfo::Buffer { forward, reverse } = self.lfo.run(
            pzip!(parameters[numeric "rate", numeric "depth"])
                .take(input.num_frames())
                .map(move |(rate, depth)| lfo::Parameters {
                    incr: rate * rate_to_incr_scale,
                    depth,
                }),
        );
        let mix = pzip!(parameters[numeric "mix", switch "bypass"])
            .map(|(mix, bypass)| if bypass { 0.0 } else { mix });

        // we only update the highpass cutoff per-buffer
        let (highpass_cutoff, routing) = pzip!(parameters[enum "highpass_cutoff", enum "routing"])
            .map(|(highpass_cutoff, routing)| {
                (
                    FromPrimitive::from_u32(highpass_cutoff).unwrap(),
                    FromPrimitive::from_u32(routing).unwrap(),
                )
            })
            .next()
            .unwrap_or((HighpassCutoffSetting::Low, RoutingSetting::Synth));
        match input.channel_layout() {
            ChannelLayout::Mono => {
                self.process_mono(input, output, forward, reverse, mix, highpass_cutoff);
            }
            ChannelLayout::Stereo => match routing {
                RoutingSetting::Synth => {
                    self.process_synth(input, output, forward, reverse, mix, highpass_cutoff);
                }
                RoutingSetting::Dimension => {
                    self.process_dimension(input, output, forward, reverse, mix, highpass_cutoff);
                }
            },
        }
    }
}
