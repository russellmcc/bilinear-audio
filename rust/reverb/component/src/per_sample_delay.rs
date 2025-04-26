/// A delay line optimized to read and write one sample at a time.
#[derive(Debug, Clone)]
pub struct PerSampleDelay {
    buffer: Vec<f32>,

    head: usize,
}

impl PerSampleDelay {
    pub fn new(delay: usize) -> Self {
        Self {
            buffer: vec![0.0; delay],
            head: 0,
        }
    }

    pub fn read(&self) -> f32 {
        self.buffer[self.head]
    }

    pub fn read_with_offset(&self, offset: usize) -> f32 {
        self.buffer[(self.head + offset) % self.buffer.len()]
    }

    pub fn write(&mut self, input: f32) {
        self.buffer[self.head] = input;
        self.head = (self.head + 1) % self.buffer.len();
    }

    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.head = 0;
    }

    pub fn get_delay(&self) -> usize {
        self.buffer.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use conformal_component::audio::all_approx_eq;

    fn process(input: impl IntoIterator<Item = f32>, delay: &mut PerSampleDelay) -> Vec<f32> {
        input
            .into_iter()
            .map(|sample| {
                let output = delay.read();
                delay.write(sample);
                output
            })
            .collect()
    }

    #[test]
    fn delay_shorter_than_buffer() {
        let mut delay = PerSampleDelay::new(3);
        assert!(all_approx_eq(
            process([1.0, 2.0, 3.0, 4.0, 5.0], &mut delay),
            [0., 0., 0., 1., 2.],
            1e-6
        ));
        assert!(all_approx_eq(
            process([6.0, 7.0, 8.0, 9.0, 10.0], &mut delay),
            [3., 4., 5., 6., 7.],
            1e-6
        ));
    }

    #[test]
    fn delay_longer_than_buffer() {
        let mut delay = PerSampleDelay::new(7);
        assert!(all_approx_eq(
            process([1.0, 2.0, 3.0], &mut delay),
            [0., 0., 0.],
            1e-6
        ));
        assert!(all_approx_eq(
            process([4.0, 5.0, 6.0], &mut delay),
            [0., 0., 0.],
            1e-6
        ));
        assert!(all_approx_eq(
            process([7.0, 8.0, 9.0], &mut delay),
            [0., 1., 2.],
            1e-6
        ));
    }

    #[test]
    fn reset() {
        let mut delay = PerSampleDelay::new(3);
        assert!(all_approx_eq(
            process([1.0, 2.0, 3.0, 4.0, 5.0], &mut delay),
            [0., 0., 0., 1., 2.],
            1e-6
        ));
        delay.reset();
        assert!(all_approx_eq(
            process([6.0, 7.0, 8.0, 9.0, 10.0], &mut delay),
            [0., 0., 0., 6., 7.],
            1e-6
        ));
    }
}
