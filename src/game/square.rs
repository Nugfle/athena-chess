use log::error;
use std::fmt::Display;
use std::hash::Hash;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum InvalidSquareError {
    #[error("Can't create Square, out of bounds: ({h}, {v})")]
    OutOfBounds { h: u8, v: u8 },
    #[error("The Literal: '{literal}' is not of lenght 2: {length}")]
    InvalidLiteralLength { literal: String, length: usize },
    #[error("The Literal: '{literal}' is not valid chess notation for a square")]
    InvalidLiteral { literal: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    horizontal: u8,
    vertical: u8,
}
impl Hash for Square {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(self.vertical * 8 + self.horizontal); // ToDo: Test whether >>3 is faster
    }
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

impl TryFrom<&str> for Square {
    type Error = InvalidSquareError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
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
            _ => return Err(InvalidSquareError::InvalidLiteral { literal: value.to_string() }),
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
            _ => return Err(InvalidSquareError::InvalidLiteral { literal: value.to_string() }),
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
