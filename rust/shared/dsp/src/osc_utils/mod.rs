/// This is a second-order lagrange step function residual.
/// It would be nice to have a blog-style derivation of this,
/// but for now the basic idea is that this is a magic signal
/// that we can add to any -1 -> 1 step function to minimally anti-alias
/// it.
#[must_use]
pub fn polyblep2_residual(phase: f32, increment: f32) -> f32 {
    if phase < increment {
        // Generate the post-jump residual.
        let t = phase / increment;
        -((t - 1.0) * (t - 1.0))
    } else if phase > 1.0 - increment {
        // Generate the pre-jump residual.
        let t = (phase - 1.0) / increment;
        (t + 1.0) * (t + 1.0)
    } else {
        0.0
    }
}

/// This is a second-order residual for the polyBLAMP, scaled for
/// derivative discontinuities going from derivatives of -increment to
/// derivatives of +increment, for example the bottom of a triangle wave.
#[must_use]
pub fn polyblamp2_residual(phase: f32, increment: f32) -> f32 {
    if phase < increment * 0.5 {
        // Generate the post-jump residual.
        let t = phase / increment;
        increment * (t - 0.5) * (t - 0.5)
    } else if phase > 1.0 - increment * 0.5 {
        // Generate the pre-jump residual.
        let t = (phase - 1.0) / increment;
        increment * (t + 0.5) * (t + 0.5)
    } else {
        0.0
    }
}

// Optimization opportunity - this could probably be well approximated
#[must_use]
pub fn increment(midi_pitch: f32, sampling_rate: f32) -> f32 {
    440f32 * 2.0f32.powf((midi_pitch - 69f32) / 12f32) / sampling_rate
}
