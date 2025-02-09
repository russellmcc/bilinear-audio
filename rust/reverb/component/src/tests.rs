use std::collections::HashMap;

use conformal_component::{
    audio::{all_approx_eq, BufferData, ChannelLayout},
    parameters::{override_defaults, ConstantBufferStates, InternalValue, StatesMap},
    ProcessingMode,
};
use dsp::iter::move_into;
use snapshots::assert_snapshot;

use super::*;

// TODO: make this into a helper since it is shared with rchorus?
fn generate_snapshot_with_params(
    effect: &mut impl EffectTrait,
    input: &[f32],
    params: impl BufferStates,
) -> Vec<f32> {
    let mut input_data = BufferData::new(ChannelLayout::Mono, input.len());
    dsp::iter::move_into(input.iter().copied(), input_data.channel_mut(0));
    let mut output_buffer = BufferData::new(ChannelLayout::Mono, input.len());
    effect.process(params, &input_data, &mut output_buffer);
    output_buffer.channel(0).to_vec()
}

struct ProcessingParams {
    sampling_rate: f32,
    max_buffer_size: usize,
    processing_mode: ProcessingMode,
}

impl Default for ProcessingParams {
    fn default() -> Self {
        Self {
            sampling_rate: 48000.0,
            max_buffer_size: 512,
            processing_mode: ProcessingMode::Realtime,
        }
    }
}

fn generate_snapshot(
    component: &impl ComponentTrait<Processor: EffectTrait>,
    input: &[f32],
    processing_params: ProcessingParams,
    param_overrides: HashMap<&'_ str, InternalValue>,
) -> Vec<f32> {
    let mut effect = component.create_processor(&ProcessingEnvironment {
        sampling_rate: processing_params.sampling_rate,
        max_samples_per_process_call: processing_params.max_buffer_size,
        channel_layout: ChannelLayout::Mono,
        processing_mode: processing_params.processing_mode,
    });
    let params = ConstantBufferStates::new(StatesMap::from(override_defaults(
        component.parameter_infos().iter().map(|i| i.into()),
        &param_overrides,
    )));
    let mut output = vec![0.0; input.len()];

    // Split input into chunks of max_buffer_size
    for (chunk, output_chunk) in input
        .chunks(processing_params.max_buffer_size)
        .zip(output.chunks_mut(processing_params.max_buffer_size))
    {
        move_into(
            generate_snapshot_with_params(&mut effect, chunk, params.clone()).into_iter(),
            output_chunk,
        );
    }

    output
}

fn generate_basic_snapshot(
    component: &impl ComponentTrait<Processor: EffectTrait>,
    input: &[f32],
    param_overrides: HashMap<&'_ str, InternalValue>,
) -> Vec<f32> {
    generate_snapshot(
        component,
        input,
        ProcessingParams::default(),
        param_overrides,
    )
}

#[test]
fn reset() {
    let mut effect = Effect::new(&ProcessingEnvironment {
        sampling_rate: 48000.0,
        max_samples_per_process_call: 100,
        channel_layout: ChannelLayout::Mono,
        processing_mode: ProcessingMode::Realtime,
    });
    let test_sig = dsp::test_utils::sine(25, 440. / 48000.);
    effect.set_processing(true);
    let params = ConstantBufferStates::new(StatesMap::from(override_defaults(
        Component::new()
            .parameter_infos()
            .iter()
            .map(std::convert::Into::into),
        &HashMap::new(),
    )));

    let initial = generate_snapshot_with_params(&mut effect, &test_sig, params.clone());
    effect.set_processing(false);
    effect.set_processing(true);
    let reset = generate_snapshot_with_params(&mut effect, &test_sig, params.clone());
    assert!(all_approx_eq(initial, reset, 1e-6));
}

#[test]
#[cfg_attr(miri, ignore)]
fn buffer_size_agnostic() {
    let test_sig = dsp::test_utils::sine(3000, 440. / 48000.);
    let buff_512 = generate_snapshot(
        &Component::new(),
        &test_sig,
        ProcessingParams {
            max_buffer_size: 512,
            ..Default::default()
        },
        HashMap::new(),
    );
    let buff_1024 = generate_snapshot(
        &Component::new(),
        &test_sig,
        ProcessingParams {
            max_buffer_size: 1024,
            ..Default::default()
        },
        HashMap::new(),
    );
    assert!(all_approx_eq(buff_512, buff_1024, 1e-6));
}

fn impulse_response_for_params(params: HashMap<&'_ str, InternalValue>) -> Vec<f32> {
    const SNAPSHOT_LENGTH: usize = 48_000 * 2;
    let mut impulse_vec = vec![0.0; SNAPSHOT_LENGTH];
    impulse_vec[0] = 1.0;
    generate_basic_snapshot(&Component::new(), &impulse_vec, params)
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
