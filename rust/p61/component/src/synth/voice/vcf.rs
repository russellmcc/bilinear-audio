#[derive(Debug, Default, Clone)]
struct State {
    s0: f64,
    s1: f64,
}

#[derive(Debug)]
pub struct Vcf {
    state: State,
    k: f64,

    /// Nonlinearity level
    v: f64,
}

#[derive(Debug, Clone)]
struct Coeffs {
    /// This controls the resonance shift (0 is svf)
    k: f64,

    /// Resonance control
    two_r: f64,

    /// integration gain (controls cutoff)
    g: f64,

    /// Nonlinearity level (0 is linear)
    v: f64,
}

// Optimization opportunity: replace this with rational approximation
fn calc_g(incr: f32) -> f64 {
    ((f64::from(incr)).clamp(0.0, 0.48) * std::f64::consts::PI).tan()
}

fn calc_two_r(resonance: f32) -> f64 {
    1.0 / f64::from(resonance).clamp(0.5, 100.0)
}

// Get the input to the s0 integrator from a given out.
fn get_ds0(x: f64, out: f64, c: &Coeffs) -> f64 {
    x + (-2.0 * c.k - 1.0 + c.two_r * c.k) * out
}

#[allow(clippy::similar_names)]
fn update_state(ds0: f64, out: f64, state: &State, c: &Coeffs) -> State {
    let gds0 = c.g * ds0;
    let y0 = state.s0 + gds0;
    let s0 = state.s0 + 2.0 * gds0;
    let gds1 = c.g * (y0 - c.two_r * out);
    let s1 = state.s1 + 2.0 * gds1;

    State { s0, s1 }
}

fn update_linear(x: f32, state: &State, c: &Coeffs) -> (f64, State) {
    let x = f64::from(x);
    let Coeffs { g, k, two_r, .. } = c;
    let denom = g * g * (two_r * k - 2.0 * k - 1.0) - two_r * g - 1.0;
    let out = (-state.s1 - g * (state.s0 + g * x)) / denom;
    let ds0 = get_ds0(x, out, c);
    (out, update_state(ds0, out, state, c))
}

/// Solve a * x^2 + b * x + c = 0
fn solve_quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    // https://math.stackexchange.com/a/2007723 for explanation
    let sqrt_discriminant = (b * b - 4.0 * a * c).sqrt();
    if b > 0.0 {
        (
            2.0 * c / (-b - sqrt_discriminant),
            (-b - sqrt_discriminant) / (2.0 * a),
        )
    } else {
        (
            (-b + sqrt_discriminant) / (2.0 * a),
            2.0 * c / (-b + sqrt_discriminant),
        )
    }
}

#[allow(clippy::many_single_char_names)]
fn get_quadratic_out_coeffs(x: f64, state: &State, c: &Coeffs) -> (f64, f64, f64) {
    let Coeffs { g, k, two_r, v } = c;
    let a = v * (1.0 + two_r * g * k * (2.0 - two_r + 2.0 * two_r * g - two_r * two_r * g));
    let b = -1.0 - g * g + v * (-x - state.s1) - two_r * g
        + g * g * k * (-2.0 + two_r)
        + v * (-g * state.s0 + k * state.s1 * (-2.0 + two_r) - two_r * g * x
            + g * k * state.s0 * (-2.0 + two_r));
    let c = state.s1 + g * state.s0 + g * g * x + v * (x * state.s1 + g * x * state.s0);
    (a, b, c)
}

fn saturate_ds0(ds0: f64, v: f64) -> f64 {
    ds0 / (1.0 + v * ds0.abs())
}

fn get_saturated_out(x: f64, state: &State, coeffs: &Coeffs) -> (f64, f64) {
    let (a, b, c) = get_quadratic_out_coeffs(x, state, coeffs);
    let (out0, out1) = solve_quadratic(a, b, c);

    // If either of these solutions are positive, they are correct!
    let ds0 = get_ds0(x, out0, coeffs);
    if ds0 > 0.0 {
        return (out0, saturate_ds0(ds0, coeffs.v));
    }

    let ds0 = get_ds0(x, out1, coeffs);
    if ds0 > 0.0 {
        return (out1, saturate_ds0(ds0, coeffs.v));
    }

    // Otherwise, we need to find the negative solution
    let (a, b, c) = get_quadratic_out_coeffs(
        x,
        state,
        &Coeffs {
            v: -coeffs.v,
            ..*coeffs
        },
    );
    let (out0, out1) = solve_quadratic(a, b, c);

    let ds0 = get_ds0(x, out0, coeffs);
    if ds0 < 0.0 {
        return (out0, saturate_ds0(ds0, coeffs.v));
    }
    let ds0 = get_ds0(x, out1, coeffs);
    if ds0 < 0.0 {
        return (out1, saturate_ds0(ds0, coeffs.v));
    }

    // If we get here, could be some rounding errors...
    (0.0, saturate_ds0(get_ds0(x, 0.0, coeffs), coeffs.v))
}

fn update_saturated(x: f32, state: &State, coeffs: &Coeffs) -> (f64, State) {
    let x = f64::from(x);
    let (out, ds0) = get_saturated_out(x, state, coeffs);
    (out, update_state(ds0, out, state, coeffs))
}

impl Vcf {
    pub fn new() -> Self {
        Self {
            state: State::default(),
            k: 0.3,
            v: 0.2,
        }
    }

    /// Note that the circuit includes a weird feature that breaks resonance
    /// tuning. This private constructor disables that feature for unit tests
    /// so we can test the tuning.
    #[cfg(test)]
    fn new_linear_with_tuned_resonance() -> Self {
        Self {
            state: State::default(),
            k: 0.0,
            v: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.state = State::default();
    }

    pub fn process(&mut self, input: f32, cutoff_incr: f32, resonance: f32) -> f32 {
        let two_r = calc_two_r(resonance);
        let g = calc_g(cutoff_incr);
        let coeffs = Coeffs {
            k: self.k,
            two_r,
            g,
            v: self.v,
        };
        let (out, state) = if self.v > 1e-10 {
            update_saturated(input, &self.state, &coeffs)
        } else {
            update_linear(input, &self.state, &coeffs)
        };
        self.state = state;

        #[allow(clippy::cast_possible_truncation)]
        let ret = out as f32;

        debug_assert!(ret.is_finite(), "out was not finite: {out}");
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Vcf;
    use assert_approx_eq::assert_approx_eq;
    use dsp::test_utils::estimate_tuning;
    use dsp::{
        f32::rescale,
        test_utils::{white_noise, windowed_rfft},
    };
    use snapshots::assert_snapshot;

    #[test]
    fn reset() {
        let mut vcf = Vcf::new();
        let mut initial = white_noise(100);
        let mut initial_clone = initial.clone();
        for sample in initial.iter_mut() {
            *sample = vcf.process(*sample, 0.1, 0.707);
        }
        let processed = initial;
        vcf.reset();
        for sample in initial_clone.iter_mut() {
            *sample = vcf.process(*sample, 0.1, 0.707);
        }
        let after_reset = initial_clone;
        for (a, b) in processed.iter().zip(after_reset.iter()) {
            assert_approx_eq!(a, b);
        }
    }

    enum CheckMode {
        /// Make a test filter that is linear, and apply tight bounds on performance
        LinearStrict,

        /// Use the production non-linear filter, and apply loose bounds on performance
        NonLinearLoose,
    }

    fn check_lowpass_action(cutoff_bin: usize, mode: CheckMode) {
        let mut vcf = match mode {
            CheckMode::LinearStrict => Vcf::new_linear_with_tuned_resonance(),
            CheckMode::NonLinearLoose => Vcf::new(),
        };
        let mut input = white_noise(4096);
        let mut processed = input.clone();
        for sample in processed.iter_mut() {
            *sample = vcf.process(*sample, cutoff_bin as f32 / 4096.0, 0.707);
        }
        let spectrum = windowed_rfft(&mut input);
        let processed_spectrum = windowed_rfft(&mut processed);

        // Check that it's significantly reducing power at high frequencies
        let power_reduction_two_octave_db = 10.0
            * (processed_spectrum[cutoff_bin * 4].norm_sqr() / spectrum[cutoff_bin * 4].norm_sqr())
                .log10();

        let epsilon = match mode {
            CheckMode::LinearStrict => 3.0,
            CheckMode::NonLinearLoose => 4.0,
        };

        // Since this is a 2-pole filter, we expect the power reduction to be close to -12dB per octave
        assert_approx_eq!(power_reduction_two_octave_db, -24.0, epsilon);

        // There should not be any power reduction at half the cutoff.
        let power_reduction_half_cutoff_db = 10.0
            * (processed_spectrum[cutoff_bin / 2].norm_sqr() / spectrum[cutoff_bin / 2].norm_sqr())
                .log10();
        assert_approx_eq!(power_reduction_half_cutoff_db, 0.0, epsilon);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn acts_as_lowpass() {
        check_lowpass_action(50, CheckMode::LinearStrict);
        check_lowpass_action(50, CheckMode::NonLinearLoose);
        check_lowpass_action(100, CheckMode::LinearStrict);
        check_lowpass_action(100, CheckMode::NonLinearLoose);
        check_lowpass_action(200, CheckMode::LinearStrict);
        check_lowpass_action(200, CheckMode::NonLinearLoose);
        // Note that higher than this frequency warping becomes an issue
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn resonance_tuning() {
        let mut vcf = Vcf::new_linear_with_tuned_resonance();
        let mut processed = white_noise(4096);
        let increment = 482.5 / 44100.0;

        for sample in processed.iter_mut() {
            *sample = vcf.process(*sample, increment, 100.0);
        }
        assert_approx_eq!(estimate_tuning(&mut processed), increment, 1e-3);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn log_sweep_snapshot() {
        let mut vcf = Vcf::new();
        let num_samples = 48000;
        let mut processed = white_noise(num_samples);
        for (index, sample) in processed.iter_mut().enumerate() {
            *sample = vcf.process(
                *sample,
                0.5 * rescale(index as f32, 0.0..=(num_samples as f32), -7.0..=0.0).exp2(),
                10.0,
            );
        }
        assert_snapshot!("vcf/sweep", 48000, processed);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn linear_sweep_snapshot() {
        // Linear sweep sometimes is better at finding instabilities...
        let mut vcf = Vcf::new();
        let num_samples = 48000;
        let mut processed = white_noise(num_samples);
        for (index, sample) in processed.iter_mut().enumerate() {
            *sample = vcf.process(
                *sample,
                rescale(index as f32, 0.0..=(num_samples as f32), 0.0..=0.5),
                10.0,
            );
        }
        assert_snapshot!("vcf/linear_sweep", 48000, processed);
    }
}
