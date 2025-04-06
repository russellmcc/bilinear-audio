use itertools::izip;

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
pub struct Settings {
    pub shapes: [Shape; 2],
    pub increments: [f32; 2],
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

    pub fn run(&mut self, Settings { increments, .. }: Settings) -> [f32; 2] {
        let mut output = [0.0; 2];
        for (oscillator, output, increment) in
            izip!(self.oscillators.iter_mut(), output.iter_mut(), increments)
        {
            *output = (oscillator.phase * std::f32::consts::TAU).sin();

            // Update the phase and wrap it to [0, 1)
            oscillator.phase += increment;
            oscillator.phase -= oscillator.phase.floor();
        }
        output
    }
}

#[cfg(test)]
mod tests;
