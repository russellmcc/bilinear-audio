use std::{collections::HashMap, ops::Range};

use conformal_component::{
    audio::{Buffer, BufferData, ChannelLayout},
    events::{Data, Event, Events, NoteData, NoteID},
    parameters::{
        override_defaults, to_infos, BufferStates, ConstantBufferStates, InternalValue, StatesMap,
    },
    synth::{Synth, CONTROLLER_PARAMETERS},
    Component, ProcessingEnvironment, Processor,
};
use dsp::iter::move_into;

use super::ProcessingParams;

/// Generate a snapshot of the effect with the given parameters.
fn generate_buffer_snapshot_with_params(
    synth: &mut impl Synth,
    num_frames: usize,
    params: impl BufferStates,
    events: impl Iterator<Item = Event> + Clone,
) -> Vec<f32> {
    let mut output_buffer = BufferData::new(ChannelLayout::Mono, num_frames);
    synth.process(
        Events::new(events, num_frames).unwrap(),
        params,
        &mut output_buffer,
    );
    output_buffer.channel(0).to_vec()
}

fn intersect_range(range: Range<usize>, num_frames: usize) -> Range<usize> {
    range.start.max(0)..range.end.min(num_frames)
}

fn events_for_range(
    events: impl Iterator<Item = Event> + Clone,
    range: Range<usize>,
) -> impl Iterator<Item = Event> + Clone {
    events.filter_map(move |event| {
        if event.sample_offset >= range.start && event.sample_offset < range.end {
            Some(Event {
                sample_offset: event.sample_offset - range.start,
                data: event.data,
            })
        } else {
            None
        }
    })
}

pub fn generate_snapshot_with_params(
    effect: &mut impl Synth,
    num_frames: usize,
    max_buffer_size: usize,
    params: &(impl BufferStates + Clone),
    events: &(impl Iterator<Item = Event> + Clone),
) -> Vec<f32> {
    let mut output = vec![0.0; num_frames];

    let mut current_range = 0..max_buffer_size;

    while current_range.start < num_frames {
        // Intersect current range with 0..num_frames
        let current_buffer_range = intersect_range(current_range, num_frames);
        let current_buffer_events = events_for_range(events.clone(), current_buffer_range.clone());
        let current_buffer_output = generate_buffer_snapshot_with_params(
            effect,
            current_buffer_range.len(),
            params.clone(),
            current_buffer_events,
        );

        move_into(
            current_buffer_output.into_iter(),
            &mut output[current_buffer_range.clone()],
        );

        current_range = current_buffer_range.end..current_buffer_range.end + max_buffer_size;
    }
    output
}

pub fn generate_snapshot<S: ::std::hash::BuildHasher>(
    component: &impl Component<Processor: Synth>,
    num_frames: usize,
    processing_params: &ProcessingParams,
    param_overrides: &HashMap<&'_ str, InternalValue, S>,
    events: &(impl Iterator<Item = Event> + Clone),
) -> Vec<f32> {
    let mut synth = component.create_processor(&ProcessingEnvironment {
        sampling_rate: processing_params.sampling_rate,
        max_samples_per_process_call: processing_params.max_buffer_size,
        channel_layout: ChannelLayout::Mono,
        processing_mode: processing_params.processing_mode,
    });
    let params = ConstantBufferStates::new(StatesMap::from(override_defaults(
        component
            .parameter_infos()
            .iter()
            .map(Into::into)
            .chain(to_infos(&CONTROLLER_PARAMETERS).iter().map(Into::into)),
        param_overrides,
    )));

    synth.set_processing(true);
    generate_snapshot_with_params(
        &mut synth,
        num_frames,
        processing_params.max_buffer_size,
        &params,
        events,
    )
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
pub fn get_single_note_events(num_frames: usize) -> impl Iterator<Item = Event> + Clone {
    let events = [
        Event {
            sample_offset: 0,
            data: Data::NoteOn {
                data: NoteData {
                    id: NoteID::from_pitch(60),
                    pitch: 60,
                    velocity: 1.0,
                    tuning: 0.0,
                },
            },
        },
        Event {
            sample_offset: (num_frames as f32 * 0.8) as usize,
            data: Data::NoteOff {
                data: NoteData {
                    id: NoteID::from_pitch(60),
                    pitch: 60,
                    velocity: 1.0,
                    tuning: 0.0,
                },
            },
        },
    ];
    events.into_iter()
}

pub fn generate_snapshot_with_reset<S: ::std::hash::BuildHasher>(
    component: &impl Component<Processor: Synth>,
    num_frames: usize,
    processing_params: &ProcessingParams,
    param_overrides: &HashMap<&'_ str, InternalValue, S>,
    events: &(impl Iterator<Item = Event> + Clone),
) -> (Vec<f32>, Vec<f32>) {
    let mut synth = component.create_processor(&ProcessingEnvironment {
        sampling_rate: processing_params.sampling_rate,
        max_samples_per_process_call: processing_params.max_buffer_size,
        channel_layout: ChannelLayout::Mono,
        processing_mode: processing_params.processing_mode,
    });
    synth.set_processing(true);
    let params = ConstantBufferStates::new(StatesMap::from(override_defaults(
        component
            .parameter_infos()
            .iter()
            .map(Into::into)
            .chain(to_infos(&CONTROLLER_PARAMETERS).iter().map(Into::into)),
        param_overrides,
    )));
    let before = generate_snapshot_with_params(
        &mut synth,
        num_frames,
        processing_params.max_buffer_size,
        &params,
        events,
    );
    synth.set_processing(false);
    synth.set_processing(true);
    let after = generate_snapshot_with_params(
        &mut synth,
        num_frames,
        processing_params.max_buffer_size,
        &params,
        events,
    );
    (before, after)
}

pub fn generate_separate_events_snapshot<S: ::std::hash::BuildHasher>(
    component: &impl Component<Processor: Synth>,
    num_frames: usize,
    processing_params: &ProcessingParams,
    param_overrides: &HashMap<&'_ str, InternalValue, S>,
    events: impl Iterator<Item = Event>,
) -> Vec<f32> {
    let mut synth = component.create_processor(&ProcessingEnvironment {
        sampling_rate: processing_params.sampling_rate,
        max_samples_per_process_call: processing_params.max_buffer_size,
        channel_layout: ChannelLayout::Mono,
        processing_mode: processing_params.processing_mode,
    });
    let params_state = StatesMap::from(override_defaults(
        component
            .parameter_infos()
            .iter()
            .map(Into::into)
            .chain(to_infos(&CONTROLLER_PARAMETERS).iter().map(Into::into)),
        param_overrides,
    ));
    let params = ConstantBufferStates::new(params_state.clone());

    let mut output = vec![0.0; num_frames];
    let mut last_processed = 0usize;
    let mut events = events.peekable();
    while last_processed < num_frames {
        while let Some(event) = events.peek() {
            if event.sample_offset > last_processed {
                break;
            }

            synth.handle_events(std::iter::once(event.data.clone()), params_state.clone());
            events.next();
        }
        let process_to = events.peek().map_or(num_frames, |e| e.sample_offset);
        let process_range = last_processed..process_to;
        let process_output = generate_snapshot_with_params(
            &mut synth,
            process_range.len(),
            processing_params.max_buffer_size,
            &params,
            &std::iter::empty(),
        );
        move_into(process_output.into_iter(), &mut output[process_range]);
        last_processed = process_to;
    }
    output
}

pub fn generate_basic_snapshot<S: ::std::hash::BuildHasher>(
    component: &impl Component<Processor: Synth>,
    num_frames: usize,
    param_overrides: &HashMap<&'_ str, InternalValue, S>,
) -> Vec<f32> {
    generate_snapshot(
        component,
        num_frames,
        &ProcessingParams::default(),
        param_overrides,
        &get_single_note_events(num_frames),
    )
}
