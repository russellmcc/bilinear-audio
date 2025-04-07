use dsp::osc_utils::polyblep2_residual;

#[derive(Default, Debug, Clone)]
pub struct Oscillator {
    phase: f32,
}

fn saw(phase: f32, increment: f32) -> f32 {
    (phase - 0.5) * 2.0 - polyblep2_residual(phase, increment)
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    pub increment: f32,

    pub shape: Shape,

    pub gain: f32,
}

impl Oscillator {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn generate(
        &mut self,
        Settings {
            increment,
            shape,
            gain,
        }: &Settings,
        output: &mut f32,
    ) {
        match shape {
            Shape::Saw => {
                *output = saw(self.phase, *increment) * *gain;
            }
            _ => {}
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    #[default]
    Saw,

    Pulse,

    PwmSaw,

    CombSaw,

    Noise,
}
