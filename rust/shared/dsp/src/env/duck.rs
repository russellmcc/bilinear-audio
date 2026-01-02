//! Weird ducking envelope use for MG delay.

use super::{Coeff, calc_coeff};

#[derive(Debug)]
pub struct Params {
    pub attack_time: f32,
    pub release_time: f32,
}

#[derive(Debug)]
pub struct Coeffs {
    attack: Coeff,
    release: Coeff,
}

#[must_use]
pub fn calc_coeffs(params: &Params, sampling_rate: f32) -> Coeffs {
    Coeffs {
        attack: calc_coeff(params.attack_time, sampling_rate),
        release: calc_coeff(params.release_time, sampling_rate),
    }
}

#[derive(Debug, Default, Clone)]
enum Stage {
    Attack {
        value: f32,
    },
    #[default]
    On,
    Release {
        value: f32,
    },
}

#[derive(Debug, Default)]
pub struct Ar {
    stage: Stage,
    note_count: usize,
}

impl Ar {
    pub fn reset(&mut self) {
        self.stage = Default::default();
        self.note_count = 0;
    }

    pub fn on(&mut self) {
        match self.stage.clone() {
            Stage::Attack { .. } | Stage::Release { .. } => {
                self.note_count += 1;
            }
            Stage::On => {
                if self.note_count > 0 {
                    self.note_count -= 1;
                } else {
                    self.stage = Stage::Release { value: 1.0 };
                    self.note_count = 1;
                }
            }
        }
    }

    pub fn off(&mut self) {
        self.note_count = self.note_count.saturating_sub(1);
    }

    pub fn process(&mut self, coeffs: &Coeffs) -> f32 {
        let (out, new_stage) = match self.stage {
            Stage::Attack { value } => match coeffs.attack {
                Coeff::Instant => (1.0, Stage::On),
                Coeff::Increment(incr) => {
                    let value = value + incr;
                    if value >= 1.0 {
                        (1.0, Stage::On)
                    } else {
                        (value, Stage::Attack { value })
                    }
                }
            },
            Stage::On => (1.0, Stage::On),
            Stage::Release { value } => match coeffs.release {
                Coeff::Instant => (0.0, Stage::Attack { value: 0.0 }),
                Coeff::Increment(incr) => {
                    let value = value - incr;
                    if value <= 0.0 {
                        (0.0, Stage::Attack { value: 0.0 })
                    } else {
                        (value, Stage::Release { value })
                    }
                }
            },
        };
        self.stage = new_stage;
        out
    }
}

#[cfg(test)]
mod tests {
    use super::{Ar, Params, calc_coeffs};
    use assert_approx_eq::assert_approx_eq;
    use snapshots::assert_snapshot;

    #[test]
    fn starts_fully_on() {
        let mut ar: Ar = Default::default();
        let coeffs = calc_coeffs(
            &Params {
                attack_time: 0.0,
                release_time: 0.0,
            },
            48000.0,
        );
        assert_eq!(
            std::iter::repeat_with(|| ar.process(&coeffs))
                .take(100)
                .collect::<Vec<_>>(),
            std::iter::repeat_n(1f32, 100).collect::<Vec<_>>()
        );
    }

    #[test]
    fn reset() {
        let mut ar: Ar = Default::default();
        let coeffs = calc_coeffs(
            &Params {
                attack_time: 0.010,
                release_time: 0.200,
            },
            48000.0,
        );
        ar.on();
        let initial = std::iter::repeat_with(|| ar.process(&coeffs))
            .take(100)
            .collect::<Vec<_>>();
        ar.reset();
        ar.on();
        let reset = std::iter::repeat_with(|| ar.process(&coeffs))
            .take(100)
            .collect::<Vec<_>>();
        for (a, b) in initial.iter().zip(reset.iter()) {
            assert_approx_eq!(a, b);
        }
    }

    #[test]
    fn handles_multiple_notes() {
        let mut ar: Ar = Default::default();
        let coeffs = calc_coeffs(
            &Params {
                attack_time: 0.0,
                release_time: 0.0,
            },
            48000.0,
        );
        ar.on();
        ar.on();
        assert_approx_eq!(ar.process(&coeffs), 0.0);
        ar.off();
        assert_approx_eq!(ar.process(&coeffs), 1.0);
        ar.on();
        assert_approx_eq!(ar.process(&coeffs), 1.0);
        ar.off();
        assert_approx_eq!(ar.process(&coeffs), 1.0);
        ar.off();
        assert_approx_eq!(ar.process(&coeffs), 1.0);
        ar.on();
        assert_approx_eq!(ar.process(&coeffs), 0.0);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn snapshot() {
        let mut ar: Ar = Default::default();
        let coeffs = calc_coeffs(
            &Params {
                attack_time: 0.200,
                release_time: 0.010,
            },
            48000.0,
        );
        ar.on();
        assert_snapshot!(
            "ar",
            48000,
            (0..48000).map(|i| {
                if i == 24000 {
                    ar.off();
                }
                ar.process(&coeffs)
            })
        );
    }
}
