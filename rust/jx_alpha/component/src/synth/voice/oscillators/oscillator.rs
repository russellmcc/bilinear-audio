use core::f32::consts::TAU;
use dsp::{
    f32::rescale,
    osc_utils::{polyblamp2_residual, polyblep2_residual},
};
use num_derive::FromPrimitive;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

#[derive(Debug, Clone)]
pub struct Oscillator {
    phase: f32,
    rng: Xoshiro256PlusPlus,

    sync_residual: Option<f32>,
}

impl Default for Oscillator {
    fn default() -> Self {
        Self {
            phase: 0.0,
            rng: Xoshiro256PlusPlus::seed_from_u64(369),
            sync_residual: None,
        }
    }
}

#[must_use]
fn saw(phase: f32, increment: f32) -> f32 {
    (phase - 0.5) * 2.0 - polyblep2_residual(phase, increment)
}

#[must_use]
fn rotate(phase: f32, x: f32) -> f32 {
    let phase = phase + (1.0 - x);
    if phase > 1.0 { phase - 1.0 } else { phase }
}

#[must_use]
pub fn pulse(phase: f32, increment: f32, width: f32) -> f32 {
    if width < increment {
        -1.0
    } else if width > 1.0 - increment {
        1.0
    } else {
        (if phase < width { -1.0 } else { 1.0 }) - polyblep2_residual(phase, increment)
            + polyblep2_residual(rotate(phase, width), increment)
    }
}

#[must_use]
pub fn pwm_saw(phase: f32, increment: f32, width: f32) -> f32 {
    // Naive waveform generation
    let naive_signal = if phase < width / 2.0 {
        -1.0
    } else if phase < 0.5 {
        (phase - 0.5) * 2.0
    } else if phase < f32::midpoint(width, 1.0) {
        -1.0
    } else {
        // phase >= (width + 1.0) / 2.0
        (phase - 0.5) * 2.0
    };

    let mut output = naive_signal;

    // discontinuity at phase = 0.0
    output -= polyblep2_residual(phase, increment);
    output -= 0.5 * polyblamp2_residual(phase, increment);

    // discontinuity at phase = 0.5 * width
    let p1 = 0.5 * width;
    let t1 = rotate(phase, p1);
    output += p1 * polyblep2_residual(t1, increment);
    output += 0.5 * polyblamp2_residual(t1, increment);

    // discontinuity at phase = 0.5
    let t2 = rotate(phase, 0.5);
    output -= 0.5 * polyblep2_residual(t2, increment);
    output -= 0.5 * polyblamp2_residual(t2, increment);

    // discontinuity at phase = 0.5 + 0.5 * width
    let p3 = 0.5 * (width + 1.0);
    let t3 = rotate(phase, p3);
    output += p3 * polyblep2_residual(t3, increment);
    output += 0.5 * polyblamp2_residual(t3, increment);

    output
}

#[must_use]
pub fn comb_saw(phase: f32, increment: f32) -> f32 {
    // Note the derivative at phase = 0.0 jumps from TAU * 8.0 * increment to 0.0,
    // and the residual is normalized to a jump of 2.0 * increment,
    // so we need to scale the residual by 4.0 * TAU to compensate.
    (phase * TAU * 8.0).sin() * phase - TAU * 4.0 * polyblamp2_residual(phase, increment)
}

/// This is a special-purpose helper function to calculate the pre-and-post jump residuals
/// for a sync-type jump controlled by one oscillator onto another.
///
/// phase and increment come from the conductor oscillator, and signal is the input of
/// the synced oscillator. The output is the pre-and-post jump residuals.
#[must_use]
pub fn get_jump_residuals(phase: f32, increment: f32, signal: f32) -> (f32, f32) {
    // Note we calcluate both the pre and post jump residuals here, since
    // we have to keep a state to know where we jumped from (since the scale
    // of the post-jump residual depends on where we jumped from).

    let t_post = phase / increment;
    let t_pre = t_post - 1.0;

    // Note this residual scale assumes that at 0 phase we will be at -1.0, but this is true of all our shapes.
    let residual_scale = rescale(signal, -1.0..=1.0, 0.0..=1.0);

    // This is the same math in `polyblep2_residual` expressed a tiny bit differently.
    let pre_jump_residual = residual_scale * t_post * t_post;
    let post_jump_residual = residual_scale * -t_pre * t_pre;
    (pre_jump_residual, post_jump_residual)
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    pub increment: f32,

    pub shape: Shape,

    /// For width-modulatable shapes, the current width
    pub width: f32,
}

impl Oscillator {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    fn generate_internal(
        &mut self,
        increment: f32,
        residual_increment: f32,
        shape: Shape,
        width: f32,
    ) -> f32 {
        assert!(increment > 0.0 && increment < 1.0);
        let ret = match shape {
            Shape::Saw => saw(self.phase, residual_increment),
            Shape::Pulse => pulse(self.phase, residual_increment, width),
            Shape::PwmSaw => pwm_saw(self.phase, residual_increment, width),
            Shape::CombSaw => comb_saw(self.phase, residual_increment),
            Shape::Noise => self.rng.gen_range(-1.0..=1.0),
        };
        self.phase += increment;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }
        ret
    }

    pub fn current_phase(&self) -> f32 {
        self.phase
    }

    pub fn generate(
        &mut self,
        Settings {
            increment,
            shape,
            width,
        }: Settings,
    ) -> f32 {
        // note since we run these oversampled, we oversample the increment as well for the residual calculations
        self.generate_internal(increment, 2.0 * increment, shape, width)
    }

    /// Generate with hard-sync to another oscillator.
    ///
    /// Note that this must be run AFTER that oscillator has been updated.
    ///
    /// The approach here is to reset the phase whenever the conductor oscillator resets,
    /// and also apply a BLEP to at least have some anti-aliasing. Note that we do NOT
    /// handle higher-order (derivative) discontinuities, instead we rely on 2x oversampling to handle
    /// the aliasing from that somewhat.
    pub fn generate_with_sync(
        &mut self,
        settings: Settings,

        conductor_osc: &Self,
        conductor_increment: f32,
    ) -> f32 {
        if let Some(sync_residual) = self.sync_residual {
            // This is the first sample after a sync, we have to ignore the residual we'd normally generate
            // at the start of a cycle and output the naive aliased waveform with a post-jump residual instead.
            let aliased_wave = match settings.shape {
                Shape::Saw => (self.phase - 0.5) * 2.0,
                Shape::Pulse | Shape::PwmSaw | Shape::Noise => -1.0,
                Shape::CombSaw => (self.phase * TAU * 8.0).sin() * self.phase,
            };
            let output = aliased_wave - sync_residual;
            self.phase += settings.increment;
            if self.phase > 1.0 {
                self.phase -= 1.0;
            }
            self.sync_residual = None;
            return output;
        }

        assert!(conductor_increment > 0.0 && conductor_increment < 1.0);

        // Note that the trick we use in non-synced mode to oversample the residual calculations
        // will interfere with our sync residuals, so we just run at 1x increment.
        let raw_out = self.generate_internal(
            settings.increment,
            settings.increment,
            settings.shape,
            settings.width,
        );

        if conductor_osc.phase < conductor_increment {
            let (pre_jump_residual, post_jump_residual) =
                get_jump_residuals(conductor_osc.phase, conductor_increment, raw_out);
            self.sync_residual = Some(post_jump_residual);

            // Reset the phase when the conductor oscillator jumps.
            self.phase = conductor_osc.phase * settings.increment / conductor_increment;

            raw_out - pre_jump_residual
        } else {
            raw_out
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, FromPrimitive)]
pub enum Shape {
    /// Standard sawtooth wave
    #[default]
    Saw,

    /// Pulse wave (width-modulatable)
    Pulse,

    /// Saw with width-modulation
    PwmSaw,

    /// Saw made out of comb-like pulses
    CombSaw,

    /// Noise as generated by a linear feedback shift register IC (mm5437)
    Noise,
}
