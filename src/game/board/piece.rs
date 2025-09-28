use std::{fmt::Display, ops::Not};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook { has_moved: bool },
    Queen,
    King { has_moved: bool },
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pawn => "",
                Self::Knight => "N",
                Self::Bishop => "B",
                Self::Rook { .. } => "R",
                Self::Queen => "Q",
                Self::King { .. } => "K",
            }
        )
    }
}

impl Piece {
    pub fn is_pawn(&self) -> bool {
        *self == Self::Pawn
    }
    pub fn is_knight(&self) -> bool {
        *self == Self::Knight
    }
    pub fn is_bishop(&self) -> bool {
        *self == Self::Bishop
    }
    pub fn is_rook(&self) -> bool {
        matches!(self, Self::Rook { .. })
    }
    pub fn is_queen(&self) -> bool {
        *self == Self::Queen
    }
    pub fn is_king(&self) -> bool {
        matches!(self, Self::King { .. })
    }

    pub fn make_moved(&mut self) {
        match self {
            Self::King { has_moved } => *has_moved = true,
            Self::Rook { has_moved } => *has_moved = true,
            _ => (),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn is_white(&self) -> bool {
        *self == Color::White
    }
    pub fn is_black(&self) -> bool {
        *self == Color::Black
    }
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

impl Not for Color {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
