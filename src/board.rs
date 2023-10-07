use crate::audio::{make_sample, AudioPlayer, Chord};
use eframe::egui;
use egui::{Button, Color32, Label, RichText, Stroke, Vec2};
use fon::{chan::Ch16, Audio};
use num_integer::Roots;
use rand::{prelude::*, thread_rng};

pub struct Card {
    chord: Chord,
    sample: Audio<Ch16, 2>,
}

const BTN_SIZE: Vec2 = Vec2::new(100., 100.);

impl Card {
    /// Returns clicked
    pub fn button_ui(&self, state: &CardState, ui: &mut egui::Ui) -> bool {
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
pub enum CardState {
    FaceDown,
    FaceUp,
}

pub struct Board {
    cards: Vec<(Option<Card>, CardState)>,
    first_guess: Option<usize>,
    wrong_guess_count: usize,
}

impl Board {
    pub fn ui(&mut self, ui: &mut egui::Ui, audio: &mut AudioPlayer) {
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

        ui.vertical(|ui| {
            for row in &mut self.cards.chunks_mut(card_count.sqrt() + 1) {
                ui.horizontal(|ui| {
                    for (card_opt, state) in row {
                        let open = *state == CardState::FaceUp;
                        if let Some(card) = card_opt {
                            // If button was pressed and it was not already face up
                            if card.button_ui(state, ui) && !open {
                                // Play sound
                                audio.play_sample(&mut card.sample);

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

        if close_first {
            *(&mut self.cards[self.first_guess.unwrap()].1) = CardState::FaceDown;
            self.first_guess = None;
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        use crate::audio::Chord as C;
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

        Self {
            cards,
            first_guess: None,
            wrong_guess_count: 0,
        }
    }
}
