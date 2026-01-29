#[derive(Debug, Default, Clone)]
pub struct SlewLimiter {
    last_value: Option<f32>,
}

impl SlewLimiter {
    pub fn reset(&mut self) {
        self.last_value = None;
    }

    /// Moves the current value towards the target value by at most `max_delta`.
    /// If uninitialized, jumps immediately to `target`.
    pub fn process(&mut self, target: f32, max_delta: f32) -> f32 {
        let value = match self.last_value {
            Some(current) => {
                let diff = target - current;
                if diff.abs() <= max_delta {
                    target
                } else {
                    current + diff.signum() * max_delta
                }
            }
            None => target,
        };
        self.last_value = Some(value);
        value
    }
}

/// Calculates the maximum delta per sample to traverse 1.0 unit in the given time.
#[must_use]
pub const fn rate_from_time(time_seconds: f32, sample_rate: f32) -> f32 {
    if time_seconds <= 0.0 {
        f32::MAX
    } else {
        1.0 / (time_seconds * sample_rate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_slew_limiter() {
        let mut limiter = SlewLimiter::default();

        // First call should snap to target
        let out = limiter.process(10.0, 0.1);
        assert_approx_eq!(out, 10.0);

        // Next call should slew
        // Target 11.0, current 10.0, step 0.1 -> 10.1
        let out = limiter.process(11.0, 0.1);
        assert_approx_eq!(out, 10.1);

        // Reset clears state
        limiter.reset();
        // Snap again
        let out = limiter.process(5.0, 0.1);
        assert_approx_eq!(out, 5.0);
    }

    #[test]
    fn test_rate_from_time() {
        let sr = 100.0;
        let time = 1.0; // 1 second
        let rate = rate_from_time(time, sr);
        assert_approx_eq!(rate, 0.01); // 1.0 / 100 samples = 0.01 per sample
    }
}
