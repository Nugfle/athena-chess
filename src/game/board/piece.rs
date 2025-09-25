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
        match self {
            Self::Pawn => true,
            _ => false,
        }
    }
    pub fn is_knight(&self) -> bool {
        match self {
            Self::Knight => true,
            _ => false,
        }
    }
    pub fn is_bishop(&self) -> bool {
        match self {
            Self::Bishop => true,
            _ => false,
        }
    }
    pub fn is_rook(&self) -> bool {
        match self {
            Self::Rook { .. } => true,
            _ => false,
        }
    }
    pub fn is_queen(&self) -> bool {
        match self {
            Self::Queen => true,
            _ => false,
        }
    }
    pub fn is_king(&self) -> bool {
        match self {
            Self::King { .. } => true,
            _ => false,
        }
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

impl Not for Color {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
