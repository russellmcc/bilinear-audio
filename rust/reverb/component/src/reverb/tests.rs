use super::*;
use conformal_component::{
    audio::{BufferData, ChannelLayout},
    ProcessingMode,
};
use snapshots::assert_snapshot;

#[test]
fn impulse_response() {
    const SNAPSHOT_LENGTH: usize = 48_000 * 2;
    const SAMPLE_RATE: f32 = 48000.0;
    let mut reverb = Reverb::new(&ProcessingEnvironment {
        sampling_rate: SAMPLE_RATE,
        max_samples_per_process_call: SNAPSHOT_LENGTH,
        channel_layout: ChannelLayout::Mono,
        processing_mode: ProcessingMode::Realtime,
    });
    let mut impulse_vec = vec![0.0; SNAPSHOT_LENGTH];
    impulse_vec[0] = 1.0;
    let mut output = BufferData::new_mono(vec![0.0; SNAPSHOT_LENGTH]);
    reverb.process(0.6, &BufferData::new_mono(impulse_vec), &mut output);
    assert_snapshot!("impulse_response", 48000, output.channel(0).iter().copied());
}