use crate::piece::Piece;
use eframe::egui;

pub const SQUARE_SIZE: f32 = 80.0;
pub const BOARD_ORIGIN: egui::Pos2 = egui::pos2(20.0, 20.0);

pub struct ChessApp {
    pub board: [[Option<Piece>; 8]; 8],
    pub selected_cell: Option<(usize, usize)>,
}

impl ChessApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        let mut board = [[None; 8]; 8];

        board[0] = [
            Some(Piece::BlackRook),
            Some(Piece::BlackKnight),
            Some(Piece::BlackBishop),
            Some(Piece::BlackQueen),
            Some(Piece::BlackKing),
            Some(Piece::BlackBishop),
            Some(Piece::BlackKnight),
            Some(Piece::BlackRook),
        ];
        board[1] = [Some(Piece::BlackPawn); 8];
        board[6] = [Some(Piece::WhitePawn); 8];
        board[7] = [
            Some(Piece::WhiteRook),
            Some(Piece::WhiteKnight),
            Some(Piece::WhiteBishop),
            Some(Piece::WhiteQueen),
            Some(Piece::WhiteKing),
            Some(Piece::WhiteBishop),
            Some(Piece::WhiteKnight),
            Some(Piece::WhiteRook),
        ];

        Self {
            board,
            selected_cell: None,
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
}

impl eframe::App for ChessApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            let painter = ui.painter();

            let legal_moves = if let Some((col, row)) = self.selected_cell {
                if let Some(piece) = self.board[row][col] {
                    piece.get_legal_moves((col, row), &self.board)
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };

            // Draw Squares
            for row in 0..8 {
                for col in 0..8 {
                    let rect = Self::square_rect(col, row);
                    let color = if (row + col) % 2 == 0 {
                        egui::Color32::from_rgb(240, 217, 181)
                    } else {
                        egui::Color32::from_rgb(181, 136, 99)
                    };

                    painter.rect_filled(rect, 0.0, color);

                    if legal_moves.contains(&(col, row)) {
                        painter.rect_filled(
                            rect,
                            0.0,
                            egui::Color32::from_rgba_unmultiplied(0, 255, 0, 100),
                        );
                    }

                    if let Some((sel_col, sel_row)) = self.selected_cell {
                        if sel_col == col && sel_row == row {
                            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(255, 255, 0));
                        }
                    }
                }
            }

            // Draw Pieces
            for row in 0..8 {
                for col in 0..8 {
                    if let Some(piece) = self.board[row][col] {
                        let rect = Self::square_rect(col, row);
                        let (name, bytes) = piece.asset_data();
                        ui.put(rect.shrink(5.0), egui::Image::from_bytes(name, bytes));
                    }
                }
            }

            // Simple Click Handling
            if ui.input(|i| i.pointer.any_pressed()) {
                if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                    let col = ((pos.x - BOARD_ORIGIN.x) / SQUARE_SIZE) as i32;
                    let row = ((pos.y - BOARD_ORIGIN.y) / SQUARE_SIZE) as i32;
                    if (0..8).contains(&col) && (0..8).contains(&row) {
                        // check if the cell is in legal moves, if so move the piece and deselect
                        if let Some((sel_col, sel_row)) = self.selected_cell {
                            if legal_moves.contains(&(col as usize, row as usize)) {
                                self.board[row as usize][col as usize] =
                                    self.board[sel_row][sel_col].take();
                                self.selected_cell = None;
                                return;
                            }
                        }

                        if Some((col as usize, row as usize)) == self.selected_cell {
                            self.selected_cell = None;
                        } else {
                            self.selected_cell = Some((col as usize, row as usize));
                        }
                    }
                }
            }
        });
    }
}
