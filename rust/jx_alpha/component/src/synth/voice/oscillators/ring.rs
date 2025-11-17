//! Ring modulator

#[derive(Default, Debug, Clone)]
pub struct Ring {
    last_x: f32,
    last_y: f32,
}

impl Ring {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn process(&mut self, x: f32, y: f32) -> f32 {
        // See https://www.russellmcc.com/conformal/app_notes/5-adaa-for-ring-mod/ for derivation.
        // We use continuous-time convolution here for alias suppression.
        let out: f32 = (1.0 / 6.0)
            * (2.0 * (self.last_x * self.last_y + x * y) + self.last_x * y + self.last_y * x);
        self.last_x = x;
        self.last_y = y;
        out
    }
}

#[cfg(test)]
mod tests {
    use conformal_component::audio::all_approx_eq;
    use dsp::test_utils::{linear_sine_sweep, sine, windowed_rfft};
    use more_asserts::{assert_gt, assert_lt};
    use snapshots::assert_snapshot;

    use super::*;

    fn mag_spectrum_for_rates(low_rate: f32, high_rate: f32, len: usize) -> Vec<f32> {
        let low_sig = sine(len, low_rate / len as f32);
        let high_sig = sine(len, high_rate / len as f32);
        let mut ring = Ring::default();
        let mut out_sig = low_sig
            .iter()
            .zip(high_sig.iter())
            .map(|(x, y)| ring.process(*x, *y))
            .collect::<Vec<_>>();
        windowed_rfft(&mut out_sig)
            .into_iter()
            .map(|x| x.norm() / 1024.0)
            .collect::<Vec<_>>()
    }

    #[test]
    fn creates_side_bands() {
        #![allow(clippy::cast_precision_loss)]

        let out_spectrum = mag_spectrum_for_rates(4.0, 14.0, 1024);
        // We expect the original bands to be supressed
        assert_lt!(out_spectrum[4], 1e-6);
        assert_lt!(out_spectrum[14], 1e-6);
        // However we expect loud side bands
        assert_gt!(out_spectrum[10], 0.1);
        assert_gt!(out_spectrum[18], 0.1);
        // And relatively low total noise outside these side bands
        let total_noise = out_spectrum
            .iter()
            .enumerate()
            .map(|(bin, mag)| {
                if bin == 10 || bin == 18 {
                    0.0
                } else {
                    mag * mag
                }
            })
            .sum::<f32>()
            / out_spectrum.len() as f32;
        assert_lt!(total_noise, 1e-4);
    }

    #[test]
    fn reset() {
        let mut ring = Ring::default();
        let a = sine(1024, 4.0 / 1024.0);
        let b = sine(1024, 14.0 / 1024.0);
        let out1 = a
            .iter()
            .zip(b.iter())
            .map(|(x, y)| ring.process(*x, *y))
            .collect::<Vec<_>>();
        ring.reset();
        let out2 = a
            .iter()
            .zip(b.iter())
            .map(|(x, y)| ring.process(*x, *y))
            .collect::<Vec<_>>();
        assert!(all_approx_eq(out1, out2, 1e-6));
    }

    #[test]
    fn alias_supression() {
        let out_spectrum = mag_spectrum_for_rates(50.0, 500.0, 1024);

        // We expect a "real" sideband at 500 - 50 = 450, and an aliased sideband at 500 + 50 = 550
        // which will be aliased to 1024 - 550 = 474.

        // Note we do allow some filtering artifacts at high frequencies, so this might not be as
        // loud as the lower-frequency tests above.
        assert_gt!(out_spectrum[450], 0.01);
        let aliasing_supression = out_spectrum[474] / out_spectrum[450];
        // We require at least 12dB of aliasing suppression.
        assert_lt!(aliasing_supression, 0.25);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn snapshot() {
        let mut ring = Ring::default();
        let a = sine(48000, 8000.0 / 48000.0);
        let b = linear_sine_sweep(48000, 48000.0, 10.0, 20_000.0);
        let out = a
            .iter()
            .zip(b.iter())
            .map(|(x, y)| ring.process(*x, *y))
            .collect::<Vec<_>>();
        assert_snapshot!("ring/snapshot", 48000, out);
    }
}
