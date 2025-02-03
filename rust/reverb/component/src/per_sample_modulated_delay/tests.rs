use snapshots::assert_snapshot;

use super::*;

#[test]
#[allow(clippy::cast_possible_truncation)]
fn lofi_chorus() {
    const SAMPLING_RATE: f32 = 48000.0;
    const MAX_DELAY: usize = (SAMPLING_RATE * 0.005) as usize;
    const DEPTH: f32 = SAMPLING_RATE * 0.004;
    const RATE: f32 = 6.0 / SAMPLING_RATE;

    let test_sig = dsp::test_utils::sine(SAMPLING_RATE as usize, 1123. / SAMPLING_RATE);
    let mut modulated_delay = PerSampleModulatedDelay::new(MAX_DELAY);
    let output = test_sig.iter().map(|x| {
        let y = modulated_delay.read(DEPTH, RATE);
        modulated_delay.write(*x * 0.25);
        y
    });
    assert_snapshot!("lofi_chorus", 48000, output);
}
