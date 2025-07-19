use super::error::InvalidSquareError;
use log::error;
use std::fmt::Display;
use std::str::FromStr;
use std::usize;

pub const A1: Square = Square::new(Rank::One, File::A);
pub const A2: Square = Square::new(Rank::Two, File::A);
pub const A3: Square = Square::new(Rank::Three, File::A);
pub const A4: Square = Square::new(Rank::Four, File::A);
pub const A5: Square = Square::new(Rank::Five, File::A);
pub const A6: Square = Square::new(Rank::Six, File::A);
pub const A7: Square = Square::new(Rank::Seven, File::A);
pub const A8: Square = Square::new(Rank::Eight, File::A);
pub const B1: Square = Square::new(Rank::One, File::B);
pub const B2: Square = Square::new(Rank::Two, File::B);
pub const B3: Square = Square::new(Rank::Three, File::B);
pub const B4: Square = Square::new(Rank::Four, File::B);
pub const B5: Square = Square::new(Rank::Five, File::B);
pub const B6: Square = Square::new(Rank::Six, File::B);
pub const B7: Square = Square::new(Rank::Seven, File::B);
pub const B8: Square = Square::new(Rank::Eight, File::B);
pub const C1: Square = Square::new(Rank::One, File::C);
pub const C2: Square = Square::new(Rank::Two, File::C);
pub const C3: Square = Square::new(Rank::Three, File::C);
pub const C4: Square = Square::new(Rank::Four, File::C);
pub const C5: Square = Square::new(Rank::Five, File::C);
pub const C6: Square = Square::new(Rank::Six, File::C);
pub const C7: Square = Square::new(Rank::Seven, File::C);
pub const C8: Square = Square::new(Rank::Eight, File::C);
pub const D1: Square = Square::new(Rank::One, File::D);
pub const D2: Square = Square::new(Rank::Two, File::D);
pub const D3: Square = Square::new(Rank::Three, File::D);
pub const D4: Square = Square::new(Rank::Four, File::D);
pub const D5: Square = Square::new(Rank::Five, File::D);
pub const D6: Square = Square::new(Rank::Six, File::D);
pub const D7: Square = Square::new(Rank::Seven, File::D);
pub const D8: Square = Square::new(Rank::Eight, File::D);
pub const E1: Square = Square::new(Rank::One, File::E);
pub const E2: Square = Square::new(Rank::Two, File::E);
pub const E3: Square = Square::new(Rank::Three, File::E);
pub const E4: Square = Square::new(Rank::Four, File::E);
pub const E5: Square = Square::new(Rank::Five, File::E);
pub const E6: Square = Square::new(Rank::Six, File::E);
pub const E7: Square = Square::new(Rank::Seven, File::E);
pub const E8: Square = Square::new(Rank::Eight, File::E);
pub const F1: Square = Square::new(Rank::One, File::F);
pub const F2: Square = Square::new(Rank::Two, File::F);
pub const F3: Square = Square::new(Rank::Three, File::F);
pub const F4: Square = Square::new(Rank::Four, File::F);
pub const F5: Square = Square::new(Rank::Five, File::F);
pub const F6: Square = Square::new(Rank::Six, File::F);
pub const F7: Square = Square::new(Rank::Seven, File::F);
pub const F8: Square = Square::new(Rank::Eight, File::F);
pub const G1: Square = Square::new(Rank::One, File::G);
pub const G2: Square = Square::new(Rank::Two, File::G);
pub const G3: Square = Square::new(Rank::Three, File::G);
pub const G4: Square = Square::new(Rank::Four, File::G);
pub const G5: Square = Square::new(Rank::Five, File::G);
pub const G6: Square = Square::new(Rank::Six, File::G);
pub const G7: Square = Square::new(Rank::Seven, File::G);
pub const G8: Square = Square::new(Rank::Eight, File::G);
pub const H1: Square = Square::new(Rank::One, File::H);
pub const H2: Square = Square::new(Rank::Two, File::H);
pub const H3: Square = Square::new(Rank::Three, File::H);
pub const H4: Square = Square::new(Rank::Four, File::H);
pub const H5: Square = Square::new(Rank::Five, File::H);
pub const H6: Square = Square::new(Rank::Six, File::H);
pub const H7: Square = Square::new(Rank::Seven, File::H);
pub const H8: Square = Square::new(Rank::Eight, File::H);

#[derive(Debug, Clone, Copy)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl TryFrom<char> for File {
    type Error = InvalidSquareError;
    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            'a' => Ok(Self::A),
            'b' => Ok(Self::B),
            'c' => Ok(Self::C),
            'd' => Ok(Self::D),
            'e' => Ok(Self::E),
            'f' => Ok(Self::F),
            'g' => Ok(Self::G),
            'h' => Ok(Self::H),
            _ => Err(InvalidSquareError::InvalidLiteral { literal: s.to_string() }),
        }
    }
}

impl Iterator for File {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            File::A => Some(File::B),
            File::B => Some(File::C),
            File::C => Some(File::D),
            File::D => Some(File::E),
            File::E => Some(File::F),
            File::F => Some(File::G),
            File::G => Some(File::H),
            File::H => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Rank {
    One = 0,
    Two = 8,
    Three = 16,
    Four = 24,
    Five = 32,
    Six = 40,
    Seven = 48,
    Eight = 56,
}

impl Iterator for Rank {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Rank::One => Some(Rank::Two),
            Rank::Two => Some(Rank::Three),
            Rank::Three => Some(Rank::Four),
            Rank::Four => Some(Rank::Five),
            Rank::Five => Some(Rank::Six),
            Rank::Six => Some(Rank::Seven),
            Rank::Seven => Some(Rank::Eight),
            Rank::Eight => None,
        }
    }
}

impl TryFrom<char> for Rank {
    type Error = InvalidSquareError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            _ => Err(InvalidSquareError::InvalidLiteral { literal: value.to_string() }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Square {
    square: u8,
}

impl Square {
    pub const fn new(rank: Rank, file: File) -> Self {
        Self {
            square: rank as u8 + file as u8,
        }
    }
    pub const fn as_index(&self) -> usize {
        self.square as usize
    }

    pub const fn file(&self) -> u8 {
        self.square % 8
    }
    pub const fn rank(&self) -> u8 {
        self.square / 8
    }
}

impl TryFrom<u8> for Square {
    type Error = InvalidSquareError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= 64 {
            Err(InvalidSquareError::OutOfBounds { h: value % 8, v: value / 8 })
        } else {
            Ok(Self { square: value })
        }
    }
}
impl TryFrom<usize> for Square {
    type Error = InvalidSquareError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value >= 64 {
            Err(InvalidSquareError::OutOfBounds {
                h: (value % 8) as u8,
                v: (value / 8) as u8,
            })
        } else {
            Ok(Self { square: value as u8 })
        }
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
        let rank = Rank::try_from(value.chars().nth(1).unwrap())?;
        let file = File::try_from(value.chars().nth(0).unwrap())?;
        Ok(Self::new(rank, file))
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            match self.square % 8 {
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
            (self.square / 8) + 1
        )
    }
}
