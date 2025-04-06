use dsp::osc_utils::polyblep2_residual;

#[derive(Default, Debug, Clone)]
pub struct Oscillator {
    phase: f32,
}

impl Oscillator {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Default, Debug, Clone)]
pub struct Oscillators {
    oscillators: [Oscillator; 2],
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum SawShape {
    Off,

    #[default]
    Saw,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum PulseShape {
    #[default]
    Off,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Shape {
    pub saw: SawShape,
    pub pulse: PulseShape,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum SubShape {
    #[default]
    Off,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    pub shapes: [Shape; 2],
    pub sub_shape: SubShape,
    pub increments: [f32; 2],
}

fn saw(phase: f32, increment: f32) -> f32 {
    (phase - 0.5) * 2.0 - polyblep2_residual(phase, increment)
}

fn single_osc(phase: f32, increment: f32, shape: Shape) -> f32 {
    let out = match shape.saw {
        SawShape::Off => 0.0,
        SawShape::Saw => saw(phase, increment),
    };

    match shape.pulse {
        PulseShape::Off => out,
    }
}

impl Oscillators {
    pub fn new() -> Self {
        Self {
            oscillators: [Oscillator::default(), Oscillator::default()],
        }
    }

    pub fn reset(&mut self) {
        for oscillator in &mut self.oscillators {
            oscillator.reset();
        }
    }

    pub fn run(
        &mut self,
        Settings {
            increments, shapes, ..
        }: Settings,
    ) -> f32 {
        // Currently we only support dco1
        single_osc(self.oscillators[0].phase, increments[0], shapes[0])
    }
}

#[cfg(test)]
mod tests;
