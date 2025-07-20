use super::occupancy::Occupancy;
use super::piece::{Color, Piece};

/// represents a square on a chess board. Can be in Range from 0 to 63
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Square(pub usize);

pub struct BitBoard {
    pub board: [Option<(Piece, Color)>; 64],
    pub occupancy: Occupancy, // tracks whether each square is occupied
}
