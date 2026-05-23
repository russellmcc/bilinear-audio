use dsp::f32::exp_approx;

#[derive(Clone)]
pub struct Lfo {
    point: f32,
    scale: f32,

    // Note that for BBDs, we average the delay over a fixed window (the LFO controls
    // the high speed clock, but the total delay is the delay of each tick * the length
    // of the BBD line).
    //
    // This is discussed in detail in [Conformal App Note 2](https://www.russellmcc.com/conformal/app_notes/2-bbd-lfo/).
    alpha: f32,

    phase: f32,
    output: [Option<f32>; 3],
}

#[derive(Clone, Copy)]
pub struct Options {
    pub min: f32,
    pub max: f32,
}

#[derive(Clone, Copy)]
pub struct Parameters {
    pub incr: f32,

    /// In percent
    pub depth: f32,
}

/// Time-constant in samples
fn alpha_from_time_constant(t: f32) -> f32 {
    1. - exp_approx(-2. / t)
}

impl Lfo {
    pub fn new(Options { min, max }: Options) -> Self {
        assert!(min < max);
        let point = (max + min) * 0.5;

        // Note that we use an artificially large time-constant for the smoothing here.
        // This was tuned heuristically to sound good.
        let alpha = alpha_from_time_constant(4. * point);

        Self {
            point,
            scale: (max - min) / 100. * 2.,
            alpha,
            output: [None; 3],
            phase: 0.,
        }
    }

    pub fn center_delay(&self) -> f32 {
        self.point
    }

    fn instant(scale: f32, phase: f32, depth: f32) -> f32 {
        depth * scale * (if phase > 0.5 { 1. - phase } else { phase } - 0.25)
    }

    fn advance_phase(phase: &mut f32, incr: f32) {
        if incr < 0.5 {
            *phase += incr;
            if *phase > 1. {
                *phase -= 1.;
            }
        }
    }

    fn wrap_phase(phase: f32) -> f32 {
        if phase > 1. { phase - 1. } else { phase }
    }

    fn smooth(alpha: f32, output: &mut Option<f32>, instant: f32) -> f32 {
        *output = match *output {
            Some(output) => Some(output + alpha * (instant - output)),
            None => Some(instant),
        };
        (*output).unwrap()
    }

    fn run_single(&mut self, Parameters { incr, depth }: Parameters) -> f32 {
        let instant = Self::instant(self.scale, self.phase, depth);
        Self::advance_phase(&mut self.phase, incr);
        Self::smooth(self.alpha, &mut self.output[0], instant)
    }

    pub fn run(&mut self, params: Parameters, forward: &mut [f32], reverse: &mut [f32]) {
        debug_assert_eq!(forward.len(), reverse.len());
        let point = self.point;
        for (forward, reverse) in forward.iter_mut().zip(reverse.iter_mut()) {
            let value = self.run_single(params);
            *forward = point + value;
            *reverse = point - value;
        }
    }

    pub fn run_three_phase_modulation(
        &mut self,
        Parameters { incr, depth }: Parameters,
        [phase_0, phase_120, phase_240]: [&mut [f32]; 3],
    ) {
        debug_assert_eq!(phase_0.len(), phase_120.len());
        debug_assert_eq!(phase_0.len(), phase_240.len());

        let [output_0, output_120, output_240] = &mut self.output;
        for ((phase_0, phase_120), phase_240) in phase_0
            .iter_mut()
            .zip(phase_120.iter_mut())
            .zip(phase_240.iter_mut())
        {
            *phase_0 = Self::smooth(
                self.alpha,
                output_0,
                Self::instant(self.scale, self.phase, depth),
            );
            *phase_120 = Self::smooth(
                self.alpha,
                output_120,
                Self::instant(self.scale, Self::wrap_phase(self.phase + 1. / 3.), depth),
            );
            *phase_240 = Self::smooth(
                self.alpha,
                output_240,
                Self::instant(self.scale, Self::wrap_phase(self.phase + 2. / 3.), depth),
            );
            Self::advance_phase(&mut self.phase, incr);
        }
    }

    pub fn reset(&mut self) {
        self.phase = 0.;
        self.output = [None; 3];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alias_surpressed() {
        let mut lfo = Lfo::new(Options { min: 5., max: 9. });
        let mut forward = [0.; 10];
        let mut reverse = [0.; 10];
        lfo.run(
            Parameters {
                incr: 0.825,
                depth: 100.,
            },
            &mut forward,
            &mut reverse,
        );
        assert_eq!(forward, [5.; 10]);
        assert_eq!(reverse, [9.; 10]);
    }
}
