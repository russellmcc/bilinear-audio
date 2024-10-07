use super::{Event, EventData, NoteExpressionCurve};
use conformal_component::{
    audio::approx_eq,
    events::{self as events, NoteData, NoteID},
};

use crate::{NoteExpressionPoint, NoteExpressionState};

#[derive(Clone, Debug, PartialEq)]
enum VoicePlayingState {
    Idle { order: usize },
    Note { order: usize, id: NoteID, pitch: u8 },
}

impl NoteExpressionState {
    fn update_note_expression(&mut self, new: NoteExpressionState) -> Option<NoteExpressionState> {
        const EPSILON: f32 = 1e-6;

        let mut any_change = false;
        let mut update =
            |field: fn(&NoteExpressionState) -> f32,
             field_mut: fn(&mut NoteExpressionState) -> &mut f32| {
                if !approx_eq(field(&new), field(self), EPSILON) {
                    any_change = true;
                }
                *field_mut(self) = field(&new);
            };
        update(|x| x.pitch_bend, |x| &mut x.pitch_bend);
        update(|x| x.aftertouch, |x| &mut x.aftertouch);
        update(|x| x.timbre, |x| &mut x.timbre);
        if any_change {
            Some(new)
        } else {
            None
        }
    }

    fn update_with(&self, event: events::NoteExpression) -> NoteExpressionState {
        let mut ret = *self;
        match event {
            events::NoteExpression::PitchBend(x) => ret.pitch_bend = x,
            events::NoteExpression::Aftertouch(x) => ret.aftertouch = x,
            events::NoteExpression::Timbre(x) => ret.timbre = x,
        }
        ret
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Voice {
    playing: VoicePlayingState,
    expression: NoteExpressionState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    voices: Vec<Voice>,

    voices_compress_order_scratch: Vec<(usize, usize)>,
}

struct EventStreamStep {
    voice: usize,
    sample_offset: usize,
    first: Option<EventData>,
    second: Option<EventData>,
    expression: Option<NoteExpressionState>,
}

impl Iterator for EventStreamStep {
    type Item = (usize, super::Event);

    fn next(&mut self) -> Option<Self::Item> {
        let first: Option<EventData> = self.first.take();
        let second = self.second.take();
        match (first, second) {
            (Some(first), Some(second)) => {
                self.first = Some(second);
                self.second = None;
                Some((
                    self.voice,
                    Event {
                        sample_offset: self.sample_offset,
                        data: first,
                    },
                ))
            }
            (Some(first), None) => {
                self.first = None;
                Some((
                    self.voice,
                    Event {
                        sample_offset: self.sample_offset,
                        data: first,
                    },
                ))
            }
            (None, Some(_)) => {
                panic!("Invariant violation")
            }
            (None, None) => None,
        }
    }
}

impl EventStreamStep {
    fn new1(voice: usize, first: Event, expression: Option<NoteExpressionState>) -> Self {
        Self {
            voice,
            sample_offset: first.sample_offset,
            first: Some(first.data),
            second: None,
            expression,
        }
    }
    fn new2(
        voice: usize,
        first: Event,
        second: EventData,
        expression: Option<NoteExpressionState>,
    ) -> Self {
        Self {
            voice,
            sample_offset: first.sample_offset,
            first: Some(first.data),
            second: Some(second),
            expression,
        }
    }
    fn new0() -> Self {
        Self {
            voice: 0,
            sample_offset: 0,
            first: None,
            second: None,
            expression: None,
        }
    }
    fn new_expression(
        voice: usize,
        sample_offset: usize,
        expression: Option<NoteExpressionState>,
    ) -> Self {
        Self {
            voice,
            sample_offset,
            first: None,
            second: None,
            expression,
        }
    }
}

fn synthetic_note_off(id: NoteID, pitch: u8) -> EventData {
    EventData::NoteOff {
        data: NoteData {
            id,
            pitch,
            velocity: 1.0,
            tuning: 0.0,
        },
    }
}

impl State {
    pub fn new(max_voices: usize) -> Self {
        assert!(max_voices > 0);
        Self {
            voices: (0..max_voices)
                .map(|i| Voice {
                    playing: VoicePlayingState::Idle { order: i },
                    expression: NoteExpressionState::default(),
                })
                .collect(),
            voices_compress_order_scratch: Vec::with_capacity(max_voices),
        }
    }

    pub fn reset(&mut self) {
        let num_voices = self.voices.len();
        self.voices.clear();
        self.voices.extend((0..num_voices).map(|i| Voice {
            playing: VoicePlayingState::Idle { order: i },
            expression: NoteExpressionState::default(),
        }));
    }

    /// Note that the events must be sorted by time!
    pub fn dispatch_events(
        mut self,
        events: impl IntoIterator<Item = events::Event>,
    ) -> impl IntoIterator<Item = (usize, Event)> {
        events
            .into_iter()
            .flat_map(move |event| self.update_single_and_dispatch(&event))
    }

    pub fn note_expressions_for_voice(
        mut self,
        voice: usize,
        events: impl Iterator<Item = events::Event> + Clone,
    ) -> NoteExpressionCurve<impl IntoIterator<Item = NoteExpressionPoint> + Clone> {
        let raw = std::iter::once(NoteExpressionPoint {
            time: 0,
            state: self.voices[voice].expression,
        })
        .chain(events.filter_map(move |event| {
            let time = event.sample_offset;
            self.update_single_and_dispatch(&event)
                .expression
                .map(|state| NoteExpressionPoint { time, state })
        }));

        NoteExpressionCurve::new(raw).unwrap()
    }

    fn update_single_and_dispatch(&mut self, event: &events::Event) -> EventStreamStep {
        match event.data {
            events::Data::NoteOn {
                data: data @ NoteData {
                    id: note_id, pitch, ..
                },
            } => {
                let mut open_index = None;
                let mut open_index_order = None;
                let mut old_voice_index = None;
                let mut old_voice_order = None;
                let mut new_voice_order = None;
                for (
                    index,
                    Voice {
                        playing,
                        expression,
                    },
                ) in self.voices.iter_mut().enumerate()
                {
                    match (playing, open_index_order) {
                        (VoicePlayingState::Idle { order }, None) => {
                            open_index = Some(index);
                            open_index_order = Some(*order);
                        }
                        (VoicePlayingState::Idle { order }, Some(open_index_order_))
                            if *order < open_index_order_ =>
                        {
                            open_index = Some(index);
                            open_index_order = Some(*order);
                        }
                        (VoicePlayingState::Note { id, .. }, _) if note_id == *id => {
                            return EventStreamStep::new1(
                                index,
                                Event {
                                    sample_offset: event.sample_offset,
                                    data: EventData::NoteOn { data },
                                },
                                expression.update_note_expression(Default::default()),
                            );
                        }
                        (VoicePlayingState::Note { order, .. }, _) => {
                            if let Some(old_voice_order_) = old_voice_order {
                                if *order < old_voice_order_ {
                                    old_voice_index = Some(index);
                                    old_voice_order = Some(*order);
                                }
                            } else {
                                old_voice_index = Some(index);
                                old_voice_order = Some(*order);
                            }
                            if let Some(new_voice_order_) = new_voice_order {
                                if *order > new_voice_order_ {
                                    new_voice_order = Some(*order);
                                }
                            } else {
                                new_voice_order = Some(*order);
                            }
                        }
                        _ => {}
                    }
                }

                let mut extra_off = None;
                let open_index = open_index.unwrap_or_else(|| {
                    // If we got here, no notes are open - we have to steal one!
                    // We always steal the oldest note.

                    // Make a synthetic note off event for the oldest note
                    if let VoicePlayingState::Note { id, pitch, .. } =
                        self.voices[old_voice_index.unwrap()].playing
                    {
                        extra_off = Some(synthetic_note_off(id, pitch));
                    } else {
                        panic!("Internal error");
                    }
                    old_voice_index.unwrap()
                });

                self.voices[open_index].playing = VoicePlayingState::Note {
                    id: note_id,
                    order: new_voice_order.map_or(0, |x| x + 1),
                    pitch,
                };
                let expression_point = self.voices[open_index]
                    .expression
                    .update_note_expression(Default::default());

                if let Some(extra_off) = extra_off {
                    EventStreamStep::new2(
                        open_index,
                        Event {
                            sample_offset: event.sample_offset,
                            data: extra_off,
                        },
                        EventData::NoteOn { data },
                        expression_point,
                    )
                } else {
                    EventStreamStep::new1(
                        open_index,
                        Event {
                            sample_offset: event.sample_offset,
                            data: EventData::NoteOn { data },
                        },
                        expression_point,
                    )
                }
            }
            events::Data::NoteOff {
                data: data @ NoteData { id: note_id, .. },
            } => {
                let order = self
                    .voices
                    .iter()
                    .filter_map(|x| {
                        if let VoicePlayingState::Idle { order } = x.playing {
                            Some(order)
                        } else {
                            None
                        }
                    })
                    .max()
                    .map_or(0, |x| x + 1);
                for (index, voice_state) in self.voices.iter_mut().enumerate() {
                    match voice_state.playing {
                        VoicePlayingState::Note { id, .. } if note_id == id => {
                            voice_state.playing = VoicePlayingState::Idle { order };
                            return EventStreamStep::new1(
                                index,
                                Event {
                                    sample_offset: event.sample_offset,
                                    data: EventData::NoteOff { data },
                                },
                                Default::default(),
                            );
                        }
                        _ => {}
                    }
                }
                EventStreamStep::new0()
            }
            events::Data::NoteExpression {
                data:
                    events::NoteExpressionData {
                        id: note_id,
                        expression: expression_data,
                    },
            } => {
                for (
                    index,
                    Voice {
                        playing,
                        expression,
                    },
                ) in self.voices.iter_mut().enumerate()
                {
                    match playing {
                        VoicePlayingState::Note { id, .. } if note_id == *id => {
                            return EventStreamStep::new_expression(
                                index,
                                event.sample_offset,
                                expression.update_note_expression(
                                    expression.update_with(expression_data),
                                ),
                            );
                        }
                        _ => {}
                    }
                }
                EventStreamStep::new0()
            }
        }
    }

    fn compress_idle_order(&mut self) {
        self.voices_compress_order_scratch.clear();
        self.voices_compress_order_scratch.extend(
            self.voices
                .iter()
                .filter_map(|voice_state| {
                    if let VoicePlayingState::Idle { order } = voice_state.playing {
                        Some(order)
                    } else {
                        None
                    }
                })
                .enumerate(),
        );
        self.voices_compress_order_scratch.sort_by_key(|x| x.1);
        for (o, (_, vo)) in self.voices_compress_order_scratch.iter_mut().enumerate() {
            *vo = o;
        }
        self.voices_compress_order_scratch.sort_by_key(|x| x.0);
        self.voices
            .iter_mut()
            .filter_map(|voice_state| {
                if let VoicePlayingState::Idle { ref mut order } = voice_state.playing {
                    Some(order)
                } else {
                    None
                }
            })
            .zip(self.voices_compress_order_scratch.iter().map(|x| x.1))
            .for_each(|(order, new_order)| *order = new_order);
    }

    fn compress_note_order(&mut self) {
        self.voices_compress_order_scratch.clear();
        self.voices_compress_order_scratch.extend(
            self.voices
                .iter_mut()
                .filter_map(|voice_state| {
                    if let VoicePlayingState::Note { ref mut order, .. } = voice_state.playing {
                        Some(*order)
                    } else {
                        None
                    }
                })
                .enumerate(),
        );
        self.voices_compress_order_scratch.sort_by_key(|x| x.1);
        for (o, (_, vo)) in self.voices_compress_order_scratch.iter_mut().enumerate() {
            *vo = o;
        }
        self.voices_compress_order_scratch.sort_by_key(|x| x.0);
        self.voices
            .iter_mut()
            .filter_map(|voice_state| {
                if let VoicePlayingState::Note { ref mut order, .. } = voice_state.playing {
                    Some(order)
                } else {
                    None
                }
            })
            .zip(self.voices_compress_order_scratch.iter().map(|x| x.1))
            .for_each(|(order, new_order)| *order = new_order);
    }

    /// Note that the events must be sorted by time!
    pub fn update(&mut self, events: impl IntoIterator<Item = events::Event>) {
        for event in events {
            self.update_single_and_dispatch(&event);
        }

        // compress orders - this keeps the `order` member bounded between buffers.
        self.compress_idle_order();
        self.compress_note_order();
    }
}

#[cfg(test)]
mod tests;
