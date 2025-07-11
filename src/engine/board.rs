use crate::engine::piece::Piece;
use crate::engine::square::Square;

pub struct Board {
    board: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn square_is_occupied(&self, square: Square) -> bool {
        self.board[square.get_vertical() as usize][square.get_horizontal() as usize].is_some()
    }
}
