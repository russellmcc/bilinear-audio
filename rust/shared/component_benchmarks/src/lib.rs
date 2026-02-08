#![allow(clippy::implicit_hasher)]

use conformal_component::{
    Component, ProcessingEnvironment, ProcessingMode, Processor,
    audio::{BufferData, BufferMut, ChannelLayout},
    effect::{self, Effect},
    events,
    parameters::{
        BufferStates, InternalValue, RampedStatesMap, SynthRampedStatesMap, SynthStatesMap,
    },
    synth::{self, Synth, SynthParamBufferStates, SynthParamStates},
};
use criterion::{BenchmarkId, Criterion, Throughput, black_box};
use dsp::test_utils::white_noise;
use std::collections::HashMap;

struct BenchmarkEffectProcessContext<P> {
    parameters: P,
}

impl<P: BufferStates> effect::ProcessContext for BenchmarkEffectProcessContext<&P> {
    fn parameters(&self) -> &impl BufferStates {
        self.parameters
    }
}

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
                let context = BenchmarkEffectProcessContext {
                    parameters: &params,
                };
                b.iter(|| {
                    effect.process(
                        black_box(&context),
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
                let context = BenchmarkEffectProcessContext {
                    parameters: &params,
                };
                b.iter(|| {
                    effect.process(
                        black_box(&context),
                        black_box(&input),
                        black_box(&mut output),
                    );
                });
            },
        );
    }
}

struct BenchmarkSynthProcessContext<E, P> {
    events: E,
    parameters: P,
}

impl<E: Iterator<Item = events::Event> + Clone, P: SynthParamBufferStates> synth::ProcessContext
    for BenchmarkSynthProcessContext<events::Events<E>, &P>
{
    fn events(&self) -> events::Events<impl Iterator<Item = events::Event> + Clone> {
        self.events.clone()
    }

    fn parameters(&self) -> &impl SynthParamBufferStates {
        self.parameters
    }
}

struct BenchmarkSynthHandleEventsContext<E, P> {
    events: E,
    parameters: P,
}

impl<E: Iterator<Item = events::Data> + Clone, P: SynthParamStates> synth::HandleEventsContext
    for BenchmarkSynthHandleEventsContext<E, &P>
{
    fn events(&self) -> impl Iterator<Item = events::Data> + Clone {
        self.events.clone()
    }

    fn parameters(&self) -> &impl SynthParamStates {
        self.parameters
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
                let user_params = component.parameter_infos();
                let params = SynthRampedStatesMap::new_const(
                    user_params.iter().map(Into::into),
                    overrides,
                    &HashMap::new(),
                    &HashMap::new(),
                );
                let mut synth = component.create_processor(&ProcessingEnvironment {
                    sampling_rate: 48000.0,
                    max_samples_per_process_call: buffer_size,
                    channel_layout,
                    processing_mode: ProcessingMode::Realtime,
                });
                synth.set_processing(true);
                let events_context = BenchmarkSynthHandleEventsContext {
                    events: (0..notes).map(|i| events::Data::NoteOn {
                        data: events::NoteData {
                            id: events::NoteID::from_id(i.into()),
                            pitch: 60 + i,
                            velocity: 0.8,
                            tuning: 0.,
                        },
                    }),
                    parameters: &SynthStatesMap::new_override_defaults(
                        component.parameter_infos().iter().map(Into::into),
                        overrides,
                        &HashMap::new(),
                        &HashMap::new(),
                    ),
                };
                // Turn on N notes
                synth.handle_events(&events_context);

                let empty_events = [];
                let empty_events =
                    events::Events::new(empty_events.iter().cloned(), buffer_size).unwrap();
                let process_context = BenchmarkSynthProcessContext {
                    events: empty_events,
                    parameters: &params,
                };
                b.iter(|| {
                    synth.process(black_box(&process_context), black_box(&mut output));
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
