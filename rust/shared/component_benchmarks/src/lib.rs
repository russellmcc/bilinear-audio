#![allow(clippy::implicit_hasher)]

use conformal_component::{
    Component, ProcessingEnvironment, ProcessingMode, Processor,
    audio::{BufferData, BufferMut, ChannelLayout},
    effect::Effect,
    events,
    parameters::{self, InternalValue, RampedStatesMap, StatesMap, override_defaults},
    synth::{CONTROLLER_PARAMETERS, Synth},
};
use criterion::{BenchmarkId, Criterion, Throughput, black_box};
use dsp::test_utils::white_noise;
use std::collections::HashMap;

pub fn benchmark_effect_mono_process<C: Component<Processor: Effect>>(
    name: &str,
    overrides: &HashMap<&'_ str, InternalValue>,
    c: &mut Criterion,
    f: impl Fn() -> C,
) {
    let mut group = c.benchmark_group(name);
    for buffer_size in &[32, 128, 512] {
        group.throughput(Throughput::Elements(*buffer_size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(buffer_size),
            buffer_size,
            |b, &buffer_size| {
                let mut input = BufferData::new(ChannelLayout::Mono, buffer_size);
                dsp::iter::move_into(
                    white_noise(buffer_size).iter().copied(),
                    input.channel_mut(0),
                );
                let input = input;
                let mut output = BufferData::new(ChannelLayout::Mono, buffer_size);
                let component = f();
                let params = RampedStatesMap::new_const(
                    component.parameter_infos().iter().map(Into::into),
                    overrides,
                );
                let mut effect = component.create_processor(&ProcessingEnvironment {
                    sampling_rate: 48000.0,
                    max_samples_per_process_call: buffer_size,
                    channel_layout: ChannelLayout::Mono,
                    processing_mode: ProcessingMode::Realtime,
                });
                effect.set_processing(true);
                b.iter(|| {
                    effect.process(
                        black_box(params.clone()),
                        black_box(&input),
                        black_box(&mut output),
                    );
                });
            },
        );
    }
}

pub fn benchmark_effect_stereo_process<C: Component<Processor: Effect>>(
    name: &str,
    overrides: &HashMap<&'_ str, InternalValue>,
    c: &mut Criterion,
    f: impl Fn() -> C,
) {
    let mut group = c.benchmark_group(name);
    for buffer_size in &[32, 128, 512] {
        group.throughput(Throughput::Elements(*buffer_size as u64 * 2));
        group.bench_with_input(
            BenchmarkId::from_parameter(buffer_size),
            buffer_size,
            |b, &buffer_size| {
                let mut input = BufferData::new(ChannelLayout::Stereo, buffer_size);
                for idx in [0, 1] {
                    dsp::iter::move_into(
                        white_noise(buffer_size).iter().copied(),
                        input.channel_mut(idx),
                    );
                }
                let input = input;
                let mut output = BufferData::new(ChannelLayout::Stereo, buffer_size);
                let component = f();
                let params = RampedStatesMap::new_const(
                    component.parameter_infos().iter().map(Into::into),
                    overrides,
                );
                let mut effect = component.create_processor(&ProcessingEnvironment {
                    sampling_rate: 48000.0,
                    max_samples_per_process_call: buffer_size,
                    channel_layout: ChannelLayout::Stereo,
                    processing_mode: ProcessingMode::Realtime,
                });
                effect.set_processing(true);
                b.iter(|| {
                    effect.process(
                        black_box(params.clone()),
                        black_box(&input),
                        black_box(&mut output),
                    );
                });
            },
        );
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn benchmark_synth_process<C: Component<Processor: Synth>>(
    name: &str,
    overrides: &HashMap<&'_ str, InternalValue>,
    notes: u8,
    channel_layout: ChannelLayout,
    c: &mut Criterion,
    f: impl Fn() -> C,
) {
    let mut group = c.benchmark_group(name);
    for buffer_size in &[32, 128, 512] {
        group.throughput(Throughput::Elements(
            *buffer_size as u64 * channel_layout.num_channels() as u64,
        ));
        group.bench_with_input(
            BenchmarkId::from_parameter(buffer_size),
            buffer_size,
            |b, &buffer_size| {
                let mut output = BufferData::new(channel_layout, buffer_size);
                let component = f();
                let user_params = {
                    let mut user_params: Vec<parameters::Info> = component.parameter_infos();
                    user_params.extend(CONTROLLER_PARAMETERS.iter().map(Into::into));
                    user_params
                };
                let params =
                    RampedStatesMap::new_const(user_params.iter().map(Into::into), overrides);
                let mut synth = component.create_processor(&ProcessingEnvironment {
                    sampling_rate: 48000.0,
                    max_samples_per_process_call: buffer_size,
                    channel_layout,
                    processing_mode: ProcessingMode::Realtime,
                });
                synth.set_processing(true);

                // Turn on N notes
                synth.handle_events(
                    (0..notes).map(|i| events::Data::NoteOn {
                        data: events::NoteData {
                            id: events::NoteID::from_id(i.into()),
                            pitch: 60 + i,
                            velocity: 0.8,
                            tuning: 0.,
                        },
                    }),
                    StatesMap::from(override_defaults(
                        component.parameter_infos().iter().map(Into::into),
                        overrides,
                    )),
                );
                let empty_events = [];
                let empty_events =
                    events::Events::new(empty_events.iter().cloned(), buffer_size).unwrap();
                b.iter(|| {
                    synth.process(
                        black_box(empty_events.clone()),
                        black_box(params.clone()),
                        black_box(&mut output),
                    );
                });
            },
        );
    }
}

pub fn benchmark_initialize_mono<C: Component>(name: &str, c: &mut Criterion, f: impl Fn() -> C) {
    c.bench_function(name, |b| {
        b.iter(|| {
            let component = f();
            let _ = component.create_processor(&ProcessingEnvironment {
                sampling_rate: 48000.0,
                max_samples_per_process_call: 512,
                channel_layout: ChannelLayout::Mono,
                processing_mode: ProcessingMode::Realtime,
            });
        });
    });
}

pub fn benchmark_initialize_stereo<C: Component>(name: &str, c: &mut Criterion, f: impl Fn() -> C) {
    c.bench_function(name, |b| {
        b.iter(|| {
            let component = f();
            let _ = component.create_processor(&ProcessingEnvironment {
                sampling_rate: 48000.0,
                max_samples_per_process_call: 512,
                channel_layout: ChannelLayout::Stereo,
                processing_mode: ProcessingMode::Realtime,
            });
        });
    });
}
