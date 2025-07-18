use super::error::ChessError;
use super::{piece::Piece, square::Square};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Default)]
pub struct Move {
    piece: Piece,
    from: Square,
    to: Square,
    takes: bool,
    check: bool,
    ambigous: bool,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            if self.takes && self.piece.is_pawn() {
                self.from.to_string().chars().nth(0).unwrap().to_string()
            } else {
                self.piece.chess_notation().to_string()
            },
            if self.ambigous { self.from.to_string() } else { "".to_string() },
            if self.takes { "x" } else { "" },
            self.to.to_string(),
            if self.check { "#" } else { "" },
        )
    }
}

impl FromStr for Move {
    type Err = ChessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m = Self::default();
        s.contains("x").then(|| m.takes = true);
        m.to = s.split("x").nth(1).unwrap().parse::<Square>()?;
        Ok(m)
    }
}

impl Move {
    pub fn new(from: Square, to: Square, piece: Piece, takes: bool, check: bool, ambigous: bool) -> Self {
        Self {
            piece,
            from,
            to,
            takes,
            check,
            ambigous,
        }
    }
}
