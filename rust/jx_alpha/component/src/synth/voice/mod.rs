use conformal_component::{events::NoteData, parameters, pzip};
use conformal_poly::{EventData, Voice as VoiceTrait};
use itertools::izip;
use oscillators::{OscillatorSettings, Shape};

mod oscillators;

const PITCH_BEND_WIDTH: f32 = 2.;

/// Converts a MIDI pitch to a phase increment
fn increment(midi_pitch: f32, sampling_rate: f32) -> f32 {
    440f32 * 2.0f32.powf((midi_pitch - 69f32) / 12f32) / sampling_rate
}

#[derive(Default, Debug, Clone)]
pub struct SharedData {}

#[derive(Clone, Debug, Default)]
pub struct Voice {
    pitch: Option<f32>,
    sampling_rate: f32,
    oscillators: oscillators::Oscillators,
}

impl VoiceTrait for Voice {
    type SharedData<'a> = SharedData;

    fn new(_max_samples_per_process_call: usize, sampling_rate: f32) -> Self {
        Self {
            pitch: None,
            oscillators: oscillators::Oscillators::new(),
            sampling_rate,
        }
    }

    fn handle_event(&mut self, event: &conformal_poly::EventData) {
        match event {
            EventData::NoteOn {
                data: NoteData { pitch, .. },
            } => {
                self.pitch = Some(f32::from(*pitch));
            }
            EventData::NoteOff { .. } => {
                self.pitch = None;
                self.oscillators.reset();
            }
        }
    }

    fn process(
        &mut self,
        events: impl IntoIterator<Item = conformal_poly::Event>,
        params: &impl parameters::BufferStates,
        note_expressions: conformal_poly::NoteExpressionCurve<
            impl Iterator<Item = conformal_poly::NoteExpressionPoint> + Clone,
        >,
        _data: Self::SharedData<'_>,
        output: &mut [f32],
    ) {
        let mut events = events.into_iter().peekable();
        for ((index, sample), (gain, global_pitch_bend), expression) in izip!(
            output.iter_mut().enumerate(),
            pzip!(params[numeric "gain", numeric "pitch_bend"]),
            note_expressions.iter_by_sample(),
        ) {
            while let Some(conformal_poly::Event {
                sample_offset,
                data,
            }) = events.peek()
            {
                if sample_offset > &index {
                    break;
                }
                self.handle_event(data);
                events.next();
            }
            if let Some(pitch) = self.pitch {
                let total_pitch_bend = global_pitch_bend * PITCH_BEND_WIDTH + expression.pitch_bend;
                let adjusted_pitch = pitch + total_pitch_bend;
                let increment = increment(adjusted_pitch, self.sampling_rate);
                *sample = self.oscillators.generate(&oscillators::Settings {
                    oscillators: [
                        OscillatorSettings {
                            increment,
                            shape: Shape::Saw,
                            gain: 1.0,
                        },
                        OscillatorSettings {
                            increment,
                            shape: Shape::Saw,
                            gain: 0.0,
                        },
                    ],
                }) * gain
                    / 100.;
            }
        }
    }

    fn quiescent(&self) -> bool {
        self.pitch.is_none()
    }

    fn reset(&mut self) {
        self.pitch = None;
        self.oscillators.reset();
    }
}
