#[derive(Debug, Default)]
pub struct SineLfo {
    phase: f32,
}

impl SineLfo {
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }

    pub fn generate(&mut self, incr: f32) -> f32 {
        // Optimization opportunity - use complex numbers to generate sin
        let ret = (self.phase * std::f32::consts::TAU).sin();
        self.phase += incr.clamp(0.0, 1.0);
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::SineLfo;
    use crate::test_utils::estimate_tuning_gen;
    use assert_approx_eq::assert_approx_eq;
    use snapshots::assert_snapshot;

    #[test]
    fn reset() {
        let mut lfo = SineLfo::default();
        let incr = 482.5 / 44100.0;
        let initial = std::iter::repeat_with(|| lfo.generate(incr))
            .take(100)
            .collect::<Vec<_>>();
        lfo.reset();
        let reset = std::iter::repeat_with(|| lfo.generate(incr))
            .take(100)
            .collect::<Vec<_>>();
        for (a, b) in initial.iter().zip(reset.iter()) {
            assert_approx_eq!(a, b);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn tuning() {
        let incr = 482.5 / 44100.0;
        let mut lfo = SineLfo::default();
        assert_approx_eq!(estimate_tuning_gen(|| lfo.generate(incr)), incr, 1e-4);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn sweep_snapshot() {
        let mut lfo = SineLfo::default();
        let num_samples = 48000;
        let initial_incr = 0.00001;
        let max_incr = 0.1;
        let mut incr = initial_incr;
        let incr_incr = (max_incr - initial_incr) / num_samples as f32;
        assert_snapshot!(
            "sine_lfo/sweep",
            48000,
            std::iter::repeat_with(|| {
                let out = lfo.generate(incr);
                incr += incr_incr;
                out
            })
            .take(num_samples)
            .collect::<Vec<_>>()
        );
    }
}
