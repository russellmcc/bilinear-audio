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
    fn process_single(&mut self, input: Input) -> Output {
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
    ) -> impl Iterator<Item = Output> + 'a {
        inputs
            .into_iter()
            .map(move |input| self.process_single(input))
    }

    pub fn process_no_high<'a, I: IntoIterator<Item = Input> + 'a>(
        &'a mut self,
        inputs: I,
    ) -> impl Iterator<Item = OutputNoHigh> + 'a {
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
    ) -> impl Iterator<Item = f64> + 'a {
        self.process(inputs).map(|Output { high, .. }| high)
    }

    pub fn process_band<'a, I: IntoIterator<Item = Input> + 'a>(
        &'a mut self,
        inputs: I,
    ) -> impl Iterator<Item = f64> + 'a {
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
    ) -> impl Iterator<Item = f64> + 'a {
        self.process_no_high(inputs)
            .map(|OutputNoHigh { low, .. }| low)
    }

    pub fn process_high_shelf<'a, I: IntoIterator<Item = GainInput> + 'a>(
        &'a mut self,
        inputs: I,
    ) -> impl Iterator<Item = f64> + 'a {
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
    ) -> impl Iterator<Item = f64> + 'a {
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
    ) -> impl Iterator<Item = f64> + 'a {
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
mod tests;
