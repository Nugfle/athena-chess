use std::fmt::Display;

use crate::engine::{
    piece::Piece,
    square::{InvalidSquareError, Square},
};

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
            self.piece.chess_notation(),
            if self.ambigous { self.from.to_string() } else { "".to_string() },
            if self.takes { "x" } else { "" },
            self.to.to_string(),
            if self.check { "#" } else { "" },
        )
    }
}

impl Move {
    pub fn new<T>(from: T, to: T, piece: Piece, takes: bool, check: bool, ambigous: bool) -> Result<Self, InvalidSquareError>
    where
        T: TryInto<Square, Error = InvalidSquareError>,
    {
        Ok(Self {
            piece,
            from: from.try_into()?,
            to: to.try_into()?,
            takes,
            check,
            ambigous,
        })
    }
    pub fn get_from(&self) -> Square {
        self.from
    }
    pub fn get_to(&self) -> Square {
        self.to
    }
    pub fn get_piece(&self) -> Piece {
        self.piece
    }
    pub fn is_check(&self) -> bool {
        self.check
    }
    pub fn takes(&self) -> bool {
        self.takes
    }
    pub fn ambigous(&self) -> bool {
        self.ambigous
    }
}
