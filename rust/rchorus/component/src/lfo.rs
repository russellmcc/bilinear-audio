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
    output: Option<f32>,
}

pub struct Buffer<F, R> {
    pub forward: F,
    pub reverse: R,
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
            output: None,
            phase: 0.,
        }
    }

    fn run_single(&mut self, Parameters { incr, depth }: Parameters) -> f32 {
        let instant = depth
            * self.scale
            * (if self.phase > 0.5 {
                1. - self.phase
            } else {
                self.phase
            } - 0.25);
        if incr < 0.5 {
            self.phase += incr;
            if self.phase > 1. {
                self.phase -= 1.;
            }
        }
        self.output = match self.output {
            Some(output) => Some(output + self.alpha * (instant - output)),
            None => Some(instant),
        };
        self.output.unwrap()
    }

    pub fn run(
        &mut self,
        params: Parameters,
        num_frames: usize,
    ) -> Buffer<impl Iterator<Item = f32> + use<>, impl Iterator<Item = f32> + use<>> {
        let mut forward_lfo = self.clone();
        let mut reverse_lfo = self.clone();

        // Note that we just separately run the LFO here and also in each
        // returned iterator. Kinda slow, but the alternative would require
        // memory storage to store the outputs!
        for _ in 0..num_frames {
            self.run_single(params);
        }

        let forward = (0..num_frames).map(move |_| forward_lfo.point + forward_lfo.run_single(params));

        let reverse = (0..num_frames).map(move |_| reverse_lfo.point - reverse_lfo.run_single(params));

        Buffer { forward, reverse }
    }

    pub fn reset(&mut self) {
        self.phase = 0.;
        self.output = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alias_surpressed() {
        let mut lfo = Lfo::new(Options { min: 5., max: 9. });

        let Buffer { forward, reverse } = lfo.run(
            Parameters {
                incr: 0.825,
                depth: 100.
            },
            10,
        );
        assert_eq!(forward.collect::<Vec<_>>(), &[5.; 10]);
        assert_eq!(reverse.collect::<Vec<_>>(), &[9.; 10]);
    }
}
