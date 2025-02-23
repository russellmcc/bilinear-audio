use component_benchmarks::{
    benchmark_effect_mono_process, benchmark_effect_stereo_process, benchmark_initialize_mono,
    benchmark_initialize_stereo,
};
use criterion::{Criterion, criterion_group, criterion_main};
use reverb_component::Component;

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_initialize_mono("reverb_initialize_mono", c, || Component::default());
    benchmark_initialize_stereo("reverb_initialize_stereo", c, || Component::default());
    benchmark_effect_mono_process(
        "reverb_process_defaults_mono",
        &[].into_iter().collect(),
        c,
        || Component::default(),
    );
    benchmark_effect_stereo_process(
        "reverb_process_defaults_stereo",
        &[].into_iter().collect(),
        c,
        || Component::default(),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
