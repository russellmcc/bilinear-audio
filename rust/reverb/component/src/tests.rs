use std::collections::HashMap;

use component_snapshots::{
    ProcessingParams,
    effect::{generate_basic_snapshot, generate_snapshot, generate_snapshot_with_reset},
};
use conformal_component::{ProcessingMode, audio::all_approx_eq, parameters::InternalValue};
use snapshots::assert_snapshot;

use super::*;

#[test]
fn reset() {
    let test_sig = dsp::test_utils::sine(25, 440. / 48000.);
    let (before, after) = generate_snapshot_with_reset(
        &Component::new(),
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
fn buffer_size_agnostic() {
    let test_sig: Vec<f32> = dsp::test_utils::sine(3000, 440. / 48000.);
    let buff_512 = generate_snapshot(
        &Component::new(),
        &test_sig,
        &ProcessingParams {
            max_buffer_size: 512,
            ..Default::default()
        },
        &HashMap::new(),
    );
    let buff_1024 = generate_snapshot(
        &Component::new(),
        &test_sig,
        &ProcessingParams {
            max_buffer_size: 1024,
            ..Default::default()
        },
        &HashMap::new(),
    );
    assert!(all_approx_eq(buff_512, buff_1024, 1e-6));
}

fn impulse_response_for_params(params: HashMap<&'_ str, InternalValue>) -> Vec<f32> {
    const SNAPSHOT_LENGTH: usize = 48_000 * 2;
    let mut impulse_vec = vec![0.0; SNAPSHOT_LENGTH];
    impulse_vec[0] = 1.0;
    generate_basic_snapshot(&Component::new(), &impulse_vec, &params)
}

#[test]
#[cfg_attr(miri, ignore)]
fn impulse_default() {
    assert_snapshot!(
        "impulse_default",
        48000,
        impulse_response_for_params(HashMap::new())
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn impulse_short() {
    assert_snapshot!(
        "impulse_short",
        48000,
        impulse_response_for_params(HashMap::from([("time", InternalValue::Numeric(0.7))]))
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn impulse_long() {
    assert_snapshot!(
        "impulse_long",
        48000,
        impulse_response_for_params(HashMap::from([("time", InternalValue::Numeric(3.1))]))
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn impulse_dark() {
    assert_snapshot!(
        "impulse_dark",
        48000,
        impulse_response_for_params(HashMap::from([
            ("brightness", InternalValue::Numeric(0.0)),
            ("tone", InternalValue::Numeric(0.0)),
        ]))
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn impulse_mid_dark() {
    assert_snapshot!(
        "impulse_mid_dark",
        48000,
        impulse_response_for_params(HashMap::from([
            ("brightness", InternalValue::Numeric(50.0)),
            ("tone", InternalValue::Numeric(50.0)),
        ]))
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn impulse_early_reflections_late() {
    assert_snapshot!(
        "impulse_early_reflections_late",
        48000,
        impulse_response_for_params(HashMap::from([(
            "early_reflections",
            InternalValue::Numeric(100.0)
        )]))
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn impulse_early_reflections_early() {
    assert_snapshot!(
        "impulse_early_reflections_early",
        48000,
        impulse_response_for_params(HashMap::from([(
            "early_reflections",
            InternalValue::Numeric(0.0)
        )]))
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn impulse_mid_density() {
    assert_snapshot!(
        "impulse_mid_density",
        48000,
        impulse_response_for_params(HashMap::from([("density", InternalValue::Numeric(50.0))]))
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn impulse_low_density() {
    assert_snapshot!(
        "impulse_low_density",
        48000,
        impulse_response_for_params(HashMap::from([("density", InternalValue::Numeric(0.0))]))
    );
}
