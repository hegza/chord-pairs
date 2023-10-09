pub mod audio;
pub mod board;
pub mod samples;

use audio::ChordPlayer;
use board::{Board, PairCount};
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1024.0, 768.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Chord Pairs",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

enum AppState {
    DifficultySelect,
    Loading(PairCount),
    Game(Game),
}

struct MyApp {
    state: AppState,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            state: AppState::DifficultySelect,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut nstate = None;
        match &mut self.state {
            AppState::DifficultySelect => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let game_type = {
                        if ui.button("10 card game").clicked() {
                            Some(PairCount::N(5))
                        } else if ui.button("20 card game").clicked() {
                            Some(PairCount::N(10))
                        } else if ui.button("40 card game").clicked() {
                            Some(PairCount::N(20))
                        } else if ui.button("MAXIMUM DIFFICULTY").clicked() {
                            Some(PairCount::Max)
                        } else {
                            None
                        }
                    };
                    if let Some(game_type) = game_type {
                        nstate = Some(AppState::Loading(game_type));
                    } else {
                        nstate = Some(AppState::DifficultySelect);
                    }
                });
            }
            AppState::Loading(game_type) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Generating chords...");
                });
                nstate = Some(AppState::Game(Game::new(game_type.clone())));
            }
            AppState::Game(game) => {
                game.update(ctx, frame);
            }
        }
        if let Some(nstate) = nstate {
            self.state = nstate;
        }
    }
}

struct Game {
    board: Board,
    audio: ChordPlayer,
}

impl Game {
    fn new(pair_count: PairCount) -> Self {
        let board = Board::new(pair_count);
        let audio = ChordPlayer::from_chords(board.chords().into_iter());

        Self { board, audio }
    }
}

impl Game {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.board.update(ui, &self.audio));
    }
}
