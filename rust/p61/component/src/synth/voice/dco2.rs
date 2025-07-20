use num_derive::FromPrimitive;

use crate::synth::osc_utils::pulse;

mod filter;

#[derive(Debug, Default)]
pub struct Dco2 {
    phase: f32,
    filter: filter::Filter,
}

#[derive(FromPrimitive, Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Saw,
    Square,
}

#[derive(FromPrimitive, Copy, Clone, Debug, PartialEq)]
pub enum Octave {
    Low,
    Medium,
    High,
}

fn saw(phase: f32, octave: Octave) -> f32 {
    let bits = match octave {
        Octave::Low => 4f32,
        Octave::Medium => 8f32,
        Octave::High => 16f32,
    };
    let crushed = (phase * (bits)).min(bits - 1.0).floor() / (bits - 1.0);
    2.0 * crushed - 1.0
}

impl Dco2 {
    pub fn reset(&mut self) {
        self.phase = 0.0;
        self.filter.reset();
    }

    /// Generates a sample of the DCO2.
    ///  - increment: The increment of the fundamental frequency.
    pub fn generate(&mut self, increment: f32, shape: Shape, octave: Octave) -> f32 {
        if increment > 0.5 {
            // We can't go higher than nyquist!
            return 0.0;
        }
        self.phase += increment;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        match shape {
            Shape::Square => pulse(self.phase, increment, 0.5),
            Shape::Saw => self.filter.process(saw(self.phase, octave), increment),
        }
    }
}

#[cfg(test)]
mod tests {
    use dsp::test_utils::{estimate_aliasing_gen, estimate_tuning_gen};

    use super::{Dco2, Octave, Shape};
    use assert_approx_eq::assert_approx_eq;
    use more_asserts::assert_lt;
    use snapshots::assert_snapshot;

    #[test]
    #[cfg_attr(miri, ignore)]
    fn tuning() {
        let increment = 482.5 / 44100.0;
        let mut dco2 = Dco2::default();
        assert_approx_eq!(
            estimate_tuning_gen(|| dco2.generate(increment, Shape::Square, Octave::Medium)),
            increment,
            1e-4
        );
    }

    #[test]
    fn reset_basics() {
        let increment = 482.5 / 44100.0;
        let mut dco2 = Dco2::default();
        let initial =
            std::iter::repeat_with(|| dco2.generate(increment, Shape::Square, Octave::Medium))
                .take(100)
                .collect::<Vec<_>>();
        dco2.reset();
        let reset =
            std::iter::repeat_with(|| dco2.generate(increment, Shape::Square, Octave::Medium))
                .take(100)
                .collect::<Vec<_>>();
        for (a, b) in initial.iter().zip(reset.iter()) {
            assert_approx_eq!(a, b);
        }
    }

    #[test]
    fn reset_saw_basics() {
        let increment = 482.5 / 44100.0;
        let mut dco2 = Dco2::default();
        let initial =
            std::iter::repeat_with(|| dco2.generate(increment, Shape::Saw, Octave::Medium))
                .take(100)
                .collect::<Vec<_>>();
        dco2.reset();
        let reset = std::iter::repeat_with(|| dco2.generate(increment, Shape::Saw, Octave::Medium))
            .take(100)
            .collect::<Vec<_>>();
        for (a, b) in initial.iter().zip(reset.iter()) {
            assert_approx_eq!(a, b);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn aliasing_suppression() {
        let increment = 0.246246246;
        let mut dco2 = Dco2::default();
        assert_lt!(
            estimate_aliasing_gen(
                || dco2.generate(increment, Shape::Square, Octave::Medium),
                increment
            ),
            -15.0
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn sweep() {
        let mut dco2 = Dco2::default();
        let max_increment = 0.1;
        let num_samples = 48000;

        assert_snapshot!(
            "dco2/sweep",
            48000,
            (0..num_samples).map(|i| {
                dco2.generate(
                    i as f32 / num_samples as f32 * max_increment,
                    Shape::Square,
                    Octave::Medium,
                )
            })
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn saw_tuning() {
        let increment = 482.5 / 44100.0;
        let mut dco2 = Dco2::default();
        assert_approx_eq!(
            estimate_tuning_gen(|| dco2.generate(increment, Shape::Saw, Octave::Low)),
            increment,
            1e-4
        );
        assert_approx_eq!(
            estimate_tuning_gen(|| dco2.generate(increment, Shape::Saw, Octave::Medium)),
            increment,
            1e-4
        );
        assert_approx_eq!(
            estimate_tuning_gen(|| dco2.generate(increment, Shape::Saw, Octave::High)),
            increment,
            1e-4
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn saw_sweep() {
        let mut dco2 = Dco2::default();
        let max_increment = 0.1;
        let num_samples = 48000;

        assert_snapshot!(
            "dco2/saw_sweep",
            48000,
            (0..num_samples).map(|i| {
                dco2.generate(
                    i as f32 / num_samples as f32 * max_increment,
                    Shape::Saw,
                    Octave::Medium,
                ) / 2.0
            })
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn aliasing_suppression_saw() {
        let increment = 0.246246246;
        let mut dco2 = Dco2::default();
        assert_lt!(
            estimate_aliasing_gen(
                || dco2.generate(increment, Shape::Saw, Octave::Medium),
                increment
            ),
            -15.0
        );
    }
}
