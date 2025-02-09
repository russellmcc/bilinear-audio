use arrayvec::ArrayVec;
use rand::{seq::SliceRandom, Rng};

pub const CHANNELS: usize = 8;

#[derive(Debug, Clone, PartialEq)]
pub struct Shuffler {
    reverses: ArrayVec<usize, CHANNELS>,
    shuffle: [usize; CHANNELS],
    scale: f32,
}

// Base case for size 2
fn hadamard_1(input: &[f32; 1]) -> [f32; 1] {
    [input[0]]
}

macro_rules! hadamard {
    ($name:ident, $size:expr, $half_name:ident) => {
        fn $name(input: &[f32; $size]) -> [f32; $size] {
            let first_half = &input[..$size / 2];
            let second_half = &input[$size / 2..];

            let a = first_half
                .iter()
                .zip(second_half.iter())
                .map(|(a, b)| a + b);
            let b = first_half
                .iter()
                .zip(second_half.iter())
                .map(|(a, b)| a - b);

            let mut a_arr = [0.0; $size / 2];
            dsp::iter::move_into(a, a_arr.iter_mut());
            let hadamard_a = $half_name(&a_arr);

            let mut b_arr = [0.0; $size / 2];
            dsp::iter::move_into(b, b_arr.iter_mut());
            let hadamard_b = $half_name(&b_arr);

            let mut output = [0.0; $size];
            dsp::iter::move_into(hadamard_a.into_iter().chain(hadamard_b), output.iter_mut());

            output
        }
    };
}

// Generate the hadamard functions for larger sizes
hadamard!(hadamard_2, 2, hadamard_1);
hadamard!(hadamard_4, 4, hadamard_2);
hadamard!(hadamard_8, 8, hadamard_4);

impl Shuffler {
    pub fn new(rng: &mut impl Rng) -> Self {
        let mut reverses = [false; CHANNELS];
        // Invert 50% of the rows
        for row in &mut reverses {
            *row = rng.gen_bool(0.5);
        }

        let mut shuffle = [0; CHANNELS];
        dsp::iter::move_into(0..CHANNELS, shuffle.iter_mut());
        shuffle.shuffle(rng);

        Self {
            reverses: reverses
                .iter()
                .enumerate()
                .filter_map(|(i, &r)| if r { Some(i) } else { None })
                .collect(),
            shuffle,
            scale: (1.0 / CHANNELS as f32).sqrt(),
        }
    }

    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn shuffle(&self, input: &[f32; CHANNELS]) -> [f32; CHANNELS] {
        // Step 1: shuffle with hadamard transform
        let mut hadamard = hadamard_8(input);

        // Step 2: reverse the rows
        for i in self.reverses.iter().copied() {
            hadamard[i] = -hadamard[i];
        }

        // Step 3: shuffle and scale the rows
        let mut shuffled = [0.0; CHANNELS];
        for (i, &j) in self.shuffle.iter().enumerate() {
            shuffled[i] = hadamard[j] * self.scale;
        }

        shuffled
    }
}

#[cfg(test)]
mod tests;
