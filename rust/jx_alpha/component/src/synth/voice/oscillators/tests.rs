use conformal_component::audio::all_approx_eq;
use snapshots::assert_snapshot;

use super::*;

const PITCH_HZ: f32 = 4242.0;
const SAMPLE_RATE: i32 = 48000;

#[allow(clippy::cast_precision_loss)]
const INCREMENT: f32 = PITCH_HZ / SAMPLE_RATE as f32;

fn snapshot_for_settings(settings: Settings, length: usize) -> Vec<f32> {
    let mut oscillators = Oscillators::new();
    std::iter::repeat_with(|| oscillators.run(settings))
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
                shapes: [Default::default(), Default::default()],
                increments: [INCREMENT, INCREMENT],
                sub_shape: Default::default(),
            },
            48000
        )
    );
}

#[test]
fn all_off_silent() {
    const LENGTH: usize = 50;
    let snapshot = snapshot_for_settings(
        Settings {
            shapes: [
                Shape {
                    saw: SawShape::Off,
                    pulse: PulseShape::Off,
                },
                Default::default(),
            ],
            increments: [INCREMENT, INCREMENT],
            sub_shape: Default::default(),
        },
        LENGTH,
    );
    let silence = vec![0.0; LENGTH];
    assert!(all_approx_eq(snapshot, silence, 1e-5));
}
