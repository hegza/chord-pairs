pub mod sound;

use crate::sound::make_sample;
use eframe::egui;
use egui::{Button, Color32, Label, RichText, Stroke, Vec2};
use fon::{chan::Ch16, Audio};
use num_integer::Roots;
use rand::{prelude::*, thread_rng};
use rodio::{buffer::SamplesBuffer, OutputStream, OutputStreamHandle};
use sound::Chord;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1024.0, 768.0)),
        ..Default::default()
    };
    eframe::run_native(
        env!("CARGO_CRATE_NAME"),
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct Card {
    chord: Chord,
    sample: Audio<Ch16, 2>,
}

const BTN_SIZE: Vec2 = Vec2::new(100., 100.);

impl Card {
    /// Returns clicked
    fn button_ui(&self, state: &CardState, ui: &mut egui::Ui) -> bool {
        let open = *state == CardState::FaceUp;
        ui.add(
            Button::new(if open { "o" } else { "?" })
                .min_size(BTN_SIZE)
                .stroke(Stroke::new(
                    2.,
                    if open {
                        Color32::LIGHT_GREEN
                    } else {
                        Color32::LIGHT_RED
                    },
                ))
                .fill(Color32::DARK_GRAY),
        )
        .clicked()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CardState {
    FaceDown,
    FaceUp,
}

struct MyApp {
    cards: Vec<(Option<Card>, CardState)>,
    audio_sink: rodio::Sink,
    first_guess: Option<usize>,
    _audio_handle: OutputStreamHandle,
    _audio_stream: OutputStream,
    wrong_guess_count: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        use sound::Chord as C;
        let all_chords = [
            C::C3Minor,
            C::D3Minor,
            C::E3Minor,
            C::F3Minor,
            C::G3Minor,
            C::A3Minor,
            C::B3Minor,
        ];
        let mut cards = all_chords
            .iter()
            .cycle()
            .take(all_chords.len() * 2)
            .map(|chord| (chord.clone(), make_sample(chord.clone(), 500)))
            .map(|(chord, sample)| Card { sample, chord })
            .map(|x| (Some(x), CardState::FaceDown))
            .collect::<Vec<_>>();
        cards.shuffle(&mut thread_rng());

        let (audio_stream, audio_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&audio_handle).unwrap();

        Self {
            cards,
            audio_sink: sink,
            _audio_handle: audio_handle,
            _audio_stream: audio_stream,
            first_guess: None,
            wrong_guess_count: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let card_count = self.cards.len();
        let first_chord = if let Some(first_guess) = self.first_guess {
            if let Some(first_card) = &self.cards[first_guess].0 {
                Some(first_card.chord.clone())
            } else {
                None
            }
        } else {
            None
        };
        let mut close_first = false;

        let mut cur_idx = 0;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                for row in &mut self.cards.chunks_mut(card_count.sqrt() + 1) {
                    ui.horizontal(|ui| {
                        for (card_opt, state) in row {
                            let open = *state == CardState::FaceUp;
                            if let Some(card) = card_opt {
                                // If button was pressed and it was not already face up
                                if card.button_ui(state, ui) && !open {
                                    // Play sound
                                    let buf =
                                        SamplesBuffer::new(2, 48_000, card.sample.as_i16_slice());
                                    self.audio_sink.append(buf);

                                    // This is the second guess
                                    if let Some(first_chord) = &first_chord {
                                        // Guess right
                                        if first_chord == &card.chord
                                            && self.first_guess != Some(cur_idx)
                                        {
                                            // Set also this one as opened
                                            *state = CardState::FaceUp;
                                            self.first_guess = None;
                                        }
                                        // Wrong guess
                                        else {
                                            // Close both
                                            *state = CardState::FaceDown;
                                            close_first = true;
                                            self.wrong_guess_count += 1;
                                        }
                                    }
                                    // This is the first guess, set current as open
                                    else {
                                        *state = CardState::FaceUp;
                                        self.first_guess = Some(cur_idx);
                                    }
                                }
                            }
                            cur_idx += 1;
                        }
                    });
                }
                ui.add(Label::new(
                    RichText::new(format!(
                        "Number of wrong guesses: {}",
                        self.wrong_guess_count
                    ))
                    .size(32.),
                ));
            });
        });

        if close_first {
            *(&mut self.cards[self.first_guess.unwrap()].1) = CardState::FaceDown;
            self.first_guess = None;
        }
    }
}
