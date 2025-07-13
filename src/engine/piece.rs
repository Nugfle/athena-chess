use std::fmt::Display;
use std::ops::Neg;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}
impl Color {
    pub fn is_black(&self) -> bool {
        match self {
            Color::Black => true,
            Color::White => false,
        }
    }
    pub fn is_white(&self) -> bool {
        match self {
            Color::Black => false,
            Color::White => true,
        }
    }
}

impl Neg for Color {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Black => write!(f, "Black"),
            Color::White => write!(f, "White"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Pawn(color) => write!(f, "{} Pawn", color.to_string()),
            Piece::Knight(color) => write!(f, "{} Knight", color.to_string()),
            Piece::Bishop(color) => write!(f, "{} Bishop", color.to_string()),
            Piece::Rook(color) => write!(f, "{} Rook", color.to_string()),
            Piece::Queen(color) => write!(f, "{} Queen", color.to_string()),
            Piece::King(color) => write!(f, "{} King", color.to_string()),
        }
    }
}

impl Piece {
    pub fn is_black(&self) -> bool {
        self.get_color().is_black()
    }
    pub fn is_white(&self) -> bool {
        self.get_color().is_white()
    }
    pub fn is_king(&self) -> bool {
        match self {
            Piece::King(_) => true,
            _ => false,
        }
    }
    pub fn get_color(&self) -> Color {
        match self {
            Piece::Pawn(color) => *color,
            Piece::Knight(color) => *color,
            Piece::Bishop(color) => *color,
            Piece::Rook(color) => *color,
            Piece::Queen(color) => *color,
            Piece::King(color) => *color,
        }
    }
    pub fn get_value(&self) -> u8 {
        match self {
            Piece::Pawn(_) => 1,
            Piece::Knight(_) => 3,
            Piece::Bishop(_) => 3,
            Piece::Rook(_) => 5,
            Piece::Queen(_) => 8,
            Piece::King(_) => 0,
        }
    }
    pub fn chess_notation(&self) -> &'static str {
        match self {
            Piece::Pawn(_) => "",
            Piece::Knight(_) => "N",
            Piece::Bishop(_) => "B",
            Piece::Rook(_) => "R",
            Piece::Queen(_) => "Q",
            Piece::King(_) => "K",
        }
    }
}
