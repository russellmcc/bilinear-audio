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
            output += oscillator.generate(settings);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use snapshots::assert_snapshot;

    use super::*;

    const PITCH_HZ: f32 = 4242.0;
    const LOW_PITCH_HZ: f32 = 1234.0;
    const SAMPLE_RATE: i32 = 48000;

    #[allow(clippy::cast_precision_loss)]
    const INCREMENT: f32 = PITCH_HZ / (SAMPLE_RATE as f32);

    #[allow(clippy::cast_precision_loss)]
    const LOW_INCREMENT: f32 = LOW_PITCH_HZ / (SAMPLE_RATE as f32);

    fn snapshot_for_settings(settings: &Settings, length: usize) -> Vec<f32> {
        let mut oscillators = Oscillators::new();
        std::iter::repeat_with(move || oscillators.generate(settings))
            .take(length)
            .collect()
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn default_saw_snapshot() {
        assert_snapshot!(
            "oscillators/basic",
            SAMPLE_RATE,
            snapshot_for_settings(
                &Settings {
                    oscillators: [
                        oscillator::Settings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            gain: 1.0,
                            width: 0.0,
                        },
                        oscillator::Settings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            gain: 0.0,
                            width: 0.0,
                        },
                    ],
                },
                48000
            )
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn square_snapshot() {
        assert_snapshot!(
            "oscillators/square",
            SAMPLE_RATE,
            snapshot_for_settings(
                &Settings {
                    oscillators: [
                        oscillator::Settings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Pulse,
                            gain: 1.0,
                            width: 0.5,
                        },
                        oscillator::Settings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            gain: 0.0,
                            width: 0.5,
                        },
                    ],
                },
                48000
            )
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn pulse_snapshot() {
        assert_snapshot!(
            "oscillators/pulse",
            SAMPLE_RATE,
            snapshot_for_settings(
                &Settings {
                    oscillators: [
                        oscillator::Settings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Pulse,
                            gain: 1.0,
                            width: 0.1,
                        },
                        oscillator::Settings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            gain: 0.0,
                            width: 0.5,
                        },
                    ],
                },
                48000
            )
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn pulse_saw_snapshot() {
        assert_snapshot!(
            "oscillators/pulse_saw",
            SAMPLE_RATE,
            snapshot_for_settings(
                &Settings {
                    oscillators: [
                        oscillator::Settings {
                            increment: LOW_INCREMENT,
                            shape: oscillator::Shape::PwmSaw,
                            gain: 1.0,
                            width: 0.5,
                        },
                        oscillator::Settings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            gain: 0.0,
                            width: 0.5,
                        },
                    ],
                },
                48000
            )
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn comb_saw_snapshot() {
        assert_snapshot!(
            "oscillators/comb_saw",
            SAMPLE_RATE,
            snapshot_for_settings(
                &Settings {
                    oscillators: [
                        oscillator::Settings {
                            increment: LOW_INCREMENT,
                            shape: oscillator::Shape::CombSaw,
                            gain: 1.0,
                            width: 0.5,
                        },
                        oscillator::Settings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            gain: 0.0,
                            width: 0.5,
                        },
                    ],
                },
                48000
            )
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn noise_snapshot() {
        assert_snapshot!(
            "oscillators/noise",
            SAMPLE_RATE,
            snapshot_for_settings(
                &Settings {
                    oscillators: [
                        oscillator::Settings {
                            increment: LOW_INCREMENT,
                            shape: oscillator::Shape::Noise,
                            gain: 1.0,
                            width: 0.5,
                        },
                        oscillator::Settings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            gain: 0.0,
                            width: 0.5,
                        },
                    ],
                },
                48000
            )
        );
    }
}
