#![allow(clippy::cast_possible_truncation)]

use crate::test_utils::{white_noise, windowed_rfft};
use assert_approx_eq::assert_approx_eq;
use more_asserts::{assert_gt, assert_lt};

use super::*;
#[derive(Debug, Clone, Copy)]
struct PowerReductions {
    low: f32,
    high: f32,
}

fn calc_power_reduction_for_filter(filter: impl Fn(&Vec<f32>) -> Vec<f32>) -> PowerReductions {
    let mut input = white_noise(4096);
    let mut processed = filter(&input);
    let spectrum = windowed_rfft(&mut input);
    let processed_spectrum = windowed_rfft(&mut processed);
    let high_freq = 2044;
    let power_reduction_at_high_freq =
        processed_spectrum[high_freq].norm_sqr() / spectrum[high_freq].norm_sqr();
    let low_freq = 50;
    let power_reduction_at_low_freq =
        processed_spectrum[low_freq].norm_sqr() / spectrum[low_freq].norm_sqr();
    PowerReductions {
        low: power_reduction_at_low_freq,
        high: power_reduction_at_high_freq,
    }
}

#[test]
#[cfg_attr(miri, ignore)]
fn lpf_lowers_high_freqs() {
    let power_reductions = calc_power_reduction_for_filter(|input| {
        let mut filter: Svf = Default::default();
        let params = RawParams {
            g: calc_g(0.25),
            two_r: 2.,
        };
        filter
            .process_low(input.iter().map(|x| Input {
                x: f64::from(*x),
                params,
            }))
            .map(|x| x as f32)
            .collect()
    });
    // Check that it's significantly reducing power at high frequencies
    assert_lt!(power_reductions.high, 0.2);

    // Also, check that it didn't reduce power in the low frequencies.
    assert_gt!(power_reductions.low, 0.99);
}

#[test]
#[cfg_attr(miri, ignore)]
fn hpf_lowers_low_freqs() {
    let power_reductions = calc_power_reduction_for_filter(|input| {
        let mut filter: Svf = Default::default();
        let params = RawParams {
            g: calc_g(0.25),
            two_r: 2.,
        };
        filter
            .process_high(input.iter().map(|x| Input {
                x: f64::from(*x),
                params,
            }))
            .map(|x| x as f32)
            .collect()
    });
    // Check that it didn't reduce power in the high frequencies.
    assert_gt!(power_reductions.high, 0.99);

    // Also, check that it's significantly reducing power at low frequencies
    assert_lt!(power_reductions.low, 0.2);
}

#[test]
#[cfg_attr(miri, ignore)]
fn bpf_lowers_both() {
    let power_reductions = calc_power_reduction_for_filter(|input| {
        let mut filter: Svf = Default::default();
        let params = RawParams {
            g: calc_g(0.25),
            two_r: 2.,
        };
        filter
            .process_band(input.iter().map(|x| Input {
                x: f64::from(*x),
                params,
            }))
            .map(|x| x as f32)
            .collect()
    });

    // Check that it's significantly reducing power at high frequencies
    assert_lt!(power_reductions.high, 0.2);

    // Also, check that it's significantly reducing power at low frequencies.
    assert_lt!(power_reductions.low, 0.2);
}

#[test]
fn low_shelf_lowers_low_freqs() {
    let power_reductions = calc_power_reduction_for_filter(|input| {
        let mut filter: Svf = Default::default();
        let params = GainRawParams {
            g: calc_g(0.25),
            two_r: 2.,
            sqrt_gain: 0.5f64.sqrt(),
        };
        filter
            .process_low_shelf(input.iter().map(|x| GainInput {
                x: f64::from(*x),
                params,
            }))
            .map(|x| x as f32)
            .collect()
    });
    assert_approx_eq!(power_reductions.low.sqrt(), 0.5, 0.05);
    assert_approx_eq!(power_reductions.high, 1.0, 0.05);
}

#[test]
fn high_shelf_lowers_high_freqs() {
    let power_reductions = calc_power_reduction_for_filter(|input| {
        let mut filter: Svf = Default::default();
        let params = GainRawParams {
            g: calc_g(0.25),
            two_r: 2.,
            sqrt_gain: 0.5f64.sqrt(),
        };
        filter
            .process_high_shelf(input.iter().map(|x| GainInput {
                x: f64::from(*x),
                params,
            }))
            .map(|x| x as f32)
            .collect()
    });
    assert_approx_eq!(power_reductions.high.sqrt(), 0.5, 0.05);
    assert_approx_eq!(power_reductions.low, 1.0, 0.05);
}

#[test]
fn bell_does_not_lower_extremes() {
    let power_reductions = calc_power_reduction_for_filter(|input| {
        let mut filter: Svf = Default::default();
        let params = GainRawParams {
            g: calc_g(0.25),
            two_r: 2.,
            sqrt_gain: 0.5f64.sqrt(),
        };
        filter
            .process_bell(input.iter().map(|x| GainInput {
                x: f64::from(*x),
                params,
            }))
            .map(|x| x as f32)
            .collect()
    });
    assert_approx_eq!(power_reductions.high, 1.0, 0.05);
    assert_approx_eq!(power_reductions.low, 1.0, 0.05);
}

#[test]
fn reset() {
    let mut filter: Svf = Default::default();
    let params = RawParams {
        g: calc_g(0.25),
        two_r: 2.,
    };
    let input = white_noise(100);
    let processed: Vec<_> = filter
        .process_band(input.iter().map(|x| Input {
            x: f64::from(*x),
            params,
        }))
        .map(|x| x as f32)
        .collect();
    filter.reset();
    let after_reset: Vec<_> = filter
        .process_band(input.iter().map(|x| Input {
            x: f64::from(*x),
            params,
        }))
        .map(|x| x as f32)
        .collect();
    for (a, b) in processed.iter().zip(after_reset.iter()) {
        assert_approx_eq!(a, b);
    }
}
