mod app;
mod game;
mod piece;

use app::ChessApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([920.0, 680.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Chess",
        options,
        Box::new(|cc| Ok(Box::new(ChessApp::new(cc)))),
    )
}

//fn move_count_recursive(state: &mut game::GameState, depth: usize) -> usize {
//    if depth == 1 {
//        return state.get_legal_move_list().len();
//    }
//
//    let mut count = 0;
//
//    let legal_moves = state.get_legal_move_list();
//
//    for (from, to) in legal_moves {
//        state.make_move(from, to);
//        count += move_count_recursive(state, depth - 1);
//        state.undo_move();
//    }
//
//    count
//}
//
//fn main() {
//    let i = 6;
//    let mut initial_state =
//        game::GameState::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
//    let move_count = move_count_recursive(&mut initial_state, i);
//    println!("Move count at depth {}: {}", i, move_count);
//}
