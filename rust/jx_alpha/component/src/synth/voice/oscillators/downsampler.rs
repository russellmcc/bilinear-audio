/// Polyphase 2-to-1 IIR downsampler.
///
/// This is based on a very clever structure in fred harris's book
/// "Multirate Signal Processing for Communication Systems",
/// Chapter 11. This is the nonlinear phase, 2-path version.
///
/// We use a butterworth design. You can get the coefficients from a nonlinear solve,
/// setting the filtur coefficients such that each numerator coefficient is equal to
/// the coefficients of (z + 1)^n (numerator polynomial is palindromic so this works).
///
/// Or, you can use the explicit formula given in "Explicit Formulas for Lattice Wave Digital Filters" by
/// Lajos Gazsi (note that lattice WDF is identical to the structure from harris's book).
/// This formula:
///
/// ```julia
/// filter_order = 5
/// N = 2 * filter_order + 1
/// final_coeffs = [tan(pi * 2 / (4 * N) * i) ^ 2 for i in 1:filter_order]
/// ```

#[derive(Default, Debug, Clone)]
pub struct Downsampler {
    a0: f32,
    a1: f32,
    a2: f32,
    a3: f32,
    a4: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    b3: f32,
    b4: f32,
}

impl Downsampler {
    pub fn process(&mut self, input: [f32; 2]) -> f32 {
        let ia0 = input[1];
        let ia1 = (ia0 - self.a1) * 0.008_586_551 + self.a0;
        let ia2 = (ia1 - self.a2) * 0.080_954_21 + self.a1;
        let ia3 = (ia2 - self.a3) * 0.247_945_07 + self.a2;
        let ia4 = (ia3 - self.a4) * 0.570_274_1 + self.a3;
        self.a0 = ia0;
        self.a1 = ia1;
        self.a2 = ia2;
        self.a3 = ia3;
        self.a4 = ia4;
        let ib0 = input[0];
        let ib1 = (ib0 - self.b1) * 0.034_943_722 + self.b0;
        let ib2 = (ib1 - self.b2) * 0.150_080_35 + self.b1;
        let ib3 = (ib2 - self.b3) * 0.383_376_18 + self.b2;
        let ib4 = (ib3 - self.b4) * 0.831_051_8 + self.b3;
        self.b0 = ib0;
        self.b1 = ib1;
        self.b2 = ib2;
        self.b3 = ib3;
        self.b4 = ib4;
        f32::midpoint(ib4, ia4)
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use dsp::test_utils::{estimate_tuning, linear_sine_sweep};
    use snapshots::assert_snapshot;

    use super::*;

    // Only works for even signal lengths
    fn downsample(sig: &[f32]) -> Vec<f32> {
        let mut downsampler = Downsampler::default();
        sig.chunks(2)
            .map(|chunk| downsampler.process(chunk.try_into().unwrap()))
            .collect::<Vec<_>>()
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn resamples_sine_approximately_correct_tuning() {
        let incr = 18.0 / 1024.0;
        assert_approx_eq!(
            estimate_tuning(&mut downsample(&dsp::test_utils::sine(1024, incr))),
            incr * 2.0
        );
    }

    #[test]
    fn approximately_preserves_energy() {
        #![allow(clippy::cast_precision_loss)]
        let sin = dsp::test_utils::sine(1024, 18.0 / 1024.0);
        let downsampled = downsample(&sin);
        let original_energy = sin.iter().map(|x| x * x).sum::<f32>() / sin.len() as f32;
        let downsampled_energy =
            downsampled.iter().map(|x| x * x).sum::<f32>() / downsampled.len() as f32;
        assert_approx_eq!(downsampled_energy / original_energy, 1.0, 0.04);
    }

    #[test]
    fn reset() {
        let mut downsampler = Downsampler::default();
        let input = [1.0, 1.0];
        let output = downsampler.process(input);
        downsampler.reset();
        let output2 = downsampler.process(input);
        assert_approx_eq!(output, output2);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn snapshot() {
        assert_snapshot!(
            "downsampler/snapshot",
            48000,
            downsample(
                &linear_sine_sweep(48000, 96000.0, 10.0, 40_000.0)
                    .into_iter()
                    .map(|x| x * 0.5)
                    .collect::<Vec<_>>()
            )
        );
    }
}
