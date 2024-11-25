#![allow(clippy::cast_possible_truncation)]

use super::*;
use snapshots::assert_snapshot;

#[test]
fn impulse_response() {
    const CHANNELS: usize = 8;
    const SNAPSHOT_LENGTH: usize = 48_000 * 2;
    const FEEDBACK: f32 = 0.85;
    const SAMPLE_RATE: f32 = 48000.0;
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
    let mut feedback_loop = MultiChannelFeedbackLoop::<CHANNELS>::new(
        DELAYS_MS.map(|d| (d / 1000.0 * SAMPLE_RATE).round() as usize),
    );
    let mut output = vec![0.0; SNAPSHOT_LENGTH];
    output[0] = feedback_loop
        .process([1.0; CHANNELS], FEEDBACK)
        .into_iter()
        .sum::<f32>();
    for output in output.iter_mut().skip(1) {
        *output = feedback_loop
            .process([0.0; CHANNELS], FEEDBACK)
            .into_iter()
            .sum::<f32>();
    }
    assert_snapshot!("impulse_response", 48000, output);
}
