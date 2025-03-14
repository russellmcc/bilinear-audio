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
mod tests;
