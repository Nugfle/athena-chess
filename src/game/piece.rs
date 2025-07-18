use std::{cmp, fmt::Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Piece {
    WhitePawn = 1,
    WhiteKnight = 2,
    WhiteBishop = 3,
    WhiteRook = 4,
    WhiteQueen = 5,
    WhiteKing = 6,
    BlackPawn = 7,
    BlackKnight = 8,
    BlackBishop = 9,
    BlackRook = 10,
    BlackQueen = 11,
    BlackKing = 12,
}

impl Default for Piece {
    fn default() -> Self {
        Self::WhitePawn
    }
}

impl PartialOrd for Piece {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.is_knight() && other.is_bishop() {
            Some(cmp::Ordering::Less)
        } else if self.is_bishop() && other.is_knight() {
            Some(cmp::Ordering::Greater)
        } else {
            Some(self.get_value().cmp(&other.get_value()))
        }
    }
}

impl Ord for Piece {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::WhitePawn => write!(f, "White Pawn"),
            Piece::WhiteKnight => write!(f, "White Knight"),
            Piece::WhiteBishop => write!(f, "White Bishop"),
            Piece::WhiteRook => write!(f, "Whit Rook"),
            Piece::WhiteQueen => write!(f, "White Queen"),
            Piece::WhiteKing => write!(f, "White King"),
            Piece::BlackPawn => write!(f, "Black Pawn"),
            Piece::BlackKnight => write!(f, "Black Knight"),
            Piece::BlackBishop => write!(f, "Black Bishop"),
            Piece::BlackRook => write!(f, "Black Rook"),
            Piece::BlackQueen => write!(f, "Black Queen"),
            Piece::BlackKing => write!(f, "Black King"),
        }
    }
}

impl Piece {
    pub fn is_black(&self) -> bool {
        match self {
            Self::BlackPawn | Self::BlackKnight | Self::BlackBishop | Self::BlackRook | Self::BlackQueen | Self::BlackKing => true,
            _ => false,
        }
    }
    pub fn is_white(&self) -> bool {
        match self {
            Self::WhitePawn | Self::WhiteKnight | Self::WhiteBishop | Self::WhiteRook | Self::WhiteQueen | Self::WhiteKing => true,
            _ => false,
        }
    }
    pub fn is_king(&self) -> bool {
        match self {
            Piece::WhiteKing | Piece::BlackKing => true,
            _ => false,
        }
    }
    pub fn is_queen(&self) -> bool {
        match self {
            Piece::WhiteQueen | Piece::BlackQueen => true,
            _ => false,
        }
    }
    pub fn is_rook(&self) -> bool {
        match self {
            Piece::WhiteRook | Piece::BlackRook => true,
            _ => false,
        }
    }
    pub fn is_bishop(&self) -> bool {
        match self {
            Piece::WhiteBishop | Piece::BlackBishop => true,
            _ => false,
        }
    }
    pub fn is_knight(&self) -> bool {
        match self {
            Piece::WhiteKnight | Piece::BlackKnight => true,
            _ => false,
        }
    }
    pub fn is_pawn(&self) -> bool {
        match self {
            Piece::WhitePawn | Piece::BlackPawn => true,
            _ => false,
        }
    }
    pub fn get_value(&self) -> u8 {
        match self {
            Piece::WhitePawn | Piece::BlackPawn => 1,
            Piece::WhiteKnight | Piece::BlackKnight => 3,
            Piece::WhiteBishop | Piece::BlackBishop => 3,
            Piece::WhiteRook | Piece::BlackRook => 5,
            Piece::WhiteQueen | Piece::BlackQueen => 8,
            Piece::WhiteKing | Piece::BlackKing => u8::MAX,
        }
    }
    pub fn chess_notation(&self) -> &'static str {
        match self {
            Piece::WhitePawn | Piece::BlackPawn => "",
            Piece::WhiteKnight | Piece::BlackKnight => "K",
            Piece::WhiteBishop | Piece::BlackBishop => "B",
            Piece::WhiteRook | Piece::BlackRook => "R",
            Piece::WhiteQueen | Piece::BlackQueen => "Q",
            Piece::WhiteKing | Piece::BlackKing => "K",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Piece::WhitePawn | Piece::BlackPawn => "P",
            Piece::WhiteKnight | Piece::BlackKnight => "K",
            Piece::WhiteBishop | Piece::BlackBishop => "B",
            Piece::WhiteRook | Piece::BlackRook => "R",
            Piece::WhiteQueen | Piece::BlackQueen => "Q",
            Piece::WhiteKing | Piece::BlackKing => "K",
        }
    }
}
