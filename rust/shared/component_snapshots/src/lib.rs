#![warn(
    nonstandard_style,
    rust_2018_idioms,
    future_incompatible,
    clippy::pedantic,
    clippy::todo
)]
#![allow(
    clippy::type_complexity,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::default_trait_access
)]

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
