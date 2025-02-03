use conformal_component::audio::{Buffer, BufferMut};
use conformal_component::ProcessingEnvironment;
use dsp::iir::svf::{calc_g, calc_two_r, GainInput, GainRawParams, Svf};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

pub use crate::diffuser::CHANNELS;
use crate::diffuser::{Diffuser, BLOCKS};
use crate::multi_channel_feedback_loop::MultiChannelFeedbackLoop;

pub struct Reverb {
    diffuser: Diffuser,
    feedback_loop: MultiChannelFeedbackLoop,
    shelves: [Svf; 2],
    shelf_g: f64,
    shelf_two_r: f64,
}

pub struct Params {
    pub mix: f32,
    pub feedback: f32,
    pub brightness: f32,
    pub damping: f32,
}

const SHELF_FREQ: f32 = 2000.0;
const SHELF_Q: f64 = 0.707;

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
            feedback_loop: MultiChannelFeedbackLoop::new(fdn_delays, env.sampling_rate),
            shelves: [Svf::default(), Svf::default()],
            shelf_g: calc_g(f64::from((SHELF_FREQ / env.sampling_rate).min(0.45))),
            shelf_two_r: calc_two_r(SHELF_Q),
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn process(&mut self, params: Params, input: &impl Buffer, output: &mut impl BufferMut) {
        if input.num_channels() == 1 {
            let input = input.channel(0);
            let output = output.channel_mut(0);
            for (input, output) in input.iter().zip(output.iter_mut()) {
                let mc_input = [*input; CHANNELS];
                let (x, er) = self.diffuser.process_mono(0.5, &mc_input);
                *output = self
                    .feedback_loop
                    .process(x, params.feedback, params.damping, 0.0, 0.0)[0]
                    + er;
                *output = params.mix
                    * self.shelves[0]
                        .process_high_shelf(std::iter::once(GainInput {
                            x: f64::from(*output),
                            params: GainRawParams {
                                g: self.shelf_g,
                                two_r: self.shelf_two_r,
                                sqrt_gain: f64::from(params.brightness),
                            },
                        }))
                        .next()
                        .unwrap() as f32
                    + (1.0 - params.mix) * *input;
            }
        } else if input.num_channels() == 2 {
            let input_l = input.channel(0);
            let input_r = input.channel(1);
            for (i, (input_l, input_r)) in input_l.iter().zip(input_r.iter()).enumerate() {
                {
                    let mc_input =
                        core::array::from_fn(|i| if i & 1 == 0 { *input_l } else { *input_r });
                    let (x, er) = self.diffuser.process_stereo(0.5, &mc_input);
                    let y =
                        self.feedback_loop
                            .process(x, params.feedback, params.damping, 0.0, 0.0);
                    for channel in 0..2 {
                        output.channel_mut(channel)[i] = params.mix
                            * self.shelves[channel]
                                .process_high_shelf(std::iter::once(GainInput {
                                    x: f64::from(y[channel] + er[channel]),
                                    params: GainRawParams {
                                        g: self.shelf_g,
                                        two_r: self.shelf_two_r,
                                        sqrt_gain: f64::from(params.brightness),
                                    },
                                }))
                                .next()
                                .unwrap() as f32
                            + (1.0 - params.mix) * (if channel == 0 { *input_l } else { *input_r });
                    }
                }
            }
        } else {
            panic!("Reverb only supports mono and stereo input");
        }
    }

    pub fn reset(&mut self) {
        self.diffuser.reset();
        self.feedback_loop.reset();
        for shelf in &mut self.shelves {
            shelf.reset();
        }
    }
}

#[cfg(test)]
mod tests;
