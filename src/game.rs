use crate::game;

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
    pub king_pin_lines: [u64; 8],

    pub legal_moves: [u64; 64],
    pub legal_mask: u64,

    pub opponent_legal_moves: [u64; 64],
    pub opponent_legal_mask: u64,

    pub white_castle_kingside: bool,
    pub white_castle_queenside: bool,
    pub black_castle_kingside: bool,
    pub black_castle_queenside: bool,

    pub en_passant_target: u64,

    pub is_check: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut game_state = GameState {
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

            occupied: 0u64,
            white_pieces: 064,
            black_pieces: 064,
            king_pin_lines: [0u64; 8],

            legal_moves: [0u64; 64],
            legal_mask: 0u64,

            opponent_legal_moves: [0u64; 64],
            opponent_legal_mask: 0u64,

            white_castle_kingside: true,
            white_castle_queenside: true,
            black_castle_kingside: true,
            black_castle_queenside: true,

            en_passant_target: 0,

            is_check: false,
        };

        game_state.update_derived();
        game_state.update_legal_moves();

        game_state
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
        self.king_pin_lines = if self.white_to_move {
            self.get_king_pin_lines(self.white_king)
        } else {
            self.get_king_pin_lines(self.black_king)
        };
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

        let remove = |bb: &mut u64| {
            *bb &= !from;
        };

        let place = |bb: &mut u64| {
            *bb |= to;
        };

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

        new_state.en_passant_target = 0;

        if self.white_pawns & from != 0 {
            remove(&mut new_state.white_pawns);
            place(&mut new_state.white_pawns);

            // en passant logic
            if to == from << 16 {
                new_state.en_passant_target = from << 8;
            }

            if self.en_passant_target != 0 && to == self.en_passant_target {
                new_state.black_pawns &= !(self.en_passant_target >> 8);
            }
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

            // castling logic
            new_state.white_castle_kingside = false;
            new_state.white_castle_queenside = false;

            if to == from << 2 {
                new_state.white_rooks &= !0x0000000000000080u64; // remove h1
                new_state.white_rooks |= 0x0000000000000020u64; // place f1
            }
            if to == from >> 2 {
                new_state.white_rooks &= !0x0000000000000001u64; // remove a1
                new_state.white_rooks |= 0x0000000000000008u64; // place d1
            }
        } else if self.black_pawns & from != 0 {
            remove(&mut new_state.black_pawns);
            place(&mut new_state.black_pawns);

            // en passant logic
            if to == from >> 16 {
                new_state.en_passant_target = from >> 8;
            }
            if self.en_passant_target != 0 && to == self.en_passant_target {
                new_state.white_pawns &= !(self.en_passant_target << 8);
            }
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

            // castling logic
            new_state.black_castle_kingside = false;
            new_state.black_castle_queenside = false;

            if to == from << 2 {
                new_state.black_rooks &= !0x8000000000000000u64; // remove h8
                new_state.black_rooks |= 0x2000000000000000u64; // place f8
            }
            if to == from >> 2 {
                new_state.black_rooks &= !0x0100000000000000u64; // remove a8
                new_state.black_rooks |= 0x0800000000000000u64; // place d8
            }
        } else {
            return self.clone();
        }

        // Revoke castling rights if a rook moves from or is captured on its starting square
        // White rooks
        if from == 0x0000000000000080 || to == 0x0000000000000080 {
            new_state.white_castle_kingside = false; // h1 rook gone
        }
        if from == 0x0000000000000001 || to == 0x0000000000000001 {
            new_state.white_castle_queenside = false; // a1 rook gone
        }
        // Black rooks
        if from == 0x8000000000000000 || to == 0x8000000000000000 {
            new_state.black_castle_kingside = false; // h8 rook gone
        }
        if from == 0x0100000000000000 || to == 0x0100000000000000 {
            new_state.black_castle_queenside = false; // a8 rook gone
        }

        new_state.white_to_move = !self.white_to_move;

        new_state.update_derived();
        new_state.update_legal_moves();

        new_state
    }

    pub fn update_legal_moves(&mut self) {
        //moves is an array with length 64 (each cell) where each element is a bitboard of legal
        //moves for the piece on that cell (or 0 if no piece)
        let own_pieces = if self.white_to_move {
            self.white_pieces
        } else {
            self.black_pieces
        };

        let opponent_pieces = if self.white_to_move {
            self.black_pieces
        } else {
            self.white_pieces
        };

        self.legal_mask = 0u64;
        self.opponent_legal_mask = 0u64;

        self.legal_moves = [0u64; 64];
        for i in 0..64 {
            self.opponent_legal_moves[i] = self.get_piece_moves(1u64 << i, opponent_pieces);
            self.opponent_legal_mask |= self.opponent_legal_moves[i];
        }

        for i in 0..64 {
            self.legal_moves[i] = self.get_piece_moves(1u64 << i, own_pieces);
            self.legal_mask |= self.legal_moves[i];
        }
    }

    pub fn get_piece_moves(&self, pos: u64, mask: u64) -> u64 {
        if pos & mask == 0 {
            return 0;
        }

        // make legal_mask with the entire board set to 1s
        let mut legal_mask = 0xFFFFFFFFFFFFFFFFu64;

        for i in 0..8 {
            if pos & self.king_pin_lines[i] != 0 {
                legal_mask = self.king_pin_lines[i];
            }
        }

        let moves = if (pos & self.white_pawns) != 0 {
            Self::get_pawn_moves(
                pos,
                self.occupied,
                self.white_pieces,
                true,
                self.en_passant_target,
            )
        } else if (pos & self.black_pawns) != 0 {
            Self::get_pawn_moves(
                pos,
                self.occupied,
                self.black_pieces,
                false,
                self.en_passant_target,
            )
        } else if (pos & self.white_knights) != 0 {
            Self::get_knight_moves(pos, self.white_pieces)
        } else if (pos & self.white_bishops) != 0 {
            Self::get_bishop_moves(pos, self.occupied, self.white_pieces)
        } else if (pos & self.white_rooks) != 0 {
            Self::get_rook_moves(pos, self.occupied, self.white_pieces)
        } else if (pos & self.white_queens) != 0 {
            Self::get_queen_moves(pos, self.occupied, self.white_pieces)
        } else if (pos & self.white_king) != 0 {
            self.get_king_moves(pos, self.white_pieces) | self.get_castling_moves()
        } else if (pos & self.black_knights) != 0 {
            Self::get_knight_moves(pos, self.black_pieces)
        } else if (pos & self.black_bishops) != 0 {
            Self::get_bishop_moves(pos, self.occupied, self.black_pieces)
        } else if (pos & self.black_rooks) != 0 {
            Self::get_rook_moves(pos, self.occupied, self.black_pieces)
        } else if (pos & self.black_queens) != 0 {
            Self::get_queen_moves(pos, self.occupied, self.black_pieces)
        } else if (pos & self.black_king) != 0 {
            self.get_king_moves(pos, self.black_pieces) | self.get_castling_moves()
        } else {
            0
        };

        moves & legal_mask
    }

    pub fn get_castling_moves(&self) -> u64 {
        let mut moves = 0u64;

        if self.white_to_move {
            let ks_clear = 0x0000000000000060u64;
            if self.white_castle_kingside && (self.occupied & ks_clear == 0) {
                moves |= 0x0000000000000040; // g1
            }
            let qs_clear = 0x000000000000000Eu64;
            if self.white_castle_queenside && (self.occupied & qs_clear == 0) {
                moves |= 0x0000000000000004; // c1
            }
        } else {
            let ks_clear = 0x6000000000000000u64;
            if self.black_castle_kingside && (self.occupied & ks_clear == 0) {
                moves |= 0x4000000000000000; // g8
            }
            let qs_clear = 0x0E00000000000000u64;
            if self.black_castle_queenside && (self.occupied & qs_clear == 0) {
                moves |= 0x0400000000000000; // c8
            }
        }

        moves
    }

    pub fn get_king_pin_lines(&self, pos: u64) -> [u64; 8] {
        let mut pin_lines = [0u64; 8];
        let directions = [
            (8, 0xFF00000000000000u64, true),   // north       (cardinal)
            (-8, 0x00000000000000FFu64, true),  // south       (cardinal)
            (1, 0xFEFEFEFEFEFEFEFEu64, true),   // east        (cardinal)
            (-1, 0x7F7F7F7F7F7F7F7Fu64, true),  // west        (cardinal)
            (9, 0xFEFEFEFEFEFEFEFEu64, false),  // north-east  (diagonal)
            (7, 0x7F7F7F7F7F7F7F7Fu64, false),  // north-west  (diagonal)
            (-7, 0xFEFEFEFEFEFEFEFEu64, false), // south-east  (diagonal)
            (-9, 0x7F7F7F7F7F7F7F7Fu64, false), // south-west  (diagonal)
        ];

        let cardinal_terminators =
            self.black_queens | self.white_queens | self.black_rooks | self.white_rooks;
        let diagonal_terminators =
            self.black_queens | self.white_queens | self.black_bishops | self.white_bishops;

        for (i, &(shift, mask, is_cardinal)) in directions.iter().enumerate() {
            let terminators = if is_cardinal {
                cardinal_terminators
            } else {
                diagonal_terminators
            };
            let mut ray = pos;
            let mut line = 0u64;
            let mut passed_own_piece = false;

            loop {
                ray = if shift > 0 {
                    ray << shift
                } else {
                    ray >> -shift
                };

                if ray == 0 || (ray & mask) == 0 {
                    break;
                }

                line |= ray;

                if ray & self.occupied != 0 {
                    if ray & self.opponent_legal_mask != 0 {
                        // Only commit the pin line if the terminating piece
                        // is a slider that can actually attack on this axis.
                        if passed_own_piece && (ray & terminators != 0) {
                            pin_lines[i] = line;
                        }
                        break;
                    } else {
                        if passed_own_piece {
                            break;
                        }
                        passed_own_piece = true;
                    }
                }
            }
        }
        pin_lines
    }
    pub fn get_knight_moves(pos: u64, own_pieces: u64) -> u64 {
        let not_a_file = 0xFEFEFEFEFEFEFEFEu64;
        let not_h_file = 0x7F7F7F7F7F7F7F7Fu64;
        let not_ab_file = 0xFCFCFCFCFCFCFCFCu64;
        let not_gh_file = 0x3F3F3F3F3F3F3F3Fu64;

        let mut moves = 0u64;
        moves |= (pos << 17) & not_a_file; // up 2, right 1
        moves |= (pos << 15) & not_h_file; // up 2, left 1
        moves |= (pos << 10) & not_ab_file; // up 1, right 2
        moves |= (pos << 6) & not_gh_file; // up 1, left 2
        moves |= (pos >> 17) & not_h_file; // down 2, left 1
        moves |= (pos >> 15) & not_a_file; // down 2, right 1
        moves |= (pos >> 10) & not_gh_file; // down 1, left 2
        moves |= (pos >> 6) & not_ab_file; // down 1, right 2
        moves & !own_pieces
    }

    pub fn get_king_moves(&self, pos: u64, own_pieces: u64) -> u64 {
        let not_a_file = 0xFEFEFEFEFEFEFEFEu64;
        let not_h_file = 0x7F7F7F7F7F7F7F7Fu64;

        let mut moves = 0u64;
        moves |= pos << 8; // north
        moves |= pos >> 8; // south
        moves |= (pos << 1) & not_a_file; // east
        moves |= (pos >> 1) & not_h_file; // west
        moves |= (pos << 9) & not_a_file; // north-east
        moves |= (pos << 7) & not_h_file; // north-west
        moves |= (pos >> 7) & not_a_file; // south-east
        moves |= (pos >> 9) & not_h_file; // south-west

        (moves & !own_pieces) & !self.opponent_legal_mask
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

    /// `ep_target`: the square a capturing pawn would land on (0 if no EP available).
    pub fn get_pawn_moves(
        pos: u64,
        occupied: u64,
        own_pieces: u64,
        is_white: bool,
        ep_target: u64,
    ) -> u64 {
        let mut moves = 0u64;
        let empty = !occupied;
        let enemy_pieces = occupied & !own_pieces;

        let not_a_file = 0xFEFEFEFEFEFEFEFEu64;
        let not_h_file = 0x7F7F7F7F7F7F7F7Fu64;
        let rank_4 = 0x00000000FF000000u64; // white double-push landing rank
        let rank_5 = 0x000000FF00000000u64; // black double-push landing rank

        if is_white {
            // Single push
            let single_push = (pos << 8) & empty;
            moves |= single_push;

            // Double push from rank 2
            moves |= (single_push << 8) & empty & rank_4;

            // Normal diagonal captures
            moves |= (pos << 9) & not_a_file & enemy_pieces;
            moves |= (pos << 7) & not_h_file & enemy_pieces;

            // En passant captures
            if ep_target != 0 {
                moves |= (pos << 9) & not_a_file & ep_target;
                moves |= (pos << 7) & not_h_file & ep_target;
            }
        } else {
            // Single push
            let single_push = (pos >> 8) & empty;
            moves |= single_push;

            // Double push from rank 7
            moves |= (single_push >> 8) & empty & rank_5;

            // Normal diagonal captures
            moves |= (pos >> 7) & not_a_file & enemy_pieces;
            moves |= (pos >> 9) & not_h_file & enemy_pieces;

            // En passant captures
            if ep_target != 0 {
                moves |= (pos >> 7) & not_a_file & ep_target;
                moves |= (pos >> 9) & not_h_file & ep_target;
            }
        }

        moves & !own_pieces
    }
}
