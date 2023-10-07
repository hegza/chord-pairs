pub mod audio;
pub mod board;

use audio::AudioPlayer;
use board::Board;
use eframe::egui;

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

struct MyApp {
    board: Board,
    audio: AudioPlayer,
}

impl Default for MyApp {
    fn default() -> Self {
        let audio = AudioPlayer::default();

        let board = Board::default();
        Self { board, audio }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.board.ui(ui, &mut self.audio);
        });
    }
}
