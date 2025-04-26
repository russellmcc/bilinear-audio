use arrayvec::ArrayVec;
use rand::{Rng, seq::SliceRandom};

pub const CHANNELS: usize = 8;

#[derive(Debug, Clone, PartialEq)]
pub struct Shuffler {
    reverses: ArrayVec<usize, CHANNELS>,
    shuffle: [usize; CHANNELS],
    scale: f32,
}

// Base case for size 2
fn hadamard_1(input: [f32; 1]) -> [f32; 1] {
    [input[0]]
}

macro_rules! hadamard {
    ($name:ident, $size:expr, $half_name:ident) => {
        fn $name(input: [f32; $size]) -> [f32; $size] {
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
            let hadamard_a = $half_name(a_arr);

            let mut b_arr = [0.0; $size / 2];
            dsp::iter::move_into(b, b_arr.iter_mut());
            let hadamard_b = $half_name(b_arr);

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
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
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

    #[must_use]
    pub fn shuffle(&self, input: &[f32; CHANNELS]) -> [f32; CHANNELS] {
        // Step 1: shuffle with hadamard transform
        let mut hadamard = hadamard_8(*input);

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
mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;
    use conformal_component::audio::all_approx_eq;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn energy_preserving() {
        let shuffler = Shuffler::new(&mut Xoshiro256PlusPlus::seed_from_u64(369));
        let input = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let output = shuffler.shuffle(&input);
        assert_approx_eq!(
            output.iter().map(|x| x * x).sum::<f32>(),
            input.iter().map(|x| x * x).sum::<f32>(),
            1e-6
        );
    }

    #[test]
    fn stateless() {
        let shuffler = Shuffler::new(&mut Xoshiro256PlusPlus::seed_from_u64(369));
        let input = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let output = shuffler.shuffle(&input);
        assert!(all_approx_eq(shuffler.shuffle(&input), output, 1e-6));
    }

    #[test]
    fn seed_matters() {
        let shuffler_a = Shuffler::new(&mut Xoshiro256PlusPlus::seed_from_u64(369));
        let shuffler_b = Shuffler::new(&mut Xoshiro256PlusPlus::seed_from_u64(420));
        let input = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert!(!all_approx_eq(
            shuffler_a.shuffle(&input),
            shuffler_b.shuffle(&input),
            1e-6
        ));
    }
}
