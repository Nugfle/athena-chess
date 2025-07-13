use log::error;
use std::fmt::Display;
use std::hash::Hash;
use std::num::TryFromIntError;
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
#[error("Can't create Square, out of bounds!")]
pub struct InvalidSquareError {}

impl From<TryFromIntError> for InvalidSquareError {
    fn from(_: TryFromIntError) -> Self {
        Self {}
    }
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
            Err(InvalidSquareError {})
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
impl TryFrom<(u8, u8)> for Square {
    type Error = InvalidSquareError;
    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1)
    }
}

impl TryFrom<(u16, u16)> for Square {
    type Error = InvalidSquareError;
    fn try_from(value: (u16, u16)) -> Result<Self, Self::Error> {
        Self::new(value.0.try_into()?, value.1.try_into()?)
    }
}
impl TryFrom<(u32, u32)> for Square {
    type Error = InvalidSquareError;
    fn try_from(value: (u32, u32)) -> Result<Self, Self::Error> {
        Self::new(value.0.try_into()?, value.1.try_into()?)
    }
}
impl TryFrom<(u64, u64)> for Square {
    type Error = InvalidSquareError;
    fn try_from(value: (u64, u64)) -> Result<Self, Self::Error> {
        Self::new(value.0.try_into()?, value.1.try_into()?)
    }
}
impl TryFrom<(i8, i8)> for Square {
    type Error = InvalidSquareError;
    fn try_from(value: (i8, i8)) -> Result<Self, Self::Error> {
        Self::new(value.0.try_into()?, value.1.try_into()?)
    }
}

impl TryFrom<(i16, i16)> for Square {
    type Error = InvalidSquareError;
    fn try_from(value: (i16, i16)) -> Result<Self, Self::Error> {
        Self::new(value.0.try_into()?, value.1.try_into()?)
    }
}
impl TryFrom<(i32, i32)> for Square {
    type Error = InvalidSquareError;
    fn try_from(value: (i32, i32)) -> Result<Self, Self::Error> {
        Self::new(value.0.try_into()?, value.1.try_into()?)
    }
}
impl TryFrom<(i64, i64)> for Square {
    type Error = InvalidSquareError;
    fn try_from(value: (i64, i64)) -> Result<Self, Self::Error> {
        Self::new(value.0.try_into()?, value.1.try_into()?)
    }
}

impl TryFrom<Result<Square, InvalidSquareError>> for Square {
    type Error = InvalidSquareError;
    fn try_from(value: Result<Square, InvalidSquareError>) -> Result<Self, Self::Error> {
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
            self.vertical + 1
        )
    }
}
