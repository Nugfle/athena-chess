use crate::game::error::InvalidSquareError;
use lazy_static::lazy_static;
use log::error;
use std::fmt::Display;
use std::str::FromStr;

lazy_static! {
    pub static ref a1: Square = "a1".parse().unwrap();
    pub static ref a2: Square = "a2".parse().unwrap();
    pub static ref a3: Square = "a3".parse().unwrap();
    pub static ref a4: Square = "a4".parse().unwrap();
    pub static ref a5: Square = "a5".parse().unwrap();
    pub static ref a6: Square = "a6".parse().unwrap();
    pub static ref a7: Square = "a7".parse().unwrap();
    pub static ref a8: Square = "a8".parse().unwrap();
    pub static ref b1: Square = "b1".parse().unwrap();
    pub static ref b2: Square = "b2".parse().unwrap();
    pub static ref b3: Square = "b3".parse().unwrap();
    pub static ref b4: Square = "b4".parse().unwrap();
    pub static ref b5: Square = "b5".parse().unwrap();
    pub static ref b6: Square = "b6".parse().unwrap();
    pub static ref b7: Square = "b7".parse().unwrap();
    pub static ref b8: Square = "b8".parse().unwrap();
    pub static ref c1: Square = "c1".parse().unwrap();
    pub static ref c2: Square = "c2".parse().unwrap();
    pub static ref c3: Square = "c3".parse().unwrap();
    pub static ref c4: Square = "c4".parse().unwrap();
    pub static ref c5: Square = "c5".parse().unwrap();
    pub static ref c6: Square = "c6".parse().unwrap();
    pub static ref c7: Square = "c7".parse().unwrap();
    pub static ref c8: Square = "c8".parse().unwrap();
    pub static ref d1: Square = "d1".parse().unwrap();
    pub static ref d2: Square = "d2".parse().unwrap();
    pub static ref d3: Square = "d3".parse().unwrap();
    pub static ref d4: Square = "d4".parse().unwrap();
    pub static ref d5: Square = "d5".parse().unwrap();
    pub static ref d6: Square = "d6".parse().unwrap();
    pub static ref d7: Square = "d7".parse().unwrap();
    pub static ref d8: Square = "d8".parse().unwrap();
    pub static ref e1: Square = "e1".parse().unwrap();
    pub static ref e2: Square = "e2".parse().unwrap();
    pub static ref e3: Square = "e3".parse().unwrap();
    pub static ref e4: Square = "e4".parse().unwrap();
    pub static ref e5: Square = "e5".parse().unwrap();
    pub static ref e6: Square = "e6".parse().unwrap();
    pub static ref e7: Square = "e7".parse().unwrap();
    pub static ref e8: Square = "e8".parse().unwrap();
    pub static ref f1: Square = "f1".parse().unwrap();
    pub static ref f2: Square = "f2".parse().unwrap();
    pub static ref f3: Square = "f3".parse().unwrap();
    pub static ref f4: Square = "f4".parse().unwrap();
    pub static ref f5: Square = "f5".parse().unwrap();
    pub static ref f6: Square = "f6".parse().unwrap();
    pub static ref f7: Square = "f7".parse().unwrap();
    pub static ref f8: Square = "f8".parse().unwrap();
    pub static ref g1: Square = "g1".parse().unwrap();
    pub static ref g2: Square = "g2".parse().unwrap();
    pub static ref g3: Square = "g3".parse().unwrap();
    pub static ref g4: Square = "g4".parse().unwrap();
    pub static ref g5: Square = "g5".parse().unwrap();
    pub static ref g6: Square = "g6".parse().unwrap();
    pub static ref g7: Square = "g7".parse().unwrap();
    pub static ref g8: Square = "g8".parse().unwrap();
    pub static ref h1: Square = "h1".parse().unwrap();
    pub static ref h2: Square = "h2".parse().unwrap();
    pub static ref h3: Square = "h3".parse().unwrap();
    pub static ref h4: Square = "h4".parse().unwrap();
    pub static ref h5: Square = "h5".parse().unwrap();
    pub static ref h6: Square = "h6".parse().unwrap();
    pub static ref h7: Square = "h7".parse().unwrap();
    pub static ref h8: Square = "h8".parse().unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Square {
    horizontal: u8,
    vertical: u8,
}

impl Square {
    pub fn new(horizontal: u8, vertical: u8) -> Result<Self, InvalidSquareError> {
        if horizontal >= 8 || vertical >= 8 {
            Err(InvalidSquareError::OutOfBounds {
                h: horizontal,
                v: vertical,
            })
        } else {
            Ok(Self {
                horizontal,
                vertical,
            })
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
                return Err(InvalidSquareError::InvalidLiteral {
                    literal: value.to_string(),
                });
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
                return Err(InvalidSquareError::InvalidLiteral {
                    literal: value.to_string(),
                });
            }
        };
        Ok(Self {
            horizontal,
            vertical,
        })
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
