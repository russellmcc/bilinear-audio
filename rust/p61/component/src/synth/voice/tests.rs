use crate::PARAMETERS;
use assert_approx_eq::assert_approx_eq;
use conformal_component::{
    events::{NoteData, NoteID},
    parameters::{ConstantBufferStates, InternalValue, StatesMap, override_synth_defaults},
};
use conformal_poly::{
    Event, EventData, NoteExpressionCurve, NoteExpressionPoint, NoteExpressionState,
    Voice as VoiceT, default_note_expression_curve,
};
use snapshots::assert_snapshot;
use std::collections::HashMap;

use super::{Dco2Shape, SharedData, Voice};

fn get_silent_mg(len: usize) -> Vec<f32> {
    vec![0f32; len]
}

fn get_sine_mg(incr: f32, len: usize) -> Vec<f32> {
    (0..len)
        .map(|x| (x as f32 * incr * std::f32::consts::TAU).sin())
        .collect()
}

fn get_shared_data_from_mg<'a, 'b: 'a>(mg: &'a Vec<f32>, wheel_mg: &'b Vec<f32>) -> SharedData<'a> {
    SharedData {
        mg_data: &mg,
        wheel_data: &wheel_mg,
    }
}

fn dummy_params() -> ConstantBufferStates<StatesMap> {
    dummy_params_with(&[])
}

fn dummy_params_with(extra_params: &[(&str, InternalValue)]) -> ConstantBufferStates<StatesMap> {
    ConstantBufferStates::new(StatesMap::from(override_synth_defaults(
        PARAMETERS.iter().cloned(),
        &HashMap::<_, _>::from_iter(
            [
                ("dco1_width", InternalValue::Numeric(25.0)),
                ("dco2_shape", InternalValue::Enum(Dco2Shape::Saw as u32)),
                ("vcf_cutoff", InternalValue::Numeric(0.0)),
                ("vcf_resonance", InternalValue::Numeric(14.2)),
                ("vcf_tracking", InternalValue::Numeric(0.0)),
                ("vcf_env", InternalValue::Numeric(100.0)),
                ("attack", InternalValue::Numeric(0.01)),
                ("decay", InternalValue::Numeric(0.1)),
                ("sustain", InternalValue::Numeric(80.0)),
                ("release", InternalValue::Numeric(0.2)),
                ("vca_level", InternalValue::Numeric(100.0)),
                ("mg_pitch", InternalValue::Numeric(100.0)),
            ]
            .into_iter()
            .chain(extra_params.iter().cloned()),
        ),
    )))
}

#[test]
fn reset_basics() {
    let mut voice = Voice::new(100, 48000.0);
    let mut output = vec![0f32; 100];
    let events = vec![Event {
        sample_offset: 0,
        data: EventData::NoteOn {
            data: NoteData {
                id: NoteID::from_pitch(60),
                pitch: 60,
                velocity: 1.0,
                tuning: 0.0,
            },
        },
    }];

    let params = dummy_params();
    voice.process(
        events.iter().cloned(),
        &params,
        default_note_expression_curve(),
        get_shared_data_from_mg(&get_silent_mg(output.len()), &get_silent_mg(output.len())),
        &mut output,
    );
    voice.reset();
    let mut reset = vec![0f32; 100];
    voice.process(
        events.iter().cloned(),
        &params,
        default_note_expression_curve(),
        get_shared_data_from_mg(&get_silent_mg(output.len()), &get_silent_mg(output.len())),
        &mut reset,
    );
    for (a, b) in output.iter().zip(reset.iter()) {
        assert_approx_eq!(a, b);
    }
}

fn snapshot_for_data_and_params(
    data: SharedData<'_>,
    params: ConstantBufferStates<StatesMap>,
    expression: NoteExpressionCurve<impl Iterator<Item = NoteExpressionPoint> + Clone>,
) -> Vec<f32> {
    let num_samples = data.mg_data.len();
    let mut voice = Voice::new(num_samples, 48000.0);
    let mut output = vec![0f32; num_samples];
    let events = vec![
        Event {
            sample_offset: 0,
            data: EventData::NoteOn {
                data: NoteData {
                    id: NoteID::from_pitch(60),
                    pitch: 60,
                    velocity: 1.0,
                    tuning: 0.0,
                },
            },
        },
        Event {
            sample_offset: 40000,
            data: EventData::NoteOff {
                data: NoteData {
                    id: NoteID::from_pitch(60),
                    pitch: 60,
                    velocity: 1.0,
                    tuning: 0.0,
                },
            },
        },
    ];

    voice.process(
        events.iter().cloned(),
        &params,
        expression,
        data,
        &mut output,
    );
    output
}

fn snapshot_for_data(data: SharedData<'_>) -> Vec<f32> {
    snapshot_for_data_and_params(data, dummy_params(), default_note_expression_curve())
}

#[test]
#[cfg_attr(miri, ignore)]
fn basic_snapshot() {
    assert_snapshot!(
        "basic",
        48000,
        snapshot_for_data(get_shared_data_from_mg(
            &get_silent_mg(48000),
            &get_silent_mg(48000)
        ))
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn modulated_snapshot() {
    assert_snapshot!(
        "modulated",
        48000,
        snapshot_for_data(get_shared_data_from_mg(
            &get_sine_mg(4.0 / 48000.0, 48000),
            &get_silent_mg(48000)
        ))
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn wheel_snapshot() {
    assert_snapshot!(
        "wheel",
        48000,
        snapshot_for_data_and_params(
            get_shared_data_from_mg(&get_silent_mg(48000), &get_sine_mg(4.0 / 48000.0, 48000)),
            dummy_params_with(&[("mod_wheel", InternalValue::Numeric(1.0))]),
            default_note_expression_curve()
        )
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn pitch_bend_snapshot() {
    assert_snapshot!(
        "pitch_bend_expression",
        48000,
        snapshot_for_data_and_params(
            get_shared_data_from_mg(&get_silent_mg(48000), &get_silent_mg(48000)),
            dummy_params(),
            NoteExpressionCurve::new(
                [
                    NoteExpressionPoint {
                        sample_offset: 0,
                        state: NoteExpressionState {
                            pitch_bend: 0f32,
                            timbre: 0f32,
                            aftertouch: 0f32,
                        },
                    },
                    NoteExpressionPoint {
                        sample_offset: 20000,
                        state: NoteExpressionState {
                            pitch_bend: 12f32,
                            timbre: 0f32,
                            aftertouch: 0f32,
                        },
                    },
                ]
                .into_iter()
            )
            .unwrap()
        )
    );
}
