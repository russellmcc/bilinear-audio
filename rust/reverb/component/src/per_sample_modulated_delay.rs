use crate::per_sample_delay::PerSampleDelay;

mod lfo;

/// A modulated delay line optimized to read and write one sample at a time.
#[derive(Debug, Clone)]
pub struct PerSampleModulatedDelay {
    delay: PerSampleDelay,
    lfo: lfo::Lfo,
}

impl PerSampleModulatedDelay {
    #[allow(clippy::cast_precision_loss)]
    pub fn new(delay: usize) -> Self {
        Self {
            delay: PerSampleDelay::new(delay),
            lfo: lfo::Lfo::new(),
        }
    }

    /// Here lfo depth is in delay samples, and rate is LFO cycles per samples.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    pub fn read(&mut self, depth: f32, rate: f32) -> f32 {
        let offset = self.lfo.run(depth, rate);

        // We use linear interpolation to get the fractional delay sample.
        let floor = (offset.floor() as usize).min(self.delay.get_delay() - 2);
        let a = self.delay.read_with_offset(floor);
        let b = self.delay.read_with_offset(floor + 1);

        let t = offset - floor as f32;
        a + (b - a) * t
    }

    pub fn write(&mut self, input: f32) {
        self.delay.write(input);
    }

    pub fn reset(&mut self) {
        self.delay.reset();
        self.lfo.reset();
    }
}

#[cfg(test)]
mod tests;
