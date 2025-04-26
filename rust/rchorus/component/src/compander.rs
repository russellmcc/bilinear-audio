//! Implements the compander from [Practical Modeling of Bucket-Brigade Device Circuits](https://www.dafx.de/paper-archive/2010/DAFx10/RaffelSmith_DAFx10_P42.pdf)

#[derive(Debug, Clone)]
pub struct PeakLevelDetector {
    state: f32,
    k_fast: f32,
    k_slow: f32,
}

// Time constants in seconds for the modeled RC filter
const RC_TIME_CONSTANT: f32 = 0.010;
const FAST_TIME_CONSTANT: f32 = RC_TIME_CONSTANT / 20.0;

// Normalizing constant to avoid division by zero.
// This is related to the maximum gain of the system.
const NORMALIZING_CONSTANT: f32 = 1e-6;

impl PeakLevelDetector {
    pub fn new(sampling_rate: f32) -> Self {
        // Note that we don't bother pre-warping here.
        // One way to think about this is we're approximating tan(x) (which would be correct)
        // with x (linear approximation around 0).

        // We use separate time constants for attack and release, for justification
        // see the "Applications for compandors NE570" app not from philips.
        let k_slow = 1. / (RC_TIME_CONSTANT * sampling_rate + 1.);
        let k_fast = 1. / (FAST_TIME_CONSTANT * sampling_rate + 1.);

        Self {
            state: 1.0,
            k_fast,
            k_slow,
        }
    }

    pub fn detect_level(&mut self, input: f32) -> f32 {
        let rectified = input.abs();
        self.state += if rectified > self.state {
            self.k_fast
        } else {
            self.k_slow
        } * (rectified - self.state);
        self.state
    }

    pub fn reset(&mut self) {
        self.state = 1.0;
    }
}

pub fn compress(signal: f32, level: f32) -> f32 {
    (signal + NORMALIZING_CONSTANT) / (level + NORMALIZING_CONSTANT)
}

pub fn expand(signal: f32, level: f32) -> f32 {
    signal * level
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    #[cfg_attr(miri, ignore)]
    fn roughly_accurate_for_sine() {
        let sampling_rate = 48000.;
        let mut detector = PeakLevelDetector::new(sampling_rate);

        let test_sig = dsp::test_utils::sine(48000, 1123. / 48000.);
        let detected = test_sig
            .iter()
            .map(|x| detector.detect_level(*x))
            .last()
            .unwrap();
        assert_approx_eq!(detected, 1.0, 0.1);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn roughly_accurate_for_sine_2() {
        let sampling_rate = 48000.;
        let mut detector = PeakLevelDetector::new(sampling_rate);

        let test_sig = dsp::test_utils::sine(48000, 1123. / 48000.);
        let detected = test_sig
            .iter()
            .map(|x| detector.detect_level(*x * 0.3))
            .last()
            .unwrap();
        assert_approx_eq!(detected, 0.3, 5e-2);
    }

    #[test]
    fn reset() {
        let sampling_rate = 48000.;
        let mut detector = PeakLevelDetector::new(sampling_rate);

        let test_sig = dsp::test_utils::sine(100, 440. / 48000.);
        let detected = test_sig
            .iter()
            .map(|x| detector.detect_level(*x * 0.3))
            .collect::<Vec<_>>();
        detector.reset();
        let after_reset = test_sig
            .iter()
            .map(|x| detector.detect_level(*x * 0.3))
            .collect::<Vec<_>>();

        for (a, b) in detected.iter().zip(after_reset.iter()) {
            assert_approx_eq!(a, b);
        }
    }
}
