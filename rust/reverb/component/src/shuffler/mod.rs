use bitvec::{order::Msb0, BitArr};
use rand::{seq::SliceRandom, Rng};

pub const CHANNELS: usize = 8;
type Row = BitArr!(for CHANNELS, in u32, Msb0);
type Matrix = [Row; CHANNELS];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shuffler {
    matrix: Matrix,
}

fn hadamard_matrix() -> Matrix {
    // Use sylvester construction
    let mut matrix = [Row::new([0]); CHANNELS];
    matrix[0].set(0, true);

    let mut done = 1;
    while done < CHANNELS {
        let todo = done * 2;
        for r in 0..done {
            for c in 0..done {
                let val = matrix[c][r];
                matrix[c].set(done + r, val);
                matrix[done + c].set(r, val);
                matrix[done + c].set(done + r, !val);
            }
        }
        done = todo;
    }
    matrix
}

impl Shuffler {
    pub fn new(rng: &mut impl Rng) -> Self {
        let mut matrix = hadamard_matrix();

        // Invert 50% of the rows
        for row in &mut matrix {
            if rng.gen_bool(0.5) {
                for i in 0..CHANNELS {
                    let val = row[i];
                    row.set(i, !val);
                }
            }
        }

        // Shuffle the rows
        matrix.shuffle(rng);

        Self { matrix }
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn shuffle(&self, input: &[f32; CHANNELS]) -> [f32; CHANNELS] {
        let mut output = [0.0; CHANNELS];
        for (i, row) in self.matrix.iter().enumerate() {
            for j in 0..CHANNELS {
                output[i] += if row[j] { input[j] } else { -input[j] };
            }
        }
        output.map(|x| x * (1.0 / CHANNELS as f32).sqrt())
    }
}

#[cfg(test)]
mod tests;
