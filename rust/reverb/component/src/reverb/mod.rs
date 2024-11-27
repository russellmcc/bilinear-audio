use conformal_component::audio::{Buffer, BufferMut};
use conformal_component::ProcessingEnvironment;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

pub use crate::diffuser::CHANNELS;
use crate::diffuser::{Diffuser, BLOCKS};
use crate::multi_channel_feedback_loop::MultiChannelFeedbackLoop;

pub struct Reverb {
    diffuser: Diffuser,
    feedback_loop: MultiChannelFeedbackLoop<CHANNELS>,
}

impl Reverb {
    #[allow(clippy::cast_possible_truncation)]
    pub fn new(env: &ProcessingEnvironment) -> Self {
        const DIFFUSER_DELAYS_MS: [f32; BLOCKS] = [20.0, 40.0, 80.0, 160.0];
        const FEEDBACK_LOOP_MIN_DELAY_MS: f32 = 100.0;
        const FEEDBACK_LOOP_MAX_DELAY_MS: f32 = 200.0;

        let min_feedback_delay_samples =
            (FEEDBACK_LOOP_MIN_DELAY_MS / 1000.0 * env.sampling_rate).round() as usize;
        let feedback_delay_range_samples = ((FEEDBACK_LOOP_MAX_DELAY_MS
            - FEEDBACK_LOOP_MIN_DELAY_MS)
            / 1000.0
            * env.sampling_rate)
            .round() as usize;

        let mut rng = Xoshiro256PlusPlus::seed_from_u64(369);

        let fdn_delays = core::array::from_fn(|i| {
            let min_for_block =
                min_feedback_delay_samples + i * feedback_delay_range_samples / CHANNELS;
            let max_for_block = min_for_block + feedback_delay_range_samples / CHANNELS;
            rng.gen_range(min_for_block..max_for_block)
        });

        Self {
            diffuser: Diffuser::new(
                &mut rng,
                DIFFUSER_DELAYS_MS.map(|d| (d / 1000.0 * env.sampling_rate).round() as usize),
            ),
            feedback_loop: MultiChannelFeedbackLoop::new(fdn_delays),
        }
    }

    pub fn process(&mut self, feedback: f32, input: &impl Buffer, output: &mut impl BufferMut) {
        if input.num_channels() == 1 {
            let input = input.channel(0);
            let output = output.channel_mut(0);
            for (input, output) in input.iter().zip(output.iter_mut()) {
                let mc_input = [*input; CHANNELS];
                *output = self
                    .feedback_loop
                    .process(self.diffuser.process(&mc_input), feedback)[0];
            }
        } else if input.num_channels() == 2 {
            let input_l = input.channel(0);
            let input_r = input.channel(1);
            for (i, (input_l, input_r)) in input_l.iter().zip(input_r.iter()).enumerate() {
                {
                    let mc_input =
                        core::array::from_fn(|i| if i & 1 == 0 { *input_l } else { *input_r });
                    let x = self
                        .feedback_loop
                        .process(self.diffuser.process(&mc_input), feedback);
                    output.channel_mut(0)[i] = x[0];
                    output.channel_mut(1)[i] = x[1];
                }
            }
        } else {
            panic!("Reverb only supports mono and stereo input");
        }
    }
}

#[cfg(test)]
mod tests;
