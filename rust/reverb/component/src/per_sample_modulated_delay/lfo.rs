#[derive(Debug, Clone, Copy, PartialEq)]
enum LfoDirection {
    Up,
    Down,
}

/// Triangle LFO for controlling the modulated delay line.
#[derive(Debug, Clone)]
pub struct Lfo {
    value: f32,
    direction: LfoDirection,
}

impl Lfo {
    pub fn new() -> Self {
        Self {
            value: 0f32,
            direction: LfoDirection::Up,
        }
    }

    pub fn reset(&mut self) {
        self.value = 0.0;
        self.direction = LfoDirection::Up;
    }

    /// Run the LFO.
    ///
    /// Here `rate` is the rate in lfo cycles/samples
    pub fn run(&mut self, depth: f32, rate: f32) -> f32 {
        if self.value > depth {
            self.direction = LfoDirection::Down;
        }

        let max = depth.max(self.value);
        let incr = if rate > 0.5 { 0.0 } else { max * 2.0 * rate };
        let out = self.value;
        match self.direction {
            LfoDirection::Down => {
                self.value -= incr;
                if self.value <= 0f32 {
                    self.value = (-self.value).min(depth);
                    self.direction = LfoDirection::Up;
                }
            }
            LfoDirection::Up => {
                self.value += incr;
                if self.value >= max {
                    self.value = max + (max - self.value);
                    self.direction = LfoDirection::Down;
                }
            }
        }

        // Flush small values to zero to prvent denormals.
        if self.value < 1e-15 {
            self.value = 0.0;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::Lfo;
    use conformal_component::audio::{all_approx_eq, approx_eq};

    #[test]
    fn basics() {
        let mut lfo = Lfo::new();
        let values = std::iter::repeat_with(move || lfo.run(2.0, 0.25))
            .take(8)
            .collect::<Vec<_>>();
        assert!(all_approx_eq(
            values,
            vec![0.0, 1.0, 2.0, 1.0, 0.0, 1.0, 2.0, 1.0],
            1e-6
        ));
    }

    #[test]
    fn avoids_aliasing_by_using_constant_lfo() {
        let mut lfo = Lfo::new();
        let values = std::iter::repeat_with(move || lfo.run(2.0, 0.6))
            .take(4)
            .collect::<Vec<_>>();
        // Assert values are approximately constnant.
        assert!(values.windows(2).all(|w| (w[0] - w[1]).abs() < 1e-6));
    }

    #[test]
    fn okay_to_have_zero_depth_equal() {
        let mut lfo = Lfo::new();
        let values = std::iter::repeat_with(move || lfo.run(0.0, 0.25))
            .take(4)
            .collect::<Vec<_>>();
        assert!(values.iter().all(|v| approx_eq(*v, 0.0, 1e-6)));
    }

    #[test]
    fn jump_to_zero_depth() {
        let mut lfo = Lfo::new();
        lfo.run(100.0, 0.25);
        lfo.run(100.0, 0.25);
        assert!(approx_eq(lfo.run(0.0, 0.25), 100.0, 1e-6));
        let x = lfo.run(0.0, 0.25);
        assert!(x < 100.0);
        assert!(lfo.run(0.0, 0.25) < x);
    }
}
