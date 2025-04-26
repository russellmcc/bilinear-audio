use crate::{kernel, polyphase_kernel::PolyphaseKernel};
use dsp::look_behind::{LookBehind, SliceLike};

/// A variable delay allows us to delay a different amount
/// each sample.
#[derive(Clone)]
pub struct ModulatedDelay {
    kernel: PolyphaseKernel,
    lookaround: u16,
    look_behind: LookBehind,
    max_delay: usize,
}

pub struct Options {
    /// This acts as a quality control for the filter.
    ///
    /// Higher lookarounds will have lower aliasing and better high-frequency fidelity,
    /// but this comes with two downsides: CPU cost is higher with higher lookarounds,
    /// and the delay must never be less than the lookaround (which is measured in samples).
    pub lookaround: u16,

    /// The maximum delay in samples.
    pub max_delay: usize,

    /// The maximum buffer size in samples.
    pub max_samples_per_process_call: usize,
}

fn dot<'a, 'b>(a: impl IntoIterator<Item = &'a f32>, b: impl IntoIterator<Item = &'b f32>) -> f32 {
    a.into_iter().zip(b).map(|(a, b)| a * b).sum()
}

pub struct Buffer<'a, B> {
    kernel: &'a PolyphaseKernel,
    lookaround: u16,
    max_delay: usize,
    view: B,
}

impl<'a, B: SliceLike + 'a> Buffer<'a, B> {
    pub fn process<'b: 'a, IDelay: std::iter::IntoIterator<Item = f32> + 'b>(
        &'a self,
        delay: IDelay,
    ) -> impl Iterator<Item = f32> + use<'a, B, IDelay> {
        delay.into_iter().enumerate().map(|(i, d)| {
            debug_assert!(d >= 0f32);

            #[allow(clippy::cast_possible_truncation)]
            let di = d.round() as usize;
            #[allow(clippy::cast_precision_loss)]
            let frac = d - di as f32;

            #[allow(clippy::cast_possible_truncation)]
            let phase = ((frac + 0.5) * f32::from(self.kernel.phases())).floor() as u16;
            debug_assert!(di <= self.max_delay);
            debug_assert!(di >= self.lookaround as usize);
            debug_assert!(self.kernel.phase(phase).len() == 2 * self.lookaround as usize + 1);
            #[allow(clippy::range_plus_one)]
            let buffer_range = (i + self.max_delay - di)
                ..(i + self.max_delay - di + 2 * self.lookaround as usize + 1);
            dot(self.kernel.phase(phase), self.view.range(buffer_range))
        })
    }
}

impl ModulatedDelay {
    pub fn new(
        Options {
            lookaround,
            max_delay,
            max_samples_per_process_call,
        }: Options,
    ) -> Self {
        // Bandwidth relative to nyquist. This can reduce aliasing at the cost of
        // attenuating high frequencies (which are likely inaudible anyways)
        const BANDWIDTH: f32 = 0.85;

        let length_per_phase = lookaround * 2 + 1;
        let num_phases: u16 = 1025;
        let mut kernel = kernel::lpf(kernel::LpfOptions {
            length: length_per_phase * num_phases,
            increment: BANDWIDTH * 0.5 / f32::from(num_phases),
        });
        for tap in &mut kernel {
            *tap *= BANDWIDTH;
        }
        let kernel = PolyphaseKernel::split(&kernel, num_phases);
        Self {
            kernel,
            lookaround,
            look_behind: LookBehind::new(
                max_delay + lookaround as usize,
                max_samples_per_process_call,
            ),
            max_delay,
        }
    }

    pub fn reset(&mut self) {
        self.look_behind.reset();
    }

    pub fn process<'a, 'b: 'a, IAudio: std::iter::IntoIterator<Item = f32>>(
        &'a mut self,
        input: IAudio,
    ) -> Buffer<'a, impl SliceLike> {
        let input = input.into_iter();
        let view = self.look_behind.process(input);
        let max_delay = self.max_delay;
        let lookaround = self.lookaround;
        let kernel = &self.kernel;
        Buffer {
            kernel,
            lookaround,
            max_delay,
            view,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;
    use dsp::test_utils::{estimate_tuning, linear_sine_sweep, sine};
    use snapshots::assert_snapshot;

    #[test]
    #[cfg_attr(miri, ignore)]
    pub fn fractional_offset_does_not_change_tuning() {
        let mut variable_delay = ModulatedDelay::new(Options {
            lookaround: 1,
            max_delay: 3,
            max_samples_per_process_call: 4096,
        });
        let incr = 440f32 / 44100f32;
        let input = sine(4096 + 3, incr);
        let delay: Vec<_> = std::iter::repeat(2.5f32).take(3).collect();
        variable_delay
            .process(input[..3].iter().cloned())
            .process(delay.iter().cloned())
            .for_each(drop);
        let delay: Vec<_> = std::iter::repeat(2.5f32).take(4096).collect();
        let estimated_incr = estimate_tuning(
            variable_delay
                .process(input[3..].iter().cloned())
                .process(delay.iter().cloned())
                .collect::<Vec<_>>()
                .as_mut_slice(),
        );
        assert_approx_eq!(incr, estimated_incr, 1e-4);
    }

    fn process_vibrato(sampling_rate: i32, input: &[f32]) -> Vec<f32> {
        let num_samples = input.len();
        let max_delay_ms = 15f32;
        let min_delay_ms = 5f32;
        let max_delay_samples = (max_delay_ms * sampling_rate as f32 / 1000f32).ceil() as usize;
        let lookaround = 8 as u16;
        let min_delay_samples = (min_delay_ms * sampling_rate as f32 / 1000f32).floor() as usize;
        assert!(min_delay_samples >= lookaround as usize);
        let mut variable_delay = ModulatedDelay::new(Options {
            lookaround,
            max_delay: max_delay_samples,
            max_samples_per_process_call: 512,
        });
        let input = input.iter().map(|x| x * 0.5).collect::<Vec<_>>();
        let vibrato_input = sine(num_samples, 5f32 / sampling_rate as f32)
            .iter()
            .map(|x| {
                let delay_ms =
                    x * 0.5 * (max_delay_ms - min_delay_ms) + (max_delay_ms + min_delay_ms) / 2f32;
                delay_ms * sampling_rate as f32 / 1000f32
            })
            .collect::<Vec<_>>();

        let mut output = vec![0f32; num_samples];
        let mut head = 0;
        while head < num_samples {
            let next_head = (head + 512).min(num_samples);
            let block = head..next_head;
            for (src, dest) in variable_delay
                .process(input[block.clone()].iter().cloned())
                .process(vibrato_input[block.clone()].iter().cloned())
                .zip(&mut output[block.clone()])
            {
                *dest = src;
            }
            head = next_head;
        }
        output
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn vibrato_sine_snapshot() {
        let sampling_rate = 48000;

        assert_snapshot!(
            "delay/vibrato_sine",
            sampling_rate,
            process_vibrato(sampling_rate, &sine(48000, 440f32 / sampling_rate as f32))
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn vibrato_sweep_snapshot() {
        let sampling_rate = 48000;

        assert_snapshot!(
            "delay/vibrato_sweep",
            sampling_rate,
            process_vibrato(
                sampling_rate,
                &linear_sine_sweep(48000, sampling_rate as f32, 10f32, 20000f32)
            )
        );
    }
}
