use std::collections::HashMap;

use super::*;
use component_snapshots::{
    ProcessingParams,
    effect::{generate_basic_snapshot, generate_snapshot_with_reset},
};
use conformal_component::{ProcessingMode, audio::all_approx_eq};
use snapshots::assert_snapshot;

#[test]
fn reset() {
    let test_sig = dsp::test_utils::sine(25, 440. / 48000.);
    let (before, after) = generate_snapshot_with_reset(
        &Component {},
        &test_sig,
        &ProcessingParams {
            max_buffer_size: 100,
            sampling_rate: 48000.0,
            processing_mode: ProcessingMode::Realtime,
        },
        &HashMap::new(),
    );
    assert!(all_approx_eq(before, after, 1e-6));
}

#[test]
#[cfg_attr(miri, ignore)]
fn snapshot_sine() {
    let test_sig: Vec<_> = dsp::test_utils::sine(48000, 440. / 48000.)
        .iter()
        .map(|x| x * 1. / 3.)
        .collect();
    assert_snapshot!(
        "sine",
        48000,
        generate_basic_snapshot(&Component {}, &test_sig, &HashMap::new())
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn snapshot_sweep() {
    let test_sig: Vec<_> = dsp::test_utils::linear_sine_sweep(48000, 48000., 10., 20000.)
        .iter()
        .map(|x| x * 1. / 4.)
        .collect();
    assert_snapshot!(
        "sweep",
        48000,
        generate_basic_snapshot(&Component {}, &test_sig, &HashMap::new())
    );
}
