use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    Pawn { en_pasent: bool },
    Knight,
    Bishop,
    Rook,
    Queen,
    King { can_castle: bool },
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pawn { .. } => "",
                Self::Knight => "N",
                Self::Bishop => "B",
                Self::Rook => "R",
                Self::Queen => "Q",
                Self::King { .. } => "K",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::White => "White",
                Color::Black => "Black",
            }
        )
    }
}
