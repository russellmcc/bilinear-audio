use dsp::f32::lerp;

#[derive(Debug, Clone, Copy)]
struct Phase {
    entry: f32,
    t: f32,
}

#[derive(Debug, Clone, Copy)]
enum Rate {
    Instant,
    Increment(f32),
}

#[derive(Debug, Clone, Copy, Default)]
enum State {
    #[default]
    Off,
    Attack(Phase),
    Decay(Phase),
    ToSustain(Phase),
    Sustain,
    Release(Phase),
}

#[derive(Default, Debug, Clone)]
pub struct Env {
    state: State,
    value: f32,
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

fn calc_rate(time: f32, sampling_rate: f32) -> Rate {
    let period = 1.0 / sampling_rate;
    if time < period {
        Rate::Instant
    } else {
        Rate::Increment(period / time)
    }
}

fn advance_t(t: f32, rate: Rate) -> f32 {
    match rate {
        Rate::Instant => 1.0,
        Rate::Increment(incr) => (t + incr).min(1.0),
    }
}

pub fn calc_coeffs(params: &Params, sampling_rate: f32) -> Coeffs {
    Coeffs {
        attack_rate: calc_rate(params.attack_time, sampling_rate),
        attack_target: params.attack_target,
        decay_rate: calc_rate(params.decay_time, sampling_rate),
        decay_target: params.decay_target,
        to_sustain_rate: calc_rate(params.to_sustain_time, sampling_rate),
        sustain: params.sustain,
        release_rate: calc_rate(params.release_time, sampling_rate),
    }
}

impl Env {
    pub fn quiescent(&self) -> bool {
        matches!(self.state, State::Off)
    }

    pub fn reset(&mut self) {
        self.state = State::Off;
        self.value = 0.0;
    }

    pub fn on(&mut self) {
        let entry = match self.state {
            State::Off => 0.0,
            State::Attack(_)
            | State::Decay(_)
            | State::ToSustain(_)
            | State::Sustain
            | State::Release(_) => self.value,
        };
        self.state = State::Attack(Phase { entry, t: 0.0 });
    }

    pub fn off(&mut self) {
        self.state = match self.state {
            State::Off => State::Off,
            State::Attack(_)
            | State::Decay(_)
            | State::ToSustain(_)
            | State::Sustain
            | State::Release(_) => State::Release(Phase {
                entry: self.value,
                t: 0.0,
            }),
        };
    }

    pub fn process(&mut self, coeffs: &Coeffs) -> f32 {
        let process_phase = |phase: Phase,
                             target: f32,
                             rate: Rate,
                             current_state: fn(Phase) -> State,
                             next_state: State| {
            let t = advance_t(phase.t, rate);
            let value = lerp(phase.entry, target, t);
            let state = if t >= 1.0 {
                next_state
            } else {
                current_state(Phase {
                    entry: phase.entry,
                    t,
                })
            };
            (value, state)
        };

        let (out, new_state) = match self.state {
            State::Off => (0.0, State::Off),
            State::Attack(phase) => process_phase(
                phase,
                coeffs.attack_target,
                coeffs.attack_rate,
                State::Attack,
                State::Decay(Phase {
                    entry: coeffs.attack_target,
                    t: 0.0,
                }),
            ),
            State::Decay(phase) => process_phase(
                phase,
                coeffs.decay_target,
                coeffs.decay_rate,
                State::Decay,
                State::ToSustain(Phase {
                    entry: coeffs.decay_target,
                    t: 0.0,
                }),
            ),
            State::ToSustain(phase) => process_phase(
                phase,
                coeffs.sustain,
                coeffs.to_sustain_rate,
                State::ToSustain,
                State::Sustain,
            ),
            State::Sustain => (coeffs.sustain, State::Sustain),
            State::Release(phase) => {
                process_phase(phase, 0.0, coeffs.release_rate, State::Release, State::Off)
            }
        };
        self.value = out;
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
    fn release_reaches_zero_with_zero_sustain_before_sustain_phase() {
        let mut env: Env = Default::default();
        let coeffs: Coeffs = calc_coeffs(
            &Params {
                attack_time: 0.5,
                attack_target: 1.0,
                decay_time: 0.5,
                decay_target: 0.5,
                to_sustain_time: 0.5,
                sustain: 0.0,
                release_time: 0.1,
            },
            48000.0,
        );
        env.on();
        for _ in 0..2400 {
            env.process(&coeffs);
        }
        env.off();
        for _ in 0..4800 {
            env.process(&coeffs);
        }
        assert_approx_eq!(env.process(&coeffs), 0.0);
        assert!(env.quiescent());
    }

    #[test]
    fn retrigger_attack_reaches_zero_target_from_nonzero_level() {
        let mut env: Env = Default::default();
        let initial_coeffs: Coeffs = calc_coeffs(
            &Params {
                attack_time: 0.5,
                attack_target: 1.0,
                decay_time: 0.5,
                decay_target: 0.5,
                to_sustain_time: 0.5,
                sustain: 0.25,
                release_time: 0.1,
            },
            48000.0,
        );
        env.on();
        for _ in 0..2400 {
            env.process(&initial_coeffs);
        }

        let retrigger_coeffs: Coeffs = calc_coeffs(
            &Params {
                attack_time: 0.1,
                attack_target: 0.0,
                decay_time: 0.5,
                decay_target: 0.0,
                to_sustain_time: 0.5,
                sustain: 0.0,
                release_time: 0.1,
            },
            48000.0,
        );
        env.on();
        for _ in 0..4800 {
            env.process(&retrigger_coeffs);
        }

        assert_approx_eq!(env.process(&retrigger_coeffs), 0.0);
    }

    #[test]
    fn decay_reaches_new_target_after_live_target_change() {
        let mut env: Env = Default::default();
        let initial_coeffs: Coeffs = calc_coeffs(
            &Params {
                attack_time: 0.0,
                attack_target: 1.0,
                decay_time: 0.5,
                decay_target: 0.5,
                to_sustain_time: 0.0,
                sustain: 0.5,
                release_time: 0.1,
            },
            48000.0,
        );
        env.on();
        env.process(&initial_coeffs);
        for _ in 0..2400 {
            env.process(&initial_coeffs);
        }

        let retargeted_coeffs: Coeffs = calc_coeffs(
            &Params {
                attack_time: 0.0,
                attack_target: 1.0,
                decay_time: 0.5,
                decay_target: 0.0,
                to_sustain_time: 0.0,
                sustain: 0.0,
                release_time: 0.1,
            },
            48000.0,
        );
        for _ in 0..24_000 {
            env.process(&retargeted_coeffs);
        }

        assert_approx_eq!(env.process(&retargeted_coeffs), 0.0);
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
