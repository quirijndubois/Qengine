use crate::game;

const NOT_A_FILE: u64 = 0xFEFEFEFEFEFEFEFE;
const NOT_H_FILE: u64 = 0x7F7F7F7F7F7F7F7F;
const NOT_AB_FILE: u64 = 0xFCFCFCFCFCFCFCFC;
const NOT_GH_FILE: u64 = 0x3F3F3F3F3F3F3F3F;
const RANK_4: u64 = 0x00000000FF000000;
const RANK_5: u64 = 0x000000FF00000000;

const KING_ATTACKS: [u64; 64] = compute_king_attacks();
const KNIGHT_ATTACKS: [u64; 64] = compute_knight_attacks();

const fn compute_king_attacks() -> [u64; 64] {
    let mut table = [0; 64];
    let mut i = 0;
    while i < 64 {
        let pos = 1u64 << i;
        let mut moves = 0u64;
        moves |= pos << 8;
        moves |= pos >> 8;
        moves |= (pos << 1) & NOT_A_FILE;
        moves |= (pos >> 1) & NOT_H_FILE;
        moves |= (pos << 9) & NOT_A_FILE;
        moves |= (pos << 7) & NOT_H_FILE;
        moves |= (pos >> 7) & NOT_A_FILE;
        moves |= (pos >> 9) & NOT_H_FILE;
        table[i] = moves;
        i += 1;
    }
    table
}

const fn compute_knight_attacks() -> [u64; 64] {
    let mut table = [0; 64];
    let mut i = 0;
    while i < 64 {
        let pos = 1u64 << i;
        let mut moves = 0u64;
        moves |= (pos << 17) & NOT_A_FILE;
        moves |= (pos << 15) & NOT_H_FILE;
        moves |= (pos << 10) & NOT_AB_FILE;
        moves |= (pos << 6) & NOT_GH_FILE;
        moves |= (pos >> 17) & NOT_H_FILE;
        moves |= (pos >> 15) & NOT_A_FILE;
        moves |= (pos >> 10) & NOT_GH_FILE;
        moves |= (pos >> 6) & NOT_AB_FILE;
        table[i] = moves;
        i += 1;
    }
    table
}

#[derive(Clone, Copy, Debug)]
pub struct MoveRecord {
    pub from: u64,
    pub to: u64,

    pub moved_piece: PieceType,
    pub captured_piece: Option<PieceType>,
    pub captured_square: u64,

    pub prev_en_passant: u64,

    pub prev_white_castle_kingside: bool,
    pub prev_white_castle_queenside: bool,
    pub prev_black_castle_kingside: bool,
    pub prev_black_castle_queenside: bool,

    pub was_white_to_move: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

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

    pub white_legal_moves: [u64; 64],
    pub white_legal_mask: u64,
    pub white_attack_mask: u64,

    pub black_legal_moves: [u64; 64],
    pub black_legal_mask: u64,
    pub black_attack_mask: u64,

    pub check_mask: u64,

    pub white_castle_kingside: bool,
    pub white_castle_queenside: bool,
    pub black_castle_kingside: bool,
    pub black_castle_queenside: bool,

    pub en_passant_target: u64,

    pub white_is_checked: bool,
    pub black_is_checked: bool,
    pub history: Vec<MoveRecord>,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

impl GameState {
    pub fn new(fen: &str) -> Self {
        let mut game_state = GameState::from_fen(fen);

        // Ensure derived bitboards and move masks are calculated immediately
        game_state.update_derived();
        game_state.update_legal_moves();

        game_state
    }

    /// Parses a FEN string into a GameState
    pub fn from_fen(fen: &str) -> Self {
        let mut state = GameState::empty();
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 1 {
            return state;
        }

        // 1. Piece Placement
        let rows: Vec<&str> = parts[0].split('/').collect();
        for (rank, row) in rows.iter().enumerate() {
            let mut file = 0;
            for c in row.chars() {
                if let Some(digit) = c.to_digit(10) {
                    file += digit as usize;
                } else {
                    let bit = 1u64 << ((7 - rank) * 8 + file);
                    match c {
                        'P' => state.white_pawns |= bit,
                        'N' => state.white_knights |= bit,
                        'B' => state.white_bishops |= bit,
                        'R' => state.white_rooks |= bit,
                        'Q' => state.white_queens |= bit,
                        'K' => state.white_king |= bit,
                        'p' => state.black_pawns |= bit,
                        'n' => state.black_knights |= bit,
                        'b' => state.black_bishops |= bit,
                        'r' => state.black_rooks |= bit,
                        'q' => state.black_queens |= bit,
                        'k' => state.black_king |= bit,
                        _ => {}
                    }
                    file += 1;
                }
            }
        }

        // 2. Active Color
        if parts.len() > 1 {
            state.white_to_move = parts[1] == "w";
        }

        // 3. Castling Rights
        if parts.len() > 2 {
            let rights = parts[2];
            state.white_castle_kingside = rights.contains('K');
            state.white_castle_queenside = rights.contains('Q');
            state.black_castle_kingside = rights.contains('k');
            state.black_castle_queenside = rights.contains('q');
        }

        // 4. En Passant Target
        if parts.len() > 3 && parts[3] != "-" {
            let coords: Vec<char> = parts[3].chars().collect();
            if coords.len() == 2 {
                let file = (coords[0] as u64) - ('a' as u64);
                let rank = (coords[1] as u64) - ('1' as u64);
                state.en_passant_target = 1u64 << (rank * 8 + file);
            }
        }

        state
    }

    /// Helper to create a zeroed-out state
    fn empty() -> Self {
        GameState {
            white_pawns: 0,
            white_knights: 0,
            white_bishops: 0,
            white_rooks: 0,
            white_queens: 0,
            white_king: 0,
            black_pawns: 0,
            black_knights: 0,
            black_bishops: 0,
            black_rooks: 0,
            black_queens: 0,
            black_king: 0,
            white_to_move: true,
            occupied: 0,
            white_pieces: 0,
            black_pieces: 0,
            king_pin_lines: [0; 8],
            white_legal_moves: [0; 64],
            white_legal_mask: 0,
            white_attack_mask: 0,
            black_legal_moves: [0; 64],
            black_legal_mask: 0,
            black_attack_mask: 0,
            check_mask: !0u64,
            white_castle_kingside: false,
            white_castle_queenside: false,
            black_castle_kingside: false,
            black_castle_queenside: false,
            en_passant_target: 0,
            white_is_checked: false,
            black_is_checked: false,
            history: Vec::new(),
        }
    }

    fn detect_piece(&self, sq: u64) -> PieceType {
        if sq & self.white_pawns | sq & self.black_pawns != 0 {
            PieceType::Pawn
        } else if sq & self.white_knights | sq & self.black_knights != 0 {
            PieceType::Knight
        } else if sq & self.white_bishops | sq & self.black_bishops != 0 {
            PieceType::Bishop
        } else if sq & self.white_rooks | sq & self.black_rooks != 0 {
            PieceType::Rook
        } else if sq & self.white_queens | sq & self.black_queens != 0 {
            PieceType::Queen
        } else {
            PieceType::King
        }
    }

    pub fn evaluate(&self) -> i32 {
        let material_score = self.white_pawns.count_ones() as i32
            - self.black_pawns.count_ones() as i32
            + 3 * (self.white_knights.count_ones() as i32 - self.black_knights.count_ones() as i32)
            + 3 * (self.white_bishops.count_ones() as i32 - self.black_bishops.count_ones() as i32)
            + 5 * (self.white_rooks.count_ones() as i32 - self.black_rooks.count_ones() as i32)
            + 9 * (self.white_queens.count_ones() as i32 - self.black_queens.count_ones() as i32);

        let mobility_score =
            self.white_attack_mask.count_ones() as i32 - self.black_attack_mask.count_ones() as i32;

        material_score * 100 + mobility_score
    }

    pub fn get_legal_move_list(&self) -> Vec<(u64, u64)> {
        // this function gets a list of all legal moves , where each element of the list is a tuple
        // of (from_bitboard, to_bitboard)
        let mut moves = Vec::new();

        if self.white_to_move {
            for i in 0..64 {
                let from_mask = 1u64 << i;
                if self.white_pieces & from_mask != 0 {
                    let to_mask = self.white_legal_moves[i];
                    let mut temp = to_mask;
                    while temp != 0 {
                        let to_bit = temp & temp.wrapping_neg();
                        moves.push((from_mask, to_bit));
                        temp &= temp - 1;
                    }
                }
            }
        } else {
            for i in 0..64 {
                let from_mask = 1u64 << i;
                if self.black_pieces & from_mask != 0 {
                    let to_mask = self.black_legal_moves[i];
                    let mut temp = to_mask;
                    while temp != 0 {
                        let to_bit = temp & temp.wrapping_neg();
                        moves.push((from_mask, to_bit));
                        temp &= temp - 1;
                    }
                }
            }
        }

        moves
    }

    pub fn get_capture_move_list(&self) -> Vec<(u64, u64)> {
        let mut moves = Vec::new();

        if self.white_to_move {
            for i in 0..64 {
                let from_mask = 1u64 << i;
                if self.white_pieces & from_mask != 0 {
                    let to_mask = self.white_legal_moves[i] & self.black_pieces;
                    let mut temp = to_mask;
                    while temp != 0 {
                        let to_bit = temp & temp.wrapping_neg();
                        moves.push((from_mask, to_bit));
                        temp &= temp - 1;
                    }
                }
            }
        } else {
            for i in 0..64 {
                let from_mask = 1u64 << i;
                if self.black_pieces & from_mask != 0 {
                    let to_mask = self.black_legal_moves[i] & self.white_pieces;
                    let mut temp = to_mask;
                    while temp != 0 {
                        let to_bit = temp & temp.wrapping_neg();
                        moves.push((from_mask, to_bit));
                        temp &= temp - 1;
                    }
                }
            }
        }

        moves
    }

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

    pub fn make_move(&mut self, from: u64, to: u64) {
        // ----------------------------
        // Save state for undo/history
        // ----------------------------
        let prev_en_passant = self.en_passant_target;

        let prev_white_castle_kingside = self.white_castle_kingside;
        let prev_white_castle_queenside = self.white_castle_queenside;
        let prev_black_castle_kingside = self.black_castle_kingside;
        let prev_black_castle_queenside = self.black_castle_queenside;

        let was_white_to_move = self.white_to_move;

        // ----------------------------
        // Validate move belongs to side
        // ----------------------------
        let own_pieces = if self.white_to_move {
            self.white_pieces
        } else {
            self.black_pieces
        };

        if from & own_pieces == 0 {
            return;
        }

        // ----------------------------
        // Detect moved piece type
        // ----------------------------
        let moved_piece = {
            if from & self.white_pawns != 0 || from & self.black_pawns != 0 {
                PieceType::Pawn
            } else if from & self.white_knights != 0 || from & self.black_knights != 0 {
                PieceType::Knight
            } else if from & self.white_bishops != 0 || from & self.black_bishops != 0 {
                PieceType::Bishop
            } else if from & self.white_rooks != 0 || from & self.black_rooks != 0 {
                PieceType::Rook
            } else if from & self.white_queens != 0 || from & self.black_queens != 0 {
                PieceType::Queen
            } else {
                PieceType::King
            }
        };

        // ----------------------------
        // Detect capture
        // ----------------------------
        let mut captured_piece = None;
        let mut captured_square = to;

        if self.occupied & to != 0 {
            captured_piece = Some({
                if to & (self.white_pawns | self.black_pawns) != 0 {
                    PieceType::Pawn
                } else if to & (self.white_knights | self.black_knights) != 0 {
                    PieceType::Knight
                } else if to & (self.white_bishops | self.black_bishops) != 0 {
                    PieceType::Bishop
                } else if to & (self.white_rooks | self.black_rooks) != 0 {
                    PieceType::Rook
                } else if to & (self.white_queens | self.black_queens) != 0 {
                    PieceType::Queen
                } else {
                    PieceType::King
                }
            });
        }

        // ----------------------------
        // Clear destination square
        // ----------------------------
        let clear_to = !to;

        self.white_pawns &= clear_to;
        self.white_knights &= clear_to;
        self.white_bishops &= clear_to;
        self.white_rooks &= clear_to;
        self.white_queens &= clear_to;
        self.white_king &= clear_to;

        self.black_pawns &= clear_to;
        self.black_knights &= clear_to;
        self.black_bishops &= clear_to;
        self.black_rooks &= clear_to;
        self.black_queens &= clear_to;
        self.black_king &= clear_to;

        // reset en passant by default
        self.en_passant_target = 0;

        let move_mask = from | to;

        // ----------------------------
        // MOVE PIECES
        // ----------------------------
        if self.white_to_move {
            if self.white_pawns & from != 0 {
                self.white_pawns ^= move_mask;

                // double pawn push
                if to == from << 16 {
                    self.en_passant_target = from << 8;
                }

                // en passant capture
                if prev_en_passant != 0 && to == prev_en_passant {
                    self.black_pawns &= !(prev_en_passant >> 8);
                    captured_square = prev_en_passant >> 8;
                }
            } else if self.white_knights & from != 0 {
                self.white_knights ^= move_mask;
            } else if self.white_bishops & from != 0 {
                self.white_bishops ^= move_mask;
            } else if self.white_rooks & from != 0 {
                self.white_rooks ^= move_mask;
            } else if self.white_queens & from != 0 {
                self.white_queens ^= move_mask;
            } else if self.white_king & from != 0 {
                self.white_king ^= move_mask;

                self.white_castle_kingside = false;
                self.white_castle_queenside = false;

                // castling moves
                if to == from << 2 {
                    self.white_rooks ^= 0x00000000000000A0; // h1 -> f1
                } else if to == from >> 2 {
                    self.white_rooks ^= 0x0000000000000009; // a1 -> d1
                }
            }
        } else {
            if self.black_pawns & from != 0 {
                self.black_pawns ^= move_mask;

                if to == from >> 16 {
                    self.en_passant_target = from >> 8;
                }

                if prev_en_passant != 0 && to == prev_en_passant {
                    self.white_pawns &= !(prev_en_passant << 8);
                    captured_square = prev_en_passant << 8;
                }
            } else if self.black_knights & from != 0 {
                self.black_knights ^= move_mask;
            } else if self.black_bishops & from != 0 {
                self.black_bishops ^= move_mask;
            } else if self.black_rooks & from != 0 {
                self.black_rooks ^= move_mask;
            } else if self.black_queens & from != 0 {
                self.black_queens ^= move_mask;
            } else if self.black_king & from != 0 {
                self.black_king ^= move_mask;

                self.black_castle_kingside = false;
                self.black_castle_queenside = false;

                // castling moves
                if to == from << 2 {
                    self.black_rooks ^= 0xA000000000000000; // h8 -> f8
                } else if to == from >> 2 {
                    self.black_rooks ^= 0x0900000000000000; // a8 -> d8
                }
            }
        }

        // ----------------------------
        // CASTLING RIGHTS (rook moves/captures)
        // ----------------------------
        if from == 0x0000000000000080 || to == 0x0000000000000080 {
            self.white_castle_kingside = false;
        }
        if from == 0x0000000000000001 || to == 0x0000000000000001 {
            self.white_castle_queenside = false;
        }
        if from == 0x8000000000000000 || to == 0x8000000000000000 {
            self.black_castle_kingside = false;
        }
        if from == 0x0100000000000000 || to == 0x0100000000000000 {
            self.black_castle_queenside = false;
        }

        // ----------------------------
        // SWITCH TURN
        // ----------------------------
        self.white_to_move = !self.white_to_move;

        // ----------------------------
        // UPDATE DERIVED STATE
        // ----------------------------
        self.update_derived();
        self.update_legal_moves();

        // ----------------------------
        // OPTIONAL: store history for undo
        // ----------------------------
        self.history.push(MoveRecord {
            from,
            to,
            moved_piece,
            captured_piece,
            captured_square,
            prev_en_passant,
            prev_white_castle_kingside,
            prev_white_castle_queenside,
            prev_black_castle_kingside,
            prev_black_castle_queenside,
            was_white_to_move,
        });
    }

    pub fn undo_move(&mut self) {
        let Some(last) = self.history.pop() else {
            return;
        };

        self.white_to_move = last.was_white_to_move;

        self.en_passant_target = last.prev_en_passant;

        self.white_castle_kingside = last.prev_white_castle_kingside;
        self.white_castle_queenside = last.prev_white_castle_queenside;
        self.black_castle_kingside = last.prev_black_castle_kingside;
        self.black_castle_queenside = last.prev_black_castle_queenside;

        // move piece back
        let from = last.from;
        let to = last.to;

        let move_mask = from | to;

        match last.moved_piece {
            PieceType::Pawn => {
                if self.white_to_move {
                    self.white_pawns ^= move_mask;
                } else {
                    self.black_pawns ^= move_mask;
                }
            }
            PieceType::Knight => {
                if self.white_to_move {
                    self.white_knights ^= move_mask;
                } else {
                    self.black_knights ^= move_mask;
                }
            }
            PieceType::Bishop => {
                if self.white_to_move {
                    self.white_bishops ^= move_mask;
                } else {
                    self.black_bishops ^= move_mask;
                }
            }
            PieceType::Rook => {
                if self.white_to_move {
                    self.white_rooks ^= move_mask;
                } else {
                    self.black_rooks ^= move_mask;
                }
            }
            PieceType::Queen => {
                if self.white_to_move {
                    self.white_queens ^= move_mask;
                } else {
                    self.black_queens ^= move_mask;
                }
            }
            PieceType::King => {
                if self.white_to_move {
                    self.white_king ^= move_mask;
                } else {
                    self.black_king ^= move_mask;
                }
            }
        }

        // restore captured piece
        if let Some(piece) = last.captured_piece {
            let sq = last.captured_square;
            match piece {
                PieceType::Pawn => {
                    if self.white_to_move {
                        self.black_pawns |= sq;
                    } else {
                        self.white_pawns |= sq;
                    }
                }
                PieceType::Knight => {
                    if self.white_to_move {
                        self.black_knights |= sq;
                    } else {
                        self.white_knights |= sq;
                    }
                }
                PieceType::Bishop => {
                    if self.white_to_move {
                        self.black_bishops |= sq;
                    } else {
                        self.white_bishops |= sq;
                    }
                }
                PieceType::Rook => {
                    if self.white_to_move {
                        self.black_rooks |= sq;
                    } else {
                        self.white_rooks |= sq;
                    }
                }
                PieceType::Queen => {
                    if self.white_to_move {
                        self.black_queens |= sq;
                    } else {
                        self.white_queens |= sq;
                    }
                }
                PieceType::King => {
                    if self.white_to_move {
                        self.black_king |= sq;
                    } else {
                        self.white_king |= sq;
                    }
                }
            }
        }

        self.update_derived();
        self.update_legal_moves();
    }

    pub fn update_legal_moves(&mut self) {
        if self.white_to_move {
            self.update_black_legal_moves(); // Generates black attack masks
            self.compute_check_mask();
            self.update_white_legal_moves();
        } else {
            self.update_white_legal_moves(); // Generates white attack masks
            self.compute_check_mask();
            self.update_black_legal_moves();
        }
    }

    pub fn compute_check_mask(&mut self) {
        let (
            our_king,
            opponent_attack_mask,
            opp_pawns,
            opp_knights,
            opp_bishops,
            opp_rooks,
            opp_queens,
        ) = if self.white_to_move {
            (
                self.white_king,
                self.black_attack_mask,
                self.black_pawns,
                self.black_knights,
                self.black_bishops,
                self.black_rooks,
                self.black_queens,
            )
        } else {
            (
                self.black_king,
                self.white_attack_mask,
                self.white_pawns,
                self.white_knights,
                self.white_bishops,
                self.white_rooks,
                self.white_queens,
            )
        };

        if opponent_attack_mask & our_king == 0 {
            self.check_mask = !0u64; // No check
            return;
        }

        let mut mask = 0u64;
        let mut checker_count = 0u32;

        let king_idx = our_king.trailing_zeros() as usize;
        let knight_checkers = KNIGHT_ATTACKS[king_idx] & opp_knights;
        if knight_checkers != 0 {
            mask |= knight_checkers;
            checker_count += knight_checkers.count_ones();
        }

        let (pawn_atk_l, pawn_atk_r) = if self.white_to_move {
            ((our_king << 9) & NOT_A_FILE, (our_king << 7) & NOT_H_FILE)
        } else {
            ((our_king >> 7) & NOT_A_FILE, (our_king >> 9) & NOT_H_FILE)
        };

        let pawn_checkers = (pawn_atk_l | pawn_atk_r) & opp_pawns;
        if pawn_checkers != 0 {
            mask |= pawn_checkers;
            checker_count += pawn_checkers.count_ones();
        }

        if checker_count >= 2 {
            self.check_mask = 0;
            return;
        }

        let king_rook_rays = Self::get_rook_moves(our_king, self.occupied, 0);
        let mut temp = king_rook_rays & (opp_rooks | opp_queens);
        while temp != 0 {
            let checker = temp & temp.wrapping_neg();
            let checker_rays = Self::get_rook_moves(checker, self.occupied, 0);
            mask |= (king_rook_rays & checker_rays) | checker;
            checker_count += 1;
            if checker_count >= 2 {
                self.check_mask = 0;
                return;
            }
            temp &= temp - 1;
        }

        let king_bishop_rays = Self::get_bishop_moves(our_king, self.occupied, 0);
        let mut temp = king_bishop_rays & (opp_bishops | opp_queens);
        while temp != 0 {
            let checker = temp & temp.wrapping_neg();
            let checker_rays = Self::get_bishop_moves(checker, self.occupied, 0);
            mask |= (king_bishop_rays & checker_rays) | checker;
            checker_count += 1;
            if checker_count >= 2 {
                self.check_mask = 0;
                return;
            }
            temp &= temp - 1;
        }

        self.check_mask = mask;
    }

    pub fn update_white_legal_moves(&mut self) {
        self.white_legal_mask = 0;
        self.white_attack_mask = 0;
        self.white_legal_moves = [0; 64];

        let mut pieces = self.white_pieces;
        while pieces != 0 {
            let pos = pieces & pieces.wrapping_neg(); // Isolate LSB
            let i = pos.trailing_zeros() as usize;

            self.white_attack_mask |= self.get_raw_attacks(pos, true, self.occupied);
            self.white_legal_moves[i] = self.get_piece_moves(pos, self.white_pieces);
            self.white_legal_mask |= self.white_legal_moves[i];

            pieces &= pieces - 1; // Clear LSB
        }
    }

    pub fn update_black_legal_moves(&mut self) {
        self.black_legal_mask = 0;
        self.black_attack_mask = 0;
        self.black_legal_moves = [0; 64];

        let mut pieces = self.black_pieces;
        while pieces != 0 {
            let pos = pieces & pieces.wrapping_neg();
            let i = pos.trailing_zeros() as usize;

            self.black_attack_mask |= self.get_raw_attacks(pos, false, self.occupied);
            self.black_legal_moves[i] = self.get_piece_moves(pos, self.black_pieces);
            self.black_legal_mask |= self.black_legal_moves[i];

            pieces &= pieces - 1;
        }
    }

    pub fn get_raw_attacks(&self, pos: u64, is_white: bool, occupied: u64) -> u64 {
        let idx = pos.trailing_zeros() as usize;

        if is_white {
            if pos & self.white_pawns != 0 {
                return ((pos << 9) & NOT_A_FILE) | ((pos << 7) & NOT_H_FILE);
            } else if pos & self.white_knights != 0 {
                return KNIGHT_ATTACKS[idx];
            } else if pos & self.white_bishops != 0 {
                return Self::get_bishop_moves(pos, occupied, 0);
            } else if pos & self.white_rooks != 0 {
                return Self::get_rook_moves(pos, occupied, 0);
            } else if pos & self.white_queens != 0 {
                return Self::get_queen_moves(pos, occupied, 0);
            } else if pos & self.white_king != 0 {
                return KING_ATTACKS[idx];
            }
        } else {
            if pos & self.black_pawns != 0 {
                return ((pos >> 7) & NOT_A_FILE) | ((pos >> 9) & NOT_H_FILE);
            } else if pos & self.black_knights != 0 {
                return KNIGHT_ATTACKS[idx];
            } else if pos & self.black_bishops != 0 {
                return Self::get_bishop_moves(pos, occupied, 0);
            } else if pos & self.black_rooks != 0 {
                return Self::get_rook_moves(pos, occupied, 0);
            } else if pos & self.black_queens != 0 {
                return Self::get_queen_moves(pos, occupied, 0);
            } else if pos & self.black_king != 0 {
                return KING_ATTACKS[idx];
            }
        }
        0
    }
    pub fn get_piece_moves(&self, pos: u64, own_pieces: u64) -> u64 {
        let mut pin_mask = !0u64;
        for &line in &self.king_pin_lines {
            if pos & line != 0 {
                pin_mask = line;
                break;
            }
        }

        let is_king = (pos & (self.white_king | self.black_king)) != 0;
        let idx = pos.trailing_zeros() as usize;

        let moves = if (pos & self.white_pawns) != 0 {
            Self::get_pawn_moves(pos, self.occupied, own_pieces, true, self.en_passant_target)
        } else if (pos & self.black_pawns) != 0 {
            Self::get_pawn_moves(
                pos,
                self.occupied,
                own_pieces,
                false,
                self.en_passant_target,
            )
        } else if (pos & (self.white_knights | self.black_knights)) != 0 {
            KNIGHT_ATTACKS[idx] & !own_pieces
        } else if (pos & (self.white_bishops | self.black_bishops)) != 0 {
            Self::get_bishop_moves(pos, self.occupied, own_pieces)
        } else if (pos & (self.white_rooks | self.black_rooks)) != 0 {
            Self::get_rook_moves(pos, self.occupied, own_pieces)
        } else if (pos & (self.white_queens | self.black_queens)) != 0 {
            Self::get_queen_moves(pos, self.occupied, own_pieces)
        } else if is_king {
            // Remove king from occupied so sliders can't hide behind it
            let occupied_without_king = self.occupied & !pos;

            let mut opp_attack_ex_king = 0u64;
            let opp_pieces = if self.white_to_move {
                self.black_pieces
            } else {
                self.white_pieces
            };

            let mut temp = opp_pieces;
            while temp != 0 {
                let p = temp & temp.wrapping_neg();
                opp_attack_ex_king |=
                    self.get_raw_attacks(p, !self.white_to_move, occupied_without_king);
                temp &= temp - 1;
            }

            ((KING_ATTACKS[idx] & !own_pieces) & !opp_attack_ex_king) | self.get_castling_moves()
        } else {
            0
        };

        if is_king {
            moves & pin_mask
        } else {
            moves & pin_mask & self.check_mask
        }
    }

    pub fn get_castling_moves(&self) -> u64 {
        let mut moves = 0u64;
        let opp_attacks = if self.white_to_move {
            self.black_attack_mask
        } else {
            self.white_attack_mask
        };

        if self.white_to_move {
            let ks_clear = 0x0000000000000060u64; // f1, g1
            let ks_safe = 0x0000000000000070u64; // e1, f1, g1

            if self.white_castle_kingside
                && (self.occupied & ks_clear == 0)
                && (opp_attacks & ks_safe == 0)
            {
                moves |= 0x0000000000000040; // g1
            }

            let qs_clear = 0x000000000000000Eu64; // b1, c1, d1
            let qs_safe = 0x000000000000001Cu64; // e1, d1, c1

            if self.white_castle_queenside
                && (self.occupied & qs_clear == 0)
                && (opp_attacks & qs_safe == 0)
            {
                moves |= 0x0000000000000004; // c1
            }
        } else {
            let ks_clear = 0x6000000000000000u64; // f8, g8
            let ks_safe = 0x7000000000000000u64; // e8, f8, g8

            if self.black_castle_kingside
                && (self.occupied & ks_clear == 0)
                && (opp_attacks & ks_safe == 0)
            {
                moves |= 0x4000000000000000; // g8
            }

            let qs_clear = 0x0E00000000000000u64; // b8, c8, d8
            let qs_safe = 0x1C00000000000000u64; // e8, d8, c8

            if self.black_castle_queenside
                && (self.occupied & qs_clear == 0)
                && (opp_attacks & qs_safe == 0)
            {
                moves |= 0x0400000000000000; // c8
            }
        }
        moves
    }

    pub fn get_king_pin_lines(&self, pos: u64) -> [u64; 8] {
        let mut pin_lines = [0u64; 8];
        let directions = [
            (8, !0xFF00000000000000u64, true),  // north
            (-8, !0x00000000000000FFu64, true), // south
            (1, NOT_A_FILE, true),              // east
            (-1, NOT_H_FILE, true),             // west
            (9, NOT_A_FILE, false),             // north-east
            (7, NOT_H_FILE, false),             // north-west
            (-7, NOT_A_FILE, false),            // south-east
            (-9, NOT_H_FILE, false),            // south-west
        ];

        let cardinal_terminators =
            self.black_queens | self.white_queens | self.black_rooks | self.white_rooks;
        let diagonal_terminators =
            self.black_queens | self.white_queens | self.black_bishops | self.white_bishops;

        let opponent_pieces = if self.white_to_move {
            self.black_pieces
        } else {
            self.white_pieces
        };

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
                    if ray & opponent_pieces != 0 {
                        if passed_own_piece && (ray & terminators) != 0 {
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

    pub fn get_rook_moves(pos: u64, occupied: u64, own_pieces: u64) -> u64 {
        let mut moves = 0u64;
        let mut ray = pos;
        while ray != 0 {
            ray <<= 8;
            moves |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        ray = pos;
        while ray != 0 {
            ray >>= 8;
            moves |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        ray = pos;
        while ray != 0 {
            ray = (ray << 1) & NOT_A_FILE;
            moves |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        ray = pos;
        while ray != 0 {
            ray = (ray >> 1) & NOT_H_FILE;
            moves |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        moves & !own_pieces
    }

    pub fn get_bishop_moves(pos: u64, occupied: u64, own_pieces: u64) -> u64 {
        let mut moves = 0u64;
        let mut ray = pos;

        while ray != 0 {
            ray = (ray << 9) & NOT_A_FILE;
            moves |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        ray = pos;
        while ray != 0 {
            ray = (ray << 7) & NOT_H_FILE;
            moves |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        ray = pos;
        while ray != 0 {
            ray = (ray >> 7) & NOT_A_FILE;
            moves |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        ray = pos;
        while ray != 0 {
            ray = (ray >> 9) & NOT_H_FILE;
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

        if is_white {
            let single_push = (pos << 8) & empty;
            moves |= single_push;
            moves |= (single_push << 8) & empty & RANK_4;
            moves |= (pos << 9) & NOT_A_FILE & enemy_pieces;
            moves |= (pos << 7) & NOT_H_FILE & enemy_pieces;
            if ep_target != 0 {
                moves |= (pos << 9) & NOT_A_FILE & ep_target;
                moves |= (pos << 7) & NOT_H_FILE & ep_target;
            }
        } else {
            let single_push = (pos >> 8) & empty;
            moves |= single_push;
            moves |= (single_push >> 8) & empty & RANK_5;
            moves |= (pos >> 7) & NOT_A_FILE & enemy_pieces;
            moves |= (pos >> 9) & NOT_H_FILE & enemy_pieces;
            if ep_target != 0 {
                moves |= (pos >> 7) & NOT_A_FILE & ep_target;
                moves |= (pos >> 9) & NOT_H_FILE & ep_target;
            }
        }
        moves & !own_pieces
    }
}
