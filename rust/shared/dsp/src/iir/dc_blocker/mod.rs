#[cfg(test)]
mod tests;

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
