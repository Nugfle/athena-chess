use log::error;
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub struct SquareFromError((u8, u8));

impl Display for SquareFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Square From Error: Can't create square from ({},{})", self.0.0, self.0.1)
    }
}

impl Error for SquareFromError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    horizontal: u8,
    vertical: u8,
}

impl Square {
    pub fn new(horizontal: u8, vertical: u8) -> Result<Self, SquareFromError> {
        if horizontal >= 8 || vertical >= 8 {
            error!("got invalid horizontal or vertical position: {}:{}", horizontal, vertical);
            return Err(SquareFromError((horizontal, vertical)));
        }
        Ok(Self { horizontal, vertical })
    }

    pub fn get_horizontal(&self) -> u8 {
        self.horizontal
    }
    pub fn get_vertical(&self) -> u8 {
        self.vertical
    }
}

impl TryFrom<(u8, u8)> for Square {
    type Error = SquareFromError;
    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1)
    }
}
impl TryFrom<Result<Square, SquareFromError>> for Square {
    type Error = SquareFromError;
    fn try_from(value: Result<Square, SquareFromError>) -> Result<Self, Self::Error> {
        value
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
            self.vertical
        )
    }
}
