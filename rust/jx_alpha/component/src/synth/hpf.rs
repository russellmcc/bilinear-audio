use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, FromPrimitive)]
pub enum Mode {
    LowBoost,
    Flat,
    LowCut1,
    LowCut2,
}

const LOW_BOOST_GAIN: f32 = 1.25;

// Note that low boost shares the same cutoff as low cut 1.
const LOW_CUT_1_CUTOFF: f32 = 220.0;
const LOW_CUT_2_CUTOFF: f32 = 720.0;

/// Note that we use TDF-2 integration.

#[derive(Debug, Clone, Copy)]
struct Coeffs {
    /// Integration constant
    k: f32,

    /// Multiplier for hpf output
    c: f32,
}

fn calc_coeffs(cutoff: f32, sampling_rate: f32) -> Coeffs {
    let increment = cutoff / sampling_rate;
    let k = (increment * std::f32::consts::TAU / 2.0).tan();
    let c = 1.0 / (1.0 + k);
    Coeffs { k, c }
}

#[derive(Debug, Clone)]
pub struct Hpf {
    state: f32,
    low_cut_1_coeffs: Coeffs,
    low_cut_2_coeffs: Coeffs,
}

impl Hpf {
    pub fn new(sampling_rate: f32) -> Self {
        Self {
            state: 0.0,
            low_cut_1_coeffs: calc_coeffs(LOW_CUT_1_CUTOFF, sampling_rate),
            low_cut_2_coeffs: calc_coeffs(LOW_CUT_2_CUTOFF, sampling_rate),
        }
    }

    pub fn process(&mut self, mode: Mode, channel: &mut [f32]) {
        let coeffs = match mode {
            Mode::LowCut2 => &self.low_cut_2_coeffs,
            _ => &self.low_cut_1_coeffs,
        };

        for sample in channel {
            let hpf_output = (*sample - self.state) * coeffs.c;
            let integrator_input = hpf_output * coeffs.k;
            let lpf_output = self.state + integrator_input;
            self.state = lpf_output + integrator_input;
            *sample = match mode {
                Mode::LowBoost => *sample + LOW_BOOST_GAIN * lpf_output,
                Mode::Flat => *sample,
                Mode::LowCut1 | Mode::LowCut2 => hpf_output,
            };
        }
    }

    pub fn reset(&mut self) {
        self.state = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use conformal_component::audio::all_approx_eq;
    use dsp::test_utils::{white_noise, windowed_rfft};
    use more_asserts::{assert_gt, assert_lt};
    use snapshots::assert_snapshot;

    use super::{Hpf, Mode};

    fn process_with_mode(input: &mut [f32], hpf: &mut Hpf, mode: Mode) -> Vec<f32> {
        let mut output = input.to_vec();
        hpf.process(mode, &mut output);
        output
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn lowers_dc_low_cut_1() {
        let mut hpf = Hpf::new(48000.0);
        let mut input = white_noise(8192);
        // Add artificial DC offset
        for input in input.iter_mut() {
            *input = *input * 0.1 + 0.9;
        }

        let mut processed = process_with_mode(&mut input, &mut hpf, Mode::LowCut1);
        let spectrum = windowed_rfft(&mut input);
        let processed_spectrum = windowed_rfft(&mut processed);

        // Check that it's significantly reducing power at DC
        let power_reduction_at_dc = processed_spectrum[0].norm_sqr() / spectrum[0].norm_sqr();
        assert_lt!(power_reduction_at_dc, 0.1);

        // Also, check that it didn't reduce power in the middle of the spectrum.
        let power_reduction_mid_spectrum =
            processed_spectrum[1000].norm_sqr() / spectrum[1000].norm_sqr();
        assert_gt!(power_reduction_mid_spectrum, 0.99);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn lowers_dc_low_cut_2() {
        let mut hpf = Hpf::new(48000.0);
        let mut input = white_noise(8192);
        // Add artificial DC offset
        for input in input.iter_mut() {
            *input = *input * 0.1 + 0.9;
        }

        let mut processed = process_with_mode(&mut input, &mut hpf, Mode::LowCut2);
        let spectrum = windowed_rfft(&mut input);
        let processed_spectrum = windowed_rfft(&mut processed);

        // Check that it's significantly reducing power at DC
        let power_reduction_at_dc = processed_spectrum[0].norm_sqr() / spectrum[0].norm_sqr();
        assert_lt!(power_reduction_at_dc, 0.1);

        // Also, check that it didn't reduce power in the high end of the spectrum.
        let power_reduction_mid_spectrum =
            processed_spectrum[1000].norm_sqr() / spectrum[2000].norm_sqr();
        assert_gt!(power_reduction_mid_spectrum, 0.99);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn raises_low_freq_boost() {
        let mut hpf = Hpf::new(48000.0);
        let mut input = white_noise(8192);

        let mut processed = process_with_mode(&mut input, &mut hpf, Mode::LowBoost);
        let spectrum = windowed_rfft(&mut input);
        let processed_spectrum = windowed_rfft(&mut processed);

        // Check that it's significantly reducing power at low frequencies
        let power_increase_at_low_freq = processed_spectrum[3].norm_sqr() / spectrum[3].norm_sqr();
        assert_approx_eq!(power_increase_at_low_freq, 2.25 * 2.25, 0.1);

        // Also, check that it didn't reduce power in the high end of the spectrum.
        let power_reduction_mid_spectrum =
            processed_spectrum[2000].norm_sqr() / spectrum[2000].norm_sqr();
        assert_approx_eq!(power_reduction_mid_spectrum, 1.0, 0.01);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn flat_is_flat() {
        let mut hpf = Hpf::new(48000.0);
        let mut input = white_noise(8192);

        let mut processed = process_with_mode(&mut input, &mut hpf, Mode::Flat);
        let spectrum = windowed_rfft(&mut input);
        let processed_spectrum = windowed_rfft(&mut processed);

        // Check that it's significantly reducing power at low frequencies
        let power_increase_at_low_freq = processed_spectrum[3].norm_sqr() / spectrum[3].norm_sqr();
        assert_approx_eq!(power_increase_at_low_freq, 1.0, 0.01);

        // Also, check that it didn't reduce power in the high end of the spectrum.
        let power_reduction_mid_spectrum =
            processed_spectrum[2000].norm_sqr() / spectrum[2000].norm_sqr();
        assert_approx_eq!(power_reduction_mid_spectrum, 1.0, 0.01);
    }

    #[test]
    fn reset() {
        let mut hpf = Hpf::new(48000.0);
        let mut initial = white_noise(30);
        let mut initial_clone = initial.clone();
        let processed = process_with_mode(&mut initial, &mut hpf, Mode::LowCut1);
        hpf.reset();
        let after_reset = process_with_mode(&mut initial_clone, &mut hpf, Mode::LowCut1);
        assert!(all_approx_eq(processed, after_reset, 1e-6));
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn snapshot() {
        let mut hpf = Hpf::new(48000.0);
        let processed = process_with_mode(
            &mut white_noise(48000)
                .iter()
                .map(|x| x / 2.0)
                .collect::<Vec<_>>(),
            &mut hpf,
            Mode::LowBoost,
        );
        assert_snapshot!("hpf/snapshot", 48000, processed);
    }
}
