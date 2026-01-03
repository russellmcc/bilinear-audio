use super::{Coeff, calc_coeff};

pub struct Params {
    pub attack_time: f32,
    pub decay_time: f32,
    pub sustain: f32,
    pub release_time: f32,
}

#[derive(Debug)]
pub struct Coeffs {
    attack: Coeff,
    decay: Coeff,
    sustain: f32,
    release: Coeff,
}

#[must_use]
pub fn calc_coeffs(params: &Params, sampling_rate: f32) -> Coeffs {
    Coeffs {
        attack: calc_coeff(params.attack_time, sampling_rate),
        decay: calc_coeff(params.decay_time, sampling_rate),
        sustain: params.sustain,
        release: calc_coeff(params.release_time, sampling_rate),
    }
}

#[derive(Debug, Default)]
enum State {
    #[default]
    Off,
    Attack {
        value: f32,
    },
    Decay {
        value: f32,
    },
    Sustain {
        value: f32,
    },
    Release {
        value: f32,
    },
}

#[derive(Debug, Default)]
pub struct Adsr {
    state: State,
}

impl Adsr {
    #[must_use]
    pub fn quiescent(&self) -> bool {
        matches!(self.state, State::Off)
    }

    pub fn reset(&mut self) {
        self.state = State::default();
    }

    pub fn on(&mut self) {
        self.state = match self.state {
            State::Off => State::Attack { value: 0.0 },
            State::Attack { value }
            | State::Decay { value }
            | State::Sustain { value }
            | State::Release { value } => State::Attack { value },
        };
    }

    pub fn off(&mut self) {
        self.state = match self.state {
            State::Off => State::Off,
            State::Attack { value }
            | State::Decay { value }
            | State::Sustain { value }
            | State::Release { value } => State::Release { value },
        };
    }

    pub fn process(&mut self, coeffs: &Coeffs) -> f32 {
        let (out, new_state) = match self.state {
            State::Off => (0.0, State::Off),
            State::Attack { value } => match coeffs.attack {
                Coeff::Instant => (1.0, State::Decay { value: 1.0 }),
                Coeff::Increment(coeff) => {
                    let new_value = value + coeff;
                    if new_value >= 1.0 {
                        (1.0, State::Decay { value: 1.0 })
                    } else {
                        (new_value, State::Attack { value: new_value })
                    }
                }
            },
            State::Decay { value } => match coeffs.decay {
                Coeff::Instant => (
                    coeffs.sustain,
                    State::Sustain {
                        value: coeffs.sustain,
                    },
                ),
                Coeff::Increment(coeff) => {
                    let new_value = value - coeff;
                    if new_value <= coeffs.sustain {
                        (
                            coeffs.sustain,
                            State::Sustain {
                                value: coeffs.sustain,
                            },
                        )
                    } else {
                        (new_value, State::Decay { value: new_value })
                    }
                }
            },
            State::Sustain { value } => (coeffs.sustain, State::Sustain { value }),
            State::Release { value } => match coeffs.release {
                Coeff::Instant => (0.0, State::Off),
                Coeff::Increment(coeff) => {
                    let new_value = value - coeff;
                    if new_value <= 0.0 {
                        (0.0, State::Off)
                    } else {
                        (new_value, State::Release { value: new_value })
                    }
                }
            },
        };
        self.state = new_state;
        out
    }
}

#[cfg(test)]
mod tests {
    use super::{Adsr, Params, calc_coeffs};
    use assert_approx_eq::assert_approx_eq;
    use snapshots::assert_snapshot;

    #[test]
    fn silence_until_turned_on() {
        let mut adsr: Adsr = Default::default();
        let coeffs = calc_coeffs(
            &Params {
                attack_time: 0.0,
                decay_time: 0.0,
                sustain: 1.0,
                release_time: 0.0,
            },
            48000.0,
        );
        assert_eq!(
            std::iter::repeat_with(|| adsr.process(&coeffs))
                .take(100)
                .collect::<Vec<_>>(),
            std::iter::repeat_n(0f32, 100).collect::<Vec<_>>()
        );
    }

    #[test]
    fn reset() {
        let mut adsr: Adsr = Default::default();
        let coeffs = calc_coeffs(
            &Params {
                attack_time: 0.010,
                decay_time: 0.100,
                sustain: 0.7,
                release_time: 0.200,
            },
            48000.0,
        );
        adsr.on();
        let initial = std::iter::repeat_with(|| adsr.process(&coeffs))
            .take(100)
            .collect::<Vec<_>>();
        adsr.reset();
        adsr.on();
        let reset = std::iter::repeat_with(|| adsr.process(&coeffs))
            .take(100)
            .collect::<Vec<_>>();
        for (a, b) in initial.iter().zip(reset.iter()) {
            assert_approx_eq!(a, b);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn snapshot() {
        let mut adsr: Adsr = Default::default();
        let coeffs = calc_coeffs(
            &Params {
                attack_time: 0.010,
                decay_time: 0.100,
                sustain: 0.7,
                release_time: 0.200,
            },
            48000.0,
        );
        adsr.on();
        assert_snapshot!(
            "adsr",
            48000,
            (0..48000).map(|i| {
                if i == 24000 {
                    adsr.off();
                }
                adsr.process(&coeffs)
            })
        );
    }
}
