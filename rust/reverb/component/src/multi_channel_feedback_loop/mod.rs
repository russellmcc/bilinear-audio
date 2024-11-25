use crate::multi_channel_per_sample_delay::MultiChannelPerSampleDelay;

pub struct MultiChannelFeedbackLoop<const CHANNELS: usize> {
    delay: MultiChannelPerSampleDelay<CHANNELS>,
}

impl<const CHANNELS: usize> MultiChannelFeedbackLoop<CHANNELS> {
    pub fn new(delay: [usize; CHANNELS]) -> Self {
        Self {
            delay: MultiChannelPerSampleDelay::new(delay),
        }
    }

    pub fn process(&mut self, mut input: [f32; CHANNELS], feedback: f32) -> [f32; CHANNELS] {
        let delayed = self.delay.read();

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
}

#[cfg(test)]
mod tests;
