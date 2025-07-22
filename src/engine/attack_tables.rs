use crate::engine::attack_tables::mask::BoardMask;

use super::board::Occupancy;
use super::board::square::*;
use attack_magic::AttackMagic;
use move_logic::create_knight_attack_pattern;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

mod attack_magic;
mod mask;
mod move_logic;

/// hold the attack tables for rook, bishop and knight, which are precomputed at engine startup.
/// During board evaluation, getting all possible moves for a piece is as simple as 2 pointer
/// lookups or ~200 ns
#[derive(Debug, Clone)]
pub struct AttackTables {
    pub rook_tables: [AttackMagic; 64],
    pub bishop_tables: [AttackMagic; 64],
    pub knight_table: [BoardMask; 64],
}

impl AttackTables {
    /// parralelized computes magic values and tables for sliding pieces as well as a simple Table for the
    /// knight
    pub fn create_tables() -> Self {
        // note the use of par_iter, so we can compute all 64 at the same time
        let mut bishop_vec: Vec<Option<AttackMagic>> = (0..64)
            .into_par_iter()
            .map(|i| Some(AttackMagic::create_attack_magic_bishop(Square::new(i).unwrap())))
            .collect();

        // as [_;64] can't be constructed from an Iterator, we manually move the elements over.
        // note that it also isn't possible to just assignt to a mutable array, because of the
        // async context.
        let bishop_tables: [AttackMagic; 64] = core::array::from_fn(|i| bishop_vec[i].take().unwrap());

        let mut rook_vec: Vec<Option<AttackMagic>> = (0..64)
            .into_par_iter()
            .map(|i| Some(AttackMagic::create_attack_magic_rook(Square::new(i).unwrap())))
            .collect();
        let rook_tables: [AttackMagic; 64] = core::array::from_fn(|i| rook_vec[i].take().unwrap());

        let mut knight_vec: Vec<Option<BoardMask>> = (0..64)
            .into_par_iter()
            .map(|i| Some(create_knight_attack_pattern(Square::new(i).unwrap())))
            .collect();
        let knight_table: [BoardMask; 64] = core::array::from_fn(|i| knight_vec[i].take().unwrap());

        Self {
            rook_tables,
            bishop_tables,
            knight_table,
        }
    }
    /// retrieves the pattern describing all attacked squares for a rook standing at square with
    /// the given occupancy of the board
    pub fn get_attack_pattern_rook(&self, square: Square, occupancy: Occupancy) -> BoardMask {
        let attack_magic = &self.rook_tables[square.as_index()];
        attack_magic.attack_patterns[occupancy.hash(attack_magic.mask, attack_magic.magic_number, attack_magic.shift)]
    }
    /// retrieves the pattern describing all attacked squares for a bishop standing at square with
    /// the given occupancy of the board
    pub fn get_attack_pattern_bishop(&self, square: Square, occupancy: Occupancy) -> BoardMask {
        let attack_magic = &self.bishop_tables[square.as_index()];
        // we need to
        attack_magic.attack_patterns[occupancy.hash(attack_magic.mask, attack_magic.magic_number, attack_magic.shift)]
    }
    /// retrieves the pattern describing all attacked squares for a Queen standing at square with
    /// the given occupancy of the board by adding the patterns of the Rook and bishop together
    pub fn get_attack_pattern_queen(&self, square: Square, occupancy: Occupancy) -> BoardMask {
        self.get_attack_pattern_rook(square, occupancy) | self.get_attack_pattern_bishop(square, occupancy)
    }
    /// retrieves the pattern describing all attacked squares for a knight standing at square.
    /// Note that the knight doesn't require an Occupancy as it is not a sliding piece.
    pub fn get_attack_pattern_knight(&self, square: Square) -> BoardMask { self.knight_table[square.as_index()] }
}
