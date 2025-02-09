use component_benchmarks::{
    benchmark_effect_mono_process, benchmark_effect_stereo_process, benchmark_initialize_mono,
    benchmark_initialize_stereo,
};
use criterion::{criterion_group, criterion_main, Criterion};
use dsp::test_utils::white_noise;
use rand::SeedableRng;
use reverb_component::Component;

fn shuffler_benchmark(c: &mut Criterion) {
    c.bench_function("shuffler", |b| {
        let shuffler = reverb_component::shuffler::Shuffler::new(
            &mut rand_xoshiro::Xoshiro256PlusPlus::seed_from_u64(369),
        );
        let mut input = [0.0; reverb_component::shuffler::CHANNELS];
        dsp::iter::move_into(
            white_noise(reverb_component::shuffler::CHANNELS)
                .iter()
                .copied(),
            input.iter_mut(),
        );
        b.iter(|| shuffler.shuffle(&input));
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_initialize_mono("reverb_initialize_mono", c, || Component::default());
    benchmark_initialize_stereo("reverb_initialize_stereo", c, || Component::default());
    benchmark_effect_mono_process(
        "reverb_process_defaults_mono",
        [].into_iter().collect(),
        c,
        || Component::default(),
    );
    benchmark_effect_stereo_process(
        "reverb_process_defaults_stereo",
        [].into_iter().collect(),
        c,
        || Component::default(),
    );
    shuffler_benchmark(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
