use crate::{
    audio::ChordPlayer,
    samples::{Chord, ChordKind, Note},
};
use eframe::egui;
use egui::{Button, Color32, Label, RichText, Stroke, Vec2};
use num_integer::Roots;
use rand::{seq::SliceRandom, thread_rng};
use strum::IntoEnumIterator;

pub struct Card {
    chord: Chord,
}

const BTN_SIZE: Vec2 = Vec2::new(100., 100.);

impl Card {
    /// Returns clicked
    pub fn button_ui(&self, state: &CardState, ui: &mut egui::Ui) -> bool {
        let text = self.text(state);
        let color = self.stroke_color(state);
        ui.add(
            Button::new(text)
                .min_size(BTN_SIZE)
                .stroke(Stroke::new(2., color))
                .fill(self.fill_color(state)),
        )
        .clicked()
    }

    fn text(&self, state: &CardState) -> String {
        match state {
            CardState::FaceDown => "?".to_owned(),
            CardState::FaceUp => "o".to_owned(),
            CardState::Revealed => format!("{}", self.chord),
        }
    }
    fn stroke_color(&self, state: &CardState) -> Color32 {
        match state {
            CardState::FaceDown => Color32::LIGHT_RED,
            CardState::FaceUp => Color32::LIGHT_BLUE,
            CardState::Revealed => Color32::LIGHT_GREEN,
        }
    }
    fn fill_color(&self, _state: &CardState) -> Color32 {
        Color32::DARK_GRAY
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CardState {
    FaceDown,
    FaceUp,
    /// Also show chord
    Revealed,
}

pub struct Board {
    cards: Vec<(Option<Card>, CardState)>,
    guess_state: GuessState,
    wrong_guess_count: usize,
}

/// How many, and which cards are currently turned over by the player
#[derive(Debug, PartialEq, Eq)]
enum GuessState {
    /// None guessed
    None,
    /// One guessed
    One(usize),
    /*
    /// Two guessed
    Two(usize, usize),
    */
}

pub enum PlayerAction {
    LookAt(usize),
}

#[derive(Clone)]
pub enum PairCount {
    Max,
    N(usize),
}

impl Board {
    pub fn new(pair_count: PairCount) -> Self {
        let all_chords_iter = Note::iter()
            .map(|note| Chord {
                basenote: note,
                kind: ChordKind::Minor,
            })
            .chain(Note::iter().map(|note| Chord {
                basenote: note,
                kind: ChordKind::Major,
            }));

        let all_chords = match pair_count {
            PairCount::Max => all_chords_iter.collect::<Vec<_>>(),
            PairCount::N(n) => all_chords_iter.take(n).collect::<Vec<_>>(),
        };

        let all_chords_twice = all_chords.iter().cycle().take(all_chords.len() * 2);
        let mut cards = all_chords_twice
            .map(|chord| Card {
                chord: chord.clone(),
            })
            .map(|x| (Some(x), CardState::FaceDown))
            .collect::<Vec<_>>();
        cards.shuffle(&mut thread_rng());

        Self {
            cards,
            wrong_guess_count: 0,
            guess_state: GuessState::None,
        }
    }

    fn look_at_card(&mut self, card_idx: usize, audio: &ChordPlayer) {
        if let Some(card) = &self.cards[card_idx].0 {
            // Play sound
            audio.play_chord(&card.chord);

            match self.guess_state {
                // This is the first guess, set current as open
                GuessState::None => {
                    self.cards[card_idx].1 = CardState::FaceUp;
                    self.guess_state = GuessState::One(card_idx);
                }
                // This is the second guess
                GuessState::One(first_guess_idx) => {
                    if let Some(first_card) = &self.cards[first_guess_idx].0 {
                        // Guess right
                        if &first_card.chord == &card.chord
                            && self.guess_state != GuessState::One(card_idx)
                        {
                            // Set both as revealed
                            self.cards[card_idx].1 = CardState::Revealed;
                            self.cards[first_guess_idx].1 = CardState::Revealed;
                        }
                        // Wrong guess
                        else {
                            // Close both and add one to wrong guess count
                            self.cards[card_idx].1 = CardState::FaceDown;
                            self.cards[first_guess_idx].1 = CardState::FaceDown;
                            self.wrong_guess_count += 1;
                        }
                        self.guess_state = GuessState::None;
                    }
                }
            }
        } else {
            eprintln!(
                "player tried to look at card {} which didn't exist",
                card_idx
            );
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui, audio: &ChordPlayer) {
        if let Some(action) = self.ui(ui) {
            match action {
                PlayerAction::LookAt(card_idx) => self.look_at_card(card_idx, audio),
            }
        }
    }

    fn ui(&self, ui: &mut egui::Ui) -> Option<PlayerAction> {
        let card_count = self.cards.len();
        let mut action = None;

        ui.vertical(|ui| {
            let row_len = card_count.sqrt() + 1;
            for (row_idx, row) in self.cards.chunks(row_len).enumerate() {
                ui.horizontal(|ui| {
                    for (col_idx, (card_opt, state)) in row.iter().enumerate() {
                        if let Some(card) = card_opt {
                            // If button was pressed and it was not already face up or revealed
                            if card.button_ui(state, ui) && state == &CardState::FaceDown {
                                let cur_idx = row_idx * row_len + col_idx;
                                action = Some(PlayerAction::LookAt(cur_idx));
                            }
                        }
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

        action
    }

    pub fn chords(&self) -> Vec<Chord> {
        self.cards
            .iter()
            .filter_map(|card| card.0.as_ref())
            .map(|card| card.chord.clone())
            .collect()
    }
}
