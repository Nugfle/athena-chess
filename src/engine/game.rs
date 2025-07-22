use super::board::BitBoard;
use super::board::piece::Piece;
use super::board::square::Square;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    piece: Piece,
    from: Square,
    to: Square,
}

#[derive(Debug, Clone)]
pub struct Game {
    board: BitBoard,
    moves: Vec<Move>,
}

impl Game {
    pub fn init() -> Self {
        Self {
            board: BitBoard::init(),
            moves: Vec::new(),
        }
    }
}
