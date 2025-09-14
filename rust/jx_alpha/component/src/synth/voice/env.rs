#[derive(Debug, Clone, Copy, Default)]
enum State {
    #[default]
    Off,
    Attack(f32),
    Decay(f32),
    ToSustain(f32),
    Sustain(f32),
    Release(f32),
}

#[derive(Debug, Clone, Copy)]
enum Rate {
    Instant,
    Increment(f32),
}

#[derive(Default, Debug, Clone)]
pub struct Env {
    state: State,
}

#[derive(Debug, Clone)]
pub struct Params {
    pub attack_time: f32,
    pub attack_target: f32,
    pub decay_time: f32,
    pub decay_target: f32,
    pub to_sustain_time: f32,
    pub sustain: f32,
    pub release_time: f32,
}

#[derive(Debug, Clone)]
pub struct Coeffs {
    attack_rate: Rate,
    attack_target: f32,
    decay_rate: Rate,
    decay_target: f32,
    to_sustain_rate: Rate,
    sustain: f32,
    release_rate: Rate,
}

fn calc_rate(time: f32, sampling_rate: f32, diff: f32) -> Rate {
    let period = 1.0 / sampling_rate;
    if time < period {
        Rate::Instant
    } else {
        Rate::Increment(diff.abs() / (time * sampling_rate))
    }
}

pub fn calc_coeffs(params: &Params, sampling_rate: f32) -> Coeffs {
    Coeffs {
        attack_rate: calc_rate(params.attack_time, sampling_rate, params.attack_target),
        attack_target: params.attack_target,
        decay_rate: calc_rate(
            params.decay_time,
            sampling_rate,
            params.decay_target - params.attack_target,
        ),
        decay_target: params.decay_target,
        to_sustain_rate: calc_rate(
            params.to_sustain_time,
            sampling_rate,
            params.sustain - params.decay_target,
        ),
        sustain: params.sustain,
        release_rate: calc_rate(params.release_time, sampling_rate, -params.sustain),
    }
}

impl Env {
    pub fn quiescent(&self) -> bool {
        matches!(self.state, State::Off)
    }

    pub fn reset(&mut self) {
        self.state = State::Off;
    }

    pub fn on(&mut self) {
        self.state = match self.state {
            State::Off => State::Attack(0.0),
            State::Attack(value)
            | State::Decay(value)
            | State::ToSustain(value)
            | State::Sustain(value)
            | State::Release(value) => State::Attack(value),
        };
    }

    pub fn off(&mut self) {
        self.state = match self.state {
            State::Off => State::Off,
            State::Attack(value)
            | State::Decay(value)
            | State::ToSustain(value)
            | State::Sustain(value)
            | State::Release(value) => State::Release(value),
        };
    }

    pub fn process(&mut self, coeffs: &Coeffs) -> f32 {
        let approach_target = |value: f32,
                               target: f32,
                               rate: Rate,
                               current_state: fn(f32) -> State,
                               new_state: fn(f32) -> State| {
            match rate {
                Rate::Instant => (target, new_state(target)),
                Rate::Increment(coeff) => {
                    if value > target {
                        let next = (value - coeff).max(target);
                        if next - target < 1e-6 {
                            (target, new_state(target))
                        } else {
                            (next, current_state(next))
                        }
                    } else {
                        let next = (value + coeff).min(target);
                        if target - next < 1e-6 {
                            (target, new_state(target))
                        } else {
                            (next, current_state(next))
                        }
                    }
                }
            }
        };

        let (out, new_state) = match self.state {
            State::Off => (0.0, State::Off),
            State::Attack(value) => approach_target(
                value,
                coeffs.attack_target,
                coeffs.attack_rate,
                State::Attack,
                State::Decay,
            ),
            State::Decay(value) => approach_target(
                value,
                coeffs.decay_target,
                coeffs.decay_rate,
                State::Decay,
                State::ToSustain,
            ),
            State::ToSustain(value) => approach_target(
                value,
                coeffs.sustain,
                coeffs.to_sustain_rate,
                State::ToSustain,
                State::Sustain,
            ),
            State::Sustain(_) => (coeffs.sustain, State::Sustain(coeffs.sustain)),
            State::Release(value) => {
                approach_target(value, 0.0, coeffs.release_rate, State::Release, |_| {
                    State::Off
                })
            }
        };
        self.state = new_state;
        out
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use conformal_component::audio::all_approx_eq;
    use snapshots::assert_snapshot;

    use super::*;

    #[test]
    fn silence_before_turned_on() {
        let mut env: Env = Default::default();
        let coeffs: Coeffs = calc_coeffs(
            &Params {
                attack_time: 0.0,
                attack_target: 1.0,
                decay_time: 0.0,
                decay_target: 1.0,
                to_sustain_time: 0.0,
                sustain: 1.0,
                release_time: 0.0,
            },
            48000.0,
        );
        assert!(all_approx_eq(
            std::iter::repeat_with(|| env.process(&coeffs))
                .take(100)
                .collect::<Vec<_>>(),
            std::iter::repeat_n(0f32, 100).collect::<Vec<_>>(),
            1e-6
        ));
    }

    #[test]
    fn quiescent_before_turned_on() {
        let env: Env = Default::default();
        assert!(env.quiescent());
    }

    #[test]
    fn gate_settings_move_quickly() {
        let mut env: Env = Default::default();
        let coeffs: Coeffs = calc_coeffs(
            &Params {
                attack_time: 0.0,
                attack_target: 1.0,
                decay_time: 0.0,
                decay_target: 1.0,
                to_sustain_time: 0.0,
                sustain: 1.0,
                release_time: 0.0,
            },
            48000.0,
        );
        env.on();
        // Definitely should be fully on after 100ms.
        for _ in 0..4800 {
            env.process(&coeffs);
        }
        assert!(!env.quiescent());
        assert_approx_eq!(env.process(&coeffs), 1.0);
        env.off();
        assert!(!env.quiescent());
        // Definitely should be fully off after 100ms.
        for _ in 0..4800 {
            env.process(&coeffs);
        }
        assert_approx_eq!(env.process(&coeffs), 0.0);
        assert!(env.quiescent());
    }

    #[test]
    fn can_change_sustain_while_sustaining() {
        let mut env: Env = Default::default();
        {
            let coeffs: Coeffs = calc_coeffs(
                &Params {
                    attack_time: 0.0,
                    attack_target: 1.0,
                    decay_time: 0.0,
                    decay_target: 1.0,
                    to_sustain_time: 0.0,
                    sustain: 1.0,
                    release_time: 0.0,
                },
                48000.0,
            );
            env.on();
            for _ in 0..4800 {
                env.process(&coeffs);
            }
            assert_approx_eq!(env.process(&coeffs), 1.0);
        }
        {
            let coeffs: Coeffs = calc_coeffs(
                &Params {
                    attack_time: 0.0,
                    attack_target: 1.0,
                    decay_time: 0.0,
                    decay_target: 1.0,
                    to_sustain_time: 0.0,
                    sustain: 0.5,
                    release_time: 0.0,
                },
                48000.0,
            );
            for _ in 0..4800 {
                env.process(&coeffs);
            }
            assert_approx_eq!(env.process(&coeffs), 0.5);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn basic_snapshot() {
        let mut env: Env = Default::default();
        let coeffs: Coeffs = calc_coeffs(
            &Params {
                attack_time: 0.5,
                attack_target: 1.0,
                decay_time: 0.5,
                decay_target: 0.5,
                to_sustain_time: 0.5,
                sustain: 0.25,
                release_time: 0.5,
            },
            48000.0,
        );
        env.on();
        assert_snapshot!(
            "env/basic",
            48000,
            (0..144_000).map(|i| {
                if i == 96000 {
                    env.off();
                }
                env.process(&coeffs)
            })
        );
    }
}
