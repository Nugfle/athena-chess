pub mod piece;
pub mod square;

use piece::{Color, Piece};

/// a representation of the board where each bit in the u64 represents the square on the board and
/// whether it is occupied. This makes checking for blocking pieces as easy as applying a mask to
/// the Occupancy and voila, you get all the squares with blocking pieces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Occupancy(pub u64);

#[derive(Debug, Clone)]
pub struct BitBoard {
    pub board: [Option<(Piece, Color)>; 64],
    pub occupancy: Occupancy, // tracks whether each square is occupied
}

impl BitBoard {
    pub fn init() -> Self {
        todo!("setup board and occupancy for a new game")
    }
}
