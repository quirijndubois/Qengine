#[derive(Clone)]
pub struct GameState {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,
    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64,

    pub white_to_move: bool,

    pub occupied: u64,
    pub white_pieces: u64,
    pub black_pieces: u64,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            white_pawns: 0x000000000000FF00,
            white_knights: 0x0000000000000042,
            white_bishops: 0x0000000000000024,
            white_rooks: 0x0000000000000081,
            white_queens: 0x0000000000000008,
            white_king: 0x0000000000000010,
            black_pawns: 0x00FF000000000000,
            black_knights: 0x4200000000000000,
            black_bishops: 0x2400000000000000,
            black_rooks: 0x8100000000000000,
            black_queens: 0x0800000000000000,
            black_king: 0x1000000000000000,

            white_to_move: true,

            occupied: 0xFFFF00000000FFFF,
            white_pieces: 0x000000000000FFFF,
            black_pieces: 0xFFFF000000000000,
        }
    }
}

impl GameState {
    pub fn update_derived(&mut self) {
        self.white_pieces = self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_queens
            | self.white_king;
        self.black_pieces = self.black_pawns
            | self.black_knights
            | self.black_bishops
            | self.black_rooks
            | self.black_queens
            | self.black_king;

        self.occupied = self.white_pieces | self.black_pieces;
    }

    pub fn make_move(&self, from: u64, to: u64) -> GameState {
        let mut new_state = self.clone();

        let own_pieces = if self.white_to_move {
            self.white_pieces
        } else {
            self.black_pieces
        };

        if from & own_pieces == 0 {
            return new_state;
        }

        // helper closure to remove a piece from a bitboard
        let remove = |bb: &mut u64| {
            *bb &= !from;
        };

        let place = |bb: &mut u64| {
            *bb |= to;
        };

        // clear destination
        new_state.white_pawns &= !to;
        new_state.white_knights &= !to;
        new_state.white_bishops &= !to;
        new_state.white_rooks &= !to;
        new_state.white_queens &= !to;
        new_state.white_king &= !to;

        new_state.black_pawns &= !to;
        new_state.black_knights &= !to;
        new_state.black_bishops &= !to;
        new_state.black_rooks &= !to;
        new_state.black_queens &= !to;
        new_state.black_king &= !to;

        // find which piece is on `from` and move it
        if self.white_pawns & from != 0 {
            remove(&mut new_state.white_pawns);
            place(&mut new_state.white_pawns);
        } else if self.white_knights & from != 0 {
            remove(&mut new_state.white_knights);
            place(&mut new_state.white_knights);
        } else if self.white_bishops & from != 0 {
            remove(&mut new_state.white_bishops);
            place(&mut new_state.white_bishops);
        } else if self.white_rooks & from != 0 {
            remove(&mut new_state.white_rooks);
            place(&mut new_state.white_rooks);
        } else if self.white_queens & from != 0 {
            remove(&mut new_state.white_queens);
            place(&mut new_state.white_queens);
        } else if self.white_king & from != 0 {
            remove(&mut new_state.white_king);
            place(&mut new_state.white_king);
        } else if self.black_pawns & from != 0 {
            remove(&mut new_state.black_pawns);
            place(&mut new_state.black_pawns);
        } else if self.black_knights & from != 0 {
            remove(&mut new_state.black_knights);
            place(&mut new_state.black_knights);
        } else if self.black_bishops & from != 0 {
            remove(&mut new_state.black_bishops);
            place(&mut new_state.black_bishops);
        } else if self.black_rooks & from != 0 {
            remove(&mut new_state.black_rooks);
            place(&mut new_state.black_rooks);
        } else if self.black_queens & from != 0 {
            remove(&mut new_state.black_queens);
            place(&mut new_state.black_queens);
        } else if self.black_king & from != 0 {
            remove(&mut new_state.black_king);
            place(&mut new_state.black_king);
        } else {
            // no piece found → invalid move
            return self.clone();
        }

        new_state.white_to_move = !self.white_to_move;
        new_state.update_derived();

        new_state
    }
    pub fn get_moves(&self, pos: u64) -> u64 {
        let own_pieces = if self.white_to_move {
            self.white_pieces
        } else {
            self.black_pieces
        };

        if pos & own_pieces == 0 {
            return 0;
        }

        if (pos & self.black_pawns) != 0 {
            Self::get_pawn_moves(pos, self.occupied, self.black_pieces, false)
        } else if (pos & self.white_knights) != 0 {
            Self::get_knight_moves(pos, self.white_pieces)
        } else if (pos & self.white_bishops) != 0 {
            Self::get_bishop_moves(pos, self.occupied, self.white_pieces)
        } else if (pos & self.white_rooks) != 0 {
            Self::get_rook_moves(pos, self.occupied, self.white_pieces)
        } else if (pos & self.white_queens) != 0 {
            Self::get_queen_moves(pos, self.occupied, self.white_pieces)
        } else if (pos & self.white_king) != 0 {
            Self::get_king_moves(pos, self.white_pieces)
        } else if (pos & self.white_pawns) != 0 {
            Self::get_pawn_moves(pos, self.occupied, self.white_pieces, true)
        } else if (pos & self.black_knights) != 0 {
            Self::get_knight_moves(pos, self.black_pieces)
        } else if (pos & self.black_bishops) != 0 {
            Self::get_bishop_moves(pos, self.occupied, self.black_pieces)
        } else if (pos & self.black_rooks) != 0 {
            Self::get_rook_moves(pos, self.occupied, self.black_pieces)
        } else if (pos & self.black_queens) != 0 {
            Self::get_queen_moves(pos, self.occupied, self.black_pieces)
        } else if (pos & self.black_king) != 0 {
            Self::get_king_moves(pos, self.black_pieces)
        } else {
            0
        }
    }

    pub fn get_knight_moves(pos: u64, own_pieces: u64) -> u64 {
        let not_a_file = 0xFEFEFEFEFEFEFEFEu64; // ~A file (left edge)
        let not_h_file = 0x7F7F7F7F7F7F7F7Fu64; // ~H file (right edge)
        let not_ab_file = 0xFCFCFCFCFCFCFCFCu64; // ~A&B files (two left edge cols)
        let not_gh_file = 0x3F3F3F3F3F3F3F3Fu64; // ~G&H files (two right edge cols)

        let mut moves = 0u64;
        moves |= (pos << 17) & not_a_file; // up 2, left 1
        moves |= (pos << 15) & not_h_file; // up 2, right 1
        moves |= (pos << 10) & not_ab_file; // up 1, left 2
        moves |= (pos << 6) & not_gh_file; // up 1, right 2
        moves |= (pos >> 17) & not_h_file; // down 2, right 1
        moves |= (pos >> 15) & not_a_file; // down 2, left 1
        moves |= (pos >> 10) & not_gh_file; // down 1, right 2
        moves |= (pos >> 6) & not_ab_file; // down 1, left 2
        moves & !own_pieces
    }

    pub fn get_king_moves(pos: u64, own_pieces: u64) -> u64 {
        let not_a_file = 0xFEFEFEFEFEFEFEFE;
        let not_h_file = 0x7F7F7F7F7F7F7F7F;

        let mut moves = 0u64;
        moves |= pos << 8; // up
        moves |= pos >> 8; // down
        moves |= (pos << 1) & not_a_file; // left
        moves |= (pos >> 1) & not_h_file; // right
        moves |= (pos << 9) & not_a_file; // up-left
        moves |= (pos << 7) & not_h_file; // up-right
        moves |= (pos >> 9) & not_h_file; // down-right
        moves |= (pos >> 7) & not_a_file; // down-left

        moves & !own_pieces
    }

    pub fn get_rook_moves(pos: u64, occupied: u64, own_pieces: u64) -> u64 {
        let mut moves = 0u64;

        // North ray
        let mut ray = pos;
        loop {
            ray <<= 8;
            if ray == 0 {
                break;
            }
            moves |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        // South ray
        let mut ray = pos;
        loop {
            ray >>= 8;
            if ray == 0 {
                break;
            }
            moves |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        // East ray
        let not_a_file = 0xFEFEFEFEFEFEFEFEu64;
        let mut ray = pos;
        loop {
            ray = (ray << 1) & not_a_file;
            if ray == 0 {
                break;
            }
            moves |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        // West ray
        let not_h_file = 0x7F7F7F7F7F7F7F7Fu64;
        let mut ray = pos;
        loop {
            ray = (ray >> 1) & not_h_file;
            if ray == 0 {
                break;
            }
            moves |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        moves & !own_pieces
    }

    pub fn get_bishop_moves(pos: u64, occupied: u64, own_pieces: u64) -> u64 {
        let mut moves = 0u64;
        let not_a_file = 0xFEFEFEFEFEFEFEFEu64;
        let not_h_file = 0x7F7F7F7F7F7F7F7Fu64;

        // North-East ray
        let mut ray = pos;
        loop {
            ray = (ray << 9) & not_a_file;
            if ray == 0 {
                break;
            }
            moves |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        // North-West ray
        let mut ray = pos;
        loop {
            ray = (ray << 7) & not_h_file;
            if ray == 0 {
                break;
            }
            moves |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        // South-East ray
        let mut ray = pos;
        loop {
            ray = (ray >> 7) & not_a_file;
            if ray == 0 {
                break;
            }
            moves |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        // South-West ray
        let mut ray = pos;
        loop {
            ray = (ray >> 9) & not_h_file;
            if ray == 0 {
                break;
            }
            moves |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        moves & !own_pieces
    }

    pub fn get_queen_moves(pos: u64, occupied: u64, own_pieces: u64) -> u64 {
        Self::get_bishop_moves(pos, occupied, own_pieces)
            | Self::get_rook_moves(pos, occupied, own_pieces)
    }

    pub fn get_pawn_moves(pos: u64, occupied: u64, own_pieces: u64, is_white: bool) -> u64 {
        let mut moves = 0u64;
        let empty = !occupied;
        let enemy_pieces = occupied & !own_pieces;

        let not_a_file = 0xFEFEFEFEFEFEFEFEu64;
        let not_h_file = 0x7F7F7F7F7F7F7F7Fu64;
        let rank_4 = 0x00000000FF000000u64; // white double push landing rank
        let rank_5 = 0x000000FF00000000u64; // black double push landing rank

        if is_white {
            // Single push forward
            let single_push = (pos << 8) & empty;
            moves |= single_push;

            // Double push from rank 2
            moves |= (single_push << 8) & empty & rank_4;

            // Captures diagonally
            moves |= (pos << 9) & not_a_file & enemy_pieces;
            moves |= (pos << 7) & not_h_file & enemy_pieces;
        } else {
            // Single push forward
            let single_push = (pos >> 8) & empty;
            moves |= single_push;

            // Double push from rank 7
            moves |= (single_push >> 8) & empty & rank_5;

            // Captures diagonally
            moves |= (pos >> 7) & not_a_file & enemy_pieces;
            moves |= (pos >> 9) & not_h_file & enemy_pieces;
        }

        moves & !own_pieces
    }
}
