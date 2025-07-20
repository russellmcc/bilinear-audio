//! This is a digital "state variable filter". This filter
//! is stable under time-varying parameters.

#[derive(Debug, Clone, Copy, Default)]
pub struct Svf {
    s0: f64,
    s1: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct RawParams {
    pub g: f64,
    pub two_r: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct GainRawParams {
    pub g: f64,
    pub two_r: f64,
    pub sqrt_gain: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Input {
    pub x: f64,
    pub params: RawParams,
}

#[derive(Debug, Clone, Copy)]
pub struct GainInput {
    pub x: f64,
    pub params: GainRawParams,
}

#[derive(Debug, Clone, Copy)]
pub struct Output {
    pub low: f64,
    pub band: f64,
    pub high: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct OutputNoHigh {
    pub low: f64,
    pub band: f64,
}

#[must_use]
pub fn calc_g(incr: f64) -> f64 {
    (std::f64::consts::TAU / 2. * incr).tan()
}

#[must_use]
pub fn calc_two_r(q: f64) -> f64 {
    1. / q
}

impl Svf {
    pub fn process_single(&mut self, input: Input) -> Output {
        let Input { x, params } = input;
        let RawParams { g, two_r: damping } = params;
        // following https://www.native-instruments.com/fileadmin/ni_media/downloads/pdf/VAFilterDesign_2.1.0.pdf
        let d = 1. / (1. + damping * g + g * g);
        let high = d * (x - (damping + g) * self.s0 - self.s1);
        let v0 = g * high;
        let band = v0 + self.s0;
        self.s0 = band + v0;
        let v1 = g * band;
        let low = v1 + self.s1;
        self.s1 = low + v1;
        Output { low, band, high }
    }

    pub fn process<'a, I: IntoIterator<Item = Input> + 'a>(
        &'a mut self,
        inputs: I,
    ) -> impl Iterator<Item = Output> {
        inputs
            .into_iter()
            .map(move |input| self.process_single(input))
    }

    pub fn process_no_high<'a, I: IntoIterator<Item = Input> + 'a>(
        &'a mut self,
        inputs: I,
    ) -> impl Iterator<Item = OutputNoHigh> {
        inputs.into_iter().map(
            move |Input {
                      x,
                      params: RawParams { g, two_r: damping },
                  }| {
                // following https://www.native-instruments.com/fileadmin/ni_media/downloads/pdf/VAFilterDesign_2.1.0.pdf
                let d = 1. / (1. + damping * g + g * g);
                let band = d * (g * (x - self.s1) + self.s0);
                let v0 = band - self.s0;
                self.s0 = v0 + band;
                let v1 = g * band;
                let low = v1 + self.s1;
                self.s1 = low + v1;

                OutputNoHigh { low, band }
            },
        )
    }

    pub fn process_high<'a, I: IntoIterator<Item = Input> + 'a>(
        &'a mut self,
        inputs: I,
    ) -> impl Iterator<Item = f64> {
        self.process(inputs).map(|Output { high, .. }| high)
    }

    pub fn process_band<'a, I: IntoIterator<Item = Input> + 'a>(
        &'a mut self,
        inputs: I,
    ) -> impl Iterator<Item = f64> {
        inputs.into_iter().map(
            move |Input {
                      x,
                      params: RawParams { g, two_r: damping },
                  }| {
                // following https://www.native-instruments.com/fileadmin/ni_media/downloads/pdf/VAFilterDesign_2.1.0.pdf
                let d = 1. / (1. + damping * g + g * g);
                let band = d * (g * (x - self.s1) + self.s0);
                let band2 = band + band;
                self.s0 = band2 - self.s0;
                let v22 = g * band2;
                self.s1 += v22;
                band
            },
        )
    }

    pub fn process_low<'a, I: IntoIterator<Item = Input> + 'a>(
        &'a mut self,
        inputs: I,
    ) -> impl Iterator<Item = f64> {
        self.process_no_high(inputs)
            .map(|OutputNoHigh { low, .. }| low)
    }

    pub fn process_high_shelf<'a, I: IntoIterator<Item = GainInput> + 'a>(
        &'a mut self,
        inputs: I,
    ) -> impl Iterator<Item = f64> {
        // For derivation, see https://www.dafx14.fau.de/papers/dafx14_aaron_wishnick_time_varying_filters_for_.pdf
        inputs.into_iter().map(move |input| {
            let GainInput {
                x,
                params:
                    GainRawParams {
                        g,
                        two_r,
                        sqrt_gain,
                    },
            } = input;
            let Output { low, band, high } = self.process_single(Input {
                x,
                params: RawParams {
                    g: g * sqrt_gain,
                    two_r,
                },
            });
            let gain = sqrt_gain * sqrt_gain;
            gain * gain * high + low + gain * two_r * band
        })
    }

    pub fn process_low_shelf<'a, I: IntoIterator<Item = GainInput> + 'a>(
        &'a mut self,
        inputs: I,
    ) -> impl Iterator<Item = f64> {
        // For derivation, see https://www.dafx14.fau.de/papers/dafx14_aaron_wishnick_time_varying_filters_for_.pdf
        inputs.into_iter().map(move |input| {
            let GainInput {
                x,
                params:
                    GainRawParams {
                        g,
                        two_r,
                        sqrt_gain,
                    },
            } = input;
            let Output { low, band, high } = self.process_single(Input {
                x,
                params: RawParams {
                    g: g / sqrt_gain,
                    two_r,
                },
            });
            let gain = sqrt_gain * sqrt_gain;
            gain * gain * low + high + gain * two_r * band
        })
    }

    pub fn process_bell<'a, I: IntoIterator<Item = GainInput> + 'a>(
        &'a mut self,
        inputs: I,
    ) -> impl Iterator<Item = f64> {
        // For derivation, see https://www.dafx14.fau.de/papers/dafx14_aaron_wishnick_time_varying_filters_for_.pdf
        inputs.into_iter().map(move |input| {
            let GainInput {
                x,
                params:
                    GainRawParams {
                        g,
                        two_r,
                        sqrt_gain,
                    },
            } = input;
            let Output { low, band, high } = self.process_single(Input {
                x,
                params: RawParams { g, two_r },
            });
            let gain = sqrt_gain * sqrt_gain;
            low + high + gain * two_r * band
        })
    }

    pub fn reset(&mut self) {
        *self = Default::default();
    }
}

#[cfg(test)]
mod tests {
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
    #[cfg_attr(miri, ignore)]
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
        assert_approx_eq!(power_reductions.low.sqrt().sqrt(), 0.5, 0.05);
        assert_approx_eq!(power_reductions.high, 1.0, 0.05);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
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
        assert_approx_eq!(power_reductions.high.sqrt().sqrt(), 0.5, 0.05);
        assert_approx_eq!(power_reductions.low, 1.0, 0.05);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
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
    #[cfg_attr(miri, ignore)]
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
}
