#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    WhiteKing,
    BlackKing,
    WhiteQueen,
    BlackQueen,
    WhitePawn,
    BlackPawn,
    WhiteRook,
    BlackRook,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
}

impl Piece {
    pub fn get_side(self) -> &'static str {
        match self {
            Piece::WhiteKing
            | Piece::WhiteQueen
            | Piece::WhitePawn
            | Piece::WhiteRook
            | Piece::WhiteKnight
            | Piece::WhiteBishop => "white",
            _ => "black",
        }
    }

    pub fn asset_data(self) -> (&'static str, &'static [u8]) {
        match self {
            Piece::WhiteKing => (
                "bytes://king_w.svg",
                include_bytes!("../sprites/king_white.svg"),
            ),
            Piece::BlackKing => (
                "bytes://king_b.svg",
                include_bytes!("../sprites/king_black.svg"),
            ),
            Piece::WhiteQueen => (
                "bytes://queen_w.svg",
                include_bytes!("../sprites/queen_white.svg"),
            ),
            Piece::BlackQueen => (
                "bytes://queen_b.svg",
                include_bytes!("../sprites/queen_black.svg"),
            ),
            Piece::WhitePawn => (
                "bytes://pawn_w.svg",
                include_bytes!("../sprites/pawn_white.svg"),
            ),
            Piece::BlackPawn => (
                "bytes://pawn_b.svg",
                include_bytes!("../sprites/pawn_black.svg"),
            ),
            Piece::WhiteRook => (
                "bytes://rook_w.svg",
                include_bytes!("../sprites/rook_white.svg"),
            ),
            Piece::BlackRook => (
                "bytes://rook_b.svg",
                include_bytes!("../sprites/rook_black.svg"),
            ),
            Piece::WhiteKnight => (
                "bytes://knight_w.svg",
                include_bytes!("../sprites/knight_white.svg"),
            ),
            Piece::BlackKnight => (
                "bytes://knight_b.svg",
                include_bytes!("../sprites/knight_black.svg"),
            ),
            Piece::WhiteBishop => (
                "bytes://bishop_w.svg",
                include_bytes!("../sprites/bishop_white.svg"),
            ),
            Piece::BlackBishop => (
                "bytes://bishop_b.svg",
                include_bytes!("../sprites/bishop_black.svg"),
            ),
        }
    }

    pub fn get_legal_moves(
        self,
        pos: (usize, usize),
        board: &[[Option<Piece>; 8]; 8],
    ) -> Vec<(usize, usize)> {
        match self {
            Piece::WhiteKing | Piece::BlackKing => get_king_moves(pos, board),
            Piece::WhiteQueen | Piece::BlackQueen => get_queen_moves(pos, board),
            Piece::BlackPawn | Piece::WhitePawn => get_pawn_moves(self, pos, board),
            Piece::WhiteRook | Piece::BlackRook => get_rook_moves(pos, board),
            Piece::WhiteKnight | Piece::BlackKnight => get_knight_moves(pos, board),
            Piece::WhiteBishop | Piece::BlackBishop => get_bishop_moves(pos, board),
        }
    }
}

fn get_king_moves(pos: (usize, usize), board: &[[Option<Piece>; 8]; 8]) -> Vec<(usize, usize)> {
    let (col, row) = pos;
    let mut moves = Vec::new();

    let piece_side = match &board[row][col] {
        Some(piece) => piece.get_side(),
        None => return moves,
    };

    let directions = [
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
    ];

    for (dc, dr) in directions {
        let mut new_col = col as isize + dc;
        let mut new_row = row as isize + dr;

        if new_col >= 0 && new_col < 8 && new_row >= 0 && new_row < 8 {
            let curr_c = new_col as usize;
            let curr_r = new_row as usize;

            match &board[curr_r][curr_c] {
                None => {
                    moves.push((curr_c, curr_r));
                }
                Some(p) => {
                    if p.get_side() != piece_side {
                        moves.push((curr_c, curr_r));
                    }
                }
            }

            new_col += dc;
            new_row += dr;
        }
    }
    moves
}

fn get_queen_moves(pos: (usize, usize), board: &[[Option<Piece>; 8]; 8]) -> Vec<(usize, usize)> {
    let mut moves = get_rook_moves(pos, board);
    moves.extend(get_bishop_moves(pos, board));
    moves
}

fn get_pawn_moves(
    piece: Piece,
    pos: (usize, usize),
    board: &[[Option<Piece>; 8]; 8],
) -> Vec<(usize, usize)> {
    match piece {
        Piece::WhitePawn => get_white_pawn_moves(piece, pos, board),
        Piece::BlackPawn => get_black_pawn_moves(piece, pos, board),
        _ => Vec::new(),
    }
}

fn get_black_pawn_moves(
    _piece: Piece,
    pos: (usize, usize),
    board: &[[Option<Piece>; 8]; 8],
) -> Vec<(usize, usize)> {
    let (col, row) = pos;
    let mut moves = Vec::new();
    if row < 7 && board[row + 1][col].is_none() {
        moves.push((col, row + 1));
        if row == 1 && board[row + 2][col].is_none() {
            moves.push((col, row + 2));
        }
    }
    if col > 0 && row < 7 {
        if let Some(piece) = board[row + 1][col - 1] {
            if piece.asset_data().0 != board[row][col].unwrap().asset_data().0 {
                moves.push((col - 1, row + 1));
            }
        }
    }
    if col < 7 && row < 7 {
        if let Some(piece) = board[row + 1][col + 1] {
            if piece.asset_data().0 != board[row][col].unwrap().asset_data().0 {
                moves.push((col + 1, row + 1));
            }
        }
    }
    moves
}

fn get_white_pawn_moves(
    _piece: Piece,
    pos: (usize, usize),
    board: &[[Option<Piece>; 8]; 8],
) -> Vec<(usize, usize)> {
    let (col, row) = pos;
    let mut moves = Vec::new();
    if row > 0 && board[row - 1][col].is_none() {
        moves.push((col, row - 1));
        if row == 6 && board[row - 2][col].is_none() {
            moves.push((col, row - 2));
        }
    }
    if col > 0 && row > 0 {
        if let Some(piece) = board[row - 1][col - 1] {
            if piece.asset_data().0 != board[row][col].unwrap().asset_data().0 {
                moves.push((col - 1, row - 1));
            }
        }
    }
    if col < 7 && row > 0 {
        if let Some(piece) = board[row - 1][col + 1] {
            if piece.asset_data().0 != board[row][col].unwrap().asset_data().0 {
                moves.push((col + 1, row - 1));
            }
        }
    }
    moves
}

fn get_rook_moves(pos: (usize, usize), board: &[[Option<Piece>; 8]; 8]) -> Vec<(usize, usize)> {
    let (col, row) = pos;
    let mut moves = Vec::new();

    let piece_side = match &board[row][col] {
        Some(piece) => piece.get_side(),
        None => return moves,
    };

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for (dc, dr) in directions {
        let mut new_col = col as isize + dc;
        let mut new_row = row as isize + dr;

        while new_col >= 0 && new_col < 8 && new_row >= 0 && new_row < 8 {
            let curr_c = new_col as usize;
            let curr_r = new_row as usize;

            match &board[curr_r][curr_c] {
                None => {
                    moves.push((curr_c, curr_r));
                }
                Some(p) => {
                    if p.get_side() != piece_side {
                        moves.push((curr_c, curr_r));
                    }
                    break;
                }
            }

            new_col += dc;
            new_row += dr;
        }
    }
    moves
}

fn get_bishop_moves(pos: (usize, usize), board: &[[Option<Piece>; 8]; 8]) -> Vec<(usize, usize)> {
    let (col, row) = pos;
    let mut moves = Vec::new();

    let piece_side = match &board[row][col] {
        Some(piece) => piece.get_side(),
        None => return moves,
    };

    let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    for (dc, dr) in directions {
        let mut new_col = col as isize + dc;
        let mut new_row = row as isize + dr;

        while new_col >= 0 && new_col < 8 && new_row >= 0 && new_row < 8 {
            let curr_c = new_col as usize;
            let curr_r = new_row as usize;

            match &board[curr_r][curr_c] {
                None => {
                    moves.push((curr_c, curr_r));
                }
                Some(p) => {
                    if p.get_side() != piece_side {
                        moves.push((curr_c, curr_r));
                    }
                    break;
                }
            }

            new_col += dc;
            new_row += dr;
        }
    }
    moves
}

fn get_knight_moves(pos: (usize, usize), board: &[[Option<Piece>; 8]; 8]) -> Vec<(usize, usize)> {
    let (col, row) = pos;
    let mut moves = Vec::new();

    let piece_side = match &board[row][col] {
        Some(piece) => piece.get_side(),
        None => return moves,
    };

    let knight_moves = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];

    for (dc, dr) in knight_moves {
        let new_col = col as isize + dc;
        let new_row = row as isize + dr;

        // 1. BOUNDS CHECK: Verify the jump stays on the board
        if new_col >= 0 && new_col < 8 && new_row >= 0 && new_row < 8 {
            let nc = new_col as usize;
            let nr = new_row as usize;

            // 2. OCCUPANCY CHECK: Safe to index now
            match &board[nr][nc] {
                None => {
                    moves.push((nc, nr));
                }
                Some(p) => {
                    // If it's an enemy, we can take it
                    if p.get_side() != piece_side {
                        moves.push((nc, nr));
                    }
                    // Note: No 'break' here!
                    // A knight's jumps are independent of each other.
                }
            }
        }
    }
    moves
}

