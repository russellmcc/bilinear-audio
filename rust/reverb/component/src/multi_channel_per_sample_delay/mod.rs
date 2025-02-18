use crate::per_sample_delay::PerSampleDelay;

/// A multi-channel delay optimized for processing sample by sample.
///
/// This is useful in feedback loops with channel mixing, since we have to operate
/// sample by sample.

#[derive(Debug, Clone)]
pub struct MultiChannelPerSampleDelay<const CHANNELS: usize> {
    delays: [PerSampleDelay; CHANNELS],
}

impl<const CHANNELS: usize> MultiChannelPerSampleDelay<CHANNELS> {
    pub fn new(delays: [usize; CHANNELS]) -> Self {
        Self {
            delays: delays.map(PerSampleDelay::new),
        }
    }

    pub fn read(&self) -> [f32; CHANNELS] {
        let mut output = [0.0; CHANNELS];
        for (delay, output) in self.delays.iter().zip(output.iter_mut()) {
            *output = delay.read();
        }
        output
    }

    pub fn write(&mut self, input: &[f32; CHANNELS]) {
        for (delay, sample) in self.delays.iter_mut().zip(input) {
            delay.write(*sample);
        }
    }

    pub fn reset(&mut self) {
        for delay in &mut self.delays {
            delay.reset();
        }
    }
}

#[cfg(test)]
mod tests;
