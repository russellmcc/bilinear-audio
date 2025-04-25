use dsp::osc_utils::polyblep2_residual;

fn rotate(phase: f32, x: f32) -> f32 {
    let phase = phase + (1.0 - x);
    if phase > 1.0 { phase - 1.0 } else { phase }
}

#[must_use]
pub fn pulse(phase: f32, increment: f32, width: f32) -> f32 {
    if width < increment {
        -1.0
    } else if width > 1.0 - increment {
        1.0
    } else {
        (if phase < width { -1.0 } else { 1.0 }) - polyblep2_residual(phase, increment)
            + polyblep2_residual(rotate(phase, width), increment)
    }
}
