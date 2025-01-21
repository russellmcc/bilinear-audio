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

const ER_MIN: [f32; BLOCKS] = [0.5, 0.0, 0.0, 0.0];
const ER_MAX: [f32; BLOCKS] = [0.125, 0.125, 0.125, 0.125];

trait ErCalc {
    fn calc(early: f32, input: &[f32; CHANNELS], block: usize) -> Self;
    fn add_assign(&mut self, other: Self);
}

impl ErCalc for f32 {
    fn calc(early: f32, input: &[f32; CHANNELS], block: usize) -> Self {
        let min = ER_MIN[block];
        let max = ER_MAX[block];
        let weight = min + (max - min) * early;
        weight * input.iter().sum::<f32>()
    }

    fn add_assign(&mut self, other: Self) {
        *self += other;
    }
}

impl ErCalc for [f32; 2] {
    fn calc(early: f32, input: &[f32; CHANNELS], block: usize) -> Self {
        let min = ER_MIN[block];
        let max = ER_MAX[block];
        let weight = min + (max - min) * early;
        // Split the evens and odds
        [
            input
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| if i % 2 == 0 { Some(v) } else { None })
                .sum::<f32>()
                * weight,
            input
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| if i % 2 == 1 { Some(v) } else { None })
                .sum::<f32>()
                * weight,
        ]
    }

    fn add_assign(&mut self, other: Self) {
        self[0] += other[0];
        self[1] += other[1];
    }
}

impl Diffuser {
    pub fn new(rng: &mut impl Rng, max_delays: [usize; BLOCKS]) -> Self {
        Self {
            blocks: core::array::from_fn(|i| DiffuserBlock::new(rng, max_delays[i])),
        }
    }

    fn process<T: ErCalc + Default>(
        &mut self,
        early: f32,
        input: &[f32; CHANNELS],
    ) -> ([f32; CHANNELS], T) {
        let mut output = *input;
        let mut er = T::default();
        for (i, block) in self.blocks.iter_mut().enumerate() {
            output = block.process(&output);
            er.add_assign(T::calc(early, &output, i));
        }
        (output, er)
    }

    /// Process one frame (one sample per channel) through the diffuser
    ///
    /// Yields fully diffuse multichannel input for feedback section and a single
    /// sample of early reflections. `early` must be between 0 and 1. When `early`
    /// is 0, the early reflections are not very diffuse but are quite early.
    /// When `early` is 1, the early reflections are more diffuse but less early.
    pub fn process_mono(&mut self, early: f32, input: &[f32; CHANNELS]) -> ([f32; CHANNELS], f32) {
        self.process::<f32>(early, input)
    }

    /// Process one frame (one sample per channel) through the diffuser with stereo early reflections.
    ///
    /// Yields fully diffuse multichannel input for feedback section and a stereo early
    pub fn process_stereo(
        &mut self,
        early: f32,
        input: &[f32; CHANNELS],
    ) -> ([f32; CHANNELS], [f32; 2]) {
        self.process::<[f32; 2]>(early, input)
    }

    pub fn reset(&mut self) {
        for block in &mut self.blocks {
            block.delay.reset();
        }
    }
}

#[cfg(test)]
mod tests;
