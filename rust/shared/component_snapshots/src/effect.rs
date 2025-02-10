use std::collections::HashMap;

use super::ProcessingParams;
use conformal_component::{
    audio::{Buffer, BufferData, BufferMut, ChannelLayout},
    effect::Effect,
    parameters::{override_defaults, BufferStates, ConstantBufferStates, InternalValue, StatesMap},
    Component, ProcessingEnvironment, Processor,
};
use dsp::iter::move_into;

/// Generate a snapshot of the effect with the given parameters.
fn generate_buffer_snapshot_with_params(
    effect: &mut impl Effect,
    input: &[f32],
    params: impl BufferStates,
) -> Vec<f32> {
    let mut input_data = BufferData::new(ChannelLayout::Mono, input.len());
    dsp::iter::move_into(input.iter().copied(), input_data.channel_mut(0));
    let mut output_buffer = BufferData::new(ChannelLayout::Mono, input.len());
    effect.process(params, &input_data, &mut output_buffer);
    output_buffer.channel(0).to_vec()
}

pub fn generate_snapshot_with_params(
    effect: &mut impl Effect,
    input: &[f32],
    max_buffer_size: usize,
    params: &(impl BufferStates + Clone),
) -> Vec<f32> {
    let mut output = vec![0.0; input.len()];

    // Split input into chunks of max_buffer_size
    for (chunk, output_chunk) in input
        .chunks(max_buffer_size)
        .zip(output.chunks_mut(max_buffer_size))
    {
        move_into(
            generate_buffer_snapshot_with_params(effect, chunk, params.clone()).into_iter(),
            output_chunk,
        );
    }

    output
}

/// Generate a snapshot of the effect with the given processing parameters and parameter overrides.
pub fn generate_snapshot<S: ::std::hash::BuildHasher>(
    component: &impl Component<Processor: Effect>,
    input: &[f32],
    processing_params: &ProcessingParams,
    param_overrides: &HashMap<&'_ str, InternalValue, S>,
) -> Vec<f32> {
    let mut effect = component.create_processor(&ProcessingEnvironment {
        sampling_rate: processing_params.sampling_rate,
        max_samples_per_process_call: processing_params.max_buffer_size,
        channel_layout: ChannelLayout::Mono,
        processing_mode: processing_params.processing_mode,
    });
    let params = ConstantBufferStates::new(StatesMap::from(override_defaults(
        component
            .parameter_infos()
            .iter()
            .map(std::convert::Into::into),
        param_overrides,
    )));
    effect.set_processing(true);
    generate_snapshot_with_params(
        &mut effect,
        input,
        processing_params.max_buffer_size,
        &params,
    )
}

/// Generate a snapshot of the effect with default processing parameters.
pub fn generate_basic_snapshot<S: ::std::hash::BuildHasher>(
    component: &impl Component<Processor: Effect>,
    input: &[f32],
    param_overrides: &HashMap<&'_ str, InternalValue, S>,
) -> Vec<f32> {
    generate_snapshot(
        component,
        input,
        &ProcessingParams::default(),
        param_overrides,
    )
}

/// Generates snapshots of an effect before and after a reset.
pub fn generate_snapshot_with_reset<S: ::std::hash::BuildHasher>(
    component: &impl Component<Processor: Effect>,
    input: &[f32],
    processing_params: &ProcessingParams,
    param_overrides: &HashMap<&'_ str, InternalValue, S>,
) -> (Vec<f32>, Vec<f32>) {
    let mut effect = component.create_processor(&ProcessingEnvironment {
        sampling_rate: processing_params.sampling_rate,
        max_samples_per_process_call: processing_params.max_buffer_size,
        channel_layout: ChannelLayout::Mono,
        processing_mode: processing_params.processing_mode,
    });
    effect.set_processing(true);
    let params = ConstantBufferStates::new(StatesMap::from(override_defaults(
        component
            .parameter_infos()
            .iter()
            .map(std::convert::Into::into),
        param_overrides,
    )));
    let before = generate_snapshot_with_params(
        &mut effect,
        input,
        processing_params.max_buffer_size,
        &params,
    );
    effect.set_processing(false);
    effect.set_processing(true);
    let after = generate_snapshot_with_params(
        &mut effect,
        input,
        processing_params.max_buffer_size,
        &params,
    );
    (before, after)
}
