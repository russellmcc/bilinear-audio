use std::collections::HashMap;

use super::Component;
use component_snapshots::{
    ProcessingParams,
    synth::{
        generate_basic_snapshot, generate_separate_events_snapshot, generate_snapshot,
        generate_snapshot_with_reset, get_single_note_events,
    },
};
use conformal_component::{ProcessingMode, audio::all_approx_eq, parameters::InternalValue};
use snapshots::assert_snapshot;
fn snapshot_param_overrides() -> HashMap<&'static str, InternalValue> {
    HashMap::from([
        ("dco1_width", InternalValue::Numeric(25.0)),
        ("dco2_shape", InternalValue::Enum(1)),
        ("vcf_cutoff", InternalValue::Numeric(0.0)),
        ("vcf_resonance", InternalValue::Numeric(14.2)),
        ("vcf_tracking", InternalValue::Numeric(0.0)),
        ("vcf_env", InternalValue::Numeric(100.0)),
        ("attack", InternalValue::Numeric(0.010)),
        ("decay", InternalValue::Numeric(0.1)),
        ("sustain", InternalValue::Numeric(80.0)),
        ("release", InternalValue::Numeric(0.2)),
        ("vca_level", InternalValue::Numeric(100.0)),
    ])
}

#[test]
fn reset() {
    let (before, after) = generate_snapshot_with_reset(
        &Component {},
        100,
        &ProcessingParams {
            sampling_rate: 48000.0,
            max_buffer_size: 100,
            processing_mode: ProcessingMode::Realtime,
        },
        &snapshot_param_overrides(),
        &get_single_note_events(100),
    );
    assert!(all_approx_eq(before, after, 1e-6));
}

#[test]
fn separate_events() {
    let component = &Component {};
    let buffer_events = generate_snapshot(
        component,
        100,
        &ProcessingParams {
            sampling_rate: 48000.0,
            max_buffer_size: 50,
            processing_mode: ProcessingMode::Realtime,
        },
        &snapshot_param_overrides(),
        &get_single_note_events(100),
    );
    let separate_events = generate_separate_events_snapshot(
        component,
        100,
        &ProcessingParams {
            sampling_rate: 48000.0,
            max_buffer_size: 50,
            processing_mode: ProcessingMode::Realtime,
        },
        &snapshot_param_overrides(),
        get_single_note_events(100),
    );

    assert!(all_approx_eq(buffer_events, separate_events, 1e-6));
}

#[test]
#[cfg_attr(miri, ignore)]
fn snapshot() {
    let component = &Component {};
    let snapshot = generate_basic_snapshot(component, 48000, &snapshot_param_overrides());
    assert_snapshot!("basic", 48000, snapshot);
}

#[test]
#[cfg_attr(miri, ignore)]
fn snapshot_pwm() {
    let component = &Component {};
    let snapshot = generate_basic_snapshot(
        component,
        48000,
        &HashMap::from([
            ("dco1_shape", InternalValue::Enum(2)),
            ("dco1_width", InternalValue::Numeric(90.0)),
            ("vcf_cutoff", InternalValue::Numeric(0.0)),
            ("vcf_resonance", InternalValue::Numeric(14.2)),
            ("vcf_tracking", InternalValue::Numeric(0.0)),
            ("vcf_env", InternalValue::Numeric(100.0)),
            ("attack", InternalValue::Numeric(0.010)),
            ("decay", InternalValue::Numeric(0.1)),
            ("sustain", InternalValue::Numeric(80.0)),
            ("release", InternalValue::Numeric(0.2)),
            ("vca_level", InternalValue::Numeric(100.0)),
            ("mg_rate", InternalValue::Numeric(75.0)),
            ("mg_delay", InternalValue::Numeric(0.8)),
        ]),
    );
    assert_snapshot!("pwm", 48000, snapshot);
}

#[test]
#[cfg_attr(miri, ignore)]
fn snapshot_defaults() {
    let component = &Component {};
    let snapshot = generate_basic_snapshot(component, 48000, &HashMap::new());
    assert_snapshot!("defaults", 48000, snapshot);
}
