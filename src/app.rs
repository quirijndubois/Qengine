use crate::game::GameState;
use crate::piece::Piece;
use eframe::egui;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::Cursor;

const MOVE_SOUND: &[u8] = include_bytes!("../sounds/Move.ogg");
const CAPTURE_SOUND: &[u8] = include_bytes!("../sounds/Capture.ogg");

pub const SQUARE_SIZE: f32 = 80.0;
pub const BOARD_SIZE: f32 = SQUARE_SIZE * 8.0;
pub const BOARD_ORIGIN: egui::Pos2 = egui::pos2(15.0, 15.0);

pub struct ChessApp {
    pub selected_cell: Option<i32>,
    pub game_state: GameState,

    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl ChessApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        Self {
            game_state: GameState::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            selected_cell: None,
            _stream,
            stream_handle,
        }
    }

    fn play_sound(&self, bytes: &'static [u8]) {
        if let Ok(sink) = Sink::try_new(&self.stream_handle) {
            let cursor = Cursor::new(bytes);
            if let Ok(source) = Decoder::new(cursor) {
                sink.append(source);
                sink.detach();
            }
        }
    }

    fn square_rect(col: usize, row: usize) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(
                BOARD_ORIGIN.x + col as f32 * SQUARE_SIZE,
                BOARD_ORIGIN.y + row as f32 * SQUARE_SIZE,
            ),
            egui::vec2(SQUARE_SIZE, SQUARE_SIZE),
        )
    }

    fn draw_square(painter: &egui::Painter, col: usize, row: usize, color: egui::Color32) {
        let rect = Self::square_rect(col, 7 - row);
        painter.rect_filled(rect, 0.0, color);
    }

    fn draw_piece(ui: &mut egui::Ui, col: usize, row: usize, piece: &Piece) {
        let rect = Self::square_rect(col as usize, 7 - row as usize);
        let (name, bytes) = piece.asset_data();
        ui.put(rect.shrink(10.0), egui::Image::from_bytes(name, bytes));
    }
}

impl eframe::App for ChessApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Fetch legal moves as a COPY to avoid borrow checker errors
            let legal_moves = if self.game_state.white_to_move {
                self.game_state.white_legal_moves
            } else {
                self.game_state.black_legal_moves
            };

            ui.horizontal_top(|ui| {
                // --- LEFT: BOARD CONTAINER ---
                ui.vertical(|ui| {
                    let (board_rect, _) = ui.allocate_at_least(
                        egui::vec2(BOARD_SIZE + 20.0, BOARD_SIZE + 20.0),
                        egui::Sense::click_and_drag(),
                    );

                    let painter = ui.painter_at(board_rect);

                    // 1. Draw Squares & Highlights
                    for row in 0..8 {
                        for col in 0..8 {
                            let is_dark = (row + col) % 2 == 0;
                            let base_color = if is_dark {
                                egui::Color32::from_rgb(181, 136, 99)
                            } else {
                                egui::Color32::from_rgb(240, 217, 181)
                            };

                            Self::draw_square(&painter, col, row, base_color);

                            let index = row * 8 + col;
                            let move_mask = 1u64 << index;

                            if let Some(sel) = self.selected_cell {
                                if sel == index as i32 {
                                    // Highlight selected square (Yellow)
                                    Self::draw_square(
                                        &painter,
                                        col,
                                        row,
                                        egui::Color32::from_rgba_unmultiplied(255, 255, 0, 120),
                                    );
                                } else if legal_moves[sel as usize] & move_mask != 0 {
                                    // Highlight legal move (Subtle Green)
                                    Self::draw_square(
                                        &painter,
                                        col,
                                        row,
                                        egui::Color32::from_rgba_unmultiplied(0, 255, 0, 50),
                                    );
                                }
                            }
                        }
                    }

                    // 2. Draw Pieces
                    for pos in 0..64 {
                        let col = pos % 8;
                        let row = pos / 8;
                        let mask = 1u64 << pos;

                        let piece_opt = if (self.game_state.white_pawns & mask) != 0 {
                            Some(Piece::WhitePawn)
                        } else if (self.game_state.white_knights & mask) != 0 {
                            Some(Piece::WhiteKnight)
                        } else if (self.game_state.white_bishops & mask) != 0 {
                            Some(Piece::WhiteBishop)
                        } else if (self.game_state.white_rooks & mask) != 0 {
                            Some(Piece::WhiteRook)
                        } else if (self.game_state.white_queens & mask) != 0 {
                            Some(Piece::WhiteQueen)
                        } else if (self.game_state.white_king & mask) != 0 {
                            Some(Piece::WhiteKing)
                        } else if (self.game_state.black_pawns & mask) != 0 {
                            Some(Piece::BlackPawn)
                        } else if (self.game_state.black_knights & mask) != 0 {
                            Some(Piece::BlackKnight)
                        } else if (self.game_state.black_bishops & mask) != 0 {
                            Some(Piece::BlackBishop)
                        } else if (self.game_state.black_rooks & mask) != 0 {
                            Some(Piece::BlackRook)
                        } else if (self.game_state.black_queens & mask) != 0 {
                            Some(Piece::BlackQueen)
                        } else if (self.game_state.black_king & mask) != 0 {
                            Some(Piece::BlackKing)
                        } else {
                            None
                        };

                        if let Some(piece) = piece_opt {
                            Self::draw_piece(ui, col, row, &piece);
                        }
                    }
                });

                // --- RIGHT: UI PANEL ---
                ui.vertical(|ui| {
                    ui.heading(egui::RichText::new("Qengine").size(32.0).strong());
                    ui.separator();

                    ui.add_space(20.0);
                    let turn = if self.game_state.white_to_move {
                        "WHITE"
                    } else {
                        "BLACK"
                    };
                    let turn_color = if self.game_state.white_to_move {
                        egui::Color32::LIGHT_BLUE
                    } else {
                        egui::Color32::LIGHT_GRAY
                    };

                    ui.label(
                        egui::RichText::new(format!("TURN: {}", turn))
                            .size(22.0)
                            .color(turn_color)
                            .strong(),
                    );

                    let total_moves: u32 = legal_moves.iter().map(|m| m.count_ones()).sum();
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new(format!("Legal Moves: {}", total_moves)).size(16.0),
                    );

                    ui.add_space(20.0);

                    let evaluation = self.game_state.evaluate();
                    let eval_text = if evaluation > 0 {
                        format!("Evaluation: +{:.2}", evaluation as f32 / 100.0)
                    } else if evaluation < 0 {
                        format!("Evaluation: {:.2}", evaluation as f32 / 100.0)
                    } else {
                        "Evaluation: 0.00".to_string()
                    };
                    ui.label(egui::RichText::new(eval_text).size(16.0));

                    ui.add_space(40.0);
                    if ui
                        .button(egui::RichText::new("RESTART GAME").size(18.0))
                        .clicked()
                    {
                        self.game_state = GameState::new(
                            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                        );
                        self.selected_cell = None;
                    }
                });

                // --- CLICK INTERACTION ---
                if ui.input(|i| i.pointer.any_pressed()) {
                    if let Some(ptr_pos) = ui.input(|i| i.pointer.interact_pos()) {
                        let col = ((ptr_pos.x - BOARD_ORIGIN.x) / SQUARE_SIZE).floor() as i32;
                        let row = 7 - ((ptr_pos.y - BOARD_ORIGIN.y) / SQUARE_SIZE).floor() as i32;

                        if (0..8).contains(&col) && (0..8).contains(&row) {
                            let index = row * 8 + col;
                            let target_mask = 1u64 << index;
                            let own_pieces = if self.game_state.white_to_move {
                                self.game_state.white_pieces
                            } else {
                                self.game_state.black_pieces
                            };

                            if let Some(sel) = self.selected_cell {
                                if sel == index {
                                    self.selected_cell = None; // Deselect
                                } else if legal_moves[sel as usize] & target_mask != 0 {
                                    let is_take = (self.game_state.occupied & target_mask) != 0;
                                    self.game_state.make_move(1u64 << sel, target_mask);

                                    if is_take {
                                        self.play_sound(CAPTURE_SOUND);
                                    } else {
                                        self.play_sound(MOVE_SOUND);
                                    }

                                    self.selected_cell = None;
                                } else if (own_pieces & target_mask) != 0 {
                                    self.selected_cell = Some(index); // Switch selection
                                } else {
                                    self.selected_cell = None; // Clicked empty/enemy invalidly
                                }
                            } else if (own_pieces & target_mask) != 0 {
                                self.selected_cell = Some(index); // Initial select
                            }
                        }
                    }
                }
            });
            ui.request_repaint();
        });
    }
}
