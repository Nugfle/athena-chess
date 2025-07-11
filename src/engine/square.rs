use log::error;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    horizontal: u8,
    vertical: u8,
}

impl Square {
    pub fn new(horizontal: u8, vertical: u8) -> Option<Self> {
        if horizontal >= 8 || vertical >= 8 {
            error!(
                "got invalid horizontal or vertical position: {}:{}",
                horizontal, vertical
            );
            return None;
        }
        Some(Self {
            horizontal,
            vertical,
        })
    }

    pub fn get_horizontal(&self) -> u8 {
        self.horizontal
    }
    pub fn get_vertical(&self) -> u8 {
        self.vertical
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
