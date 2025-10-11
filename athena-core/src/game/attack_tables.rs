//! Attack pattern generation and lookup using magic bitboards.
//!
//! This module implements magic bitboards for efficient move generation:
//! - Pre-calculated attack tables for all pieces and squares
//! - Parallel computation of magic numbers and attack patterns
//! - Persistent caching of attack tables between runs
//! - Efficient lookup using perfect hashing
//!
//! # Implementation Details
//!
//! The module uses the following techniques:
//! - Magic bitboards for sliding pieces (rooks and bishops)
//! - Pre-calculated patterns for knights
//! - Parallel computation using rayon
//! - Serialization of tables for persistence
//!
//! # Performance
//!
//! Attack pattern lookup is extremely fast:
//! - Two array lookups for sliding pieces (~200ns)
//! - Single array lookup for knights
//! - No runtime computation needed

use super::board::Occupancy;
use super::board::square::*;
use super::mask::BoardMask;
use attack_magic::AttackMagic;
use log::{info, warn};
use move_logic::create_knight_attack_pattern;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::env::current_dir;
use std::env::home_dir;
use std::fs;

mod attack_magic;
mod move_logic;

/// Pre-computed attack tables for efficient move generation.
///
/// This struct holds attack patterns for all pieces and squares:
/// - Rook attack patterns using magic bitboards
/// - Bishop attack patterns using magic bitboards
/// - Knight attack patterns (simple lookup table)
///
/// The tables are computed once at engine startup and optionally cached to disk.
/// During move generation, retrieving attack patterns requires only 1-2 array
/// lookups, making move generation extremely fast.
///
/// # Performance
///
/// - Sliding piece lookup: ~200ns (two array accesses)
/// - Knight lookup: ~100ns (single array access)
/// - No runtime computation required
///
/// # Examples
///
/// ```
/// use athena_core::game::AttackTables;
///
/// let tables = AttackTables::init();
/// let square = Square::E4;
/// let occupancy = board.get_occupancy();
///
/// // Get attack patterns
/// let rook_attacks = tables.get_attack_pattern_rook(square, occupancy);
/// let bishop_attacks = tables.get_attack_pattern_bishop(square, occupancy);
/// let knight_attacks = tables.get_attack_pattern_knight(square);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackTables {
    #[serde(with = "serde_arrays")]
    pub rook_tables: [AttackMagic; 64],
    #[serde(with = "serde_arrays")]
    pub bishop_tables: [AttackMagic; 64],
    #[serde(with = "serde_arrays")]
    pub knight_table: [BoardMask; 64],
}

impl AttackTables {
    pub fn init_table() -> Self {
        let base_dir = home_dir().unwrap_or(current_dir().unwrap());
        let file_path = base_dir
            .join(".config/athena-engine")
            .with_file_name("attack-tables.txt");

        if !fs::exists(&file_path).unwrap() {
            fs::create_dir_all(&file_path).unwrap();
        }

        fs::File::options()
            .create(true)
            .read(true)
            .write(true)
            .open(file_path)
            .and_then(|f| match serde_json::from_reader(&f) {
                Ok(t) => Ok(t),
                Err(e) => {
                    warn!("failed to deserialize attack tables: {}", e);
                    let t = Self::create_tables();
                    match serde_json::to_writer(f, &t) {
                        Err(e) => warn!("failed to store attack tables: {}", e),
                        Ok(_) => info!("saved attack tables to file"),
                    }
                    Ok(t)
                }
            })
            .unwrap_or_else(|e| {
                warn!("can't open attack table config file: {}", e);
                Self::create_tables()
            })
    }

    /// parralelized computes magic values and tables for sliding pieces as well as a simple Table for the
    /// knight
    fn create_tables() -> Self {
        // note the use of par_iter, so we can compute all 64 at the same time
        let mut bishop_vec: Vec<Option<AttackMagic>> = (0..64)
            .into_par_iter()
            .map(|i| {
                Some(AttackMagic::create_attack_magic_bishop(
                    Square::new(i).unwrap(),
                ))
            })
            .collect();

        // as [_;64] can't be constructed from an Iterator, we manually move the elements over.
        // note that it also isn't possible to just assignt to a mutable array, because of the
        // async context.
        let bishop_tables: [AttackMagic; 64] =
            core::array::from_fn(|i| bishop_vec[i].take().unwrap());

        let mut rook_vec: Vec<Option<AttackMagic>> = (0..64)
            .into_par_iter()
            .map(|i| {
                Some(AttackMagic::create_attack_magic_rook(
                    Square::new(i).unwrap(),
                ))
            })
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
        attack_magic.attack_patterns[occupancy.hash(
            attack_magic.mask,
            attack_magic.magic_number,
            attack_magic.shift,
        )]
    }
    /// retrieves the pattern describing all attacked squares for a bishop standing at square with
    /// the given occupancy of the board
    pub fn get_attack_pattern_bishop(&self, square: Square, occupancy: Occupancy) -> BoardMask {
        let attack_magic = &self.bishop_tables[square.as_index()];
        // we need to
        attack_magic.attack_patterns[occupancy.hash(
            attack_magic.mask,
            attack_magic.magic_number,
            attack_magic.shift,
        )]
    }
    /// retrieves the pattern describing all attacked squares for a Queen standing at square with
    /// the given occupancy of the board by adding the patterns of the Rook and bishop together
    pub fn get_attack_pattern_queen(&self, square: Square, occupancy: Occupancy) -> BoardMask {
        self.get_attack_pattern_rook(square, occupancy)
            | self.get_attack_pattern_bishop(square, occupancy)
    }
    /// retrieves the pattern describing all attacked squares for a knight standing at square.
    /// Note that the knight doesn't require an Occupancy as it is not a sliding piece.
    pub fn get_attack_pattern_knight(&self, square: Square) -> BoardMask {
        self.knight_table[square.as_index()]
    }
}
