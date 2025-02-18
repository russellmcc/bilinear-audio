use crate::multi_channel_per_sample_delay::MultiChannelPerSampleDelay;
use crate::{diffuser::CHANNELS, per_sample_modulated_delay::PerSampleModulatedDelay};
use dsp::iir::svf::{calc_g, calc_two_r, GainInput, GainRawParams, Svf};

const UNMODULATED_CHANNELS: usize = CHANNELS - 1;
const FILTER_CHANNELS: usize = CHANNELS;

pub struct MultiChannelFeedbackLoop {
    delay: MultiChannelPerSampleDelay<UNMODULATED_CHANNELS>,
    modulated_delay: PerSampleModulatedDelay,
    shelf_g: f64,
    shelf_two_r: f64,
    shelf: [Svf; FILTER_CHANNELS],
}

const SHELF_FREQ: f32 = 2000.0;
const SHELF_Q: f64 = 0.707;

impl MultiChannelFeedbackLoop {
    pub fn new(delay: [usize; CHANNELS], sampling_rate: f32) -> Self {
        Self {
            delay: MultiChannelPerSampleDelay::new(
                delay[..UNMODULATED_CHANNELS].try_into().unwrap(),
            ),
            modulated_delay: PerSampleModulatedDelay::new(delay[UNMODULATED_CHANNELS]),
            shelf: core::array::from_fn(|_| Svf::default()),
            shelf_g: calc_g(f64::from((SHELF_FREQ / sampling_rate).min(0.45))),
            shelf_two_r: calc_two_r(SHELF_Q),
        }
    }

    /// Here "damping" is the sqrt of the damping gain.
    ///
    /// So, lower values of damping mean more damping!
    ///
    /// `modulation_depth` is in samples and controls depth of modulation. 4ms is fine.
    /// `modulation_rate` is in cycles per sample and controls speed of modulation. 6 hz is fine.
    #[allow(clippy::cast_possible_truncation)]
    pub fn process(
        &mut self,
        mut input: [f32; CHANNELS],
        feedback: f32,
        damping: f32,
        modulation_depth: f32,
        modulation_rate: f32,
    ) -> [f32; CHANNELS] {
        let delayed = {
            let delayed = self.delay.read();
            let modulated_delay = self.modulated_delay.read(modulation_depth, modulation_rate);

            let mut output = [0f32; CHANNELS];
            let (a, b) = output.split_at_mut(delayed.len());
            a.copy_from_slice(&delayed);
            b.copy_from_slice(&[modulated_delay]);

            output
        };

        let mut filtered = delayed;
        for (filter, channel) in self.shelf.iter_mut().zip(filtered.iter_mut()) {
            // We apply damping only to the last unmodulated channel
            *channel = filter
                .process_high_shelf(std::iter::once(GainInput {
                    x: f64::from(*channel),
                    params: GainRawParams {
                        g: self.shelf_g,
                        two_r: self.shelf_two_r,
                        sqrt_gain: f64::from(damping),
                    },
                }))
                .next()
                .unwrap() as f32;
        }

        // We use a householder matrix to mix the channels of the delayed output into the input
        for (i, input) in input.iter_mut().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            for (j, delayed) in filtered.iter().enumerate() {
                *input +=
                    feedback * (if i == j { 1.0 } else { 0.0 } - 2.0 / (CHANNELS as f32)) * delayed;
            }
        }
        self.delay
            .write(input[..UNMODULATED_CHANNELS].try_into().unwrap());
        self.modulated_delay.write(input[UNMODULATED_CHANNELS]);
        delayed
    }

    pub fn reset(&mut self) {
        self.delay.reset();
        self.modulated_delay.reset();
        for shelf in &mut self.shelf {
            shelf.reset();
        }
    }
}

#[cfg(test)]
mod tests;
