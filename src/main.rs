mod app;
mod game;
mod piece;

use app::ChessApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Chess",
        options,
        Box::new(|cc| Ok(Box::new(ChessApp::new(cc)))),
    )
}
