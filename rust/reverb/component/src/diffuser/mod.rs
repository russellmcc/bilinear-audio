use rand::Rng;

pub use crate::shuffler::CHANNELS;
use crate::{multi_channel_per_sample_delay::MultiChannelPerSampleDelay, shuffler::Shuffler};

#[derive(Debug, Clone)]
struct DiffuserBlock {
    shuffler: Shuffler,
    delay: MultiChannelPerSampleDelay<CHANNELS>,
}

impl DiffuserBlock {
    fn new(rng: &mut impl Rng, max_delay: usize) -> Self {
        // We ensure that each max_delay / CHANNELS section gets at least one channel of delay.
        let mut delays = [0; CHANNELS];
        for (i, delay) in delays.iter_mut().enumerate() {
            let range_start = i * (max_delay / CHANNELS);
            let range_end = range_start + (max_delay / CHANNELS);
            *delay = rng.gen_range(range_start..range_end);
        }

        Self {
            shuffler: Shuffler::new(rng),
            delay: MultiChannelPerSampleDelay::new(delays),
        }
    }

    fn process(&mut self, input: &[f32; CHANNELS]) -> [f32; CHANNELS] {
        let ret = self.shuffler.shuffle(&self.delay.read());
        self.delay.write(input);
        ret
    }
}

pub const BLOCKS: usize = 4;

pub struct Diffuser {
    blocks: [DiffuserBlock; BLOCKS],
}
impl Diffuser {
    pub fn new(rng: &mut impl Rng, max_delays: [usize; BLOCKS]) -> Self {
        Self {
            blocks: core::array::from_fn(|i| DiffuserBlock::new(rng, max_delays[i])),
        }
    }

    pub fn process(&mut self, input: &[f32; CHANNELS]) -> [f32; CHANNELS] {
        let mut output = *input;
        for block in &mut self.blocks {
            output = block.process(&output);
        }
        output
    }
}

#[cfg(test)]
mod tests;
