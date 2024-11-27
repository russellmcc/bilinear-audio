use super::*;

use assert_approx_eq::assert_approx_eq;

const CHANNELS: usize = 2;

fn process(
    input: impl IntoIterator<Item = [f32; CHANNELS]>,
    delay: &mut MultiChannelPerSampleDelay<CHANNELS>,
) -> Vec<[f32; CHANNELS]> {
    let mut output = Vec::new();
    for samples in input {
        let x = delay.read();
        delay.write(&samples);
        output.push(x);
    }
    output
}

fn check_process(
    input: impl IntoIterator<Item = [f32; CHANNELS]>,
    expected: impl IntoIterator<Item = [f32; CHANNELS]>,
    delay: &mut MultiChannelPerSampleDelay<CHANNELS>,
) {
    for (actual, expected) in process(input, delay).into_iter().zip(expected) {
        for (a, e) in actual.iter().zip(expected) {
            assert_approx_eq!(a, e, 1e-6);
        }
    }
}

#[test]
fn delay_longer_and_shorter_than_buffer() {
    let mut delay = MultiChannelPerSampleDelay::new([4, 2]);
    check_process(
        [[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]],
        [[0.0, 0.0], [0.0, 0.0], [0.0, 2.0]],
        &mut delay,
    );
    check_process(
        [[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]],
        [[0.0, 4.0], [1.0, 6.0], [3.0, 8.0]],
        &mut delay,
    );
}
