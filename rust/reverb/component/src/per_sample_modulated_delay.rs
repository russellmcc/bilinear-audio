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
mod tests {
    use snapshots::assert_snapshot;

    use super::*;

    #[test]
    #[allow(clippy::cast_possible_truncation)]
    #[cfg_attr(miri, ignore)]
    fn lofi_chorus() {
        const SAMPLING_RATE: f32 = 48000.0;
        const MAX_DELAY: usize = (SAMPLING_RATE * 0.005) as usize;
        const DEPTH: f32 = SAMPLING_RATE * 0.004;
        const RATE: f32 = 6.0 / SAMPLING_RATE;

        let test_sig = dsp::test_utils::sine(SAMPLING_RATE as usize, 1123. / SAMPLING_RATE);
        let mut modulated_delay = PerSampleModulatedDelay::new(MAX_DELAY);
        let output = test_sig.iter().map(|x| {
            let y = modulated_delay.read(DEPTH, RATE);
            modulated_delay.write(*x * 0.25);
            y
        });
        assert_snapshot!("modulated_delay/lofi_chorus", 48000, output);
    }
}
