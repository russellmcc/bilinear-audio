#[derive(Default, Debug, Clone)]
enum State {
    #[default]
    Off,
}

/// Emulates an imaginary cross mod based on and gates in an asic
///
/// The idea is that the alpha oscillators are implemented as digital
/// counters in an asic, and the designers of a two-osc synth might have wanted a
/// different flavor of cross mod from ring modulation. E.g., the JX-3p has a "metal"
/// cross mod mode. The metal from JX-3p is not implementable with a digital oscillator,
/// so designers would have to have gone with something different. One approach would be to
/// "and" the top bit of one oscillator with all bits ofthe other oscillator's output.
/// This is kind of similar in flavor to the "PWM-saw".
///
/// To implement this, we add BLEPs to each transition to at least address some of the
/// aliasing from the sharp discontinuities. Note that we do not handle higher-order
/// (derivative) discontinuities, so this still aliases a bunch :(.
#[derive(Default, Debug, Clone)]
pub struct And {
    state: State,
}

impl And {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn process(&mut self, input: f32, conductor_phase: f32, conductor_increment: f32) -> f32 {
        if conductor_phase < 0.5 { -1.0 } else { input }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use conformal_component::audio::all_approx_eq;
    use dsp::{f32::rescale, test_utils::white_noise};

    #[test]
    fn low_phase_stays_off() {
        let mut and = And::default();
        for _ in 0..100 {
            and.process(1.0, 0.0, 0.01);
        }
        assert_approx_eq!(and.process(1.0, 0.0, 0.01), -1.0);
    }

    #[test]
    fn high_phase_turns_on() {
        let mut and = And::default();
        for _ in 0..100 {
            and.process(1.0, 0.95, 0.01);
        }
        assert_approx_eq!(and.process(1.0, 0.95, 0.01), 1.0);
    }

    #[test]
    fn reset() {
        let mut and = And::default();
        let input = white_noise(100);
        let phase_input = white_noise(100)
            .into_iter()
            .map(|x| rescale(x, -1.0..=1.0, 0.0..=1.0))
            .collect::<Vec<_>>();
        let output = input
            .iter()
            .zip(phase_input.iter())
            .map(|(x, phase)| and.process(*x, *phase, 0.01))
            .collect::<Vec<_>>();
        and.reset();
        let output_reset = input
            .iter()
            .zip(phase_input.iter())
            .map(|(x, phase)| and.process(*x, *phase, 0.01))
            .collect::<Vec<_>>();
        assert!(all_approx_eq(output, output_reset, 1e-6));
    }
}
