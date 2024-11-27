#![allow(clippy::cast_possible_truncation)]

use super::*;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use snapshots::assert_snapshot;

#[test]
fn impulse_response() {
    const SNAPSHOT_LENGTH: usize = 48_000 * 2;
    const SAMPLE_RATE: f32 = 48000.0;
    const DELAYS_MS: [f32; BLOCKS] = [20.0, 40.0, 80.0, 160.0];
    let mut diffuser = Diffuser::new(
        &mut Xoshiro256PlusPlus::seed_from_u64(369),
        DELAYS_MS.map(|d| (d / 1000.0 * SAMPLE_RATE).round() as usize),
    );
    let mut output = vec![0.0; SNAPSHOT_LENGTH];
    output[0] = diffuser.process(&[1.0; CHANNELS])[0];
    for output in output.iter_mut().skip(1) {
        *output = diffuser.process(&[0.0; CHANNELS])[0];
    }
    assert_snapshot!("impulse_response", 48000, output);
}
