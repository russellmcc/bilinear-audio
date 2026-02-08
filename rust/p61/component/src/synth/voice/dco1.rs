use dsp::{f32::exp_approx, osc_utils::polyblep2_residual};

use crate::synth::osc_utils::pulse;

#[derive(Debug, Default)]
pub struct Dco1 {
    phase: f32,
}

/// This very loosely emulates the waveshape of the DCO1 on the
/// Poly-61, which charges a capacitor with a voltage source
/// (rather than a current source, which would yield a linear ramp).
fn saw_waveshape(phase: f32, note: f32) -> f32 {
    let shaped = 1.0 - exp_approx(-phase * 10.0 * 2f32.log2());

    // At low frequencies the effect is more pronounced.
    // We emulate this by blending the unshaped phase with the
    // shaped phase depending on the note.
    let key = 1.0 - ((note - 30.0) / 50.0).clamp(0.0, 1.0);
    2.0 * (key * shaped + (1.0 - key) * phase) - 1.0
}

fn saw(phase: f32, increment: f32, note: f32) -> f32 {
    saw_waveshape(phase, note) - polyblep2_residual(phase, increment)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Saw,
    Pulse { width: f32 },
}

impl Dco1 {
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }

    /// Generates a sample of the DCO1.
    ///  - increment: The increment of the fundamental frequency.
    ///  - note: The note of the fundamental frequency (MIDI note number)
    ///  - shape: the shape.
    pub fn generate(&mut self, increment: f32, note: f32, shape: Shape) -> f32 {
        if increment > 0.5 {
            // We can't go higher than nyquist!
            return 0.0;
        }
        self.phase += increment;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        match shape {
            Shape::Saw => saw(self.phase, increment, note),
            Shape::Pulse { width } => pulse(self.phase, increment, width),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Dco1, Shape};
    use assert_approx_eq::assert_approx_eq;
    use conformal_component::audio::all_approx_eq;
    use dsp::test_utils::{estimate_aliasing_gen, estimate_tuning_gen};
    use more_asserts::assert_lt;
    use snapshots::assert_snapshot;

    #[test]
    #[cfg_attr(miri, ignore)]
    fn saw_tuning() {
        let increment = 482.5 / 44100.0;
        let mut dco1 = Dco1::default();
        assert_approx_eq!(
            estimate_tuning_gen(|| dco1.generate(increment, 10.0, Shape::Saw)),
            increment,
            1e-4
        );
        assert_approx_eq!(
            estimate_tuning_gen(|| dco1.generate(increment, 100.0, Shape::Saw)),
            increment,
            1e-4
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn pulse_tuning() {
        let increment = 482.5 / 44100.0;
        let mut dco1 = Dco1::default();
        assert_approx_eq!(
            estimate_tuning_gen(|| dco1.generate(increment, 10.0, Shape::Pulse { width: 0.25 })),
            increment,
            1e-4
        );
    }

    #[test]
    fn reset_basics() {
        let increment = 482.5 / 44100.0;
        let mut dco1 = Dco1::default();
        let initial = std::iter::repeat_with(|| dco1.generate(increment, 10.0, Shape::Saw))
            .take(100)
            .collect::<Vec<_>>();
        dco1.reset();
        let reset = std::iter::repeat_with(|| dco1.generate(increment, 10.0, Shape::Saw))
            .take(100)
            .collect::<Vec<_>>();
        assert!(all_approx_eq(initial, reset, 2e-6));
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn saw_aliasing_suppression() {
        let increment = 0.246246246;
        let mut dco1 = Dco1::default();
        assert_lt!(
            estimate_aliasing_gen(|| dco1.generate(increment, 10.0, Shape::Saw), increment),
            -13.0
        );
        assert_lt!(
            estimate_aliasing_gen(|| dco1.generate(increment, 100.0, Shape::Saw), increment),
            -13.0
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn pulse_aliasing_suppression() {
        let increment = 0.246246246;
        let mut dco1 = Dco1::default();
        assert_lt!(
            estimate_aliasing_gen(
                || dco1.generate(increment, 10.0, Shape::Pulse { width: 0.25 }),
                increment
            ),
            -13.0
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn saw_sweep_snapshot() {
        let mut dco1 = Dco1::default();
        let max_increment = 0.1;
        let num_samples = 48000;

        assert_snapshot!(
            "dco1/saw_sweep",
            48000,
            (0..num_samples).map(|i| {
                dco1.generate(
                    i as f32 / num_samples as f32 * max_increment,
                    10.0 + i as f32 / num_samples as f32 * 100.0,
                    Shape::Saw,
                )
            })
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn pulse_sweep_snapshot() {
        let mut dco1 = Dco1::default();
        let max_increment = 0.1;
        let num_samples = 48000;

        assert_snapshot!(
            "dco1/pulse_sweep",
            48000,
            (0..num_samples).map(|i| {
                dco1.generate(
                    i as f32 / num_samples as f32 * max_increment,
                    10.0 + i as f32 / num_samples as f32 * 100.0,
                    Shape::Pulse { width: 0.25 },
                )
            })
        );
    }
}
