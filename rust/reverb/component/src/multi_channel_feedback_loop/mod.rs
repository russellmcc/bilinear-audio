use crate::multi_channel_per_sample_delay::MultiChannelPerSampleDelay;
use dsp::iir::svf::{calc_g, calc_two_r, GainInput, GainRawParams, Svf};

pub struct MultiChannelFeedbackLoop<const CHANNELS: usize> {
    delay: MultiChannelPerSampleDelay<CHANNELS>,
    shelf_g: f64,
    shelf_two_r: f64,
    shelf: Svf,
}

const SHELF_FREQ: f32 = 2000.0;
const SHELF_Q: f64 = 0.707;

fn calc_g_limited(incr: f32) -> f64 {
    if incr > 0.45 {
        calc_g(0.45)
    } else {
        calc_g(incr as f64)
    }
}

impl<const CHANNELS: usize> MultiChannelFeedbackLoop<CHANNELS> {
    pub fn new(delay: [usize; CHANNELS], sampling_rate: f32) -> Self {
        Self {
            delay: MultiChannelPerSampleDelay::new(delay),
            shelf: Svf::default(),
            shelf_g: calc_g_limited(SHELF_FREQ / sampling_rate),
            shelf_two_r: calc_two_r(SHELF_Q),
        }
    }

    /// Here "damping" is the sqrt of the damping gain.
    #[allow(clippy::cast_possible_truncation)]
    pub fn process(
        &mut self,
        mut input: [f32; CHANNELS],
        feedback: f32,
        damping: f32,
    ) -> [f32; CHANNELS] {
        let delayed = {
            let mut delayed = self.delay.read();

            // We apply damping only to the first channel
            delayed[0] = self
                .shelf
                .process_high_shelf(std::iter::once(GainInput {
                    x: f64::from(delayed[0]),
                    params: GainRawParams {
                        g: self.shelf_g,
                        two_r: self.shelf_two_r,
                        sqrt_gain: f64::from(damping),
                    },
                }))
                .next()
                .unwrap() as f32;
            delayed
        };

        // We use a householder matrix to mix the channels of the delayed output into the input
        for (i, input) in input.iter_mut().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            for (j, delayed) in delayed.iter().enumerate() {
                *input +=
                    feedback * (if i == j { 1.0 } else { 0.0 } - 2.0 / (CHANNELS as f32)) * delayed;
            }
        }
        self.delay.write(&input);

        delayed
    }

    pub fn reset(&mut self) {
        self.delay.reset();
        self.shelf.reset();
    }
}

#[cfg(test)]
mod tests;
