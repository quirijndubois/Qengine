use crate::game::GameState;
use crate::piece::Piece;
use eframe::egui;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::Cursor;

// Embed sound files at compile time
const MOVE_SOUND: &[u8] = include_bytes!("../sounds/Move.ogg");
const CAPTURE_SOUND: &[u8] = include_bytes!("../sounds/Capture.ogg");

pub const SQUARE_SIZE: f32 = 80.0;
pub const BOARD_ORIGIN: egui::Pos2 = egui::pos2(20.0, 20.0);

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
            game_state: GameState::new(),
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
                sink.detach(); // plays without blocking
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

    fn draw_square(painter: &egui::Painter, col: usize, row: usize, r: u8, g: u8, b: u8, a: u8) {
        let color = egui::Color32::from_rgba_unmultiplied(r, g, b, a);
        let rect = Self::square_rect(col, 7 - row);
        painter.rect_filled(rect, 0.0, color);
    }

    fn draw_piece(ui: &mut egui::Ui, col: usize, row: usize, piece: &Piece) {
        let rect = Self::square_rect(col as usize, 7 - row as usize).shrink(5.0);
        let (name, bytes) = piece.asset_data();
        ui.put(rect.shrink(5.0), egui::Image::from_bytes(name, bytes));
    }
}

impl eframe::App for ChessApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            let painter = ui.painter();

            //convert selected cell to mask
            let legal_moves = if let Some(sel_index) = self.selected_cell {
                let sel_col = sel_index as usize % 8;
                let sel_row = sel_index as usize / 8;
                let pos = 1u64 << (sel_row * 8 + sel_col);
                self.game_state.get_piece_moves(pos)
            } else {
                0
            };

            // Draw Squares
            for row in 0..8 {
                for col in 0..8 {
                    let color = if (row + col) % 2 == 0 {
                        (181, 136, 99)
                    } else {
                        (240, 217, 181)
                    };

                    Self::draw_square(painter, col, row, color.0, color.1, color.2, 255);

                    let move_mask = 1u64 << (row * 8 + col);
                    if legal_moves & move_mask != 0 {
                        Self::draw_square(painter, col, row, 0, 255, 0, 100);
                    }

                    if let Some(sel_index) = self.selected_cell {
                        let sel_col = sel_index as usize % 8;
                        let sel_row = sel_index as usize / 8;
                        if sel_col == col && sel_row == row {
                            Self::draw_square(painter, col, row, 255, 255, 0, 255);
                        }
                    }
                }
            }

            for pos in 0..64 {
                let col = pos % 8;
                let row = pos / 8;

                let piece_opt = if (self.game_state.white_pawns >> pos) & 1 == 1 {
                    Some(Piece::WhitePawn)
                } else if (self.game_state.white_knights >> pos) & 1 == 1 {
                    Some(Piece::WhiteKnight)
                } else if (self.game_state.white_bishops >> pos) & 1 == 1 {
                    Some(Piece::WhiteBishop)
                } else if (self.game_state.white_rooks >> pos) & 1 == 1 {
                    Some(Piece::WhiteRook)
                } else if (self.game_state.white_queens >> pos) & 1 == 1 {
                    Some(Piece::WhiteQueen)
                } else if (self.game_state.white_king >> pos) & 1 == 1 {
                    Some(Piece::WhiteKing)
                } else if (self.game_state.black_pawns >> pos) & 1 == 1 {
                    Some(Piece::BlackPawn)
                } else if (self.game_state.black_knights >> pos) & 1 == 1 {
                    Some(Piece::BlackKnight)
                } else if (self.game_state.black_bishops >> pos) & 1 == 1 {
                    Some(Piece::BlackBishop)
                } else if (self.game_state.black_rooks >> pos) & 1 == 1 {
                    Some(Piece::BlackRook)
                } else if (self.game_state.black_queens >> pos) & 1 == 1 {
                    Some(Piece::BlackQueen)
                } else if (self.game_state.black_king >> pos) & 1 == 1 {
                    Some(Piece::BlackKing)
                } else {
                    None
                };

                if let Some(piece) = piece_opt {
                    Self::draw_piece(ui, col as usize, row as usize, &piece);
                }
            }

            // handle clicks
            if ui.input(|i| i.pointer.any_pressed()) {
                if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                    let col = ((pos.x - BOARD_ORIGIN.x) / SQUARE_SIZE) as i32;
                    let row = 7 - ((pos.y - BOARD_ORIGIN.y) / SQUARE_SIZE) as i32;
                    let index = row * 8 + col;
                    if (0..8).contains(&col) && (0..8).contains(&row) {
                        if let Some(sel_index) = self.selected_cell {
                            let from_mask = 1u64 << sel_index;
                            let to_mask = 1u64 << index;

                            if self.game_state.get_moves(from_mask) & to_mask == 0 {
                                self.selected_cell = None;
                                return;
                            }

                            let is_take = self.game_state.occupied & to_mask != 0;

                            self.game_state = self.game_state.make_move(from_mask, to_mask);

                            if is_take {
                                self.play_sound(CAPTURE_SOUND);
                            } else {
                                self.play_sound(MOVE_SOUND);
                            }

                            self.selected_cell = None;
                            return;
                        }

                        if Some(index) == self.selected_cell {
                            self.selected_cell = None;
                        } else {
                            self.selected_cell = Some(index);
                        }
                    }
                }
                ui.request_repaint();
            }

            ui.request_repaint();
        });
    }
}
