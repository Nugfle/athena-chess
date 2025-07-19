use super::{piece::Piece, square::Square};

#[derive(Debug, Clone, Default, Copy)]
pub struct Move {
    piece: Piece,
    from: Square,
    to: Square,
}

impl Move {
    pub fn new(from: Square, to: Square, piece: Piece) -> Self {
        Self { piece, from, to }
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
}
