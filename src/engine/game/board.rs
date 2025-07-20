use std::usize;

use crate::engine::game::{attack_table::AttackTables, bit_board::BitBoard, piece::Color};

pub struct Board<'a, const N: usize, const M: usize> {
    pub attack_table: &'a AttackTables<N, M>,
    pub bitboard: &'a BitBoard,
}

impl<'a, const N: usize, const M: usize> Board<'a, N, M> {
    pub fn new(attack_table: &'a AttackTables<N, M>, bitboard: &'a BitBoard) -> Self {
        return Self { attack_table, bitboard };
    }
    pub fn get_moves(&self, color: Color) -> Vec<BitBoard> {
        Vec::new()
    }
}
