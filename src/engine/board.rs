pub mod piece;
pub mod square;

use std::ops::BitAnd;

use piece::{Color, Piece};
use square::Square;

/// a representation of the board where each bit in the u64 represents the square on the board and
/// whether it is occupied. This makes checking for blocking pieces as easy as applying a mask to
/// the Occupancy and voila, you get all the squares with blocking pieces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Occupancy(pub u64);

impl Occupancy {
    pub fn add_square(&mut self, square: Square) { self.0 |= 1_u64 << square.as_u8(); }
    pub fn with_square(&self, square: Square) -> Self { Occupancy(self.0 | 1_u64 << square.as_u8()) }
    pub fn remove_square(&mut self, square: Square) { self.0 &= !(1_u64 << square.as_u8()); }
    pub fn with_square_removed(&self, square: Square) -> Self { Occupancy(self.0 & !(1_u64 << square.as_u8())) }
    pub fn is_occupied(&self, square: Square) -> bool { self.0 & 1_u64 << square.as_u8() != 0 }
}

/// represents the current Board state.
#[derive(Debug, Clone)]
pub struct BitBoard {
    pub board: [Option<(Piece, Color)>; 64],

    // tracks whether each square is occupied, must be kept in sync with the board. Should only be
    // used for lookups in the Attack Tables.
    pub occupancy: Occupancy,
}

impl BitBoard {
    fn place(&mut self, piece: Piece, color: Color, square: Square) {
        self.board[square.as_index()] = Some((piece, color));
        self.occupancy.add_square(square);
    }
    pub fn init() -> Self { todo!("setup board and occupancy for a new game") }
}
