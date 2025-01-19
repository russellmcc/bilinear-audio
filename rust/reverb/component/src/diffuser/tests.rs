#![allow(clippy::cast_possible_truncation)]

use super::*;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use snapshots::assert_snapshot;

fn impulse_response_snapshot_test(name: &str, run: impl Fn(&mut Diffuser, f32) -> f32) {
    const SNAPSHOT_LENGTH: usize = 48_000 * 2;
    const SAMPLING_RATE: f32 = 48000.0;
    const DELAYS_MS: [f32; BLOCKS] = [20.0, 40.0, 80.0, 160.0];
    let mut diffuser = Diffuser::new(
        &mut Xoshiro256PlusPlus::seed_from_u64(369),
        DELAYS_MS.map(|d| (d / 1000.0 * SAMPLING_RATE).round() as usize),
    );
    let mut output = vec![0.0; SNAPSHOT_LENGTH];
    output[0] = run(&mut diffuser, 1.0);
    for output in output.iter_mut().skip(1) {
        *output = run(&mut diffuser, 0.0);
    }
    assert_snapshot!(name, 48000, output);
}

#[test]
fn impulse_response() {
    impulse_response_snapshot_test("impulse_response", |diffuser, input| {
        diffuser.process_mono(0.0, &[input; CHANNELS]).0[0]
    });
}

#[test]
fn impulse_response_er_low() {
    impulse_response_snapshot_test("er_low", |diffuser, input| {
        diffuser.process_mono(0.0, &[input; CHANNELS]).1
    });
}

#[test]
fn impulse_response_er_high() {
    impulse_response_snapshot_test("er_high", |diffuser, input| {
        diffuser.process_mono(1.0, &[input; CHANNELS]).1
    });
}
