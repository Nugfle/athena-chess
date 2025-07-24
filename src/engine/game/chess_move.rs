use std::fmt::Display;

use super::board::piece::Piece;
use super::board::square::Square;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    piece: Piece,
    from: Square,
    to: Square,
}
impl Move {
    pub fn new(piece: Piece, from: Square, to: Square) -> Self { Self { piece, from, to } }
    pub fn get_from(&self) -> Square { self.from }
    pub fn get_to(&self) -> Square { self.to }
    pub fn get_piece(&self) -> Piece { self.piece }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}{}{}", self.piece, self.from, self.to) }
}
