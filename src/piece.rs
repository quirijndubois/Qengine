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
}
