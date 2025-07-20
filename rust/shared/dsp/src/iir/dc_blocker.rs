#[derive(Debug, Clone)]
pub struct DcBlocker {
    state: f32,
    k: f32,
    coeff: f32,
}

const CUTOFF: f32 = 5.0;

impl DcBlocker {
    /// # Panics
    ///
    /// If sampling rate is less than 5 Hz.
    #[must_use]
    pub fn new(sampling_rate: f32) -> Self {
        Self::new_with_custom_cutoff(sampling_rate, CUTOFF)
    }

    /// # Panics
    ///
    /// If sampling rate is less than 10 Hz.
    #[must_use]
    pub fn new_with_custom_cutoff(sampling_rate: f32, cutoff: f32) -> Self {
        assert!(cutoff < sampling_rate / 2.0);
        let increment = cutoff / sampling_rate;

        // Note that we don't bother pre-warping here.
        // One way to think about this is we're approximating tan(x) (which would be correct)
        // with x (linear approximation around 0).
        //
        // In practice, this will add a bit of sample-rate dependent error but not in a way that
        // really matters since all we care about is removing DC
        let k = increment * std::f32::consts::PI;
        let coeff = 1.0 / (1.0 + k);
        Self {
            state: 0.0,
            k: 2.0 * k,
            coeff,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let output = (input - self.state) * self.coeff;
        self.state += self.k * output;
        output
    }

    pub fn reset(&mut self) {
        self.state = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::DcBlocker;
    use crate::test_utils::{white_noise, windowed_rfft};
    use assert_approx_eq::assert_approx_eq;
    use more_asserts::{assert_gt, assert_lt};
    use snapshots::assert_snapshot;

    #[test]
    fn reset() {
        let mut blocker = DcBlocker::new(48000.0);
        let mut initial = white_noise(100);
        let mut initial_clone = initial.clone();
        for sample in initial.iter_mut() {
            *sample = blocker.process(*sample);
        }
        let processed = initial;
        blocker.reset();
        for sample in initial_clone.iter_mut() {
            *sample = blocker.process(*sample);
        }
        let after_reset = initial_clone;
        for (a, b) in processed.iter().zip(after_reset.iter()) {
            assert_approx_eq!(a, b);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn lowers_dc() {
        let mut blocker = DcBlocker::new(48000.0);
        let mut input = white_noise(8192);
        // Add artificial DC offset
        for input in input.iter_mut() {
            *input = *input * 0.1 + 0.9;
        }
        let mut processed = input.clone();
        for sample in processed.iter_mut() {
            *sample = blocker.process(*sample);
        }
        let spectrum = windowed_rfft(&mut input);
        let processed_spectrum = windowed_rfft(&mut processed);

        // Check that it's significantly reducing power at DC
        let power_reduction_at_dc = processed_spectrum[0].norm_sqr() / spectrum[0].norm_sqr();
        assert_lt!(power_reduction_at_dc, 0.1);

        // Also, check that it didn't reduce power in the middle of the spectrum.
        let power_reduction_mid_spectrum =
            processed_spectrum[1000].norm_sqr() / spectrum[1000].norm_sqr();
        assert_gt!(power_reduction_mid_spectrum, 0.99);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn snapshot() {
        let mut blocker = DcBlocker::new(48000.0);
        let mut processed = white_noise(48000);
        for sample in &mut processed {
            *sample = blocker.process(*sample / 2.);
        }
        assert_snapshot!("dc_blocker/snapshot", 48000, processed);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn snapshot_high() {
        let mut blocker = DcBlocker::new_with_custom_cutoff(48000.0, 80.0);
        let mut processed = white_noise(48000);
        for sample in &mut processed {
            *sample = blocker.process(*sample / 2.);
        }
        assert_snapshot!("dc_blocker/snapshot_high", 48000, processed);
    }
}
