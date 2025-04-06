use snapshots::assert_snapshot;

use super::*;

fn snapshot_for_settings(settings: Settings, output: usize) -> Vec<f32> {
    let mut oscillators = Oscillators::new();
    std::iter::repeat_with(|| oscillators.run(settings)[output])
        .take(48000)
        .collect()
}

#[test]
#[cfg_attr(miri, ignore)]
#[allow(clippy::cast_precision_loss)]
fn default_saw_snapshot() {
    const PITCH_HZ: f32 = 8000.0;
    const SAMPLE_RATE: i32 = 48000;
    const INCREMENT: f32 = PITCH_HZ / SAMPLE_RATE as f32;
    assert_snapshot!(
        "basic",
        SAMPLE_RATE,
        snapshot_for_settings(
            Settings {
                shapes: [Shape::default(), Shape::default()],
                increments: [INCREMENT, INCREMENT],
            },
            0
        )
    );
}
