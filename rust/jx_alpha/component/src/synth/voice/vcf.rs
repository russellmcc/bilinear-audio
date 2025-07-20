use std::f32::consts::SQRT_2;

use dsp::{
    f32::lerp,
    iir::svf,
    iir::svf::{Svf, calc_g, calc_two_r},
};

#[derive(Default, Debug, Clone)]
pub struct Settings {
    /// The cutoff frequency, represented as an phase increment per sample.
    pub cutoff_incr: f32,

    /// The "resonance" parameter, represented as 0->1 (0 is no resonance, 1 is maximum resonance)
    pub resonance: f32,
}

#[derive(Default, Debug, Clone)]
pub struct Vcf {
    stages: [Svf; 2],
}

impl Vcf {
    #[allow(clippy::cast_possible_truncation)]
    pub fn process(&mut self, input: f32, settings: &Settings) -> f32 {
        // TODO: scale resonance properly.
        let g = calc_g(f64::from(settings.cutoff_incr));
        let q = f64::from(lerp(SQRT_2 / 2.0, 2.0, settings.resonance));
        let two_r = calc_two_r(q);
        let params = svf::RawParams { g, two_r };
        let output0 = self.stages[0].process_single(svf::Input {
            // Note that we scale the input by "R" parameter to reduce gain as resonance increases.
            // We normalize this so the gain is 1.0 at resonance = 0.0.
            x: ((1.0 - std::f64::consts::SQRT_2 * 0.5) + two_r * 0.5) * f64::from(input),
            params,
        });
        let output1 = self.stages[1].process_single(svf::Input {
            x: output0.low,
            params,
        });
        output1.low as f32
    }

    pub fn reset(&mut self) {
        for stage in &mut self.stages {
            stage.reset();
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::cast_precision_loss)]

    use super::*;
    use conformal_component::audio::all_approx_eq;
    use dsp::{
        f32::rescale_points,
        test_utils::{white_noise, windowed_rfft},
    };
    use more_asserts::{assert_gt, assert_lt};
    use snapshots::assert_snapshot;

    fn lerp_settings(settings: [&Settings; 2], t: f32) -> Settings {
        Settings {
            cutoff_incr: lerp(settings[0].cutoff_incr, settings[1].cutoff_incr, t),
            resonance: lerp(settings[0].resonance, settings[1].resonance, t),
        }
    }

    fn process_swept_settings(input: &[f32], vcf: &mut Vcf, settings: [&Settings; 2]) -> Vec<f32> {
        let mut output = input.to_vec();
        for (idx, sample) in output.iter_mut().enumerate() {
            *sample = vcf.process(
                *sample,
                &lerp_settings(
                    settings,
                    rescale_points(idx as f32, 0.0, input.len() as f32, 0.0, 1.0),
                ),
            );
        }
        output
    }

    fn process_constant_settings(input: &[f32], vcf: &mut Vcf, settings: &Settings) -> Vec<f32> {
        process_swept_settings(input, vcf, [settings, settings])
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn lowers_high_frequencies() {
        let mut vcf = Vcf::default();
        let mut input: Vec<f32> = white_noise(8192)
            .into_iter()
            .map(|x| x * 0.5)
            .collect::<Vec<_>>();

        let mut processed = process_constant_settings(
            &input,
            &mut vcf,
            &Settings {
                cutoff_incr: 0.05,
                resonance: 0.0,
            },
        );
        let spectrum = windowed_rfft(&mut input);
        let processed_spectrum = windowed_rfft(&mut processed);

        // Check that it's significantly reducing power at quarter-nyquist (a pretty high frequency)
        let power_reduction_at_half_nyquist =
            processed_spectrum[2048].norm_sqr() / spectrum[2048].norm_sqr();
        assert_lt!(power_reduction_at_half_nyquist, 0.1);

        // Also, check that it didn't reduce power near DC.
        let power_reduction_near_dc = processed_spectrum[1].norm_sqr() / spectrum[1].norm_sqr();
        assert_gt!(power_reduction_near_dc, 0.99);
        assert_lt!(power_reduction_near_dc, 1.01);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn resonance_increases_gain_at_cutoff() {
        let mut vcf = Vcf::default();
        let input: Vec<f32> = white_noise(8192)
            .into_iter()
            .map(|x| x * 0.5)
            .collect::<Vec<_>>();

        let mut processed_low_res = process_constant_settings(
            &input,
            &mut vcf,
            &Settings {
                cutoff_incr: 0.25,
                resonance: 0.0,
            },
        );
        let mut processed_high_res = process_constant_settings(
            &input,
            &mut vcf,
            &Settings {
                cutoff_incr: 0.25,
                resonance: 1.0,
            },
        );
        let processed_spectrum_low_res = windowed_rfft(&mut processed_low_res);
        let processed_spectrum_high_res = windowed_rfft(&mut processed_high_res);

        // The cutoff frequency is around half-nyquist so should show up in bin 2048.
        let resonance_power_ratio = processed_spectrum_high_res[2048].norm_sqr()
            / processed_spectrum_low_res[2048].norm_sqr();
        assert_gt!(resonance_power_ratio, 16.0);
    }

    #[test]
    fn reset_basics() {
        let mut vcf = Vcf::default();
        let input = white_noise(32);
        let settings = Settings {
            cutoff_incr: 0.10,
            resonance: 0.0,
        };
        let processed_first = process_constant_settings(&input, &mut vcf, &settings);
        vcf.reset();
        let processed_second = process_constant_settings(&input, &mut vcf, &settings);
        assert!(all_approx_eq(processed_first, processed_second, 1e-6));
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn sweep_low_res_snapshot() {
        let mut vcf = Vcf::default();
        let input = white_noise(48000)
            .into_iter()
            .map(|x| x * 0.5)
            .collect::<Vec<_>>();
        let processed = process_swept_settings(
            &input,
            &mut vcf,
            [
                &Settings {
                    cutoff_incr: 0.01,
                    resonance: 0.0,
                },
                &Settings {
                    cutoff_incr: 0.25,
                    resonance: 0.0,
                },
            ],
        );
        assert_snapshot!("vcf/sweep_low_res", 48000, processed);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn sweep_high_res_snapshot() {
        let mut vcf = Vcf::default();
        let input = white_noise(48000)
            .into_iter()
            .map(|x| x * 0.5)
            .collect::<Vec<_>>();
        let processed = process_swept_settings(
            &input,
            &mut vcf,
            [
                &Settings {
                    cutoff_incr: 0.01,
                    resonance: 1.0,
                },
                &Settings {
                    cutoff_incr: 0.25,
                    resonance: 1.0,
                },
            ],
        );
        assert_snapshot!("vcf/sweep_high_res", 48000, processed);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn res_sweep_snapshot() {
        let mut vcf = Vcf::default();
        let input = white_noise(48000)
            .into_iter()
            .map(|x| x * 0.5)
            .collect::<Vec<_>>();
        let processed = process_swept_settings(
            &input,
            &mut vcf,
            [
                &Settings {
                    cutoff_incr: 0.1,
                    resonance: 0.0,
                },
                &Settings {
                    cutoff_incr: 0.1,
                    resonance: 1.0,
                },
            ],
        );
        assert_snapshot!("vcf/res_sweep", 48000, processed);
    }
}
