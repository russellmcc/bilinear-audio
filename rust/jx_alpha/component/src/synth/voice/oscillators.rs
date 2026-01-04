use num_derive::FromPrimitive;
use oscillator::Oscillator;

mod and;
mod downsampler;
mod oscillator;
mod ring;

#[derive(Default, Debug)]
pub struct Oscillators {
    #[allow(clippy::struct_field_names)]
    oscillators: [Oscillator; 2],
    pwm_lfos: [dsp::sine_lfo::SineLfo; 2],
    ring: ring::Ring,
    and: and::And,
    downsampler: downsampler::Downsampler,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct OscillatorSettings {
    pub increment: f32,

    pub shape: Shape,

    pub gain: f32,

    /// For width-modulatable shapes, the pwm width
    ///
    /// Note that if rate is 0, this acts as a manual control of width.
    pub pwm_depth: f32,

    /// For width-modulatable shapes, the pwm incr
    pub pwm_incr: f32,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, FromPrimitive)]
pub enum CrossModulation {
    /// No cross modulation
    #[default]
    Off,

    /// Ring modulation
    Ring,

    /// "And" modulation
    And,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, FromPrimitive)]
pub enum Sync {
    /// No sync
    #[default]
    Off,

    /// Hard sync where osc 1 resets on osc 0's cycle
    Hard,
}

#[derive(Default, Debug, Clone)]
pub struct Settings {
    pub oscillators: [OscillatorSettings; 2],
    pub x_mod: CrossModulation,
    pub sync: Sync,
}

fn to_raw_settings(val: OscillatorSettings, lfo_out: f32) -> oscillator::Settings {
    oscillator::Settings {
        increment: val.increment / 2.0,
        shape: val.shape,
        width: if val.pwm_incr == 0.0 {
            0.5 - (val.pwm_depth * 0.475)
        } else {
            (lfo_out * 0.475) * val.pwm_depth + 0.5
        },
    }
}

pub use oscillator::Shape;

impl Oscillators {
    pub fn reset(&mut self) {
        for oscillator in &mut self.oscillators {
            oscillator.reset();
        }
        self.ring.reset();
        self.downsampler.reset();
        self.and.reset();
        for lfo in &mut self.pwm_lfos {
            lfo.reset();
        }
    }

    fn generate_high_rate(&mut self, settings: &Settings) -> f32 {
        let [osc0, osc1] = &mut self.oscillators;
        let pwm0_out = self.pwm_lfos[0].generate(settings.oscillators[0].pwm_incr);
        let pwm1_out = self.pwm_lfos[1].generate(settings.oscillators[1].pwm_incr);
        let osc0_settings = to_raw_settings(settings.oscillators[0], pwm0_out);
        let osc1_settings = to_raw_settings(settings.oscillators[1], pwm1_out);
        let osc0_out = osc0.generate(osc0_settings);
        let osc1_out = match settings.sync {
            Sync::Off => osc1.generate(osc1_settings),
            Sync::Hard => osc1.generate_with_sync(osc1_settings, osc0, osc0_settings.increment),
        };
        osc0_out * settings.oscillators[0].gain
            + settings.oscillators[1].gain
                * match settings.x_mod {
                    CrossModulation::Ring => self.ring.process(osc0_out, osc1_out),
                    CrossModulation::And => {
                        self.and
                            .process(osc1_out, osc0.current_phase(), osc0_settings.increment)
                    }
                    CrossModulation::Off => osc1_out,
                }
    }

    pub fn generate(&mut self, settings: &Settings) -> f32 {
        let a = self.generate_high_rate(settings);
        let b = self.generate_high_rate(settings);
        self.downsampler.process([a, b])
    }
}

#[cfg(test)]
mod tests {
    use conformal_component::audio::all_approx_eq;
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
        let mut oscillators = Oscillators::default();
        std::iter::repeat_with(move || oscillators.generate(settings) * 0.5)
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
                        OscillatorSettings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            gain: 1.0,
                            ..Default::default()
                        },
                        OscillatorSettings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                48000,
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
                        OscillatorSettings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Pulse,
                            gain: 1.0,
                            ..Default::default()
                        },
                        OscillatorSettings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
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
                        OscillatorSettings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Pulse,
                            gain: 1.0,
                            pwm_depth: 0.8,
                            pwm_incr: 0.0,
                        },
                        OscillatorSettings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
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
                        OscillatorSettings {
                            increment: LOW_INCREMENT,
                            shape: oscillator::Shape::PwmSaw,
                            gain: 1.0,
                            ..Default::default()
                        },
                        OscillatorSettings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
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
                        OscillatorSettings {
                            increment: LOW_INCREMENT,
                            shape: oscillator::Shape::CombSaw,
                            gain: 1.0,
                            ..Default::default()
                        },
                        OscillatorSettings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
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
                        OscillatorSettings {
                            increment: LOW_INCREMENT,
                            shape: oscillator::Shape::Noise,
                            gain: 1.0,
                            ..Default::default()
                        },
                        OscillatorSettings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                48000
            )
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn ring_snapshot() {
        assert_snapshot!(
            "oscillators/ring",
            SAMPLE_RATE,
            snapshot_for_settings(
                &Settings {
                    oscillators: [
                        OscillatorSettings {
                            increment: LOW_INCREMENT,
                            shape: oscillator::Shape::Pulse,
                            ..Default::default()
                        },
                        OscillatorSettings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            gain: 1.0,
                            ..Default::default()
                        },
                    ],
                    x_mod: CrossModulation::Ring,
                    ..Default::default()
                },
                48000
            )
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn and_snapshot() {
        assert_snapshot!(
            "oscillators/and",
            SAMPLE_RATE,
            snapshot_for_settings(
                &Settings {
                    oscillators: [
                        OscillatorSettings {
                            increment: LOW_INCREMENT,
                            shape: oscillator::Shape::Pulse,
                            gain: 0.0,
                            pwm_depth: 0.5,
                            pwm_incr: 0.0,
                        },
                        OscillatorSettings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            gain: 1.0,
                            pwm_depth: 0.5,
                            pwm_incr: 0.0,
                        },
                    ],
                    x_mod: CrossModulation::And,
                    ..Default::default()
                },
                48000
            )
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn sync_snapshot() {
        assert_snapshot!(
            "oscillators/sync",
            SAMPLE_RATE,
            snapshot_for_settings(
                &Settings {
                    oscillators: [
                        OscillatorSettings {
                            increment: LOW_INCREMENT,
                            shape: oscillator::Shape::Saw,
                            ..Default::default()
                        },
                        OscillatorSettings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            gain: 1.0,
                            ..Default::default()
                        },
                    ],
                    sync: Sync::Hard,
                    ..Default::default()
                },
                48000,
            )
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn pwm_snapshot() {
        assert_snapshot!(
            "oscillators/pwm",
            SAMPLE_RATE,
            snapshot_for_settings(
                &Settings {
                    oscillators: [
                        OscillatorSettings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Pulse,
                            gain: 1.0,
                            pwm_depth: 0.5,
                            pwm_incr: 1.0 / SAMPLE_RATE as f32,
                        },
                        OscillatorSettings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                48000,
            )
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn pwm_saw_snapshot() {
        assert_snapshot!(
            "oscillators/pwm_saw",
            SAMPLE_RATE,
            snapshot_for_settings(
                &Settings {
                    oscillators: [
                        OscillatorSettings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::PwmSaw,
                            gain: 1.0,
                            pwm_depth: 0.8,
                            pwm_incr: 1.0 / SAMPLE_RATE as f32,
                        },
                        OscillatorSettings {
                            increment: INCREMENT,
                            shape: oscillator::Shape::Saw,
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                48000,
            )
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn reset() {
        let mut oscillators = Oscillators::default();
        let settings = Settings {
            oscillators: [
                OscillatorSettings {
                    increment: LOW_INCREMENT,
                    shape: oscillator::Shape::Pulse,
                    ..Default::default()
                },
                OscillatorSettings {
                    increment: INCREMENT,
                    shape: oscillator::Shape::Saw,
                    gain: 1.0,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let length = 1024;
        let initial = std::iter::repeat_with(|| oscillators.generate(&settings))
            .take(length)
            .collect::<Vec<_>>();
        oscillators.reset();
        let reset = std::iter::repeat_with(|| oscillators.generate(&settings))
            .take(length)
            .collect::<Vec<_>>();
        assert!(all_approx_eq(initial, reset, 1e-6));
    }
}
