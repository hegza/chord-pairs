pub mod audio;
pub mod board;
pub mod samples;

use audio::ChordPlayer;
use board::Board;
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

struct MyApp {
    board: Board,
    audio: ChordPlayer,
}

impl Default for MyApp {
    fn default() -> Self {
        let board = Board::default();
        let audio = ChordPlayer::from_chords(board.chords().into_iter());

        Self { board, audio }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.board.update(ui, &self.audio);
        });
    }
}
