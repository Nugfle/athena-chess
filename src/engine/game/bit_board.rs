use std::fmt::Display;

/// represents a square on a chess board. Can be in Range from 0 to 63
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Square(pub u8);
