use snapshots::assert_snapshot;

use super::*;

const PITCH_HZ: f32 = 4242.0;
const SAMPLE_RATE: i32 = 48000;

#[allow(clippy::cast_precision_loss)]
const INCREMENT: f32 = PITCH_HZ / SAMPLE_RATE as f32;

fn snapshot_for_settings(settings: Settings, length: usize) -> Vec<f32> {
    let mut oscillators = Oscillators::new();
    std::iter::repeat_with(|| oscillators.generate(&settings))
        .take(length)
        .collect()
}

#[test]
#[cfg_attr(miri, ignore)]
fn default_saw_snapshot() {
    assert_snapshot!(
        "basic",
        SAMPLE_RATE,
        snapshot_for_settings(
            Settings {
                oscillators: [
                    oscillator::Settings {
                        increment: INCREMENT,
                        shape: oscillator::Shape::Saw,
                        gain: 1.0,
                    },
                    oscillator::Settings {
                        increment: INCREMENT,
                        shape: oscillator::Shape::Saw,
                        gain: 0.0,
                    },
                ],
            },
            48000
        )
    );
}
