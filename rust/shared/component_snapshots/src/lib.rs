#![allow(clippy::implicit_hasher)]

use conformal_component::ProcessingMode;

pub mod effect;
pub mod synth;

pub struct ProcessingParams {
    pub sampling_rate: f32,
    pub max_buffer_size: usize,
    pub processing_mode: ProcessingMode,
}

impl Default for ProcessingParams {
    fn default() -> Self {
        Self {
            sampling_rate: 48000.0,
            max_buffer_size: 512,
            processing_mode: ProcessingMode::Realtime,
        }
    }
}
