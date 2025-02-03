#![allow(clippy::cast_possible_truncation)]

use super::*;
use crate::diffuser::CHANNELS;
use snapshots::assert_snapshot;

fn impulse_response_for_damping(name: &str, damping: f32, depth: f32, rate: f32) {
    const SNAPSHOT_LENGTH: usize = 48_000 * 2;
    const FEEDBACK: f32 = 0.85;
    const SAMPLING_RATE: f32 = 48000.0;
    const DELAYS_MS: [f32; CHANNELS] = [
        110.147_446,
        113.727_97,
        132.422_56,
        148.699_4,
        162.113_42,
        166.034_85,
        177.914_43,
        193.846_86,
    ];
    let mut feedback_loop = MultiChannelFeedbackLoop::new(
        DELAYS_MS.map(|d| (d / 1000.0 * SAMPLING_RATE).round() as usize),
        SAMPLING_RATE,
    );
    let mut output = vec![0.0; SNAPSHOT_LENGTH];
    output[0] = feedback_loop.process(
        [1.0; CHANNELS],
        FEEDBACK,
        damping,
        depth * SAMPLING_RATE,
        rate / SAMPLING_RATE,
    )[0];
    for output in output.iter_mut().skip(1) {
        *output = feedback_loop.process(
            [0.0; CHANNELS],
            FEEDBACK,
            damping,
            depth * SAMPLING_RATE,
            rate / SAMPLING_RATE,
        )[0];
    }
    assert_snapshot!(name, 48000, output);
}

#[test]
fn impulse_response() {
    impulse_response_for_damping("impulse_response", 1.0, 0.0, 0.0);
}

#[test]
fn impulse_response_damped() {
    impulse_response_for_damping("impulse_response_damped", 0.5, 0.0, 0.0);
}

#[test]
fn impulse_response_modulated() {
    impulse_response_for_damping("impulse_response_modulated", 1.0, 0.004, 6.0);
}
