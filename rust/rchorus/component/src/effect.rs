use std::array;

use crate::compander::{PeakLevelDetector, compress, expand};
use crate::nonlinearity::nonlinearity;
use crate::{anti_aliasing_filter::AntiAliasingFilter, lfo, modulated_delay};
use conformal_component::{
    ProcessingEnvironment, Processor,
    audio::{Buffer, BufferMut, ChannelLayout},
    effect::Effect as EffectT,
    parameters::{self, BufferStates},
    pzip,
};
use dsp::iir::dc_blocker::DcBlocker;
use itertools::izip;
use num_traits::cast;
use rtsan_standalone::nonblocking;

struct DelayChannel {
    delay: modulated_delay::ModulatedDelay,

    pre_filter: AntiAliasingFilter,
    post_filter: AntiAliasingFilter,
    dc_blocker: DcBlocker,
    dc_blocker_high: DcBlocker,
    detector: PeakLevelDetector,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum HipassCutoffSetting {
    // DC blocking only
    Low,

    // ~80 hz, emulating some chorus designs
    High,
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
            dc_blocker_high: DcBlocker::new_with_custom_cutoff(sampling_rate, 80.0),
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
        hipass_cutoff: HipassCutoffSetting,
    ) -> modulated_delay::Buffer<'a, impl dsp::look_behind::SliceLike> {
        match hipass_cutoff {
            HipassCutoffSetting::Low => self.dc_blocker_high.reset(),
            HipassCutoffSetting::High => self.dc_blocker.reset(),
        }
        let dc_blocker = match hipass_cutoff {
            HipassCutoffSetting::Low => &mut self.dc_blocker,
            HipassCutoffSetting::High => &mut self.dc_blocker_high,
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

    fn process_mono<
        I: Buffer,
        O: BufferMut,
        F: Iterator<Item = f32>,
        R: Iterator<Item = f32>,
        M: Iterator<Item = f32> + Clone,
    >(
        &mut self,
        input: &I,
        output: &mut O,
        forward: F,
        reverse: R,
        mix: M,
    ) {
        let delay_buffer =
            self.channels[0].process(input.channel(0).iter().copied(), HipassCutoffSetting::Low);
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

    fn process_stereo<
        I: Buffer,
        O: BufferMut,
        F: Iterator<Item = f32>,
        R: Iterator<Item = f32>,
        M: Iterator<Item = f32> + Clone,
    >(
        &mut self,
        input: &I,
        output: &mut O,
        forward: F,
        reverse: R,
        mix: M,
    ) {
        let mixed = izip!(input.channel(0), input.channel(1)).map(|(l, r)| (l + r) * 0.5);
        let delay_buffer = self.channels[0].process(mixed, HipassCutoffSetting::Low);

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
}

impl EffectT for Effect {
    #[nonblocking]
    fn handle_parameters<P: parameters::States>(&mut self, _: P) {}

    #[nonblocking]
    fn process<P: BufferStates, I: Buffer, O: BufferMut>(
        &mut self,
        parameters: P,
        input: &I,
        output: &mut O,
    ) {
        debug_assert_eq!(input.channel_layout(), output.channel_layout());
        debug_assert_eq!(input.num_frames(), output.num_frames());
        let rate_to_incr_scale = self.rate_to_incr_scale;
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
        match input.channel_layout() {
            ChannelLayout::Mono => self.process_mono(input, output, forward, reverse, mix),
            ChannelLayout::Stereo => self.process_stereo(input, output, forward, reverse, mix),
        }
    }
}
