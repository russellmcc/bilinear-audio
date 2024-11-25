/// A delay line optimized to read and write one sample at a time.
#[derive(Debug)]
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

    pub fn write(&mut self, input: f32) {
        self.buffer[self.head] = input;
        self.head = (self.head + 1) % self.buffer.len();
    }
}

#[cfg(test)]
mod tests;
