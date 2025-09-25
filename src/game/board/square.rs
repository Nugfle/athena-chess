use std::fmt::Display;

use crate::game::error::ChessError;

pub const A1: Square = Square::from_rank_file(Rank::One, File::A);
pub const A2: Square = Square::from_rank_file(Rank::Two, File::A);
pub const A3: Square = Square::from_rank_file(Rank::Three, File::A);
pub const A4: Square = Square::from_rank_file(Rank::Four, File::A);
pub const A5: Square = Square::from_rank_file(Rank::Five, File::A);
pub const A6: Square = Square::from_rank_file(Rank::Six, File::A);
pub const A7: Square = Square::from_rank_file(Rank::Seven, File::A);
pub const A8: Square = Square::from_rank_file(Rank::Eight, File::A);
pub const B1: Square = Square::from_rank_file(Rank::One, File::B);
pub const B2: Square = Square::from_rank_file(Rank::Two, File::B);
pub const B3: Square = Square::from_rank_file(Rank::Three, File::B);
pub const B4: Square = Square::from_rank_file(Rank::Four, File::B);
pub const B5: Square = Square::from_rank_file(Rank::Five, File::B);
pub const B6: Square = Square::from_rank_file(Rank::Six, File::B);
pub const B7: Square = Square::from_rank_file(Rank::Seven, File::B);
pub const B8: Square = Square::from_rank_file(Rank::Eight, File::B);
pub const C1: Square = Square::from_rank_file(Rank::One, File::C);
pub const C2: Square = Square::from_rank_file(Rank::Two, File::C);
pub const C3: Square = Square::from_rank_file(Rank::Three, File::C);
pub const C4: Square = Square::from_rank_file(Rank::Four, File::C);
pub const C5: Square = Square::from_rank_file(Rank::Five, File::C);
pub const C6: Square = Square::from_rank_file(Rank::Six, File::C);
pub const C7: Square = Square::from_rank_file(Rank::Seven, File::C);
pub const C8: Square = Square::from_rank_file(Rank::Eight, File::C);
pub const D1: Square = Square::from_rank_file(Rank::One, File::D);
pub const D2: Square = Square::from_rank_file(Rank::Two, File::D);
pub const D3: Square = Square::from_rank_file(Rank::Three, File::D);
pub const D4: Square = Square::from_rank_file(Rank::Four, File::D);
pub const D5: Square = Square::from_rank_file(Rank::Five, File::D);
pub const D6: Square = Square::from_rank_file(Rank::Six, File::D);
pub const D7: Square = Square::from_rank_file(Rank::Seven, File::D);
pub const D8: Square = Square::from_rank_file(Rank::Eight, File::D);
pub const E1: Square = Square::from_rank_file(Rank::One, File::E);
pub const E2: Square = Square::from_rank_file(Rank::Two, File::E);
pub const E3: Square = Square::from_rank_file(Rank::Three, File::E);
pub const E4: Square = Square::from_rank_file(Rank::Four, File::E);
pub const E5: Square = Square::from_rank_file(Rank::Five, File::E);
pub const E6: Square = Square::from_rank_file(Rank::Six, File::E);
pub const E7: Square = Square::from_rank_file(Rank::Seven, File::E);
pub const E8: Square = Square::from_rank_file(Rank::Eight, File::E);
pub const F1: Square = Square::from_rank_file(Rank::One, File::F);
pub const F2: Square = Square::from_rank_file(Rank::Two, File::F);
pub const F3: Square = Square::from_rank_file(Rank::Three, File::F);
pub const F4: Square = Square::from_rank_file(Rank::Four, File::F);
pub const F5: Square = Square::from_rank_file(Rank::Five, File::F);
pub const F6: Square = Square::from_rank_file(Rank::Six, File::F);
pub const F7: Square = Square::from_rank_file(Rank::Seven, File::F);
pub const F8: Square = Square::from_rank_file(Rank::Eight, File::F);
pub const G1: Square = Square::from_rank_file(Rank::One, File::G);
pub const G2: Square = Square::from_rank_file(Rank::Two, File::G);
pub const G3: Square = Square::from_rank_file(Rank::Three, File::G);
pub const G4: Square = Square::from_rank_file(Rank::Four, File::G);
pub const G5: Square = Square::from_rank_file(Rank::Five, File::G);
pub const G6: Square = Square::from_rank_file(Rank::Six, File::G);
pub const G7: Square = Square::from_rank_file(Rank::Seven, File::G);
pub const G8: Square = Square::from_rank_file(Rank::Eight, File::G);
pub const H1: Square = Square::from_rank_file(Rank::One, File::H);
pub const H2: Square = Square::from_rank_file(Rank::Two, File::H);
pub const H3: Square = Square::from_rank_file(Rank::Three, File::H);
pub const H4: Square = Square::from_rank_file(Rank::Four, File::H);
pub const H5: Square = Square::from_rank_file(Rank::Five, File::H);
pub const H6: Square = Square::from_rank_file(Rank::Six, File::H);
pub const H7: Square = Square::from_rank_file(Rank::Seven, File::H);
pub const H8: Square = Square::from_rank_file(Rank::Eight, File::H);

/// an enum representing the files on a chess board used for save construction of squares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

/// an enum representing the ranks on a chess board used for save construction of squares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
}

/// represents a square on a chess board. Can be in Range from 0 to 63
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct Square(u8);

impl TryFrom<usize> for Square {
    type Error = ChessError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if 0 < value && value < 64 {
            Ok(Square(value as u8))
        } else {
            Err(ChessError::InvalidSquare { square: value as u8 })
        }
    }
}

impl Square {
    /// use of this function is highly discouraged, as it can easily lead to errors. Please use the
    /// from_rank_file method instead.
    ///```
    /// use athena_chess::game::*;
    /// assert_eq!(Square::new(9).unwrap(), B2);
    ///```
    pub fn new(s: u8) -> Result<Self, ChessError> {
        if s < 64 {
            Ok(Self(s))
        } else {
            Err(ChessError::InvalidSquare { square: s })
        }
    }

    ///```
    /// use athena_chess::game::*;
    /// assert_eq!(Square::from_rank_file(Rank::Four, File::A), A4);
    ///```
    pub const fn from_rank_file(rank: Rank, file: File) -> Self {
        Self(rank as u8 * 8 + file as u8)
    }

    ///```
    /// use athena_chess::game::*;
    /// assert_eq!(A1.as_index(), 0);
    /// assert_eq!(B2.as_index(), 9);
    /// assert_eq!(H8.as_index(), 63);
    ///```
    pub fn as_index(&self) -> usize {
        self.0.into()
    }

    ///```
    /// use athena_chess::game::*;
    /// assert_eq!(A1.as_u8(), 0);
    /// assert_eq!(B2.as_u8(), 9);
    /// assert_eq!(H8.as_u8(), 63);
    ///```
    pub fn as_u8(&self) -> u8 {
        self.0
    }

    ///```
    /// use athena_chess::game::*;
    /// assert_eq!(E4.get_rank(), Rank::Four);
    /// assert_eq!(A1.get_rank(), Rank::One);
    ///```
    pub fn get_rank(&self) -> Rank {
        match self.as_u8() / 8 {
            0 => Rank::One,
            1 => Rank::Two,
            2 => Rank::Three,
            3 => Rank::Four,
            4 => Rank::Five,
            5 => Rank::Six,
            6 => Rank::Seven,
            7 => Rank::Eight,
            _ => panic!("invalid square, is outside of board"),
        }
    }

    ///```
    /// use athena_chess::game::*;
    /// assert_eq!(E4.get_file(), File::E);
    /// assert_eq!(A1.get_file(), File::A);
    ///```
    pub fn get_file(&self) -> File {
        match self.as_u8() % 8 {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("invalid square, is outside of board"),
        }
    }

    /// moves the square by delta on the current rank
    ///```
    /// use athena_chess::game::*;
    /// assert_eq!(E4.move_on_rank(-2).unwrap(), C4);
    /// assert_eq!(A1.move_on_rank(1).unwrap(), B1);
    ///```
    pub fn move_on_rank(&self, delta: i8) -> Result<Self, ChessError> {
        let s = self.0 as i8 + delta;
        if !(0..64).contains(&s) {
            return Err(ChessError::InvalidSquare { square: s as u8 });
        }
        let n = Self::new(s as u8).unwrap();
        if n.get_rank() != self.get_rank() {
            return Err(ChessError::InvalidSquare { square: n.as_u8() });
        }
        Ok(n)
    }

    /// moves the square by delta on the current file
    ///```
    /// use athena_chess::game::*;
    /// assert_eq!(E4.move_on_file(-2).unwrap(), E2);
    /// assert_eq!(A1.move_on_file(1).unwrap(), A2);
    ///´´´
    pub fn move_on_file(&self, delta: i8) -> Result<Self, ChessError> {
        let s = self.0 as i8 + delta * 8;
        if !(0..64).contains(&s) {
            return Err(ChessError::InvalidSquare { square: s as u8 });
        }
        let n = Self::new(s as u8).unwrap();
        if n.get_file() != self.get_file() {
            return Err(ChessError::InvalidSquare { square: n.as_u8() });
        }
        Ok(n)
    }

    /// moves the square by delta on the current file
    ///```
    /// use athena_chess::game::*;
    /// assert_eq!(E4.get_delta_rank(E8), 4);
    /// assert_eq!(C8.get_delta_rank(C6), -2);
    /// assert_eq!(A3.get_delta_rank(G5), 2);
    ///´´´
    pub fn get_delta_rank(&self, other: Self) -> i8 {
        other.get_rank() as i8 - self.get_rank() as i8
    }

    /// moves the square by delta on the current file
    ///```
    /// use athena_chess::game::*;
    /// assert_eq!(A4.get_delta_file(E4), 4);
    /// assert_eq!(F8.get_delta_file(C8), -3);
    /// assert_eq!(A3.get_delta_file(G5), 6);
    ///´´´
    pub fn get_delta_file(&self, other: Self) -> i8 {
        other.get_file() as i8 - self.get_file() as i8
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            match self.0 % 8 {
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
            self.0 / 8 + 1
        )
    }
}
