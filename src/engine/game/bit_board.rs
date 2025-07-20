use super::occupancy::Occupancy;
use super::piece::{Color, Piece};

pub struct BitBoard {
    pub board: [Option<(Piece, Color)>; 64],
    pub occupancy: Occupancy, // tracks whether each square is occupied
}
