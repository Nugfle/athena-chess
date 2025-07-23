#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Piece {
    #[default]
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Color {
    #[default]
    White,
    Black,
}
