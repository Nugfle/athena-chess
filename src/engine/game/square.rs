use super::error::InvalidSquareError;
use log::error;
use std::fmt::Display;
use std::str::FromStr;

pub const A1: Square = Square { vertical: 0, horizontal: 0 };
pub const A2: Square = Square { vertical: 1, horizontal: 0 };
pub const A3: Square = Square { vertical: 2, horizontal: 0 };
pub const A4: Square = Square { vertical: 3, horizontal: 0 };
pub const A5: Square = Square { vertical: 4, horizontal: 0 };
pub const A6: Square = Square { vertical: 5, horizontal: 0 };
pub const A7: Square = Square { vertical: 6, horizontal: 0 };
pub const A8: Square = Square { vertical: 7, horizontal: 0 };

pub const B1: Square = Square { vertical: 0, horizontal: 1 };
pub const B2: Square = Square { vertical: 1, horizontal: 1 };
pub const B3: Square = Square { vertical: 2, horizontal: 1 };
pub const B4: Square = Square { vertical: 3, horizontal: 1 };
pub const B5: Square = Square { vertical: 4, horizontal: 1 };
pub const B6: Square = Square { vertical: 5, horizontal: 1 };
pub const B7: Square = Square { vertical: 6, horizontal: 1 };
pub const B8: Square = Square { vertical: 7, horizontal: 1 };

pub const C1: Square = Square { vertical: 0, horizontal: 2 };
pub const C2: Square = Square { vertical: 1, horizontal: 2 };
pub const C3: Square = Square { vertical: 2, horizontal: 2 };
pub const C4: Square = Square { vertical: 3, horizontal: 2 };
pub const C5: Square = Square { vertical: 4, horizontal: 2 };
pub const C6: Square = Square { vertical: 5, horizontal: 2 };
pub const C7: Square = Square { vertical: 6, horizontal: 2 };
pub const C8: Square = Square { vertical: 7, horizontal: 2 };

pub const D1: Square = Square { vertical: 0, horizontal: 3 };
pub const D2: Square = Square { vertical: 1, horizontal: 3 };
pub const D3: Square = Square { vertical: 2, horizontal: 3 };
pub const D4: Square = Square { vertical: 3, horizontal: 3 };
pub const D5: Square = Square { vertical: 4, horizontal: 3 };
pub const D6: Square = Square { vertical: 5, horizontal: 3 };
pub const D7: Square = Square { vertical: 6, horizontal: 3 };
pub const D8: Square = Square { vertical: 7, horizontal: 3 };

pub const E1: Square = Square { vertical: 0, horizontal: 4 };
pub const E2: Square = Square { vertical: 1, horizontal: 4 };
pub const E3: Square = Square { vertical: 2, horizontal: 4 };
pub const E4: Square = Square { vertical: 3, horizontal: 4 };
pub const E5: Square = Square { vertical: 4, horizontal: 4 };
pub const E6: Square = Square { vertical: 5, horizontal: 4 };
pub const E7: Square = Square { vertical: 6, horizontal: 4 };
pub const E8: Square = Square { vertical: 7, horizontal: 4 };

pub const F1: Square = Square { vertical: 0, horizontal: 5 };
pub const F2: Square = Square { vertical: 1, horizontal: 5 };
pub const F3: Square = Square { vertical: 2, horizontal: 5 };
pub const F4: Square = Square { vertical: 3, horizontal: 5 };
pub const F5: Square = Square { vertical: 4, horizontal: 5 };
pub const F6: Square = Square { vertical: 5, horizontal: 5 };
pub const F7: Square = Square { vertical: 6, horizontal: 5 };
pub const F8: Square = Square { vertical: 7, horizontal: 5 };

pub const G1: Square = Square { vertical: 0, horizontal: 6 };
pub const G2: Square = Square { vertical: 1, horizontal: 6 };
pub const G3: Square = Square { vertical: 2, horizontal: 6 };
pub const G4: Square = Square { vertical: 3, horizontal: 6 };
pub const G5: Square = Square { vertical: 4, horizontal: 6 };
pub const G6: Square = Square { vertical: 5, horizontal: 6 };
pub const G7: Square = Square { vertical: 6, horizontal: 6 };
pub const G8: Square = Square { vertical: 7, horizontal: 6 };

pub const H1: Square = Square { vertical: 0, horizontal: 7 };
pub const H2: Square = Square { vertical: 1, horizontal: 7 };
pub const H3: Square = Square { vertical: 2, horizontal: 7 };
pub const H4: Square = Square { vertical: 3, horizontal: 7 };
pub const H5: Square = Square { vertical: 4, horizontal: 7 };
pub const H6: Square = Square { vertical: 5, horizontal: 7 };
pub const H7: Square = Square { vertical: 6, horizontal: 7 };
pub const H8: Square = Square { vertical: 7, horizontal: 7 };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Square {
    horizontal: u8,
    vertical: u8,
}

impl Square {
    pub fn new(horizontal: u8, vertical: u8) -> Result<Self, InvalidSquareError> {
        if horizontal >= 8 || vertical >= 8 {
            Err(InvalidSquareError::OutOfBounds { h: horizontal, v: vertical })
        } else {
            Ok(Self { horizontal, vertical })
        }
    }

    pub fn horizontal(&self) -> u8 {
        self.horizontal
    }
    pub fn vertical(&self) -> u8 {
        self.vertical
    }
}

impl FromStr for Square {
    type Err = InvalidSquareError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 2 {
            return Err(InvalidSquareError::InvalidLiteralLength {
                literal: value.to_string(),
                length: value.len(),
            });
        }
        let horizontal = match value.chars().nth(0).unwrap() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => {
                return Err(InvalidSquareError::InvalidLiteral { literal: value.to_string() });
            }
        };
        let vertical = match value.chars().nth(1).unwrap() {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => {
                return Err(InvalidSquareError::InvalidLiteral { literal: value.to_string() });
            }
        };
        Ok(Self { horizontal, vertical })
    }
}

impl TryFrom<(u8, u8)> for Square {
    type Error = InvalidSquareError;
    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1)
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.horizontal >= 8 || self.vertical >= 8 {
            error!("got invalid square: {:?}", self);
            return Err(std::fmt::Error::default());
        }
        write!(
            f,
            "{}{}",
            match self.horizontal {
                0 => "a",
                1 => "b",
                2 => "c",
                3 => "d",
                4 => "e",
                5 => "f",
                6 => "g",
                7 => "h",
                _ => panic!(),
            },
            self.vertical + 1
        )
    }
}
