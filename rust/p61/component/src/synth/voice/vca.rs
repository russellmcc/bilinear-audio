//! "Vibe"-level emulation of the poly-61 VCA, which is a single-transistor.
//! This uses some pretty obscure transistor modes, ([reference](https://www.timstinchcombe.co.uk/synth/MS20_study.pdf) - section 3.1)
//!
//! We "capture the vibe" of this just by making a waveshape that is non-linear
//! in both the control and signal inputs - it's just made up.

use dsp::f32::rescale;
use dsp::iir::dc_blocker;

#[derive(Debug)]
pub struct Vca {
    dc_blocker: dc_blocker::DcBlocker,
}

impl Vca {
    pub fn new(sampling_rate: f32) -> Self {
        Self {
            dc_blocker: dc_blocker::DcBlocker::new(sampling_rate),
        }
    }

    pub fn reset(&mut self) {
        self.dc_blocker.reset();
    }

    pub fn process(&mut self, input: f32, control: f32) -> f32 {
        let y = rescale(input, -1.0..=1.0, 0.0..=1.0);
        let input_shaped = rescale(y * y, 0.0..=1.0, -1.0..=1.0);
        let input_dc_blocked = self.dc_blocker.process(input_shaped);
        input_dc_blocked * control * control
    }
}

#[cfg(test)]
mod tests {
    use dsp::{
        f32::rescale_inverted,
        test_utils::{linear_sine_sweep, white_noise},
    };

    use snapshots::assert_snapshot;

    use super::Vca;

    use assert_approx_eq::assert_approx_eq;
    use more_asserts::assert_gt;

    #[test]
    fn reset() {
        let mut vca = Vca::new(48000.0);
        let mut initial = white_noise(100);
        let mut initial_clone = initial.clone();
        for sample in initial.iter_mut() {
            *sample = vca.process(*sample, 0.5);
        }
        let processed = initial;
        vca.reset();
        for sample in initial_clone.iter_mut() {
            *sample = vca.process(*sample, 0.5);
        }
        let after_reset = initial_clone;
        for (a, b) in processed.iter().zip(after_reset.iter()) {
            assert_approx_eq!(a, b);
        }
    }

    #[test]
    fn control_signal_effects_volume() {
        let mut vca = Vca::new(48000.0);
        let mut initial = white_noise(100);
        let mut initial_clone = initial.clone();
        for sample in initial.iter_mut() {
            *sample = vca.process(*sample, 0.8);
        }
        let processed_loud = initial;
        vca.reset();
        for sample in initial_clone.iter_mut() {
            *sample = vca.process(*sample, 0.25);
        }
        let processed_quiet = initial_clone;
        let processed_loud_power = processed_loud.iter().map(|x| x * x).sum::<f32>();
        let processed_quiet_power = processed_quiet.iter().map(|x| x * x).sum::<f32>();
        let power_ratio_db = 10.0 * (processed_loud_power / processed_quiet_power).log10();
        assert_gt!(power_ratio_db, 10.0);
    }

    #[test]
    fn silent_at_zero_control() {
        let mut vca = Vca::new(48000.0);
        let mut initial = white_noise(100);
        for sample in initial.iter_mut() {
            *sample = vca.process(*sample, 0.0);
        }
        let processed = initial;
        for sample in processed.iter() {
            assert_approx_eq!(*sample, 0.0);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn snapshot() {
        let mut vca = Vca::new(48000.0);
        let mut processed = linear_sine_sweep(48000, 48000.0, 20.0, 10000.0);
        for (index, sample) in processed.iter_mut().enumerate() {
            *sample = vca.process(
                *sample,
                rescale_inverted(index as f32, 0.0..=48000.0, 0.0..=1.0),
            );
        }
        assert_snapshot!("vca/sweep", 48000, processed);
    }
}
