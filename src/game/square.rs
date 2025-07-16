use crate::game::error::InvalidSquareError;
use log::error;
use std::fmt::Display;
use std::str::FromStr;

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
