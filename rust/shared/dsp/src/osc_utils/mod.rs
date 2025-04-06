/// This is a second-order lagrange step function residual.
/// It would be nice to have a blog-style derivation of this,
/// but for now the basic idea is that this is a magic signal
/// that we can add to any -1 -> 1 step function to minimally anti-alias
/// it.
#[must_use]
pub fn polyblep2_residual(phase: f32, increment: f32) -> f32 {
    if phase < increment {
        // Generate the beginning residual.
        let t = phase / increment;
        -((t - 1.0) * (t - 1.0))
    } else if phase > 1.0 - increment {
        // Generate the ending residual.
        let t = (phase - 1.0) / increment;
        (t + 1.0) * (t + 1.0)
    } else {
        0.0
    }
}

// Optimization opportunity - this could probably be well approximated
#[must_use]
pub fn increment(midi_pitch: f32, sampling_rate: f32) -> f32 {
    440f32 * 2.0f32.powf((midi_pitch - 69f32) / 12f32) / sampling_rate
}
