use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use crate::game::board::square::Square;

/// a mask to overlay over a Occupancy
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardMask(pub u64);

impl BitOr<BoardMask> for BoardMask {
    type Output = BoardMask;
    fn bitor(self, rhs: BoardMask) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl BitAnd<BoardMask> for BoardMask {
    type Output = BoardMask;
    fn bitand(self, rhs: BoardMask) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl BitXor<BoardMask> for BoardMask {
    type Output = BoardMask;
    fn bitxor(self, rhs: BoardMask) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
impl BitOrAssign<BoardMask> for BoardMask {
    fn bitor_assign(&mut self, rhs: BoardMask) {
        self.0 |= rhs.0
    }
}
impl BitAndAssign<BoardMask> for BoardMask {
    fn bitand_assign(&mut self, rhs: BoardMask) {
        self.0 &= rhs.0
    }
}
impl BitXorAssign<BoardMask> for BoardMask {
    fn bitxor_assign(&mut self, rhs: BoardMask) {
        self.0 ^= rhs.0
    }
}
impl Not for BoardMask {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
impl Into<Vec<Square>> for BoardMask {
    fn into(self) -> Vec<Square> {
        let mut v = Vec::with_capacity(self.0.count_ones() as usize);
        for i in 0..64 {
            if self.0 & (1_u64 << i) != 0 {
                v.push(Square::new(i).unwrap());
            }
        }
        v
    }
}

impl Into<Vec<Square>> for &BoardMask {
    fn into(self) -> Vec<Square> {
        let mut v = Vec::with_capacity(self.0.count_ones() as usize);
        for i in 0..64 {
            if self.0 & (1_u64 << i) != 0 {
                v.push(Square::new(i).unwrap());
            }
        }
        v
    }
}

impl BoardMask {
    pub fn add_square(&mut self, square: Square) {
        self.0 |= 1_u64 << square.as_u8();
    }
    pub fn with_square(&self, square: Square) -> Self {
        Self(self.0 | 1_u64 << square.as_u8())
    }
    pub fn remove_square(&mut self, square: Square) {
        self.0 &= !(1_u64 << square.as_u8());
    }
    pub fn with_square_removed(&self, square: Square) -> Self {
        Self(self.0 & !(1_u64 << square.as_u8()))
    }
    pub fn contains(&self, square: Square) -> bool {
        self.0 & (1_u64 << square.as_u8()) != 0
    }
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }
    pub fn add_squares(&mut self, squares: impl IntoIterator<Item = Square>) {
        squares.into_iter().for_each(|sq| self.add_square(sq));
    }

    pub fn as_squares(&self) -> Vec<Square> {
        self.into()
    }
}
