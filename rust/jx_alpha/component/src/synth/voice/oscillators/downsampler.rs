/// Polyphase 2-to-1 IIR downsampler.
///
/// This is based on a very clever structure in fred harris's book
/// "Multirate Signal Processing for Communication Systems",
/// Chapter 11. This is the nonlinear phase, 2-path version.
///
/// We use an elliptic design for a short transition band.
///
/// Honestly, as of this writing I have no idea whatsoever how to derive this from scratch,
/// but "Explicit Formulas for Lattice Wave Digital Filters" by
/// Lajos Gazsi 1985 has a design procedure for this.
///
/// Note that lattice WDF is identical to the structure from harris's book.

#[derive(Default, Debug, Clone)]
pub struct Downsampler {
    a0: f32,
    a1: f32,
    a2: f32,
    a3: f32,
    b0: f32,
    b1: f32,
    b2: f32,
}

impl Downsampler {
    pub fn process(&mut self, input: [f32; 2]) -> f32 {
        let ia0 = input[1];
        let ia1 = (ia0 - self.a1) * 0.061_861_712 + self.a0;
        let ia2 = (ia1 - self.a2) * 0.433_903_75 + self.a1;
        let ia3 = (ia2 - self.a3) * 0.878_088_06 + self.a2;
        self.a0 = ia0;
        self.a1 = ia1;
        self.a2 = ia2;
        self.a3 = ia3;
        let ib0 = input[0];
        let ib1 = (ib0 - self.b1) * 0.223_432_57 + self.b0;
        let ib2 = (ib1 - self.b2) * 0.654_140_25 + self.b1;
        self.b0 = ib0;
        self.b1 = ib1;
        self.b2 = ib2;
        f32::midpoint(ib2, ia3)
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
