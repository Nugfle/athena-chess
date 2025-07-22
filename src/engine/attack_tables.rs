use super::board::Occupancy;
use super::board::square::*;
use attack_magic::AttackMagic;
use move_logic::create_knight_attack_pattern;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::usize;

#[cfg(test)]
pub mod attack_magic;
#[cfg(not(test))]
mod attack_magic;

mod move_logic;

/// hold the attack tables for rook, bishop and knight, which are precomputed at engine startup.
/// During board evaluation, getting all possible moves for a piece is as simple as 2 pointer
/// lookups or ~200 ns
#[derive(Debug, Clone)]
pub struct AttackTables {
    pub rook_tables: [AttackMagic; 64],
    pub bishop_tables: [AttackMagic; 64],
    pub knight_table: [u64; 64],
}

impl AttackTables {
    pub fn create_tables() -> Self {
        let mut bishop_tables: [AttackMagic; 64] = core::array::from_fn(|_| AttackMagic::default());
        let mut rook_tables: [AttackMagic; 64] = core::array::from_fn(|_| AttackMagic::default());
        let mut knight_table: [u64; 64] = [0; 64];

        let bishop_pairs: Vec<(usize, AttackMagic)> = (0..64)
            .into_par_iter()
            .map(|i| (i, AttackMagic::create_attack_magic_bishop(Square(i as u8))))
            .collect();
        for (i, p) in bishop_pairs {
            bishop_tables[i] = p;
        }

        let rook_pairs: Vec<(usize, AttackMagic)> = (0..64)
            .into_par_iter()
            .map(|i| (i, AttackMagic::create_attack_magic_rook(Square(i as u8))))
            .collect();
        for (i, r) in rook_pairs {
            rook_tables[i] = r;
        }
        let knight_pairs: Vec<(usize, u64)> = (0..64)
            .into_par_iter()
            .map(|i| (i, create_knight_attack_pattern(Square(i as u8))))
            .collect();
        for (i, k) in knight_pairs {
            knight_table[i] = k;
        }

        Self {
            rook_tables,
            bishop_tables,
            knight_table,
        }
    }
    pub fn get_attack_pattern_rook(&self, square: Square, occupancy: Occupancy) -> u64 {
        let attack_magic = &self.rook_tables[square.0 as usize];
        attack_magic.attack_patterns[occupancy.hash(attack_magic.mask, attack_magic.magic_number, attack_magic.shift)]
    }
    pub fn get_attack_pattern_bishop(&self, square: Square, occupancy: Occupancy) -> u64 {
        let attack_magic = &self.bishop_tables[square.0 as usize];
        // we need to
        attack_magic.attack_patterns[occupancy.hash(attack_magic.mask, attack_magic.magic_number, attack_magic.shift)]
    }
    pub fn get_attack_pattern_knight(&self, square: Square, occupancy: Occupancy) -> u64 { self.knight_table[square.0 as usize] }
    pub fn get_attack_pattern_queen(&self, square: Square, occupancy: Occupancy) -> u64 {
        self.get_attack_pattern_rook(square, occupancy) | self.get_attack_pattern_bishop(square, occupancy)
    }
}
