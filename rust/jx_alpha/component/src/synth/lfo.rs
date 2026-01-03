use dsp::f32::rescale_points;
use num_derive::FromPrimitive;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

#[derive(Debug)]
pub struct Lfo {
    phase: f32,
    point: f32,
    next_point: f32,
    rng: Xoshiro256PlusPlus,
}

#[derive(FromPrimitive, Debug, Clone, Copy)]
pub enum Shape {
    Sine,
    Square,
    Rand,
}

impl Default for Lfo {
    fn default() -> Self {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(369);
        Self {
            phase: 0.0,
            point: 0.0,
            next_point: rng.gen_range(-1.0f32..=1.0f32),
            rng,
        }
    }
}

impl Lfo {
    pub fn reset(&mut self) {
        self.phase = 0.0;
        self.point = 0.0;
        self.rng = Xoshiro256PlusPlus::seed_from_u64(369);
        self.next_point = self.rng.gen_range(-1.0f32..=1.0f32);
    }

    pub fn generate(&mut self, incr: f32, shape: Shape) -> f32 {
        let ret = match shape {
            Shape::Sine => (self.phase * std::f32::consts::TAU).sin(),
            Shape::Square => {
                if self.phase > 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            Shape::Rand => rescale_points(self.phase, 0.0f32, 1.0f32, self.point, self.next_point),
        };
        self.phase += incr;
        if self.phase > 1.0 {
            self.point = self.next_point;
            self.next_point = self.rng.gen_range(-1.0f32..=1.0f32);
            self.phase -= 1.0;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::{Lfo, Shape};
    use assert_approx_eq::assert_approx_eq;
    use conformal_component::audio::all_approx_eq;
    use dsp::test_utils::estimate_tuning_gen;
    use snapshots::assert_snapshot;

    #[test]
    fn reset() {
        let mut lfo = Lfo::default();
        let incr = 482.5 / 44100.0;
        let initial = std::iter::repeat_with(|| lfo.generate(incr, Shape::Rand))
            .take(100)
            .collect::<Vec<_>>();
        lfo.reset();
        let reset = std::iter::repeat_with(|| lfo.generate(incr, Shape::Rand))
            .take(100)
            .collect::<Vec<_>>();
        assert!(all_approx_eq(initial, reset, 1e-6));
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn tuning() {
        let incr = 482.5 / 44100.0;
        let mut lfo = Lfo::default();
        assert_approx_eq!(
            estimate_tuning_gen(|| lfo.generate(incr, Shape::Sine)),
            incr,
            1e-4
        );
    }

    fn sweep_snapshot_for_shape(shape: Shape) -> Vec<f32> {
        let num_samples = 48000;
        let initial_incr = 0.00001;
        let max_incr = 0.1;
        let mut incr = initial_incr;
        let incr_incr = (max_incr - initial_incr) / num_samples as f32;
        let mut lfo = Lfo::default();
        std::iter::repeat_with(move || {
            let out = lfo.generate(incr, shape);
            incr += incr_incr;
            out
        })
        .take(num_samples)
        .collect()
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn sine_sweep_snapshot() {
        assert_snapshot!(
            "lfo/sine_sweep",
            48000,
            sweep_snapshot_for_shape(Shape::Sine)
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn square_sweep_snapshot() {
        assert_snapshot!(
            "lfo/square_sweep",
            48000,
            sweep_snapshot_for_shape(Shape::Square)
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn rand_sweep_snapshot() {
        assert_snapshot!(
            "lfo/rand_sweep",
            48000,
            sweep_snapshot_for_shape(Shape::Rand)
        );
    }
}
