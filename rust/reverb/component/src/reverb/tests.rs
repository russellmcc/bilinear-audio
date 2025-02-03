use super::*;
use conformal_component::{
    audio::{BufferData, ChannelLayout},
    ProcessingMode,
};
use snapshots::assert_snapshot;

fn impulse_response_for_params(params: Params) -> Vec<f32> {
    const SNAPSHOT_LENGTH: usize = 48_000 * 2;
    const SAMPLING_RATE: f32 = 48000.0;
    let mut reverb = Reverb::new(&ProcessingEnvironment {
        sampling_rate: SAMPLING_RATE,
        max_samples_per_process_call: SNAPSHOT_LENGTH,
        channel_layout: ChannelLayout::Mono,
        processing_mode: ProcessingMode::Realtime,
    });
    let mut impulse_vec = vec![0.0; SNAPSHOT_LENGTH];
    impulse_vec[0] = 1.0;
    let mut output = BufferData::new_mono(vec![0.0; SNAPSHOT_LENGTH]);
    reverb.process(params, &BufferData::new_mono(impulse_vec), &mut output);
    output.channel(0).iter().copied().collect()
}

#[test]
fn impulse_response() {
    assert_snapshot!(
        "impulse_response",
        48000,
        impulse_response_for_params(Params {
            feedback: 0.6,
            damping: 1.0,
            brightness: 1.0,
            mix: 1.0,
            modulation_depth_seconds: 0.0,
            modulation_rate_hz: 0.0,
        })
    );
}

#[test]
fn impulse_response_modulated_damped() {
    assert_snapshot!(
        "impulse_response_modulated_damped",
        48000,
        impulse_response_for_params(Params {
            feedback: 0.6,
            damping: 0.5,
            brightness: 1.0,
            mix: 1.0,
            modulation_depth_seconds: 0.004,
            modulation_rate_hz: 6.0,
        })
    );
}
