use std::fmt::Display;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Square {
    pub fn new(s: u8) -> Option<Self> {
        if s < 64 { Some(Self(s)) } else { None }
    }
    pub const fn from_rank_file(rank: Rank, file: File) -> Self {
        Self(rank as u8 * 8 + file as u8)
    }
    pub fn as_index(&self) -> usize {
        self.0.into()
    }
    pub fn as_u8(&self) -> u8 {
        self.0
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
