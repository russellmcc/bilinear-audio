use oscillator::Oscillator;

mod oscillator;

#[derive(Default, Debug, Clone)]
pub struct Oscillators {
    oscillators: [Oscillator; 2],
}

#[derive(Default, Debug, Clone)]
pub struct Settings {
    pub oscillators: [oscillator::Settings; 2],
}

pub use oscillator::Settings as OscillatorSettings;
pub use oscillator::Shape;

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

    pub fn generate(&mut self, settings: &Settings) -> f32 {
        let mut output = 0.0;
        for (oscillator, settings) in self.oscillators.iter_mut().zip(settings.oscillators.iter()) {
            oscillator.generate(settings, &mut output);
        }
        output
    }
}

#[cfg(test)]
mod tests;
