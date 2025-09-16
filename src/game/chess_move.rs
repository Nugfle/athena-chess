use std::fmt::Display;

use super::board::piece::Piece;
use super::board::square::Square;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    piece: Piece,
    from: Square,
    to: Square,
    takes: Option<Piece>,
}
impl Move {
    pub fn new(piece: Piece, from: Square, to: Square, takes: Option<Piece>) -> Self {
        Self { piece, from, to, takes }
    }
    pub fn get_from(&self) -> Square {
        self.from
    }
    pub fn get_to(&self) -> Square {
        self.to
    }
    pub fn get_piece(&self) -> Piece {
        self.piece
    }
    pub fn set_takes(&mut self, piece: Option<Piece>) {
        self.takes = piece
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.piece,
            self.from,
            self.takes.map(|_| "x").unwrap_or(""),
            self.to
        )
    }
}
