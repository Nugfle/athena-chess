use crate::engine::{piece::Piece, square::Square};

pub struct Move {
    from: Square,
    to: Square,
    piece: Piece,
}
